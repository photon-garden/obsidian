use assets::{ImageAsset, Placeholder};
use maud::{html, Markup, Render};

pub struct Image<'a> {
    pub asset: &'a ImageAsset,
    pub class: &'a str,
}

impl<'a> Image<'a> {
    pub fn new(asset: &'a ImageAsset) -> Self {
        Self { asset, class: "" }
    }

    #[allow(dead_code)]
    pub fn class(mut self, class: impl Into<&'a str>) -> Self {
        self.class = class.into();
        self
    }
}

impl Render for Image<'_> {
    fn render(&self) -> Markup {
        match &self.asset.placeholder {
            Placeholder::Color { css_string } => {
                image_with_color_placeholder(self.class, self.asset, css_string)
            }
            Placeholder::Lqip { data_uri } => image_with_lqip(self.class, self.asset, data_uri),
        }
    }
}

fn image_with_color_placeholder(
    class: &str,
    asset: &ImageAsset,
    placeholder_color_css_string: &str,
) -> Markup {
    html!(img
        class={(class) " select-none"}
        style={"background-color: " (placeholder_color_css_string)}
        alt=(asset.alt)
        src=(asset.src)
        srcset=(asset.srcset);
    )
}

fn image_with_lqip(class: &str, asset: &ImageAsset, data_uri: &str) -> Markup {
    html!(
        div
            class={"select-none relative overflow-hidden " (class)}
        {

            // LQIP.
            img
                alt=(asset.alt)
                class="shrink-0 min-w-full min-h-full object-cover"
                style="image-rendering: pixelated; image-rendering: -moz-crisp-edges; image-rendering: crisp-edges;"
                src=(data_uri);

            // Actual image.
            img
                alt=(asset.alt)
                class="absolute top-0 left-0 min-w-full min-h-full object-cover"
                src=(asset.src)
                srcset=(asset.srcset);
        }
    )
}
