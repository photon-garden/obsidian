use crate::parse_macro_arguments::*;
use std::path::PathBuf;
use syn::{
    parse::{Parse, ParseStream},
    Ident, LitStr, Result as SynResult, Token,
};

use super::build_time_image::PlaceholderToGenerate;

/// This struct represents the input to the `include_image!` macro.
pub struct IncludeImageInput {
    pub absolute_path_to_image: PathBuf,
    pub placeholder_to_generate: PlaceholderToGenerate,
    pub alt: Alt,
    pub debug: bool,
}

impl Parse for IncludeImageInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let error_message = r#"Please make sure to pass arguments to include_image! like this. The path should be relative to the workspace root. By default this will detect the dominant color of the image and use that as the placeholder.

include_image!(
    path_to_image: \"src/images/santoka.jpg\",
    alt: \"Taneda Santōka\",
);

There are also some optional arguments. `placeholder` can be `lqip` or `automatic_color`. `lqip` generates a low resolution version of the image as a base64 string, suitable for embedding directly into html. `automatic_color` computes the dominant color of the image. `debug` can be `true` or `false`.

include_image!(
    path_to_image: \"src/images/santoka.jpg\",
    alt: \"Taneda Santōka\",
    placeholder: lqip,
    debug: true,
);
"#;
        let error = syn::Error::new(input.span(), error_message);

        // This argument is required, so if it's not present we
        // convert None to an error and return early.
        let string_path_to_image_starting_at_workspace_root =
            parse_named_string_argument("path_to_image", &input).ok_or(error.clone())?;

        let absolute_path_to_image = assets_runtime::paths::workspace_root_dir()
            .join(string_path_to_image_starting_at_workspace_root);

        let alt = parse_alt(&input).ok_or(error.clone())?;

        let placeholder_to_generate = parse_placeholder_to_generate(&input).ok_or(error)?;

        // This argument is optional, so we default to `false` if it's not present.
        let debug = parse_named_bool_argument("debug", &input).unwrap_or(false);

        Ok(IncludeImageInput {
            absolute_path_to_image,
            alt,
            placeholder_to_generate,
            debug,
        })
    }
}

fn parse_alt(input: &ParseStream) -> Option<Alt> {
    let maybe_argument_name = parse_argument_name_and_colon("alt", input);

    // If there's no argument name, generate alt text automatically.
    if maybe_argument_name.is_none() {
        return Some(Alt::Automatic);
    }

    // If there's an argument name, the value should be a string literal
    // containing the alt text.
    let argument_value_literal: LitStr = input.parse().ok()?;
    let alt_str = argument_value_literal.value();
    let alt = Alt::Literal(alt_str);

    // Parse the optional comma after the argument. Note how
    // we ignore the error here since the comma is optional.
    let _: Result<Token![,], _> = input.parse();

    Some(alt)
}

pub enum Alt {
    Automatic,
    Literal(String),
}

fn parse_placeholder_to_generate(input: &ParseStream) -> Option<PlaceholderToGenerate> {
    let maybe_argument_name = parse_argument_name_and_colon("placeholder", input);

    if maybe_argument_name.is_none() {
        return Some(PlaceholderToGenerate::AutomaticallyDetectedColor);
    }

    // If there's an argument name, the value should be an ident

    let argument_value_literal: Ident = input.parse().ok()?;
    let placeholder_str = argument_value_literal.to_string();
    let placeholder = match placeholder_str.as_str() {
        "lqip" => PlaceholderToGenerate::Lqip,
        "automatic_color" => PlaceholderToGenerate::AutomaticallyDetectedColor,
        _ => panic!(
            "Invalid placeholder: {}. Should be either lqip or automatic_color.",
            placeholder_str
        ),
    };

    // Parse the optional comma after the argument. Note how
    // we ignore the error here since the comma is optional.
    let _: Result<Token![,], _> = input.parse();

    Some(placeholder)
}
