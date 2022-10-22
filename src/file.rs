use lazy_static::lazy_static;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::{DirEntry, WalkDir};

pub fn files_in_vault(vault_path: &'static str) -> impl Iterator<Item = File> {
    WalkDir::new(vault_path)
        .into_iter()
        .filter_entry(|entry| !is_hidden(entry))
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| File::from_dir_entry(vault_path, &entry))
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

#[derive(Debug, Clone)]
pub struct File {
    pub stem: String,
    pub file_name: String,
    pub path_from_vault_root: String,
    pub path_from_vault_root_without_extension: String,
    pub absolute_path: PathBuf,
    pub created_at: SystemTime,
    pub contents: Contents,
}

impl File {
    fn from_dir_entry(vault_path: &str, entry: &DirEntry) -> File {
        let absolute_path = entry.path().to_path_buf();

        let stem = absolute_path
            .file_stem()
            .expect("Error getting file stem.")
            .to_str()
            .expect("Error converting file stem to string.")
            .to_owned();

        let extension = absolute_path
            .extension()
            .expect("Couldn't get file extension.")
            .to_str()
            .expect("Error converting file stem to string.")
            .to_owned();

        let mut path_from_vault_root = absolute_path
            .to_str()
            .expect("Error converting path to unicode.")
            .replace(vault_path, "");

        if path_from_vault_root.starts_with('/') {
            path_from_vault_root = path_from_vault_root[1..].to_string();
        }

        // Add 1 for the "." character.
        let extension_len = extension.len() + 1;
        let extension_index = path_from_vault_root.len() - extension_len;
        let path_from_vault_root_without_extension =
            path_from_vault_root[0..extension_index].to_string();

        let contents = Contents::from_extension(&extension, &absolute_path);

        let file_name = absolute_path
            .file_name()
            .expect("Error getting file name.")
            .to_str()
            .expect("Error converting file name to string.")
            .to_owned();

        let created_at = entry
            .metadata()
            .expect("Error reading file metadata.")
            .created()
            .expect("Couldn't get created_at timestamp for file.");

        File {
            stem,
            file_name,
            path_from_vault_root,
            path_from_vault_root_without_extension,
            absolute_path,
            created_at,
            contents,
        }
    }

    pub fn is_image(&self) -> bool {
        matches!(self.contents, Contents::Image {})
    }
}

#[derive(Debug, Clone)]
pub enum Contents {
    Markdown { text: String },
    Image {},
    Audio {},
    Video {},
    Pdf {},
    Unknown {},
}

impl Contents {
    fn from_extension(extension: &str, absolute_path: &Path) -> Contents {
        lazy_static! {
            static ref image_extensions: Vec<&'static str> =
                vec!["png", "jpg", "jpeg", "gif", "bmp", "svg"];
            static ref audio_extensions: Vec<&'static str> =
                vec!["mp3", "webm", "wav", "m4a", "ogg", "3gp", "flac"];
            static ref video_extensions: Vec<&'static str> =
                vec!["mp4", "webm", "ogv", "mov", "mkv"];
        }

        if extension == "md" {
            let text =
                fs::read_to_string(absolute_path).expect("Error reading markdown file contents.");
            Contents::Markdown { text }
        } else if extension == "pdf" {
            Contents::Pdf {}
        } else if image_extensions.contains(&extension) {
            Contents::Image {}
        } else if audio_extensions.contains(&extension) {
            Contents::Audio {}
        } else if video_extensions.contains(&extension) {
            Contents::Video {}
        } else {
            Contents::Unknown {}
        }
    }
}
