use maud::Markup;

pub trait MarkupExtension {
    fn into_axum_html_response(self) -> axum::response::Html<String>;
}

impl MarkupExtension for Markup {
    fn into_axum_html_response(self) -> axum::response::Html<String> {
        let body = self.into_string();
        axum::response::Html(body)
    }
}
