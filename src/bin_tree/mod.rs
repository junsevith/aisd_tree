use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::bin_tree::tree_node::{Node, NodePointer, print_node};

mod tree_node;
mod tree_cursor;

pub struct BinTree<T: Ord> {
    root: Option<NodePointer<T>>,
    height: usize,
    size: usize,
}

impl<T: Ord> BinTree<T> {
    pub fn new() -> Self {
        BinTree { root: None, height: 0, size: 0 }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn update_height(&mut self, height: usize) {
        if height > self.height {
            self.height = height;
        }
    }

    pub fn max(&self) -> Option<NodePointer<T>> {
        match &self.root {
            None => None,
            Some(root) => Some(max_from(root.clone()))
        }
    }

    pub fn insert(&mut self, val: T) {
        match &self.root {
            None => {
                self.root = Some(Rc::new_cyclic(|weak| {
                    RefCell::new(Node::new(val, weak.clone()))
                }));
                self.height = 1;
                self.size = 1;
            }
            Some(root) => {
                let mut current = root.clone();
                let mut height = 2;

                loop {
                    let next;
                    {
                        let mut curr_ref = current.borrow_mut();
                        let child = if val < curr_ref.val {
                            &mut curr_ref.left
                        } else {
                            &mut curr_ref.right
                        };

                        match child {
                            Some(node) => {
                                next = node.clone();
                            }
                            None => {
                                self.update_height(height);
                                *child = Some(Node::new_pointer(val, Rc::downgrade(&current)));
                                break;
                            }
                        }
                    }
                    current = next;
                    height += 1;
                }
            }
        }
    }

    pub fn delete(&mut self, val: T) -> bool {
        let mut current = self.root.clone();
        let mut height = 1;
        loop {
            match current {
                None => {
                    return false;
                }
                Some(node) => {
                    let mut node = node.borrow_mut();
                    if val == node.val {
                        let mut replacement = None;
                        match (&node.left, &node.right) {
                            (None, None) => {
                                replacement = None;
                            }
                            (Some(node), None) | (None, Some(node)) => {
                                replacement = Some(node.clone());
                            }
                            (Some(left), Some(right)) => {

                            }
                        }
                        return true;
                    } else if val < node.val {
                        current = node.left.clone();
                    } else {
                        current = node.right.clone();
                    }
                    height += 1;
                }
            }
        }
    }
}

impl<T: Debug + Ord> Debug for BinTree<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if let Some(root) = &self.root {
            print_node(f, root, String::new());
        }
        Ok(())
    }
}

fn max_from<T: Ord>(start: NodePointer<T>) -> NodePointer<T> {
    let mut current = start;
    while let Some(right) = current.clone().borrow().right.clone() {
        current = right;
    }
    current
}
