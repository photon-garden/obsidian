#![allow(non_upper_case_globals)]

mod file;
mod vault;
mod vault_item;

use file::files_in_vault;
use vault_item::parse_files;

pub use file::File;
pub use vault::Vault;
pub use vault_item::{Reference, VaultItem};

pub fn vault_items(vault_path: &'static str) -> Vec<VaultItem> {
    let files = files_in_vault(vault_path).collect();
    parse_files(files)
}
