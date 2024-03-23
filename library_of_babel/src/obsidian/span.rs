use std::ops::Range;

#[derive(Debug, Clone)]
/// A span of text in a document.
pub struct Span {
    pub range: Range<usize>,
    pub text: String,
}

impl Span {
    pub fn new(text: &str, range: Range<usize>) -> Span {
        Span {
            range,
            text: text.to_string(),
        }
    }

    pub fn shift_range(&mut self, cumulative_range_shift: i64) {
        let range = &mut self.range;
        let range_start: i64 = range
            .start
            .try_into()
            .expect("Error converting usize into i64.");
        let range_end: i64 = range
            .end
            .try_into()
            .expect("Error converting usize into i64.");

        let range_start_accounting_for_shift = range_start + cumulative_range_shift;
        let range_end_accounting_for_shift = range_end + cumulative_range_shift;
        let range_accounting_for_shift =
            (range_start_accounting_for_shift as usize)..(range_end_accounting_for_shift as usize);

        *range = range_accounting_for_shift;
    }

    pub fn update_text(&mut self, new_text: &str, page_contents: &mut String) -> i64 {
        page_contents.replace_range(self.range.clone(), new_text);

        let old_text_len = self.text.len();
        let new_text_len = new_text.len();

        let range = &mut self.range;
        let range_start = range.start;
        let range_end = range_start + new_text_len;
        let new_range = range_start..range_end;

        *range = new_range;

        (new_text_len as i64) - (old_text_len as i64)
    }
}
