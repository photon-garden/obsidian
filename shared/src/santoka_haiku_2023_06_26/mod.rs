use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

static JSON: &str = include_str!("santoka_haiku_2023_06_26.json");
pub static DATABASE: Lazy<Database> = Lazy::new(load);

pub fn load() -> Database {
    serde_json::from_str(JSON).unwrap()
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Database {
    pub created_at: String, // An ISO 8601 timestamp.
    pub poems: Vec<Poem>,
    pub translators: Vec<Translator>,
    pub publications: Vec<Publication>,
}

impl Database {
    // As long as our tests pass, translator and publication can't panic
    // even though they contain unwrap. That's because:
    // - Code outside this module can't construct publication ids, so all publication
    //   ids come from poems in the database.
    // - Our tests confirm that all poems have valid publication ids.
    // - We include the serialized database at compile time and it's immutable.

    pub fn translator(&self, id: TranslatorId) -> &Translator {
        self.translators
            .iter()
            .find(|translator| translator.id == id)
            .unwrap()
    }

    pub fn publication(&self, id: PublicationId) -> &Publication {
        self.publications
            .iter()
            .find(|publication| publication.id == id)
            .unwrap()
    }
}

#[derive(Debug, serde::Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Poem {
    pub id: PoemId,
    pub translator_id: TranslatorId,
    pub publication_id: PublicationId,
    pub english_text: String,
    pub japanese_text: Option<String>,
}

impl Poem {
    pub fn japanese_text_is_romaji(&self) -> bool {
        match &self.japanese_text {
            Some(japanese_text) => uses_romaji(japanese_text),
            None => false,
        }
    }
}

fn uses_romaji(text: &str) -> bool {
    text.chars().any(|c| c.is_ascii())
}

#[derive(Debug, serde::Deserialize, PartialEq, Clone, Copy)]
pub struct PoemId(usize);

impl Poem {
    pub fn japanese_text_or_default(&self) -> &str {
        self.japanese_text
            .as_ref()
            .map_or("", |japanese_text: &String| &japanese_text[..])
    }
}

#[derive(Debug, serde::Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Publication {
    pub id: PublicationId,
    pub name: String,
    pub year: Option<u16>,
    pub translator_id: TranslatorId,
    pub description: String,
}

const NUM_PREVIEW_POEMS: usize = 3;

impl Publication {
    pub fn poems(&self) -> impl Iterator<Item = &Poem> {
        DATABASE
            .poems
            .iter()
            .filter(|poem| poem.publication_id == self.id)
    }

    pub fn preview_poems(&self) -> impl Iterator<Item = &Poem> {
        self.poems().take(NUM_PREVIEW_POEMS)
    }

    pub fn non_preview_poems(&self) -> impl Iterator<Item = &Poem> {
        self.poems().skip(NUM_PREVIEW_POEMS)
    }

    pub fn year_or_unknown(&self) -> String {
        self.year
            .as_ref()
            .map_or("unknown".to_string(), |year: &u16| year.to_string())
    }

    pub fn has_non_preview_poems(&self) -> bool {
        self.non_preview_poems().next().is_some()
    }

    pub fn has_japanese_text(&self) -> bool {
        self.poems().any(|poem| poem.japanese_text.is_some())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct PublicationId(usize);

impl enum_iterator::Sequence for PublicationId {
    const CARDINALITY: usize = 21;

    fn first() -> Option<Self> {
        DATABASE
            .publications
            .first()
            .map(|publication| publication.id)
    }

    fn last() -> Option<Self> {
        DATABASE
            .publications
            .last()
            .map(|publication| publication.id)
    }

    fn previous(&self) -> Option<Self> {
        let index = DATABASE
            .publications
            .iter()
            .position(|publication| publication.id == *self)
            .expect("Failed to find publication id.");

        if index == 0 {
            return None;
        }

        DATABASE
            .publications
            .get(index - 1)
            .map(|publication| publication.id)
    }

    fn next(&self) -> Option<Self> {
        let index = DATABASE
            .publications
            .iter()
            .position(|publication| publication.id == *self)
            .expect("Failed to find publication id.");

        DATABASE
            .publications
            .get(index + 1)
            .map(|publication| publication.id)
    }
}

impl FromStr for PublicationId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<usize>() {
            Ok(parsed) => {
                let publication_id = PublicationId(parsed);
                Ok(publication_id)
            }
            Err(e) => Err(format!("Failed to parse publication id: {}", e)),
        }
    }
}

impl Display for PublicationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Translator {
    pub id: TranslatorId,
    pub name: String,
}

#[derive(Debug, serde::Deserialize, PartialEq, Clone, Copy)]
pub struct TranslatorId(usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_database() {
        load();
    }

    #[test]
    fn test_primary_key_constraints() {
        let db = load();
        for poem in &db.poems {
            let publication_id = poem.publication_id;
            let translator_id = poem.translator_id;

            // These functions panic if the record isn't found, failing the test.
            db.publication(publication_id);
            db.translator(translator_id);
        }
    }
}
