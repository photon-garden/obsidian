use super::file::files_in_vault;
use super::file::{Contents, File};
use super::*;

#[derive(Debug, Clone)]
pub enum VaultItem {
    Page(Page),
    NonPage { id: VaultItemId, file: File },
}

pub fn vault_items(vault_path: &'static str) -> Vec<VaultItem> {
    let files = files_in_vault(vault_path).collect();
    parse_files(files)
}

fn parse_files(mut files: Vec<File>) -> Vec<VaultItem> {
    files.sort_unstable_by_key(|file| file.created_at);
    let files_slice: Vec<_> = files.iter().collect();

    files
        .iter()
        .map(|file| VaultItem::from_file(file, &files_slice))
        .collect()
}

impl VaultItem {
    pub fn from_file(file: &File, files: &[&File]) -> VaultItem {
        match &file.contents {
            Contents::Markdown { text } => {
                let page = Page::parse(files, file.clone(), text.clone());
                VaultItem::Page(page)
            }

            _ => VaultItem::NonPage {
                id: VaultItemId::from_file(&file),
                file: file.clone(),
            },
        }
    }

    pub fn id(&self) -> &VaultItemId {
        match self {
            VaultItem::Page(page) => &page.id,
            VaultItem::NonPage { id, .. } => id,
        }
    }

    pub fn file(&self) -> &File {
        match self {
            VaultItem::Page(page) => &page.file,
            VaultItem::NonPage { file, .. } => file,
        }
    }

    pub fn try_into_page(&self) -> Option<&Page> {
        self.try_into().ok()
    }

    pub fn try_into_page_mut(&mut self) -> Option<&mut Page> {
        self.try_into().ok()
    }

    pub fn is_image(&self) -> bool {
        self.file().is_image()
    }

    pub fn find_and_replace_text_for_references<GetNewReferenceText>(
        &mut self,
        get_new_reference_text: GetNewReferenceText,
    ) where
        GetNewReferenceText: Fn(&Reference) -> String,
    {
        if let VaultItem::Page(page) = self {
            page.find_and_replace_text_for_references(get_new_reference_text);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VaultItemId(String);

impl VaultItemId {
    pub fn from_path_from_vault_root(path_from_vault_root: String) -> VaultItemId {
        VaultItemId(path_from_vault_root)
    }

    pub fn from_file(file: &File) -> VaultItemId {
        VaultItemId(file.path_from_vault_root.clone())
    }

    pub fn path_from_vault_root(&self) -> &str {
        &self.0
    }
}

impl From<&str> for VaultItemId {
    fn from(value: &str) -> Self {
        VaultItemId(value.to_string())
    }
}
