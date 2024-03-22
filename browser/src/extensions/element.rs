use web_sys::Element;

pub trait ElementExtension {
    fn show(&self);
    fn hide(&self);
}

impl ElementExtension for Element {
    fn show(&self) {
        self.class_list().remove_1("hidden").unwrap();
    }

    fn hide(&self) {
        self.class_list().add_1("hidden").unwrap();
    }
}
