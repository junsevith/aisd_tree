use std::cell::{RefCell, RefMut};
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::bin_tree::tree_node::{Node, NodePointer};

mod tree_node;

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
        match &self.root {
            None => 0,
            Some(root) => root.borrow().measure_height()
        }
    }

    pub fn max(&self) -> Option<NodePointer<T>> {
        match &self.root {
            None => None,
            Some(root) => Some(max_from(root.clone()))
        }
    }

    pub fn insert(&mut self, val: T) {
        self.size += 1;
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
                    current = {
                        let mut curr_ref = current.borrow_mut();
                        let child = if val < curr_ref.val {
                            &mut curr_ref.left
                        } else {
                            &mut curr_ref.right
                        };

                        match child {
                            Some(child) => {
                                child.clone()
                            }
                            None => {
                                if height > self.height {
                                    self.height = height;
                                }
                                *child = Some(Node::new_pointer(val, Rc::downgrade(&current)));
                                break;
                            }
                        }
                    };
                    height += 1;
                }
            }
        }
    }

    pub fn delete(&mut self, val: T) -> bool {
        self.size -= 1;
        if self.root.is_none() {
            return false;
        } else {
            // handling case when root is to be deleted
            {
                let mut root_ref = self.root.as_ref().unwrap().borrow_mut();
                if root_ref.val == val {
                    // delete root
                    let new = get_replacement(root_ref);
                    self.root = new;
                    match &self.root {
                        Some(new) => {
                            new.borrow_mut().parent = Rc::downgrade(&self.root.as_ref().unwrap());
                        }
                        _ => {}
                    }
                    return true;
                }
            }

            //when root is not the one to be deleted
            let mut current = self.root.clone().unwrap();
            // let mut child = &mut self.root;
            let mut comp = Ord::cmp(&val, &current.borrow().val);
            loop {
                current = {
                    let mut current_ref = current.borrow_mut();

                    //because of the precious iteration we don't need to check if values are equal
                    let child = if comp.is_lt() {
                        &mut current_ref.left
                    } else {
                        &mut current_ref.right
                    };

                    match child {
                        None => {
                            return false;
                        }
                        Some(child_ptr) => {
                            let mut child_ref = child_ptr.borrow_mut();

                            //comparing values
                            comp = Ord::cmp(&val, &child_ref.val);

                            if comp.is_eq() {
                                // delete
                                let new = get_replacement(child_ref);
                                *child = new;

                                return true;

                            } else {
                                child_ptr.clone()
                            }
                        }

                    }
                };
            }
        }
    }

    #[deprecated]
    fn delete3(&mut self, val: T) -> bool {
        if self.root.is_none() {
            return false;
        } else {
            let mut current_ptr = self.root.clone();
            let mut height = 2;

            let mut fin = false;
            loop {
                current_ptr = {
                    let new = match &current_ptr {
                        None => {
                            return false;
                        }
                        Some(node) => {
                            let mut node_ref = node.borrow_mut();
                            if node_ref.val == val {
                                // delete
                                let mut new = get_replacement(node_ref);

                                fin = true;
                                new
                            } else if val < node_ref.val {
                                node_ref.left.clone()
                            } else {
                                node_ref.right.clone()
                            }
                        }
                    };
                    if fin {
                        current_ptr.unwrap().swap(&mut new.unwrap());
                        if height == self.height {
                            self.height -= 1;
                        }
                        return true;
                    } else {
                        new
                    }
                };
                height += 1;
            }
        }
    }
}


fn get_replacement<T: Ord>(mut node: RefMut<Node<T>>) -> Option<NodePointer<T>> {
    let mut new =
        match (&node.left, &node.right) {
        (Some(_left), Some(right)) => {
            let replacement = min_from(right.clone());
            {
                let mut replacement_ref = replacement.borrow_mut();

                //connecting replacement's children to replacement's parent
                if Rc::ptr_eq(&replacement, right) {
                    node.right = replacement_ref.right.clone();
                } else {
                    replacement_ref.parent.upgrade().unwrap().borrow_mut().left = replacement_ref.right.clone();
                }

                //setting correct parent in replacement's former child
                if let Some(new_right) = &replacement_ref.right {
                    new_right.borrow_mut().parent = replacement_ref.parent.clone();
                }

                // if let Some(new_left) = &replacement_ref.left {
                //     new_left.borrow_mut().parent = replacement_ref.parent.clone();
                // }

                //setting replacement's children to be the same as node's children
                replacement_ref.left = node.left.clone();
                replacement_ref.right = node.right.clone();


                //setting correct parent in replacement's children
                if let Some(left) = &replacement_ref.left {
                    left.borrow_mut().parent = Rc::downgrade(&replacement);
                }
                if let Some(right) = &replacement_ref.right {
                    right.borrow_mut().parent = Rc::downgrade(&replacement);
                }
            }

            Some(replacement)
        }
        (Some(left), None) => {
            let ret = Some(left.clone());
            node.left = None;
            ret
        }
        (None, Some(right)) => {
            let ret = Some(right.clone());
            node.right = None;
            ret
        }
        (None, None) => {
            None
        }
    };

    if let Some(new) = &mut new {
        let mut new_ref = new.borrow_mut();

        //setting correct parent in replacement
        new_ref.parent = node.parent.clone();
        // new_ref.left = node.left.clone();
        // new_ref.right = node.right.clone();

    };

    new
}


impl<T: Debug + Ord> Debug for BinTree<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if let Some(root) = &self.root {
            root.borrow().print_node(f, String::new());
        } else {
            write!(f, "Empty tree")?;
        }
        Ok(())
    }
}

fn max_from<T: Ord>(start: NodePointer<T>) -> NodePointer<T> {
    let mut current = start;
    // while let Some(right) = current.clone().borrow().right.clone() {
    //     current = right;
    // }

    loop {
        current = {
            let current_ref = current.borrow();
            match &current_ref.right {
                Some(right) => right.clone(),
                None => break,
            }
        }
    }
    current
}

fn min_from<T: Ord>(start: NodePointer<T>) -> NodePointer<T> {
    let mut current = start;
    loop {
        current = {
            let current_ref = current.borrow();
            match &current_ref.left {
                Some(left) => left.clone(),
                None => break,
            }
        }
    }
    current
}


