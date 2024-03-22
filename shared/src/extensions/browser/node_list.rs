// use crate::prelude::*;
use web_sys::{Node, NodeList};

pub trait NodeListExtension {
    fn into_iter(self) -> NodeListIterator;
}

impl NodeListExtension for NodeList {
    fn into_iter(self) -> NodeListIterator {
        NodeListIterator {
            index: 0,
            node_list: self,
        }
    }
}

pub struct NodeListIterator {
    index: u32,
    node_list: NodeList,
}

impl Iterator for NodeListIterator {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node_list.get(self.index);
        self.index += 1;
        node
    }
}
