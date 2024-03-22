use std::path::{Path, PathBuf};

pub fn workspace_root_dir() -> PathBuf {
    let cargo_workspace_dir = std::env!("CARGO_WORKSPACE_DIR");
    Path::new(&cargo_workspace_dir).to_path_buf()
}

pub fn assets_dir() -> PathBuf {
    workspace_root_dir().join("assets")
}

pub fn built_assets_dir() -> PathBuf {
    assets_dir().join(built_assets_dir_name())
}

pub fn output_file_path(url_path: &Path) -> PathBuf {
    let url_path_without_prefix = url_path
        .strip_prefix(built_assets_browser_prefix())
        .unwrap();

    built_assets_dir().join(url_path_without_prefix)
}

pub fn built_assets_dir_name() -> &'static str {
    "built"
}

pub fn assets_macros_dir() -> PathBuf {
    assets_dir().join("macro")
}

pub fn target_dir() -> PathBuf {
    workspace_root_dir().join("target")
}

pub fn cargo_install_dir() -> PathBuf {
    target_dir().join("cargo_install")
}

pub fn path_to_cargo_install_binary(binary_name: &str) -> PathBuf {
    cargo_install_dir().join("bin").join(binary_name)
}

/// When loading assets in the browser, URL paths should
/// start with this prefix.
///
/// For example, if you have an asset at `assets/built/built.css`,
/// then the URL path to that asset in the browser should be
/// `/built-assets/built.css`.
pub fn built_assets_browser_prefix() -> PathBuf {
    PathBuf::from("built-assets")
}

pub fn asset_url_path(sub_url_path: &Path) -> PathBuf {
    built_assets_browser_prefix().join(sub_url_path)
}

pub fn built_image_path(path_starting_from_images_dir: &Path) -> PathBuf {
    built_assets_dir().join(path_starting_from_images_dir)
}
