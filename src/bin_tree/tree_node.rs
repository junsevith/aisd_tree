use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::{Rc, Weak};
use itertools::Itertools;


pub(super) type NodePointer<T> = Rc<RefCell<Node<T>>>;
pub(super) type NodeWeak<T> = Weak<RefCell<Node<T>>>;

pub enum Direction {
    Left,
    Right,
}

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

    pub(super) fn measure_height(&self) -> usize {
        let left_height = match &self.left {
            Some(left) => left.borrow().measure_height(),
            None => 0,
        };
        let right_height = match &self.right {
            Some(right) => right.borrow().measure_height(),
            None => 0,
        };
        1 + left_height.max(right_height)
    }
}

impl<T: Ord + Debug> Node<T> {
    pub(super) fn print_node(&self, f: &mut Formatter, road: String) {
        if let Some(right) = &self.right {
            right.borrow().print_node(f, road.clone() + "u");
        }

        write!(f, "   ").unwrap();
        for (x, y) in road.chars().tuple_windows() {
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
        writeln!(f, "{:?}", &self.val).unwrap();

        if let Some(left) = &self.left {
            left.borrow().print_node(f, road + "d");
        }
    }
}

