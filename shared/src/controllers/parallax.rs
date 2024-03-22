pub static NAME: &str = "controller:parallax";

#[cfg(feature = "browser")]
pub use self::browser::mount_parallax;

#[cfg(feature = "server")]
pub use self::server::parallax;

#[cfg(feature = "browser")]
pub mod browser {
    // use super::*;
    use gloo::events::EventListener;
    use web_sys::Event;
    use web_sys::HtmlElement;

    pub fn mount_parallax(target_element: HtmlElement) {
        let window = web_sys::window().expect("web_sys::window() failed.");
        let document = window.document().expect("window.document() failed.");

        let apply_parallax = move || {
            let scroll_y = window.scroll_y().expect("window.scroll_y() failed.");
            // Start
            let min = -16.0;
            let parallax_scroll_amount = min + scroll_y / 10.0;
            let translate_y = format!("{}px", parallax_scroll_amount);

            // We set the --tw-translate-y property instead of setting
            // transform: translateY() directly because we want to
            // play nice with Tailwind. If we just the style value directly
            // it would wipe out any other transform values that Tailwind
            // might have set.
            target_element
                .style()
                .set_property("--tw-translate-y", &translate_y)
                .expect("set_property failed.");
        };

        EventListener::new(&document, "scroll", move |_: &Event| {
            apply_parallax();
        })
        .forget();
    }
}

#[cfg(feature = "server")]
pub mod server {
    use super::*;

    pub fn parallax() -> String {
        format!("{NAME} -translate-y-[16px] scale-105")
    }
}
