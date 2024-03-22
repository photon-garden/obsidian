use cfg_if::cfg_if;
use mime::Mime;

pub const LQIP_MIME_TYPE: Mime = mime::IMAGE_JPEG;
pub const RESIZED_IMAGE_MIME_TYPE: Mime = mime::IMAGE_JPEG;

#[derive(Clone)]
pub struct ImageAsset {
    pub alt: String,
    pub placeholder: Placeholder,

    pub width: u32,
    pub height: u32,

    pub srcset: String,
    pub src: String,

    pub mime_type: Mime,
}

#[derive(Clone)]
pub enum Placeholder {
    Lqip { data_uri: String },
    Color { css_string: String },
}

cfg_if! {
if #[cfg(feature = "build_time")] {

    use proc_macro2::TokenStream;
    use quote::{quote, ToTokens};

    impl ToTokens for ImageAsset {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            log::info!("Converting ImageAsset to tokens.");

            let alt = &self.alt;
            let placeholder = match &self.placeholder {
                Placeholder::Lqip { data_uri } => {
                    quote! {
                        assets::Placeholder::Lqip {
                            data_uri: #data_uri.to_string(),
                        }
                    }
                }
                Placeholder::Color { css_string } => {
                    quote! {
                        assets::Placeholder::Color {
                            css_string: #css_string.to_string(),
                        }
                    }
                }
            };

            let width = self.width;
            let height = self.height;

            let srcset = &self.srcset;
            let src = &self.src;

            let mime_type = &self.mime_type.to_string();

            let quoted = quote! {
                assets::ImageAsset {
                    alt: #alt.to_string(),
                    placeholder: #placeholder,

                    width: #width,
                    height: #height,

                    srcset: #srcset.to_string(),
                    src: #src.to_string(),

                    mime_type: #mime_type.parse().unwrap(),
                }
            };

            // log::info!("quoted: {}", quoted);

            tokens.extend(quoted);

            // log::info!("Extended tokens.");
        }
    }

}
}
