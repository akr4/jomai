use anyhow::Result;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};

pub fn infer_title(contents: &str) -> Result<Option<String>> {
    let mut parser = Parser::new(contents);
    let heading_start = parser.find(|event| {
        if let Event::Start(Tag::Heading(level, _, _)) = event {
            *level == HeadingLevel::H1
        } else {
            false
        }
    });

    if heading_start.is_none() {
        return Ok(None);
    }

    let text = parser.next();
    let heading_end = parser.next();

    match (text, heading_end) {
        (Some(Event::Text(text)), Some(Event::End(Tag::Heading(_, _, _)))) => Ok(Some(text.into_string())),
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::infer_title;

    #[test]
    fn test() {
        let markdown = r#"# title
aaa
"#;
        let title = infer_title(markdown).unwrap();
        assert_eq!(title, Some("title".to_string()));
    }

    #[test]
    fn no_h1() {
        let markdown = r#"## h2
aaa
"#;
        let title = infer_title(markdown).unwrap();
        assert_eq!(title, None);
    }
}
