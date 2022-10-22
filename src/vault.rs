use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::{File, Reference, VaultItem};

#[derive(Debug)]
pub struct Vault {
    vault_path: PathBuf,
    vault_items_by_id: HashMap<String, VaultItem>,
}

impl Vault {
    pub fn load_from_disk(vault_path: &'static str) -> Vault {
        let vault_items = crate::vault_items(vault_path);
        let mut vault_items_by_id: HashMap<String, VaultItem> =
            HashMap::with_capacity(vault_items.len());

        for item in vault_items {
            let id = item.id().to_string();
            vault_items_by_id.insert(id, item);
        }

        Vault {
            vault_path: vault_path.into(),
            vault_items_by_id,
        }
    }

    pub fn vault_item(&self, id: &str) -> Option<&VaultItem> {
        self.vault_items_by_id.get(id)
    }

    pub fn vault_item_mut(&mut self, id: &str) -> Option<&mut VaultItem> {
        self.vault_items_by_id.get_mut(id)
    }

    pub fn files(&self) -> Vec<&File> {
        self.vault_items_by_id
            .values()
            .map(|vault_item| vault_item.file())
            .collect()
    }

    pub fn referenced_vault_item(&self, reference: &Reference) -> Option<&VaultItem> {
        reference
            .vault_item_id
            .as_ref()
            .and_then(|id| self.vault_item(id))
    }

    pub fn vault_path(&self) -> &Path {
        &self.vault_path
    }
}
