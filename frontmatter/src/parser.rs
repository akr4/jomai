use anyhow::{anyhow, Result};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Frontmatter<'a> {
    pub title: Option<&'a str>,
    pub tags: Option<Vec<String>>,
}

pub struct ParseResult<'a> {
    pub frontmatter: Option<Frontmatter<'a>>,
    pub body: &'a str,
}

#[derive(Copy, Clone)]
enum Separator {
    Hyphen,
    Plus,
}

impl Separator {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Hyphen => "---",
            Self::Plus => "+++",
        }
    }
}

pub fn parse(s: &str) -> Result<ParseResult> {
    let (start, separator) = match find_start_of_frontmatter(s) {
        None => {
            return Ok(ParseResult {
                frontmatter: None,
                body: s,
            });
        }
        Some((start, separator)) => (start, separator),
    };

    // start - 1 to keep the line separator to search \n---\n
    let end = find_end_of_frontmatter(&s[(start - 1)..], separator);
    if end.is_none() {
        return Err(anyhow!("Could not find end of frontmatter"));
    }
    let end = start - 1 + end.unwrap();

    let frontmatter = &s[start..end];
    let body = &s[(end + separator.as_str().len() + 1)..];

    let frontmatter = match separator {
        Separator::Hyphen => serde_yaml::from_str(frontmatter)?,
        Separator::Plus => toml::from_str(frontmatter)?,
    };

    Ok(ParseResult {
        frontmatter: Some(frontmatter),
        body,
    })
}

fn find_start_of_frontmatter(s: &str) -> Option<(usize, Separator)> {
    if s.starts_with(&format!("{}\n", Separator::Hyphen.as_str())) {
        return Some((4, Separator::Hyphen));
    }
    if s.starts_with(&format!("{}\n", Separator::Plus.as_str())) {
        return Some((4, Separator::Plus));
    }
    None
}

fn find_end_of_frontmatter(s: &str, separator: Separator) -> Option<usize> {
    s.find(&format!("\n{}\n", separator.as_str())).map(|x| x + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yaml() {
        let result = parse(
            r#"---
title: aaa
---
body
"#,
        );
        let result = result.unwrap();
        let frontmatter = result.frontmatter.unwrap();
        assert_eq!(frontmatter.title, Some("aaa"));
        assert_eq!(result.body, "body\n");
    }

    #[test]
    fn toml() {
        let result = parse(
            r#"+++
title = "aaa"
+++
body
"#,
        );
        let result = result.unwrap();
        let frontmatter = result.frontmatter.unwrap();
        assert_eq!(frontmatter.title, Some("aaa"));
        assert_eq!(result.body, "body\n");
    }

    #[test]
    fn no_frontmatter() {
        let result = parse(
            r#"body
"#,
        );
        let result = result.unwrap();
        assert!(result.frontmatter.is_none());
    }
}
