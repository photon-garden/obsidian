use super::*;
use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Link {
    pub is_embed: bool,
    pub link_text: LinkText,
    pub text: String,
    pub vault_item_id: Option<VaultItemId>,
}

/// Link text is the text between double brackets in a wikilink.
/// In this example, the link text is "Richard Feynman":
///
/// ```
/// [[Richard Feynman]]
/// ```
pub type LinkText = String;

/// Link text is the text between double brackets in a wikilink.
/// In this example, the link text is "Richard Feynman":
///
/// ```
/// [[Richard Feynman]]
/// ```
pub type LinkTextStr = str;

/// A wikilink is a string formatted like this: `[[Richard Feynman]]`.
pub type WikilinkStr = str;

impl Link {
    /// `string` is often the contents of a page.
    /// `files` is necessary because we try and resolve which file the
    /// reference is pointing to.
    pub fn parse_references(string: &str, files: &[&File]) -> Vec<Link> {
        lazy_static! {
            static ref match_references: Regex =
                Regex::new(r"!?\[\[.+?\]\]").expect("Error compiling regex.");
        }

        match_references
            .find_iter(string)
            .map(|current_match| {
                let matched_text = current_match.as_str();
                Link::new(matched_text, files)
            })
            .collect()
    }

    pub fn new(matched_text: &str, files: &[&File]) -> Link {
        let is_embed = matched_text.starts_with("![[");

        // This is the text between [[ and ]].
        let link_text: LinkText = matched_text.replace(['!', '[', ']'], "");

        let vault_item_id = Link::find_vault_item_id(&link_text, files);

        Link {
            is_embed,
            text: matched_text.to_string(),
            link_text,
            vault_item_id,
        }
    }

    pub fn extract_link_text_from_wikilink(wikilink: &str) -> LinkText {
        wikilink.replace(['!', '[', ']'], "")
    }

    /// This function expects that files has been sorted so that
    /// oldest files are first.
    ///
    /// We need a list of files because we need to resolve the reference
    /// to a specific file.
    pub fn find_vault_item_id<'f>(
        link_text: &LinkTextStr,
        files: &'f [&File],
    ) -> Option<VaultItemId> {
        Link::find_closest_matching_file(&link_text, files).map(VaultItemId::from_file)
    }

    /// This function expects that files has been sorted so that
    /// oldest files are first.
    fn find_closest_matching_file<'f>(
        link_text: &LinkTextStr,
        files: &'f [&File],
    ) -> Option<&'f File> {
        let most_specific_to_least_specific = [
            |link_text: &LinkTextStr, file: &File| file.path_from_vault_root == link_text,
            |link_text: &LinkTextStr, file: &File| {
                file.path_from_vault_root_without_extension == link_text
            },
            |link_text: &LinkTextStr, file: &File| file.file_name == link_text,
            |link_text: &LinkTextStr, file: &File| file.file_name_without_extension == link_text,
        ];

        for does_file_match in most_specific_to_least_specific {
            let maybe_match = files.iter().find(|&&file| does_file_match(link_text, file));

            if let Some(matching_file) = maybe_match {
                return Some(matching_file);
            }
        }

        None
    }

    pub fn refers_to(&self, target_id: &VaultItemId) -> bool {
        match &self.vault_item_id {
            Some(id) => id == target_id,
            None => false,
        }
    }

    /// Tries to parse `self.link_text` as a YYYY.MM.DD date.
    pub fn try_as_date(&self) -> Option<NaiveDate> {
        let substrings: Vec<_> = self.link_text.split('.').collect();
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

lazy_static! {
    pub static ref match_references: Regex =
        Regex::new(r"!?\[\[.+?\]\]").expect("Error compiling regex.");
}
