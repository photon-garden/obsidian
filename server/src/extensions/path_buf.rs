use std::path::PathBuf;

pub trait PathBufExtension {
    fn to_string(&self) -> String;
}

impl PathBufExtension for PathBuf {
    fn to_string(&self) -> String {
        self.to_string_lossy().to_string()
    }
}
