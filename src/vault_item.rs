use super::file::{Contents, File};
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Range;

pub fn parse_files(mut files: Vec<File>) -> Vec<VaultItem> {
    files.sort_unstable_by_key(|file| file.created_at);
    files
        .iter()
        .map(|file| VaultItem::from_file(file, &files))
        .collect()
}

#[derive(Debug, Clone)]
pub enum VaultItem {
    Page {
        file: File,
        contents: String,
        references: Vec<Reference>,
        tags: Vec<String>,
    },
    Attachment {
        file: File,
    },
}

impl VaultItem {
    pub fn from_file(file: &File, files: &[File]) -> VaultItem {
        match &file.contents {
            Contents::Markdown { text } => {
                let parsed_page_contents = parse_page_contents(text, files);
                VaultItem::Page {
                    file: file.clone(),
                    contents: text.clone(),
                    references: parsed_page_contents.references,
                    tags: parsed_page_contents.tags,
                }
            }
            _ => VaultItem::Attachment { file: file.clone() },
        }
    }

    pub fn id(&self) -> &str {
        &self.file().path_from_vault_root
    }

    pub fn file(&self) -> &File {
        match self {
            VaultItem::Page {
                file,
                contents: _contents,
                references: _references,
                tags: _tags,
            } => file,
            VaultItem::Attachment { file } => file,
        }
    }

    // pub fn try_to_page(&self) -> Result<Page, ()> {
    //     self.try_into()
    // }

    pub fn to_page(&self) -> Page {
        self.try_into()
            .expect("Tried to convert an attachment into a page.")
    }

    pub fn find_and_replace_text_for_references<GetNewReferenceText>(
        &mut self,
        get_new_reference_text: GetNewReferenceText,
    ) where
        GetNewReferenceText: Fn(&Reference) -> String,
    {
        if let VaultItem::Page {
            file: _file,
            contents,
            references,
            tags: _tags,
        } = self
        {
            let mut cumulative_range_shift: i64 = 0;
            for reference in references {
                let new_text = get_new_reference_text(reference);
                reference.shift_range(cumulative_range_shift);
                let range_shift_for_this_reference = reference.update_text(&new_text, contents);
                cumulative_range_shift += range_shift_for_this_reference;
            }
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

    pub fn is_image(&self) -> bool {
        self.file().is_image()
    }
}

fn parse_page_contents(page_contents: &str, files: &[File]) -> ParsedPageContents {
    let references = parse_references(page_contents, files);
    let tags = parse_tags(page_contents);
    ParsedPageContents { references, tags }
}

fn parse_references(page_contents: &str, files: &[File]) -> Vec<Reference> {
    lazy_static! {
        static ref match_references: Regex =
            Regex::new(r"!?\[\[.+?\]\]").expect("Error compiling regex.");
    }

    match_references
        .find_iter(page_contents)
        // capture[0] gives us the entire match, but we want the first
        // capture group, which is why we do capture[1]
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

#[derive(Debug, Clone)]
pub struct Reference {
    pub is_embed: bool,
    pub range: Range<usize>,
    pub text_between_double_brackets: String,
    pub text: String,
    pub vault_item_id: Option<String>,
}

impl Reference {
    fn new(matched_text: &str, range: std::ops::Range<usize>, files: &[File]) -> Reference {
        let is_embed = matched_text.starts_with("![[");

        // This is the text between [[ and ]].
        let text_between_double_brackets = matched_text.replace(['!', '[', ']'], "");

        let potential_matches: Vec<_> = files
            .iter()
            .filter(|file| {
                Reference::ways_to_refer_to_file(file)
                    .contains(&text_between_double_brackets.as_str())
            })
            .collect();

        if potential_matches.is_empty() {
            return Reference {
                is_embed,
                range,
                text: matched_text.to_string(),
                text_between_double_brackets,
                vault_item_id: None,
            };
        }

        let file = Reference::find_closest_matching_file(
            &text_between_double_brackets,
            &potential_matches,
        );

        Reference {
            is_embed,
            range,
            text: matched_text.to_string(),
            text_between_double_brackets,
            vault_item_id: Some(file.path_from_vault_root.clone()),
        }
    }

    fn ways_to_refer_to_file(file: &File) -> Vec<&str> {
        vec![
            &file.stem,                                   // richard feynman
            &file.file_name,                              // richard feynman.md
            &file.path_from_vault_root_without_extension, // people/richard feynman
            &file.path_from_vault_root,                   // people/richard feynman.md
        ]
    }

    // This function expects that files has been sorted so that
    // oldest files are first.
    fn find_closest_matching_file<'f>(
        text_between_double_brackets: &str,
        files: &'f [&File],
    ) -> &'f File {
        // Test from most specific to least specific.

        for file in files.iter() {
            if text_between_double_brackets == file.path_from_vault_root {
                return file;
            }
        }

        for file in files.iter() {
            if text_between_double_brackets == file.path_from_vault_root_without_extension {
                return file;
            }
        }

        for file in files.iter() {
            if text_between_double_brackets == file.file_name {
                return file;
            }
        }

        for file in files.iter() {
            if text_between_double_brackets == file.stem {
                return file;
            }
        }

        panic!("Couldn't find a matching file.")
    }

    fn shift_range(&mut self, cumulative_range_shift: i64) {
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

    fn update_text(&mut self, new_text: &str, page_contents: &mut String) -> i64 {
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

pub struct Page<'a> {
    pub file: &'a File,
    pub contents: &'a String,
    pub references: &'a Vec<Reference>,
    pub tags: &'a Vec<String>,
}

impl<'a> TryFrom<&'a VaultItem> for Page<'a> {
    type Error = ();

    fn try_from(vault_item: &'a VaultItem) -> Result<Self, Self::Error> {
        match vault_item {
            VaultItem::Page {
                file,
                contents,
                references,
                tags,
            } => Ok(Page {
                file,
                contents,
                references,
                tags,
            }),
            VaultItem::Attachment { file: _file } => Err(()),
        }
    }
}
