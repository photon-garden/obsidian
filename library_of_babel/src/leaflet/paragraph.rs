use super::line::Line;
use super::metadata::Metadata;

#[derive(Debug)]
pub struct Paragraph {
    pub lines: Vec<Line>,
    pub metadata: Metadata,
}

impl Paragraph {
    pub fn new(metadata: Metadata, lines: Vec<Line>) -> Self {
        Paragraph { lines, metadata }
    }
}
