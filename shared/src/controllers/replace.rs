use crate::route::Route;
use serde::{Deserialize, Serialize};

pub static NAME: &str = "controller:replace";

#[cfg(feature = "browser")]
pub use self::browser::*;

#[cfg(feature = "server")]
pub use self::server::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Props {
    load_html_from: Route,
}

#[cfg(feature = "browser")]
pub mod browser {
    use super::*;
    use crate::extensions::IntoRcExtension;
    use gloo::console;
    use gloo::events::EventListener;
    use gloo::net::http::Request;
    use wasm_bindgen_futures::spawn_local;
    use web_sys::HtmlElement;

    pub fn mount_replace(target_element: HtmlElement, props: Props) {
        let target_element_rc = target_element.into_rc();
        let outer_target_element = target_element_rc.clone();
        let inner_target_element = target_element_rc.clone();

        let load_html_from = props.load_html_from;

        // We use `once` since we replace this element and we don't
        // want to leak event listeners.
        EventListener::once(&outer_target_element, "click", move |_| {
            spawn_local(async move {
                let maybe_new_html = get_text_from_url(&load_html_from.to_string()).await;
                match maybe_new_html {
                    Ok(new_html) => {
                        inner_target_element.set_outer_html(&new_html);
                    }
                    Err(error) => {
                        let formatted_error = format!("{:?}", error);
                        console::error!("Error fetching new html: {}", formatted_error);
                    }
                }
            });
        })
        .forget();
    }

    async fn get_text_from_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = Request::get(url).send().await?;
        let text = response.text().await?;
        Ok(text)
    }
}

#[cfg(feature = "server")]
pub mod server {
    use super::super::get_class;
    use super::*;
    use crate::route::Route;

    pub fn replace(load_html_from: Route) -> String {
        get_class(NAME, Props { load_html_from })
    }
}
