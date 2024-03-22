use maud::{html, Markup};

pub fn page() -> Markup {
    html! {
        h1 { "404" }
        p { "Page not found." }
    }
}
