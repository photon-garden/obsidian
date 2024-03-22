#[derive(PartialEq, Debug, Eq, Hash, Clone)]
pub struct NormalizedString(String);

impl NormalizedString {
    pub fn new(s: &str) -> Self {
        NormalizedString(s.trim().to_lowercase())
    }
}
