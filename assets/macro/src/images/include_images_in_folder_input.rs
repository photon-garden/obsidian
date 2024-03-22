use crate::parse_macro_arguments::*;
use std::path::PathBuf;
use syn::{
    parse::{Parse, ParseStream},
    Result as SynResult,
};

/// This struct represents the input to the `include_images_in_folder!` macro.
pub struct IncludeImagesInFolderInput {
    pub absolute_path_to_images_dir: PathBuf,
    pub debug: bool,
}

impl Parse for IncludeImagesInFolderInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let error_message = r#"Please make sure to pass arguments to include_images! like this:

include_images!(path_to_images_dir: \"src/original_images\");

The path should be relative to the workspace root.

You can also pass an optional `debug` argument like this:

include_images!(path_to_images_dir: \"src/original_images\", debug: true);
"#;
        let error = syn::Error::new(input.span(), error_message);

        // This argument is required, so if it's not present we
        // convert None to an error and return early.
        let string_path_to_images_dir_starting_at_workspace_root =
            parse_named_string_argument("path_to_images_dir", &input).ok_or(error)?;

        let absolute_path_to_images_dir = assets_runtime::paths::workspace_root_dir()
            .join(string_path_to_images_dir_starting_at_workspace_root);

        // This argument is optional, so we default to `false` if it's not present.
        let debug = parse_named_bool_argument("debug", &input).unwrap_or(false);

        Ok(IncludeImagesInFolderInput {
            absolute_path_to_images_dir,
            debug,
        })
    }
}
