use crate::parse_macro_arguments::*;
use assets_runtime::{paths::*, CssAsset};
use proc_macro::TokenStream;
use quote::quote;
use std::str::FromStr;
use std::{path::PathBuf, time::Duration};
use syn::{
    parse::{Parse, ParseStream},
    Result as SynResult,
};

pub fn include(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as IncludeTailwindInput);
    crate::logger::init_logger(input.debug);

    log::info!("Including Tailwind.");

    let config = tailwind_config_path();
    let config_str = config
        .to_str()
        .expect("Error converting the path to the Tailwind config to a string.");
    log::info!("Using Tailwind config at {}.", config_str);

    let input_file = workspace_root_dir().join(&input.path_to_input_file);
    let input_file_str = input_file
        .to_str()
        .expect("Error converting the path to the Tailwind input CSS file to a string.");
    log::info!("Using Tailwind input CSS file at {}.", input_file_str);

    if !input_file.exists() {
        let error_message = format!("File not found: {}", input_file_str);

        return syn::Error::new(input.span, error_message)
            .to_compile_error()
            .into();
    }

    let output_file = output_file_path(&input.url_path);
    let output_file_str = output_file
        .to_str()
        .expect("Error converting the path to the Tailwind output CSS file to a string.");
    log::info!("Saving Tailwind output to {}.", output_file_str);

    let mut tailwind_args = vec![
        "--config",
        config_str,
        "--input",
        input_file_str,
        "--output",
        output_file_str,
    ];

    if input.minify {
        tailwind_args.push("--minify");
    }

    log::info!("Invoking Tailwind CLI.");
    let tailwind_cli_output = tailwind_cli::run(tailwind_args);

    if let Err(error) = tailwind_cli_output {
        let error_message = format!("Error including Tailwind:\n{}", error);
        log::error!("{}", error_message);
        return syn::Error::new(input.span, error_message)
            .to_compile_error()
            .into();
    }

    log::info!("Successfully built Tailwind.");

    let built_css = std::fs::read_to_string(output_file).expect("Error reading built.css file.");

    let css_asset = CssAsset::new(input.url_path, built_css, input.performance_budget);

    let output = quote! {
        #css_asset
    };

    output.into()
}

pub fn tailwind_config_path() -> PathBuf {
    assets_macros_dir().join("tailwind.config.js")
}

struct IncludeTailwindInput {
    path_to_input_file: PathBuf,
    url_path: PathBuf,
    performance_budget: Duration,
    minify: bool,
    debug: bool,
    span: proc_macro2::Span,
}

impl Parse for IncludeTailwindInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let input_span = input.span();

        let error_message = r#"Please make sure to pass arguments to include_tailwind! like this:

include_tailwind!(
    path_to_input_file: \"src/main.css\",
    url_path: \"built.css\",
    performance_budget_millis: 300,
    minify: true,
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

        // True if we're in release mode (i.e. `cargo build --release`).
        let release_mode = !cfg!(debug_assertions);
        // Default to minifying in release mode.
        let minify_by_default = release_mode;
        let minify = parse_named_bool_argument("minify", &input).unwrap_or(minify_by_default);

        let debug = parse_named_bool_argument("debug", &input).unwrap_or(false);

        Ok(IncludeTailwindInput {
            path_to_input_file,
            url_path,
            performance_budget,
            minify,
            debug,
            span: input_span,
        })
    }
}
