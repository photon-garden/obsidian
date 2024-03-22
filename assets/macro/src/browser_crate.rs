use crate::parse_macro_arguments::*;
use assets_runtime::{paths::*, BrowserCrateAsset, JsAsset, WasmAsset};
use proc_macro::TokenStream;
use quote::quote;
use std::path::PathBuf;
use std::time::Duration;
use syn::{
    parse::{Parse, ParseStream},
    Result as SynResult,
};

pub fn include(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as IncludeBrowserCrateInput);
    crate::logger::init_logger(input.debug);

    let final_path_to_built_wasm = output_file_path(&input.wasm_url_path);
    let final_path_to_built_js = output_file_path(&input.js_url_path);

    let maybe_wasm_pack_output = run_wasm_pack(&input);
    let wasm_pack_output = match maybe_wasm_pack_output {
        Ok(wasm_pack_output) => wasm_pack_output,
        Err(error) => return error,
    };

    std::fs::create_dir_all(built_assets_dir()).expect("Error creating built assets dir.");

    // Move the built wasm from the temporary directory where wasm-pack
    // saved it to its final location in the built assets directory.
    std::fs::rename(
        &wasm_pack_output.path_to_built_wasm,
        &final_path_to_built_wasm,
    )
    .expect("Error moving the built wasm file to the final location.");

    // Move the built JavaScript from the temporary directory where wasm-pack
    // saved it to its final location in the built assets directory.
    log::info!("Moving JS file to {:?}", &final_path_to_built_js);
    if !wasm_pack_output.path_to_built_js.exists() {
        let error_message = format!(
            "Error moving the built JS file to the final location. The built JS file doesn't exist at {:?}.",
            &wasm_pack_output.path_to_built_js
        );
        log::error!("{}", error_message);
        let error: TokenStream = syn::Error::new(input.span, error_message)
            .to_compile_error()
            .into();
        return error;
    }
    std::fs::rename(&wasm_pack_output.path_to_built_js, &final_path_to_built_js)
        .expect("Error moving the built JS file to the final location.");

    let minified_js_string = if input.production {
        overwrite_js_with_minified(final_path_to_built_js)
    } else {
        std::fs::read_to_string(&final_path_to_built_js).expect("Error reading JS file.")
    };

    let wasm_bytes =
        std::fs::read(&final_path_to_built_wasm).expect("Error reading the built wasm file.");

    let browser_crate_asset = BrowserCrateAsset {
        wasm: WasmAsset::new(
            input.wasm_url_path,
            wasm_bytes,
            input.wasm_performance_budget,
        ),
        js: JsAsset::new(
            input.js_url_path,
            minified_js_string,
            input.js_performance_budget,
        ),
    };

    // Clean up the temporary directory where wasm-pack saved the built files.
    std::fs::remove_dir_all(wasm_pack_output.out_dir).expect("Error deleting out_dir.");

    let output = quote! {
        #browser_crate_asset
    };

    output.into()
}

fn run_wasm_pack(input: &IncludeBrowserCrateInput) -> Result<WasmPackOutput, TokenStream> {
    log::info!("Including browser crate.");

    // Generate a unique name for the out directory so that
    // if we're running multiple builds in parallel, they
    // don't interfere with each other.
    let uuid = uuid::Uuid::new_v4().to_string();
    let sub_folder_name = format!("browser_{}", uuid);
    let out_dir = target_dir().join(sub_folder_name);
    let out_dir_str = out_dir.to_str().unwrap();
    let mut wasm_pack_args = vec![
        "build",
        "--no-pack", // For some reason generating a package.json causes errors when running `spin build`: Error: invalid type: sequence, expected a string at line 3 column 19
        "--target",
        "web",
        "--out-dir",
        out_dir_str,
        "--out-name",
        "browser",
    ];

    if input.production {
        wasm_pack_args.push("--release");
    } else {
        wasm_pack_args.push("--dev");
    }

    let browser_crate = workspace_root_dir().join(&input.path_to_browser_crate);
    let browser_crate_str = browser_crate.to_str().unwrap();
    log::info!("Looking for browser crate at {}.", browser_crate_str);
    if !browser_crate.exists() {
        let error_message = format!("Folder not found: {}", browser_crate_str);

        let error: TokenStream = syn::Error::new(input.span, error_message)
            .to_compile_error()
            .into();

        return Err(error);
    }
    if !browser_crate.is_dir() {
        let error_message = format!("{} is not a folder:", browser_crate_str);

        let error: TokenStream = syn::Error::new(input.span, error_message)
            .to_compile_error()
            .into();

        return Err(error);
    }
    wasm_pack_args.push(browser_crate_str);

    // cargo arguments
    if !input.production {
        wasm_pack_args.push("--features");
        wasm_pack_args.push("dev");
    }

    log::info!(
        "Invoking wasm-pack CLI with these arguments: {:?}",
        &wasm_pack_args
    );

    lib_wasm_pack::run(wasm_pack_args)
        .map(|_| {
            log::info!("Successfully built browser crate.");

            WasmPackOutput {
                path_to_built_wasm: out_dir.join("browser_bg.wasm"),
                path_to_built_js: out_dir.join("browser.js"),
                out_dir,
            }
        })
        .map_err(|original_error| {
            let error_message = format!("Error including browser crate. {}", original_error);
            log::error!("{}", error_message);
            let error: TokenStream = syn::Error::new(input.span, error_message)
                .to_compile_error()
                .into();

            error
        })
}

