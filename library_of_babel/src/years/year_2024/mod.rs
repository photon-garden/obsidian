use crate::leaflet;
use crate::obsidian::Vault;

pub fn test_leaflet() {
    println!("Testing Leaflet parser on 2024 haiku...");

    let vault = Vault::production_vault();

    let haiku_2024_note = vault
        .item_at_path("Haiku 2024.md")
        .expect("Couldn't find the Haiku 2024 note.")
        .try_into_page()
        .expect("Haiku 2024 note wasn't a page.");

    let note_contents = haiku_2024_note.contents.clone();
    dbg!(&note_contents);
    let leaflet_document = leaflet::Document::from_str(&vault, note_contents).unwrap();
    dbg!(&leaflet_document);
}
