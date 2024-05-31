use std::cell::Ref;
use crate::bin_tree::tree_node::{Node, NodePointer};

pub(super) struct TreeCursor<T: Ord> {
    current: NodePointer<T>,
    height: usize,
}

impl<T: Ord> TreeCursor<T> {
    pub(super) fn new(current: NodePointer<T>) -> Self {
        TreeCursor { current, height: 0 }
    }

    pub(super) fn current_pointer(&self) -> NodePointer<T> {
        self.current.clone()
    }

    pub(super)  fn current(&self) -> Ref<Node<T>> {
        self.current.borrow()
    }

    pub(super) fn height(&self) -> usize {
        self.height
    }
}