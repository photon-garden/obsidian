use super::match_references;
use super::{File, LinkTextStr, Reference, Span, VaultItemId};
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct ReferenceSpan {
    pub reference: Reference,
    pub span: Span,
}

impl ReferenceSpan {
    pub fn parse_reference_spans(string: &str, files: &[&File]) -> Vec<ReferenceSpan> {
        match_references
            .find_iter(string)
            .map(|current_match| {
                let matched_text = current_match.as_str();
                let matched_range = current_match.range();

                ReferenceSpan::new(matched_text, matched_range, files)
            })
            .collect()
    }

    fn new(matched_text: &str, matched_range: Range<usize>, files: &[&File]) -> ReferenceSpan {
        let reference = Reference::new(matched_text, files);
        let span = Span::new(matched_text, matched_range);

        ReferenceSpan { reference, span }
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
        &self.reference.link_text
    }

    pub fn refers_to(&self, target_id: &VaultItemId) -> bool {
        self.reference.refers_to(target_id)
    }
}
