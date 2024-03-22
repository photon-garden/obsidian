use crate::extensions::VecExtension;
use crate::obsidian::Vault;
use crate::File;
use crate::Reference;

pub fn convert_2023_haiku_to_csv() {
    println!("Converting 2023 haiku to CSV...");

    let vault = Vault::production_vault();
    let vault_files: Vec<_> = vault.files().collect();

    let haiku_2023_note = vault
        .item_at_path("creations/Haiku 2023.md")
        .expect("Couldn't find the Haiku 2023 note.")
        .try_into_page()
        .expect("Haiku 2023 note wasn't a page.");
    let lines: Vec<_> = haiku_2023_note.contents.lines().collect();

    let parsed_lines: Vec<_> = lines
        .split(|line| line.trim() == "---")
        .into_iter()
        .skip(1) // Everything before the first "---" is the intro to the doc and can be ignored.
        .flat_map(|lines_in_section| parse_lines(lines_in_section, &vault_files))
        .collect();

    let mut csv_writer =
        csv::Writer::from_path("src/years/year_2023/unprocessed_2023_haiku.csv").unwrap();

    for line in parsed_lines {
        csv_writer.serialize(line).unwrap();
    }

    csv_writer.flush().unwrap();
}

#[derive(serde::Serialize)]
struct ParsedLine {
    kind: EmbedOrPoem,
    /// If kind == Embed, this is the path to the embedded file.
    /// If kind == MaybePoem, this is the text of the poem.
    text: String,
    date: String,
}

#[derive(serde::Serialize)]
enum EmbedOrPoem {
    /// Usually a photo or video.
    Embed,
    /// Usually a poem.
    MaybePoem,
}

fn parse_lines(lines_in_section: Vec<&str>, vault_files: &[&File]) -> Vec<ParsedLine> {
    let mut lines_with_letters_or_numbers = lines_in_section
        .into_iter()
        .filter(contains_letters_or_numbers);
    let first_line_with_letters_or_numbers = lines_with_letters_or_numbers
        .next()
        .expect("Expected each section to have at least one line with letters.");
    let references_on_first_line =
        Reference::parse_references(first_line_with_letters_or_numbers, vault_files);
    if references_on_first_line.len() != 1 {
        panic!("Expected only a single reference on the first line.");
    }

    let date = references_on_first_line
        .first()
        .unwrap()
        .try_as_date()
        .unwrap();

    lines_with_letters_or_numbers
        .map(|line| {
            let references = Reference::parse_references(line, vault_files);
            let (text, kind) = if references.len() == 1 && references.first().unwrap().is_embed {
                let reference = references.into_iter().next().unwrap();
                let vault_item_id = reference.vault_item_id.unwrap();
                let path_to_embedded_file = vault_item_id.path_from_vault_root();
                let path_string = path_to_embedded_file.to_string();
                (path_string, EmbedOrPoem::Embed)
            } else {
                let line_string = line.to_string();
                (line_string, EmbedOrPoem::MaybePoem)
            };

            ParsedLine {
                date: date.to_string(),
                text,
                kind,
            }
        })
        .collect()
}

fn contains_letters_or_numbers(string: &&str) -> bool {
    string.chars().any(|char| char.is_alphanumeric())
}
