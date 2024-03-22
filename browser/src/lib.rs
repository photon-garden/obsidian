// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// use gloo::console;
use prelude::*;
use shared::controllers::*;
use shared::extensions::browser::*;
use web_sys::HtmlElement;
// use serde::de::DeserializeOwned;
// use shared::prelude::*;

#[cfg(feature = "dev")]
mod dev;
mod extensions;
mod prelude;

// Called when the wasm module is instantiated.
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    #[cfg(feature = "dev")]
    dev::main();

    mount_controller(
        show_if_scrolled::NAME,
        show_if_scrolled::mount_show_if_scrolled,
    );
    mount_controller(parallax::NAME, parallax::mount_parallax);
    mount_controller(show_hide::NAME, show_hide::mount_show_hide);
    mount_controller_with_props(replace::NAME, replace::mount_replace);

    // alert("Hello from Rust!");

    Ok(())
}

fn mount_controller(controller_name: &'static str, mount: fn(HtmlElement)) {
    // console::log!("Mounting components named:", controller_name);

    let window = web_sys::window().expect("web_sys::window() failed.");
    let document = window.document().expect("window.document() failed.");

    let elements = document.find_controllers(controller_name);

    for element in elements {
        mount(element);
    }
}

fn mount_controller_with_props<Props: serde::de::DeserializeOwned>(
    controller_name: &'static str,
    mount: fn(HtmlElement, Props),
) {
    // console::log!("Mounting components named:", controller_name);

    let window = web_sys::window().expect("web_sys::window() failed.");
    let document = window.document().expect("window.document() failed.");

    let elements = document.find_controllers(controller_name);

    for element in elements {
        let props = element
            .parse_props::<Props>(controller_name)
            .expect("Failed to parse props.");
        mount(element, props);
    }
}

// #[wasm_bindgen]
// extern "C" {
//     // JS alert function.
//     fn alert(s: &str);
// }

// Callable from JS.
// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello!");
// }
