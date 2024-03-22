use super::line::Line;
use super::parse_error::ParseError;
use super::raw_section::RawSection;
use super::schema::Schema;
use super::section::Section;

#[derive(Debug)]
pub struct Document {
    pub schema: Schema,
    pub sections: Vec<Section>,
}

impl Document {
    pub fn from_str(document_text: String) -> Result<Self, ParseError> {
        let mut raw_sections = get_raw_sections(document_text)
            .into_iter()
            .filter(|raw_section| raw_section.is_a_leaflet_section());

        let schema_raw_section = raw_sections.next().ok_or(ParseError::NoLeafletSections)?;
        dbg!(&schema_raw_section);
        let schema = Schema::from_raw_section(&schema_raw_section)?;

        type Sections = Vec<Section>;
        let sections = raw_sections
            .map(|raw_section| Section::from_raw_section(&schema, raw_section))
            .collect::<Result<Sections, ParseError>>()?;

        Ok(Document { schema, sections })
    }
}

fn get_raw_sections(document_text: String) -> Vec<RawSection> {
    use crate::extensions::VecExtension;

    let numbered_lines: Vec<_> = document_text
        .lines()
        .enumerate()
        .map(|(line_number, line_str)| Line::new(line_number, line_str.to_string()))
        .collect();

    numbered_lines
        .split(|line| line.is_section_separator())
        .into_iter()
        .map(RawSection::new)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_raw_sections() {
        let document_text = r#"---
line 1
line 2
---
---
line 5"#
            .to_string();

        let raw_sections = get_raw_sections(document_text);

        assert_eq!(raw_sections.len(), 2);

        assert_eq!(raw_sections[0].text, "line 1\nline 2");
        assert_eq!(raw_sections[0].starting_line_number, 1);

        assert_eq!(raw_sections[1].text, "line 5");
        assert_eq!(raw_sections[1].starting_line_number, 5);
    }
}
