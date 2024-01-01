use super::*;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Page {
    pub id: VaultItemId,
    pub file: File,
    pub contents: String,
    pub references: Vec<Reference>,
    pub tags: Vec<String>,
}

impl Page {
    pub fn parse(files: &[&File], file: File, contents: String) -> Page {
        let parsed_page_contents = parse_page_contents(&contents, files);
        Page {
            id: VaultItemId::from_file(&file),
            file,
            contents,
            references: parsed_page_contents.references,
            tags: parsed_page_contents.tags,
        }
    }

    pub fn find_and_replace_text_for_references<GetNewReferenceText>(
        &mut self,
        get_new_reference_text: GetNewReferenceText,
    ) where
        GetNewReferenceText: Fn(&Reference) -> String,
    {
        let mut cumulative_range_shift: i64 = 0;
        for reference in &mut self.references {
            let new_text = get_new_reference_text(reference);
            reference.shift_range(cumulative_range_shift);
            let range_shift_for_this_reference =
                reference.update_text(&new_text, &mut self.contents);
            cumulative_range_shift += range_shift_for_this_reference;
        }
    }

    pub fn replace_reference_text(
        reference_to_update: &mut Reference,
        new_text: String,
        references: &mut Vec<Reference>,
        contents: &mut String,
    ) {
        let mut cumulative_range_shift: i64 = 0;
        for reference in references {
            reference.shift_range(cumulative_range_shift);
            if reference.range == reference_to_update.range {
                let range_shift_for_this_reference = reference.update_text(&new_text, contents);
                cumulative_range_shift += range_shift_for_this_reference;
            }
        }
    }

    pub fn has_a_reference_to(&self, target_id: &VaultItemId) -> bool {
        self.references
            .iter()
            .find(|reference| reference.refers_to(target_id))
            .is_some()
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
    let references = parse_references(page_contents, files);
    let tags = parse_tags(page_contents);
    ParsedPageContents { references, tags }
}

fn parse_references(page_contents: &str, files: &[&File]) -> Vec<Reference> {
    lazy_static! {
        static ref match_references: Regex =
            Regex::new(r"!?\[\[.+?\]\]").expect("Error compiling regex.");
    }

    match_references
        .find_iter(page_contents)
        .map(|current_match| {
            let matched_text = current_match.as_str();
            let matched_range = current_match.range();

            Reference::new(matched_text, matched_range, files)
        })
        .collect()
}

fn parse_tags(page_contents: &str) -> Vec<String> {
    lazy_static! {
        static ref match_tags: Regex = Regex::new(r"#(\S+)").expect("Error compiling regex.");
    }

    match_tags
        .captures_iter(page_contents)
        // capture[0] gives us the entire match, but we want the first
        // capture group, which is why we do capture[1]
        .map(|capture| capture[1].to_string())
        .collect()
}

struct ParsedPageContents {
    references: Vec<Reference>,
    tags: Vec<String>,
}
