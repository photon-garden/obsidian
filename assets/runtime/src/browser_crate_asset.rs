use crate::{JsAsset, WasmAsset};
use cfg_if::cfg_if;

pub struct BrowserCrateAsset {
    pub wasm: WasmAsset,
    pub js: JsAsset,
}

cfg_if! {
if #[cfg(feature = "build_time")] {

    use proc_macro2::TokenStream;
    use quote::{quote, ToTokens};

    impl ToTokens for BrowserCrateAsset {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            log::info!("Converting BrowserCrateAsset to tokens.");

            let wasm = &self.wasm;
            let js = &self.js;

            let quoted = quote! {
                assets::BrowserCrateAsset {
                    wasm: #wasm,
                    js: #js,
                }
            };

            // log::info!("quoted: {}", quoted);

            tokens.extend(quoted);

            // log::info!("Extended tokens.");
        }
    }

}
}
