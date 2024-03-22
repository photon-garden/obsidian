use crate::{built_assets_browser_prefix, output_file_path};
use cfg_if::cfg_if;
use std::path::PathBuf;
use std::time::Duration;

#[derive(PartialEq)]
pub struct WasmAsset {
    pub url_path: PathBuf, // Used for loading the asset in the browser.
    pub url_path_starting_from_built_assets_dir: PathBuf, // Used for saving the asset to disk.
    pub bytes: Vec<u8>,
    pub load_time_budget: Duration,
}

impl WasmAsset {
    pub fn new(url_path: PathBuf, bytes: Vec<u8>, load_time_budget: Duration) -> Self {
        let url_path_starting_from_built_assets_dir = url_path
            .strip_prefix(built_assets_browser_prefix())
            .expect("Error stripping prefix.")
            .to_path_buf();

        let asset = Self {
            url_path,
            url_path_starting_from_built_assets_dir,
            bytes,
            load_time_budget,
        };

        #[cfg(feature = "build_time")]
        asset.check_performance_budget();

        asset
    }
}

cfg_if! {
if #[cfg(feature = "build_time")] {
    use crate::performance_budget::HasPerformanceBudget;
    use proc_macro2::TokenStream;
    use quote::{quote, ToTokens};

    impl HasPerformanceBudget for WasmAsset {
        fn load_time_budget(&self) -> Duration {
            self.load_time_budget
        }

        fn size_in_bytes(&self) -> usize{
            self.bytes.len()
        }

        fn path_for_reporting_asset_over_budget(&self) -> &std::path::Path {
            &self.url_path_starting_from_built_assets_dir
        }
    }


    impl ToTokens for WasmAsset {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            log::info!("Converting WasmAsset to tokens.");

            let url_path = self.url_path.to_str().unwrap();
            // log::info!("url_path: {}", url_path);

            let url_path_starting_from_built_assets_dir = self
                .url_path_starting_from_built_assets_dir
                .to_str()
                .unwrap();
            // log::info!(
            //     "url_path_starting_from_built_assets_dir: {}",
            //     url_path_starting_from_built_assets_dir
            // );

            let path_on_disk = path_on_disk(self);
            let path_on_disk = path_on_disk.to_str().unwrap();

            let load_time_budget_millis = self.load_time_budget.as_millis() as u64;
            // log::info!("load_time_budget_millis: {}", load_time_budget_millis);

            let quoted = quote! {
                assets::WasmAsset {
                    url_path: std::path::PathBuf::from(#url_path),
                    url_path_starting_from_built_assets_dir: std::path::PathBuf::from(#url_path_starting_from_built_assets_dir),
                    bytes: include_bytes!(#path_on_disk).to_vec(),
                    load_time_budget: std::time::Duration::from_millis(#load_time_budget_millis),
                }
            };

            // log::info!("quoted: {}", quoted);

            tokens.extend(quoted);

            // log::info!("Extended tokens.");
        }
    }

    fn path_on_disk(wasm_asset: &WasmAsset) -> PathBuf {
        output_file_path(&wasm_asset.url_path)
    }
}
}
