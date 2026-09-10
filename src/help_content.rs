use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};

const HELP_MARKDOWN: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/help.md"));

pub fn extract_key_help_content(heading_name: &str) -> Option<String> {
    let parser = Parser::new(HELP_MARKDOWN);

    // advance to the relevant part of the document
    for event in parser {
        match &event {
            Event::Start(Tag::Heading { level, .. }) => {
                if *level == HeadingLevel::H2 {
                }
            },
            _ => {
            },
        }
    }
    
   
    None
}


