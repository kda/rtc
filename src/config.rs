use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;


#[derive(Debug, Deserialize)]
pub struct Constant {
    name: String,
    value: f64,
}

#[derive(Default, Debug, Deserialize)]
pub struct Config {
    constants: Option<HashMap<String, Constant>>,
}

impl Config {
    pub fn load(opt_filename: Option<String>) -> Self {
        let filename = if let Some(passed_filename) = opt_filename {
            passed_filename
        } else {
            if let Some(default_filename) = Config::find_file() {
                default_filename
            } else {
                return Self::default();
            }
        };

        let Ok(file) = File::open(&filename) else {
            panic!("ERROR: unable to open config file {filename}");
        };
        let reader = BufReader::new(file);
        return Self::parse(reader);
    }

    fn parse<BR: BufRead>(br: BR) -> Self {
        match yaml_serde::from_reader(br) {
            Ok(config) => config,
            e => { panic!("ERROR: unable to parse config {:?}", e); },
        }
    }

    const DEFAULT_FILENAME: &str = ".rtc.yaml";

    fn find_file() -> Option<String> {
        // First: find it in the current working directory
        if let Ok(mut cwd) = env::current_dir() {
            cwd.push(Self::DEFAULT_FILENAME);
            if cwd.is_file() {
                return cwd.into_os_string().into_string().ok();
            }
        }

        // Second: find it in home directory
        if let Some(mut home) = env::home_dir() {
            home.push(Self::DEFAULT_FILENAME);
            if home.is_file() {
                return home.into_os_string().into_string().ok();
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
		use std::fs;

    // Fails when there is config present (which will happen)
    #[test]
    #[ignore]
    fn test_find_file_nonexistent() {
        assert!(Config::find_file().is_none());
		}

    // Corrupts existing config
    #[test]
    #[ignore]
    fn test_find_file_in_cwd() {
        fs::File::create(Config::DEFAULT_FILENAME).expect("Failed to create temporary test file");
        let result = Config::find_file();
        let _ = fs::remove_file(Config::DEFAULT_FILENAME);
        assert!(result.is_some(), "Expected to find the config file in CWD, but got None");
        let path_str = result.unwrap();
        assert!(path_str.ends_with(Config::DEFAULT_FILENAME), "Path {path_str} should point to {}", Config::DEFAULT_FILENAME);
    }

    // Corrupts existing config
    #[test]
    #[ignore]
    fn test_find_file_in_home() {
        let Some(mut home) = env::home_dir() else {
            panic!("ERROR: unable to get home directory");
        };
        home.push(Config::DEFAULT_FILENAME);
        fs::File::create(&home).expect("Failed to create temporary test file");
        let result = Config::find_file();
        let _ = fs::remove_file(&home);
        assert!(result.is_some(), "Expected to find the config file in HOME, but got None");
        let path_str = result.unwrap();
        assert!(path_str.ends_with(Config::DEFAULT_FILENAME), "Path {path_str} should point to {}", Config::DEFAULT_FILENAME);
    }

    #[test]
    fn test_empty() {
        let config_content = r#"
        "#;
        let reader = BufReader::new(config_content.as_bytes());
        Config::parse(reader);
    }

    #[test]
    #[should_panic(expected = "ERROR: unable to parse config Err(Error(\"invalid type: string \\\"junk\\\", expected struct Config\", line: 2, column: 13))")]
    fn test_malformed() {
        let config_content = r#"
            junk
        "#;
        let reader = BufReader::new(config_content.as_bytes());
        Config::parse(reader);
    }

    #[test]
    fn test_well_formed() {
        let config_content = r#"
            constants:
                k:
                    name: kmh / mph
                    value: 0.621
                test:
                    name: testing
                    value: 1234567890
                    "#;
        let reader = BufReader::new(config_content.as_bytes());
        let config = Config::parse(reader);
        let Some(constants) = config.constants else {
            panic!("ERROR: did not find 'constants' in config");
        };
        let Some(ce) = constants.get("k") else {
            panic!("ERROR: 'constants' did not contain key 'k'");
        };
        assert!(ce.name == "kmh / mph");
        assert!(ce.value == 0.621);
        assert!(constants.contains_key("test"));
    }
}
