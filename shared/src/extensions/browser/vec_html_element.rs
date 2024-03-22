use super::*;
use web_sys::HtmlElement;

pub trait VecHtmlElementExtension {
    fn show(&self);
    fn hide(&self);
}

impl VecHtmlElementExtension for Vec<HtmlElement> {
    fn show(&self) {
        for element in self.iter() {
            element.show();
        }
    }

    fn hide(&self) {
        for element in self.iter() {
            element.hide();
        }
    }
}
