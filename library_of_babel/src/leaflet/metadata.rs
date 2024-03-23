use chrono::NaiveDate;

use super::line::Line;
use super::normalized_string::NormalizedString;
use super::schema::{ExpectedType, FieldDefinition, Schema};
use crate::obsidian::{Vault, VaultItemId, WikiLinkString};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Metadata {
    pub fields: HashMap<NormalizedString, FieldValue>,
}

impl Metadata {
    pub fn new() -> Self {
        Metadata {
            fields: HashMap::new(),
        }
    }

    pub fn try_add_field(&mut self, vault: &Vault, schema: &Schema, line: &Line) -> LineWas {
        let text_without_comments = line.text_without_comments.trim();
        if text_without_comments.is_empty() {
            return LineWas::Empty;
        }

        let mut split = text_without_comments.split(':');

        // If there's a value before the colon,
        let key = match split.next() {
            Some(key) => key.trim(),
            None => return LineWas::OtherText,
        };

        // And if there's a value after the colon,
        let value = match split.next() {
            Some(value) => value.trim(),
            None => return LineWas::OtherText,
        };

        // And the key is a field name specified in the schema,
        let field_definition = match schema.field_definition(key) {
            Some(field_definition) => field_definition,
            None => return LineWas::OtherText,
        };

        // And the value is what the schema expects for that field,
        let field_value =
            match self.try_parse_field_value(vault, &field_definition.expected_type, value) {
                Some(field_value) => field_value,
                None => return LineWas::OtherText,
            };

        self.fields.insert(NormalizedString::new(key), field_value);
        LineWas::ValidMetadataField
    }

    pub fn is_valid(&self, schema: &Schema) -> Valid {
        let required_fields_that_are_missing: Vec<_> = schema
            .required_fields()
            .filter(|required_field| self.fields.get(&required_field.name).is_none())
            .collect();

        if required_fields_that_are_missing.is_empty() {
            Valid::Yes
        } else {
            let owned_missing_fields: Vec<_> = required_fields_that_are_missing
                .into_iter()
                .cloned()
                .collect();

            Valid::No {
                missing_fields: owned_missing_fields,
            }
        }
    }

    fn try_parse_field_value(
        &self,
        vault: &Vault,
        expected_type: &ExpectedType,
        value: &str,
    ) -> Option<FieldValue> {
        match expected_type {
            ExpectedType::Link => {
                let wiki_link = value.parse::<WikiLinkString>().ok()?;
                let vault_item_id = vault.vault_item_id_by_wiki_link(value);
                let field_value = FieldValue::Link {
                    wiki_link_text: wiki_link,
                    vault_item_id,
                };
                Some(field_value)
            }
            ExpectedType::String => {
                let parsed = FieldValue::String(value.trim().to_string());
                Some(parsed)
            }
            ExpectedType::U64 => value.trim().parse::<u64>().map(FieldValue::U64).ok(),
            ExpectedType::YyyyMmDd => {
                let cleaned = {
                    let cleaned = value.trim();
                    let cleaned = match cleaned.strip_prefix("[[") {
                        Some(cleaned) => cleaned,
                        None => cleaned,
                    };
                    let cleaned = match cleaned.strip_suffix("]]") {
                        Some(cleaned) => cleaned,
                        None => cleaned,
                    };
                    cleaned
                };

                let mut split = cleaned.split('.');
                let year = split.next()?.parse::<i32>().ok()?;
                let month = split.next()?.parse::<u32>().ok()?;
                let day = split.next()?.parse::<u32>().ok()?;

                let date = NaiveDate::from_ymd_opt(year, month, day)?;
                Some(FieldValue::YyyyMmDd(date))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum FieldValue {
    YyyyMmDd(NaiveDate),
    String(String),
    U64(u64),
    Link {
        wiki_link_text: WikiLinkString,
        /// Will be `None` if the link is to a non-existent page.
        vault_item_id: Option<VaultItemId>,
    },
}

pub enum LineWas {
    ValidMetadataField,
    Empty,
    OtherText,
}

pub enum Valid {
    Yes,
    No {
        missing_fields: Vec<FieldDefinition>,
    },
}
