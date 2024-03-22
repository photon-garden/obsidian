// pub mod replace;
pub mod parallax;
pub mod replace;
pub mod show_hide;
pub mod show_if_scrolled;

fn get_class(controller_name: &str, props: impl serde::Serialize) -> String {
    use base64::Engine;
    let json_props = serde_json::to_string(&props).expect("Failed to serialize props.");
    let base64_props = base64::prelude::BASE64_STANDARD.encode(json_props.as_bytes());

    format!(
        "{name}:{props}",
        name = controller_name,
        props = base64_props
    )
}

pub fn controller_name_to_selector(controller_name: &str) -> String {
    // format!(".{}", component_name).replace(':', "\\:")

    // This matches any element who has a class starting
    // with `component_name`. This is important for controllers
    // with props, which are marked by a class like this:
    //
    // controller:replace:ewogIHJvdXRlOiAiL3BvZW1zLzEyMyIKfQ==
    //
    // Where the random-looking string at the end is the base64
    // encoding of the props.
    //
    // In contrast, controllers without props are marked by a class
    // like this:
    //
    // controller:show-hide
    format!(
        "[class^='{}'], [class*=' {}']",
        controller_name, controller_name
    )
}
