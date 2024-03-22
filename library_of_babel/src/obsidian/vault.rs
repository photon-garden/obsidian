use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::{vault_items, File, Page, Reference, VaultItem, VaultItemId};

#[derive(Debug)]
pub struct Vault {
    path: PathBuf,
    items_by_id: HashMap<VaultItemId, VaultItem>,
}

impl Vault {
    pub fn production_vault() -> Vault {
        Vault::load_from_disk("/Users/photon-garden/library-of-babel")
    }

    pub fn development_vault() -> Vault {
        Vault::load_from_disk("/Users/photon-garden/obsidian-dev")
    }

    fn load_from_disk(vault_path: &'static str) -> Vault {
        let items = vault_items(vault_path);
        let mut items_by_id: HashMap<VaultItemId, VaultItem> = HashMap::with_capacity(items.len());

        for item in items {
            let id = item.id().clone();
            items_by_id.insert(id, item);
        }

        Vault {
            path: vault_path.into(),
            items_by_id,
        }
    }

    pub fn item_at_path(&self, path_from_vault_root: &str) -> Option<&VaultItem> {
        let id = VaultItemId::from_path_from_vault_root(path_from_vault_root.to_string());
        self.item(&id)
    }

    pub fn item(&self, id: &VaultItemId) -> Option<&VaultItem> {
        self.items_by_id.get(id)
    }

    pub fn item_mut(&mut self, id: &VaultItemId) -> Option<&mut VaultItem> {
        self.items_by_id.get_mut(id)
    }

    pub fn items(&self) -> impl Iterator<Item = &VaultItem> {
        self.items_by_id.values()
    }

    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut VaultItem> {
        self.items_by_id.values_mut()
    }

    pub fn files(&self) -> impl Iterator<Item = &File> {
        self.items().map(|vault_item| vault_item.file())
    }

    pub fn pages(&self) -> impl Iterator<Item = &Page> {
        self.items().filter_map(|item| item.try_into_page())
    }

    pub fn pages_mut(&mut self) -> impl Iterator<Item = &mut Page> {
        self.items_mut().filter_map(|item| item.try_into_page_mut())
    }

    pub fn referenced_item(&self, reference: &Reference) -> Option<&VaultItem> {
        reference
            .vault_item_id
            .as_ref()
            .and_then(|id| self.item(id))
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn find_or_create_page<GetNewPageContents>(
        &mut self,
        id: VaultItemId,
        get_new_page_contents: GetNewPageContents,
    ) -> &VaultItem
    where
        GetNewPageContents: FnOnce() -> String,
    {
        if self.item(&id).is_none() {
            let contents = get_new_page_contents();
            let new_page = self.create_page(&id, contents);
            self.items_by_id.insert(id.clone(), new_page);
        }

        self.item(&id).unwrap()
    }

    pub fn absolute_path_to_item(&self, id: &VaultItemId) -> PathBuf {
        let path_from_vault_root = id.path_from_vault_root();
        Path::new(&self.path).join(path_from_vault_root)
    }

    pub fn path_str(&self) -> &str {
        self.path
            .to_str()
            .expect("Error converting vault path to str.")
    }

    pub fn create_page(&self, id: &VaultItemId, contents: String) -> VaultItem {
        let absolute_path_to_new_page = self.absolute_path_to_item(id);
        let vault_path_str = &self.path_str();
        let file = File::create(vault_path_str, absolute_path_to_new_page, contents);
        VaultItem::from_file(&file, &self.file_vec())
    }

    fn file_vec(&self) -> Vec<&File> {
        self.items_by_id.values().map(|item| item.file()).collect()
    }

    // fn create_page(&mut self, id: VaultItemId) -> Page {
    // let page =
    //     self.items_by_id.insert(id, v)
    // }
}
