use super::{line::Line, schema::FieldDefinition};

#[derive(Debug)]
pub enum ParseError {
    /// The whole document was made up of section separators ("---") and skipped sections ("this isn't leaflet").
    NoLeafletSections,
    FirstSectionIsNotSchema,
    FieldDefinitionMissingName {
        line: Line,
    },
    FieldDefinitionMissingType {
        line: Line,
    },
    UnexpectedFieldType {
        line: Line,
        type_text: String,
    },
    MissingRequiredFields {
        line: Line,
        missing_fields: Vec<FieldDefinition>,
    },
}

impl ParseError {
    pub fn field_definition_missing_name(line: &Line) -> Self {
        ParseError::FieldDefinitionMissingName { line: line.clone() }
    }

    pub fn field_definition_missing_type(line: &Line) -> Self {
        ParseError::FieldDefinitionMissingType { line: line.clone() }
    }

    pub fn unexpected_field_type(line: &Line, type_text: &str) -> Self {
        ParseError::UnexpectedFieldType {
            line: line.clone(),
            type_text: type_text.to_string(),
        }
    }

    pub fn missing_required_fields(line: Line, missing_fields: Vec<FieldDefinition>) -> Self {
        ParseError::MissingRequiredFields {
            line,
            missing_fields,
        }
    }
}
