use super::*;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Reference {
    pub is_embed: bool,
    pub range: Range<usize>,
    pub text_between_double_brackets: String,
    pub text: String,
    pub vault_item_id: Option<VaultItemId>,
}

impl Reference {
    pub fn new(matched_text: &str, range: std::ops::Range<usize>, files: &[&File]) -> Reference {
        let is_embed = matched_text.starts_with("![[");

        // This is the text between [[ and ]].
        let text_between_double_brackets = matched_text.replace(['!', '[', ']'], "");

        let vault_item_id =
            Reference::find_closest_matching_file(&text_between_double_brackets, files)
                .map(VaultItemId::from_file);

        Reference {
            is_embed,
            range,
            text: matched_text.to_string(),
            text_between_double_brackets,
            vault_item_id,
        }
    }

    fn ways_to_refer_to_file(file: &File) -> Vec<&str> {
        vec![
            &file.file_name,                              // richard feynman.md
            &file.file_name_without_extension,            // richard feynman
            &file.path_from_vault_root,                   // people/richard feynman.md
            &file.path_from_vault_root_without_extension, // people/richard feynman
        ]
    }

    // This function expects that files has been sorted so that
    // oldest files are first.
    fn find_closest_matching_file<'f>(
        text_between_double_brackets: &str,
        files: &'f [&File],
    ) -> Option<&'f File> {
        let potential_matches: Vec<_> = files
            .iter()
            .filter(|file| {
                Reference::ways_to_refer_to_file(file).contains(&text_between_double_brackets)
            })
            .collect();

        if potential_matches.is_empty() {
            return None;
        }

        // Test from most specific to least specific.
        for file in potential_matches.iter() {
            if text_between_double_brackets == file.path_from_vault_root {
                return Some(file);
            }
        }

        for file in potential_matches.iter() {
            if text_between_double_brackets == file.path_from_vault_root_without_extension {
                return Some(file);
            }
        }

        for file in potential_matches.iter() {
            if text_between_double_brackets == file.file_name {
                return Some(file);
            }
        }

        for file in potential_matches.iter() {
            if text_between_double_brackets == file.file_name_without_extension {
                return Some(file);
            }
        }

        None
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

    pub fn refers_to(&self, target_id: &VaultItemId) -> bool {
        match &self.vault_item_id {
            Some(id) => id == target_id,
            None => false,
        }
    }
}

// pub enum Reference {
//     FileExists {
//         is_embed: bool,
//         range: Range<usize>,
//         text_between_double_brackets: String,
//         text: String,
//         vault_item_id: String,
//     },
//     FileDoesntExistYet {
//         is_embed: bool,
//         range: Range<usize>,
//         text: String,
//         text_between_double_brackets: String,
//     },
// }
