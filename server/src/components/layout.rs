use maud::{html, Markup, PreEscaped, Render, DOCTYPE};
use shared::route::Route;

use crate::assets::ASSETS;
use crate::components::*;
use crate::css_class_groups::*;

pub struct Layout {
    title: &'static str,
    description: &'static str,
    slot: Markup,
}

impl Layout {
    pub fn new(title: &'static str, description: &'static str, slot: Markup) -> Self {
        Self {
            title,
            description,
            slot,
        }
    }
}

impl Render for Layout {
    fn render(&self) -> Markup {
        html! {
            (DOCTYPE)
            html lang="en" {
                head {
                    meta charset="UTF-8";
                    link rel="icon" href=(ASSETS.favicon.url_path.to_string_lossy().to_string());
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    meta http_equiv="X-UA-Compatible" content="ie=edge";
                    meta name="description" content=(self.description);
                    (stylesheet(&ASSETS.css))
                    title {
                        (self.title)
                    }
                }

                body
                    class={"
                        flex flex-col items-center selection:bg-neutral-200/75 dark:selection:bg-neutral-700/75 "
                        (bg_background())} {
                    (self.slot)
                    (main_js())
                }
            }
        }
    }
}

fn main_js() -> Markup {
    let browser_js_path = ASSETS
        .browser_crate
        .js
        .url_path
        .to_string_lossy()
        .to_string();
    let contents =
        include_str!("../assets/main.js").replace("{browser_js_filename}", &browser_js_path);
    html! {
        script type="module" {
            (PreEscaped(contents))
        }
    }
}
