use assets::{LightDarkImageAsset, LightDarkPlaceholder, LQIP_MIME_TYPE};
use maud::{html, Markup, Render};

pub struct LightDarkImage<'a> {
    asset: &'a LightDarkImageAsset,
    class: &'a str,
    above_the_fold: bool,
    is_largest_contentful_paint: bool,
}

impl<'a> LightDarkImage<'a> {
    pub fn new(asset: &'a LightDarkImageAsset) -> LightDarkImage<'a> {
        LightDarkImage {
            asset,
            class: "",
            above_the_fold: false,
            is_largest_contentful_paint: false,
        }
    }

    pub fn class(mut self, class: &'a str) -> Self {
        self.class = class;
        self
    }

    pub fn above_the_fold(mut self, above_the_fold: bool) -> Self {
        self.above_the_fold = above_the_fold;
        self
    }

    pub fn is_largest_contentful_paint(mut self, is_largest_contentful_paint: bool) -> Self {
        self.is_largest_contentful_paint = is_largest_contentful_paint;
        self
    }
}

impl<'a> Render for LightDarkImage<'a> {
    fn render(&self) -> Markup {
        match &self.asset.placeholder {
            LightDarkPlaceholder::Color {
                light_mode_css_string,
                dark_mode_css_string,
            } => light_dark_image_with_color_placeholder(
                self.asset,
                self.class,
                self.above_the_fold,
                self.is_largest_contentful_paint,
                light_mode_css_string,
                dark_mode_css_string,
            ),

            LightDarkPlaceholder::Lqip {
                light_mode_data_uri,
                dark_mode_data_uri,
            } => light_dark_image_with_lqip(
                self.asset,
                self.class,
                self.above_the_fold,
                self.is_largest_contentful_paint,
                light_mode_data_uri,
                dark_mode_data_uri,
            ),
        }
    }
}

pub fn light_dark_image_with_lqip<'a>(
    asset: &'a LightDarkImageAsset,
    class: &'a str,
    above_the_fold: bool,
    is_largest_contentful_paint: bool,
    light_mode_data_uri: &'a str,
    dark_mode_data_uri: &'a str,
) -> Markup {
    // style: "image-rendering: pixelated; image-rendering: -moz-crisp-edges; image-rendering: crisp-edges;",
    html! {
        div
            class={"light-dark-image-with-lqip-placeholder select-none relative overflow-hidden " (class)} {
            picture
                class="shrink-0 min-w-full min-h-full object-cover blur-lg" {

                source
                    media="(prefers-color-scheme: light)"
                    srcset=(light_mode_data_uri)
                    type=(LQIP_MIME_TYPE.to_string()) {}

               source
                    media="(prefers-color-scheme: dark)"
                    srcset=(dark_mode_data_uri)
                    type=(LQIP_MIME_TYPE.to_string()) {}

               img
                    alt=(asset.light_mode.alt)
                    class="shrink-0 min-w-full min-h-full object-cover"
                    src=(asset.light_mode.src) {}
           }

           picture
                class="absolute top-0 left-0 min-w-full min-h-full object-cover" {

                source
                    media="(prefers-color-scheme: light)"
                    srcset=(asset.light_mode.srcset)
                    type=(asset.light_mode.mime_type) {}

                source
                    media="(prefers-color-scheme: dark)"
                    srcset=(asset.dark_mode.srcset)
                    type=(asset.dark_mode.mime_type) {}

                @let loading = if above_the_fold { "eager" } else { "lazy" };
                @let fetch_priority = if is_largest_contentful_paint { "high" } else { "auto" };
                img
                    loading=(loading)
                    fetchpriority=(fetch_priority)
                    alt=(asset.light_mode.alt)
                    class="absolute top-0 left-0 min-w-full min-h-full object-cover"
                    src=(asset.light_mode.src) {}
            }
        }
    }
}

pub fn light_dark_image_with_color_placeholder<'a>(
    asset: &'a LightDarkImageAsset,
    class: &'a str,
    above_the_fold: bool,
    is_largest_contentful_paint: bool,
    light_mode_css_string: &'a str,
    dark_mode_css_string: &'a str,
) -> Markup {
    // style: "image-rendering: pixelated; image-rendering: -moz-crisp-edges; image-rendering: crisp-edges;",
    html! {
        div
            class={"light-dark-image-with-color-placeholder select-none relative h-[500px] " (class)} {

            div
                class="absolute top-0 left-0 shrink-0 min-w-full min-h-full object-cover"
                background_color=(dark_mode_css_string) {}

            div
                class="absolute top-0 left-0 shrink-0 min-w-full min-h-full object-cover dark:hidden"
                background_color=(light_mode_css_string) {}

            picture
                class="absolute top-0 left-0 min-w-full min-h-full object-cover" {
                source
                    media="(prefers-color-scheme: light)"
                    srcset=(asset.light_mode.srcset)
                    type=(asset.light_mode.mime_type) {}

                source
                    media="(prefers-color-scheme: dark)"
                    srcset=(asset.dark_mode.srcset)
                    type=(asset.dark_mode.mime_type) {}

                @let loading = if above_the_fold { "eager" } else { "lazy" };
                @let fetch_priority = if is_largest_contentful_paint { "high" } else { "auto" };
                img
                    loading=(loading)
                    fetchpriority=(fetch_priority)
                    alt=(asset.light_mode.alt)
                    class="absolute top-0 left-0 min-w-full min-h-full object-cover"
                    src=(asset.light_mode.src) {}
            }
       }
    }
}

// source {
//     //
//     "media": "(prefers-color-scheme: light)",
//     "srcset": asset.light_mode.srcset(),
//     "type": asset.light_mode.mime_type()
// }

// source {
//     //
//     "media": "(prefers-color-scheme: dark)",
//     "srcset": asset.dark_mode.srcset(),
//     "type": asset.dark_mode.mime_type()
// }

// img {
//     //
//     "loading": if *above_the_fold { "eager" } else { "lazy" },
//     "fetchpriority": if *is_largest_contentful_paint { "high" } else { "auto" },
//     alt: asset.alt,
//     class: "absolute top-0 left-0 min-w-full min-h-full object-cover",
//     src: asset.light_mode.src()
// }
