use std::cell::RefCell;
use std::rc::Rc;
use crate::rb_tree::tree_node::{Color, Node, NodePointer, NodeWeak};

pub fn new_pointer<T: Ord>(val: T, parent: NodeWeak<T>) -> NodePointer<T> {
    Rc::new(RefCell::new(Node::new(val, parent)))
}

pub fn parent<T: Ord>(node: &NodePointer<T>) -> NodePointer<T> {
    node.borrow().parent()
}

pub fn color<T: Ord>(node: &Option<NodePointer<T>>) -> Color {
    match node {
        Some(node) => node.borrow().color.clone(),
        None => Color::Black,
    }
}


