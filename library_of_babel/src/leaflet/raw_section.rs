use super::line::Line;

#[derive(Debug)]
pub struct RawSection {
    pub starting_line_number: usize,
    pub lines: Vec<Line>,
    pub text: String,
}

impl RawSection {
    pub fn new(lines: Vec<Line>) -> Self {
        let starting_line_number = lines.first().unwrap().absolute_line_number;
        let text = lines
            .iter()
            .map(|line| line.text.as_str())
            .collect::<String>();
        RawSection {
            starting_line_number,
            lines,
            text,
        }
    }

    pub fn is_a_leaflet_section(&self) -> bool {
        !self.text.to_lowercase().contains("this isn't leaflet")
    }
}
