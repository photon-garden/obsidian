use crate::built_assets_browser_prefix;
use cfg_if::cfg_if;
use std::{path::PathBuf, time::Duration};

#[derive(PartialEq)]
pub struct FileAsset {
    pub url_path: PathBuf, // Used for loading the asset in the browser.
    pub url_path_starting_from_built_assets_dir: PathBuf, // Used for saving the asset to disk.
    pub load_time_budget: Duration,
    pub size_in_bytes: usize, // For checking performance budgets.
}

impl FileAsset {
    pub fn new(url_path: PathBuf, load_time_budget: Duration, size_in_bytes: usize) -> Self {
        let url_path_starting_from_built_assets_dir = url_path
            .strip_prefix(built_assets_browser_prefix())
            .expect("Error stripping prefix.")
            .to_path_buf();

        let asset = Self {
            url_path,
            url_path_starting_from_built_assets_dir,
            load_time_budget,
            size_in_bytes,
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

    impl HasPerformanceBudget for FileAsset {
        fn load_time_budget(&self) -> Duration {
            self.load_time_budget
        }

        fn size_in_bytes(&self) -> usize{
            self.size_in_bytes
        }

        fn path_for_reporting_asset_over_budget(&self) -> &std::path::Path {
            &self.url_path_starting_from_built_assets_dir
        }
    }

    impl ToTokens for FileAsset {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            log::info!("Converting FileAsset to tokens.");

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


            let load_time_budget_millis = self.load_time_budget.as_millis() as u64;
            // log::info!("load_time_budget_millis: {}", load_time_budget_millis);

            let size_in_bytes = self.size_in_bytes;
            // log::info!("size_in_bytes: {}", size_in_bytes);

            let quoted = quote! {
                assets::FileAsset {
                    url_path: std::path::PathBuf::from(#url_path),
                    url_path_starting_from_built_assets_dir: std::path::PathBuf::from(#url_path_starting_from_built_assets_dir),
                    load_time_budget: std::time::Duration::from_millis(#load_time_budget_millis),
                    size_in_bytes: #size_in_bytes,
                }
            };
            // log::info!("quoted: {}", quoted);

            tokens.extend(quoted);
            // log::info!("Extended tokens.");
        }
    }

}
}
