use super::*;

#[derive(Debug, Clone)]
pub struct Page {
    pub id: VaultItemId,
    pub file: File,
    pub contents: String,
    pub reference_spans: Vec<ReferenceSpan>,
    pub tags: Vec<String>,
}

impl Page {
    pub fn parse(files: &[&File], file: File, contents: String) -> Page {
        let parsed_page_contents = parse_page_contents(&contents, files);
        Page {
            id: VaultItemId::from_file(&file),
            file,
            contents,
            reference_spans: parsed_page_contents.reference_spans,
            tags: parsed_page_contents.tags,
        }
    }

    pub fn find_and_replace_text_for_references<GetNewReferenceText>(
        &mut self,
        get_new_reference_text: GetNewReferenceText,
    ) where
        GetNewReferenceText: Fn(&ReferenceSpan) -> String,
    {
        let mut cumulative_range_shift: i64 = 0;
        for reference_span in &mut self.reference_spans {
            let new_text = get_new_reference_text(reference_span);
            reference_span.shift_range(cumulative_range_shift);
            let range_shift_for_this_reference =
                reference_span.update_text(&new_text, &mut self.contents);
            cumulative_range_shift += range_shift_for_this_reference;
        }
    }

    pub fn replace_reference_text(
        reference_span_to_update: &mut ReferenceSpan,
        new_text: String,
        reference_spans: &mut Vec<ReferenceSpan>,
        contents: &mut String,
    ) {
        let mut cumulative_range_shift: i64 = 0;
        for reference_span in reference_spans {
            reference_span.shift_range(cumulative_range_shift);
            if reference_span.range() == reference_span_to_update.range() {
                let range_shift_for_this_reference =
                    reference_span.update_text(&new_text, contents);
                cumulative_range_shift += range_shift_for_this_reference;
            }
        }
    }

    pub fn find_reference_by_link_text(&self, link_text: &LinkTextStr) -> Option<&ReferenceSpan> {
        self.reference_spans
            .iter()
            .find(|reference_span| reference_span.link_text() == link_text)
    }

    pub fn has_a_reference_to(&self, target_id: &VaultItemId) -> bool {
        self.reference_spans
            .iter()
            .any(|reference_span| reference_span.refers_to(target_id))
    }
}

impl<'a> TryFrom<&'a VaultItem> for &'a Page {
    type Error = ();

    fn try_from(vault_item: &'a VaultItem) -> Result<Self, Self::Error> {
        match vault_item {
            VaultItem::Page(page) => Ok(page),
            VaultItem::NonPage { .. } => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a mut VaultItem> for &'a mut Page {
    type Error = ();

    fn try_from(vault_item: &'a mut VaultItem) -> Result<Self, Self::Error> {
        match vault_item {
            VaultItem::Page(page) => Ok(page),
            VaultItem::NonPage { .. } => Err(()),
        }
    }
}

fn parse_page_contents(page_contents: &str, files: &[&File]) -> ParsedPageContents {
    let reference_spans = ReferenceSpan::parse_reference_spans(page_contents, files);
    let tags = Tag::parse_tags(page_contents);
    ParsedPageContents {
        reference_spans,
        tags,
    }
}

struct ParsedPageContents {
    reference_spans: Vec<ReferenceSpan>,
    tags: Vec<String>,
}
