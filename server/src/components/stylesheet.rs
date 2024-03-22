use assets::CssAsset;
use maud::{html, Markup};

pub fn stylesheet(asset: &CssAsset) -> Markup {
    html! {
        link rel="stylesheet" href={(asset.url_path.to_string_lossy())} type="text/css" media="screen";
    }
}
