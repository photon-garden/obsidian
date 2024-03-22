use super::*;

pub struct LightDarkImageAsset {
    pub light_mode: ImageAsset,
    pub dark_mode: ImageAsset,
    pub placeholder: LightDarkPlaceholder,
}

impl LightDarkImageAsset {
    pub fn new(light_mode: ImageAsset, dark_mode: ImageAsset) -> LightDarkImageAsset {
        let placeholder =
            LightDarkPlaceholder::new(&light_mode.placeholder, &dark_mode.placeholder);

        LightDarkImageAsset {
            light_mode,
            dark_mode,
            placeholder,
        }
    }
}

#[derive(PartialEq)]
pub enum LightDarkPlaceholder {
    Lqip {
        light_mode_data_uri: String,
        dark_mode_data_uri: String,
    },
    Color {
        light_mode_css_string: String,
        dark_mode_css_string: String,
    },
}

impl LightDarkPlaceholder {
    pub fn new(light_mode: &Placeholder, dark_mode: &Placeholder) -> LightDarkPlaceholder {
        match (light_mode, dark_mode) {
            (
                Placeholder::Lqip {
                    data_uri: light_mode_data_uri,
                },
                Placeholder::Lqip {
                    data_uri: dark_mode_data_uri,
                },
            ) => LightDarkPlaceholder::Lqip {
                light_mode_data_uri: light_mode_data_uri.clone(),
                dark_mode_data_uri: dark_mode_data_uri.clone(),
            },
            (
                Placeholder::Color {
                    css_string: light_mode_css_string,
                },
                Placeholder::Color {
                    css_string: dark_mode_css_string,
                },
            ) => LightDarkPlaceholder::Color {
                light_mode_css_string: light_mode_css_string.clone(),
                dark_mode_css_string: dark_mode_css_string.clone(),
            },
            (_, _) => panic!(
                "When defining a light-dark image asset, all images must have the same kind of placeholder (LQIP or color)."
            ),
        }
    }
}
