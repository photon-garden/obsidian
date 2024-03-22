use assets::*;
use once_cell::sync::Lazy;

pub mod processed_2023_haiku;

pub static ASSETS: Lazy<Assets> = Lazy::new(Assets::new);

pub struct Assets {
    pub css: CssAsset,
    pub browser_crate: BrowserCrateAsset,
    pub favicon: FileAsset,
    pub logo: FileAsset,
    pub hero_image: LightDarkImageAsset,
}

impl Assets {
    pub fn new() -> Self {
        let css = assets::include_tailwind!(
            path_to_input_file: "server/src/assets/main.css",
            url_path: "built-assets/built.css",
            performance_budget_millis: 150,
        );

        let browser_crate = assets::include_browser_crate!(
            path_to_browser_crate: "browser",
            js_url_path: "built-assets/browser.js",
            js_performance_budget_millis: 150,
            wasm_url_path: "built-assets/browser_bg.wasm",
            wasm_performance_budget_millis: 310,
            production: true,
        );

        let favicon = assets::include_file!(
            path_to_input_file: "server/src/assets/images/favicon.ico",
            url_path: "built-assets/favicon.ico",
            performance_budget_millis: 150,
        );

        let logo = assets::include_file!(
            path_to_input_file: "server/src/assets/images/logo.png",
            url_path: "built-assets/favicon.ico",
            performance_budget_millis: 275,
        );

        // This image is decorative, so we skip the alt text.
        let hero_image_light = assets::include_image!(
            path_to_image: "server/src/assets/images/hasui_light.jpeg",
            alt: "",
            placeholder: automatic_color,
        );

        // This image is decorative, so we skip the alt text.
        let hero_image_dark = assets::include_image!(
            path_to_image: "server/src/assets/images/hasui_dark.jpeg",
            alt: "",
            placeholder: automatic_color,
        );

        let hero_image = LightDarkImageAsset::new(hero_image_light, hero_image_dark);

        Self {
            css,
            browser_crate,
            favicon,
            logo,
            hero_image,
        }
    }
}

impl Default for Assets {
    fn default() -> Self {
        Self::new()
    }
}
