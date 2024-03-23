use crate::Link;

use super::LinkText;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct WikiLinkString {
    /// Includes double brackets.
    pub text: String,
    /// Excludes double brackets.
    pub link_text: LinkText,
}

impl WikiLinkString {
    pub fn new(text: String) -> Self {
        let link_text = Link::extract_link_text(&text);
        WikiLinkString { text, link_text }
    }
}

impl FromStr for WikiLinkString {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("[[") && s.ends_with("]]") {
            Ok(WikiLinkString::new(s.to_string()))
        } else {
            Err(())
        }
    }
}
