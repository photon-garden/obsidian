use crate::parse_macro_arguments::*;
use assets_runtime::{paths::*, FontAsset};
use proc_macro::TokenStream;
use quote::quote;
use std::str::FromStr;
use std::{path::PathBuf, time::Duration};
use syn::{
    parse::{Parse, ParseStream},
    Result as SynResult,
};

pub fn include(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as IncludeFontInput);
    crate::logger::init_logger(input.debug);

    log::info!("Including font: {}", input.path_to_input_file.display());

    let input_file_path = workspace_root_dir().join(&input.path_to_input_file);

    if !input_file_path.exists() {
        let error_message = format!("File not found: {}", input_file_path.display());

        return syn::Error::new(input.span, error_message)
            .to_compile_error()
            .into();
    }

    let size_in_bytes = std::fs::metadata(&input_file_path)
        .expect("Error getting metadata for font file.")
        .len();

    let output_file_path = output_file_path(&input.url_path);

    let output_dir = output_file_path
        .parent()
        .expect("Error getting fonts directory.");

    std::fs::create_dir_all(output_dir).expect("Error creating built assets dir.");

    std::fs::copy(input_file_path, output_file_path).expect("Error copying font file.");

    let font_asset = FontAsset::new(
        input.url_path,
        input.performance_budget,
        size_in_bytes as usize,
    );

    let output = quote! {
        #font_asset
    };

    output.into()
}

struct IncludeFontInput {
    path_to_input_file: PathBuf,
    url_path: PathBuf,
    performance_budget: Duration,
    debug: bool,
    span: proc_macro2::Span,
}

impl Parse for IncludeFontInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let input_span = input.span();

        let error_message = r#"Please make sure to pass arguments to include_font! like this:

include_font!(
    path_to_input_file: \"src/fonts/MyFont.otf\",
    url_path: \"fonts/MyFont.otf\",
    performance_budget_millis: 300,
    debug: true,
);
"#;

        let error = syn::Error::new(input_span, error_message);

        let path_to_input_file_string =
            parse_named_string_argument("path_to_input_file", &input).ok_or(error.clone())?;

        let path_to_input_file = PathBuf::from_str(&path_to_input_file_string)
            .expect("Error parsing path_to_input_file.");

        let url_path_string = parse_url_path_argument("url_path", &input)
            .map_err(|err| err.into_syn_error(input_span))?;
        let url_path = PathBuf::from_str(&url_path_string).expect("Error parsing url_path.");

        let performance_budget_millis =
            parse_named_u64_argument("performance_budget_millis", &input).ok_or(error.clone())?;
        let performance_budget = Duration::from_millis(performance_budget_millis);

        let debug = parse_named_bool_argument("debug", &input).unwrap_or(false);

        Ok(IncludeFontInput {
            path_to_input_file,
            url_path,
            performance_budget,
            debug,
            span: input_span,
        })
    }
}