struct WasmPackOutput {
    path_to_built_wasm: PathBuf,
    path_to_built_js: PathBuf,
    out_dir: PathBuf,
}

fn overwrite_js_with_minified(path_to_js: PathBuf) -> String {
    let source = std::fs::read_to_string(&path_to_js).expect("Error reading JS file.");
    let js_bytes = source.as_bytes();

    let session = minify_js::Session::new();
    let mut minified_bytes = Vec::new();
    minify_js::minify(
        &session,
        minify_js::TopLevelMode::Module,
        js_bytes,
        &mut minified_bytes,
    )
    .unwrap();

    let minified_string =
        String::from_utf8(minified_bytes).expect("Error converting minified JS bytes to string.");

    let path_to_minified_js = path_to_js;
    std::fs::write(path_to_minified_js, &minified_string)
        .expect("Error writing minified JS to file.");

    minified_string
}

struct IncludeBrowserCrateInput {
    path_to_browser_crate: PathBuf,

    js_url_path: PathBuf,
    js_performance_budget: Duration,

    wasm_url_path: PathBuf,
    wasm_performance_budget: Duration,

    production: bool,
    debug: bool,

    span: proc_macro2::Span,
}

impl Parse for IncludeBrowserCrateInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let input_span = input.span();

        let error_message = r#"Please make sure to pass arguments to include_browser_crate! like this:

include_browser_crate!(
    path_to_browser_crate: \"browser\",
    js_url_path: \"browser.js\",
    js_performance_budget: 200,
    wasm_url_path: \"browser_bg.wasm\",
    wasm_performance_budget: 200,
    production: true,
    debug: true,
);
"#;
        let error = syn::Error::new(input.span(), error_message);

        let path_to_browser_crate =
            parse_named_string_argument("path_to_browser_crate", &input).ok_or(error.clone())?;
        // eprintln!("path_to_browser_crate: {:?}", path_to_browser_crate);

        let js_url_path = parse_url_path_argument("js_url_path", &input)
            .map_err(|err| err.into_syn_error(input_span))?;
        // eprintln!("js_url_path: {:?}", js_url_path);

        let js_performance_budget_millis =
            parse_named_u64_argument("js_performance_budget_millis", &input)
                .ok_or(error.clone())?;
        let js_performance_budget = Duration::from_millis(js_performance_budget_millis);
        // eprintln!("js_performance_budget: {:?}", js_performance_budget);

        let wasm_url_path = parse_url_path_argument("wasm_url_path", &input)
            .map_err(|err| err.into_syn_error(input_span))?;
        // eprintln!("wasm_url_path: {:?}", wasm_url_path);

        let wasm_performance_budget_millis =
            parse_named_u64_argument("wasm_performance_budget_millis", &input)
                .ok_or(error.clone())?;
        let wasm_performance_budget = Duration::from_millis(wasm_performance_budget_millis);
        // eprintln!("wasm_performance_budget: {:?}", wasm_performance_budget);

        // True if we're in release mode (i.e. `cargo build --release`).
        let release_mode = !cfg!(debug_assertions);
        // Default to production: true in release mode.
        let production_default = release_mode;
        let production =
            parse_named_bool_argument("production", &input).unwrap_or(production_default);

        // eprintln!("production: {:?}", production);

        let debug = parse_named_bool_argument("debug", &input).unwrap_or(false);
        // eprintln!("debug: {:?}", debug);

        Ok(IncludeBrowserCrateInput {
            path_to_browser_crate: PathBuf::from(path_to_browser_crate),
            js_url_path: PathBuf::from(js_url_path),
            js_performance_budget,
            wasm_url_path: PathBuf::from(wasm_url_path),
            wasm_performance_budget,
            production,
            debug,
            span: input_span,
        })
    }
}
