use super::line::Line;
use super::metadata::{LineWas, Metadata, Valid};
use super::paragraph::Paragraph;
use super::parse_error::ParseError;
use super::raw_section::RawSection;
use super::schema::Schema;
use crate::obsidian::Vault;

#[derive(Debug)]
pub struct Section {
    pub paragraphs: Vec<Paragraph>,
}

impl Section {
    pub fn from_raw_section(
        vault: &Vault,
        schema: &Schema,
        raw_section: RawSection,
    ) -> Result<Self, ParseError> {
        let mut metadata = Metadata::new();
        let mut paragraph_lines = vec![];
        let mut paragraphs = vec![];

        let mut save_paragraph = |metadata: &Metadata, paragraph_lines: Vec<Line>| {
            if paragraph_lines.is_empty() {
                return;
            }

            let paragraph = Paragraph::new(metadata.clone(), paragraph_lines);
            paragraphs.push(paragraph);
        };

        for line in raw_section.lines {
            match metadata.try_add_field(vault, schema, &line) {
                LineWas::ValidMetadataField => continue,
                LineWas::Empty => {
                    save_paragraph(&metadata, paragraph_lines);
                    paragraph_lines = vec![];
                }
                LineWas::OtherText => match metadata.is_valid(schema) {
                    Valid::Yes => paragraph_lines.push(line),
                    Valid::No { missing_fields } => {
                        return Err(ParseError::missing_required_fields(line, missing_fields));
                    }
                },
            }
        }

        save_paragraph(&metadata, paragraph_lines);

        Ok(Section { paragraphs })
    }
}
