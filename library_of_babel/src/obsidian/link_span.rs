use super::match_references;
use super::{File, Link, LinkTextStr, Span, VaultItemId};
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct LinkSpan {
    pub link: Link,
    pub span: Span,
}

impl LinkSpan {
    pub fn parse_reference_spans(string: &str, files: &[&File]) -> Vec<LinkSpan> {
        match_references
            .find_iter(string)
            .map(|current_match| {
                let matched_text = current_match.as_str();
                let matched_range = current_match.range();

                LinkSpan::new(matched_text, matched_range, files)
            })
            .collect()
    }

    fn new(matched_text: &str, matched_range: Range<usize>, files: &[&File]) -> LinkSpan {
        let link = Link::new(matched_text, files);
        let span = Span::new(matched_text, matched_range);

        LinkSpan { link, span }
    }

    pub fn shift_range(&mut self, cumulative_range_shift: i64) {
        self.span.shift_range(cumulative_range_shift);
    }

    pub fn update_text(&mut self, new_text: &str, page_contents: &mut String) -> i64 {
        self.span.update_text(new_text, page_contents)
    }

    pub fn range(&self) -> Range<usize> {
        self.span.range
    }

    pub fn link_text(&self) -> &LinkTextStr {
        &self.link.link_text
    }

    pub fn refers_to(&self, target_id: &VaultItemId) -> bool {
        self.link.refers_to(target_id)
    }
}
