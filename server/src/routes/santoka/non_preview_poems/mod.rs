use super::*;
use maud::{html, Markup};

pub fn page(publication_id: PublicationId) -> Markup {
    let publication = DATABASE.publication(publication_id);
    html! {
        @for non_preview_poem in publication.non_preview_poems() {
            (poem(non_preview_poem))
        }
    }
}
