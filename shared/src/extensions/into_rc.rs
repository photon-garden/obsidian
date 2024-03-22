use std::rc::Rc;

pub trait IntoRcExtension<T> {
    fn into_rc(self) -> Rc<T>;
}

impl<T> IntoRcExtension<T> for T {
    fn into_rc(self) -> Rc<T> {
        Rc::new(self)
    }
}
