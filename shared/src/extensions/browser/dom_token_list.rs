// use crate::prelude::*;
use web_sys::DomTokenList;

pub trait DomTokenListExtension {
    fn into_iter(self) -> DomTokenListIterator;
}

impl DomTokenListExtension for DomTokenList {
    fn into_iter(self) -> DomTokenListIterator {
        DomTokenListIterator {
            index: 0,
            dom_token_list: self,
        }
    }
}

pub struct DomTokenListIterator {
    index: u32,
    dom_token_list: DomTokenList,
}

impl Iterator for DomTokenListIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let class = self.dom_token_list.get(self.index);
        self.index += 1;
        class
    }
}
