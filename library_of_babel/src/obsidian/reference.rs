use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::Regex;

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
    /// `string` is often the contents of a page.
    /// `files` is necessary because we try and resolve which file the
    /// reference is pointing to.
    pub fn parse_references(string: &str, files: &[&File]) -> Vec<Reference> {
        lazy_static! {
            static ref match_references: Regex =
                Regex::new(r"!?\[\[.+?\]\]").expect("Error compiling regex.");
        }

        match_references
            .find_iter(string)
            .map(|current_match| {
                let matched_text = current_match.as_str();
                let matched_range = current_match.range();

                Reference::new(matched_text, matched_range, files)
            })
            .collect()
    }

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

    // This function expects that files has been sorted so that
    // oldest files are first.
    fn find_closest_matching_file<'f>(
        text_between_double_brackets: &str,
        files: &'f [&File],
    ) -> Option<&'f File> {
        let most_specific_to_least_specific = [
            |text_between_double_brackets: &str, file: &File| {
                file.path_from_vault_root == text_between_double_brackets
            },
            |text_between_double_brackets: &str, file: &File| {
                file.path_from_vault_root_without_extension == text_between_double_brackets
            },
            |text_between_double_brackets: &str, file: &File| {
                file.file_name == text_between_double_brackets
            },
            |text_between_double_brackets: &str, file: &File| {
                file.file_name_without_extension == text_between_double_brackets
            },
        ];

        for does_file_match in most_specific_to_least_specific {
            let maybe_match = files
                .iter()
                .find(|&&file| does_file_match(text_between_double_brackets, file));

            if let Some(matching_file) = maybe_match {
                return Some(matching_file);
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

    /// Tries to parse `self.text_between_double_brackets` as a YYYY.MM.DD date.
    pub fn try_as_date(&self) -> Option<NaiveDate> {
        let substrings: Vec<_> = self.text_between_double_brackets.split('.').collect();
        let [year_string, month_string, day_string] = if substrings.len() == 3 {
            [substrings[0], substrings[1], substrings[2]]
        } else {
            return None;
        };

        let year_number: i32 = year_string.parse().ok()?;
        let month_number: u8 = month_string.parse().ok()?;
        if !crate::date::is_valid_month(month_number) {
            return None;
        }
        let day_number: u8 = day_string.parse().ok()?;
        if !crate::date::is_valid_day_of_month(day_number) {
            return None;
        }

        NaiveDate::from_ymd_opt(year_number, month_number as u32, day_number as u32)
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
