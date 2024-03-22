pub static NAME: &str = "controller:show-hide";
static SHOW_BY_DEFAULT_NAME: &str = "controller:show-hide:show-by-default";
static HIDE_BY_DEFAULT_NAME: &str = "controller:show-hide:hide-by-default";
static TOGGLE_NAME: &str = "controller:show-hide:toggle";

#[cfg(feature = "browser")]
pub use self::browser::*;

#[cfg(feature = "server")]
pub use self::server::*;

#[cfg(feature = "browser")]
pub mod browser {
    use super::super::controller_name_to_selector;
    use super::*;
    use crate::extensions::*;
    use gloo::console;
    use gloo::events::EventListener;
    use std::cell::Cell;
    use wasm_bindgen::JsCast;
    use web_sys::HtmlElement;
    use web_sys::ScrollBehavior;
    use web_sys::ScrollIntoViewOptions;

    pub fn mount_show_hide(target_element: HtmlElement) {
        let container = target_element;

        let elements_to_show_by_default =
            container.find_controllers(SHOW_BY_DEFAULT_NAME).into_rc();

        let elements_to_hide_by_default =
            container.find_controllers(HIDE_BY_DEFAULT_NAME).into_rc();

        let maybe_scroll_anchor = get_scroll_anchor(&container).into_rc();

        let toggles = {
            let toggles = container.find_controllers(TOGGLE_NAME);
            if toggles.is_empty() {
                let selector = controller_name_to_selector(TOGGLE_NAME);
                console::warn!(
                    "Expected at least one child element inside of",
                    container,
                    "to match the selector",
                    selector,
                    "but didn't find any."
                );
                return;
            }
            toggles
        };

        let in_default_state = Cell::new(true).into_rc();

        for toggle in toggles {
            let in_default_state = in_default_state.clone();
            let elements_to_show_by_default = elements_to_show_by_default.clone();
            let elements_to_hide_by_default = elements_to_hide_by_default.clone();
            let maybe_scroll_anchor = maybe_scroll_anchor.clone();

            EventListener::new(&toggle, "click", move |_event| {
                if in_default_state.get() {
                    // Leave the default state.
                    in_default_state.set(false);
                    elements_to_show_by_default.hide();
                    elements_to_hide_by_default.show();
                } else {
                    // Enter the default state.
                    in_default_state.set(true);
                    elements_to_show_by_default.show();
                    elements_to_hide_by_default.hide();
                }
                if let Some(scroll_anchor) = maybe_scroll_anchor.as_ref() {
                    let mut options = ScrollIntoViewOptions::new();
                    options.behavior(ScrollBehavior::Instant);
                    scroll_anchor.scroll_into_view_with_scroll_into_view_options(&options);
                }
            })
            .forget();
        }
    }

    fn get_scroll_anchor(container: &HtmlElement) -> Option<HtmlElement> {
        let previous_container = container.previous_element_sibling()?;
        let maybe_anchor = previous_container
            .query_selector(".scroll-anchor-when-hiding-publication")
            .ok()?;
        maybe_anchor.map(|element| element.dyn_into::<HtmlElement>().expect("HtmlElement"))
    }
}

#[cfg(feature = "server")]
pub mod server {
    use super::*;

    #[derive(PartialEq, Clone)]
    pub struct ShowHide {}

    impl ShowHide {
        pub fn new() -> ShowHide {
            ShowHide {}
        }

        pub fn container(&self) -> &'static str {
            NAME
        }

        pub fn toggle(&self) -> &'static str {
            TOGGLE_NAME
        }

        pub fn show_by_default(&self) -> &'static str {
            SHOW_BY_DEFAULT_NAME
        }

        pub fn hide_by_default(&self) -> String {
            format!("{HIDE_BY_DEFAULT_NAME} hidden")
        }
    }

    impl Default for ShowHide {
        fn default() -> Self {
            ShowHide::new()
        }
    }
}
