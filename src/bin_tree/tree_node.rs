use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::{Rc, Weak};
use itertools::Itertools;


pub(super) type NodePointer<T> = Rc<RefCell<Node<T>>>;
pub(super) type NodeWeak<T> = Weak<RefCell<Node<T>>>;

pub struct Node<T: Ord> {
    pub(super) parent: NodeWeak<T>,
    pub(super) val: T,
    pub(super) left: Option<NodePointer<T>>,
    pub(super) right: Option<NodePointer<T>>,
}

impl<T: Ord> Node<T> {
    pub(super) fn new(val: T, parent: NodeWeak<T>) -> Self {
        Node {
            parent,
            val,
            left: None,
            right: None,
        }
    }

    pub(super) fn new_pointer(val: T, parent: NodeWeak<T>) -> NodePointer<T> {
        Rc::new(RefCell::new(Node::new(val, parent)))
    }

    pub fn val(&self) -> &T {
        &self.val
    }
}

pub(super) fn print_node<T: Debug + Ord>(f: &mut Formatter, node: &NodePointer<T>, road: String) {
        let node = node.borrow();

        if let Some(right) = &node.right {
            print_node(f, right, road.clone() + "u");
        }

        write!(f, "   ").unwrap();
        for (x,y) in road.chars().tuple_windows() {
            if x != y {
                write!(f, "│  ").unwrap();
            } else {
                write!(f, "   ").unwrap();
            }
        }
        if let Some(last) = road.chars().last() {
            if last == 'u' {
                write!(f, "╭──").unwrap();
            } else {
                write!(f, "╰──").unwrap();
            }
        }
        writeln!(f, "{:?}", node.val).unwrap();

        if let Some(left) = &node.left {
            print_node(f, left, road + "d");
        }
}