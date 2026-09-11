use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

const HELP_MARKDOWN: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/help.md"));

pub fn extract_key_help_content(heading_name: &str) -> Option<String> {
    let parser = Parser::new(HELP_MARKDOWN);

    let mut title_h1_found = false;
    let mut in_h1 = false;
    let mut h1_title = String::new();
    let mut title_h2_found = false;
    let mut in_h2 = false;
    let mut h2_title = String::new();
    //let mut in_h2_content = false;
    let mut h2_content = String::new();

    // advance to the relevant part of the document
    for event in parser {
        match &event {
            Event::Start(Tag::Heading { level, .. }) => {
                if title_h2_found {
                    break;
                }

                if *level == HeadingLevel::H1 {
                    in_h1 = true;
                    h1_title.clear();
                } else if *level == HeadingLevel::H2 && title_h1_found {
                    in_h2 = true;
                    h2_title.clear();
                }
            },
            Event::Text(text) => {
                if in_h1 {
                    h1_title.push_str(text);
                } else if in_h2 {
                    h2_title.push_str(text);
                } else if title_h2_found {
                    //println!("INFO: text: {}", text);
                    h2_content.push_str(text);
                }
            },
            Event::End(TagEnd::Heading(HeadingLevel::H1)) => {
                if in_h1 {
                    in_h1 = false;
                    if h1_title.trim() == "Help For Each Specific Key" {
                        title_h1_found = true;
                    }
                }
            },
            Event::End(TagEnd::Heading(HeadingLevel::H2)) => {
                if in_h2 {
                    in_h2 = false;
                    if h2_title.trim() == heading_name {
                        title_h2_found = true;
/* kda_COMMENTED_OUT
                        in_h2_content = true;
  kda_COMMENTED_OUT */
                    }
                }
            },
            _ => {
                if title_h2_found {
                    // println!("INFO: anything: {:?}", event);
                    match &event {
                        Event::SoftBreak | Event::HardBreak => {
                            // println!("INFO: found break");
                            //h2_content.push('\n');
                        },
                        Event::Start(Tag::Paragraph) => {
                            // println!("INFO: paragraph begin");
                            if h2_content.len() > 0 {
                                h2_content.push_str("\n\n");
                            }
                        },
                        Event::Start(Tag::List(_ordered)) => {
                            // println!("INFO: list begin {:?}", ordered);
                            h2_content.push('\n');
                        }
                        Event::Start(Tag::Item) => {
                            // println!("INFO: item begin");
                            h2_content.push_str("- ");
                        }
                        Event::End(TagEnd::Item) => {
                            // println!("INFO: item end");
                            h2_content.push('\n');
                        }
                        _ => {
                            // println!("INFO: default capture {:?}", event);
                        },
                    }
                }
            },
        }
    }

    if title_h2_found {
        Some(h2_content)
    } else {
        None
    }
}


