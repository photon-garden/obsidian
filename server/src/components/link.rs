use crate::css_class_groups::link_classes;
use maud::{html, Markup, Render};
use shared::route::Route;

pub struct Link<'class> {
    class: String,
    href: Option<Route>,
    slot: Markup,
    default_classes: &'class str,
}

impl<'class> Link<'class> {
    pub fn new() -> Self {
        Self {
            class: "".to_string(),
            href: None,
            slot: html! {},
            default_classes: link_classes(),
        }
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = class.into();
        self
    }

    pub fn href(mut self, href: Route) -> Self {
        self.href = Some(href);
        self
    }

    pub fn slot(mut self, slot: impl Render) -> Self {
        self.slot = slot.render();
        self
    }

    pub fn without_default_classes(mut self) -> Self {
        self.default_classes = "";
        self
    }
}

impl<'class> Render for Link<'class> {
    fn render(&self) -> Markup {
        let href = match self.href {
            Some(ref href) => href.to_string(),
            None => "javascript:void(0);".to_string(),
        };

        html! {
            a
                class={(self.class) " " (self.default_classes)}
                href=(href) {
                (self.slot)
            }
        }
    }
}
