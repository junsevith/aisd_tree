use std::cell::{RefCell, RefMut};
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::bin_tree::tree_node::{Node, NodePointer};
use crate::experiment::Stats;

mod tree_node;

pub struct BinTree<T: Ord> {
    root: Option<NodePointer<T>>,
    size: usize,
}

impl<T: Ord> BinTree<T> {
    pub fn new() -> Self {
        BinTree { root: None, size: 0 }
    }

    pub fn height(&self) -> usize {
        match &self.root {
            None => 0,
            Some(root) => root.borrow().measure_height()
        }
    }

    pub fn height2(&self) -> usize {
        match &self.root {
            None => 0,
            Some(root) => {
                let mut max_height = 1;
                let mut queue = VecDeque::new();
                queue.push_back((1,root.clone()));
                while !queue.is_empty() {
                    let (height, node) = queue.pop_front().unwrap();
                    let node_ref = node.borrow();

                    if height > max_height {
                        max_height = height;
                    }

                    if let Some(left) = &node_ref.left {
                        queue.push_back((height+1, left.clone()));
                    }
                    if let Some(right) = &node_ref.right {
                        queue.push_back((height+1, right.clone()));
                    }
                }
                max_height
            }
        }
    }

    pub fn insert(&mut self, val: T, stats: &mut Stats) {
        self.size += 1;
        match &self.root {
            None => {
                stats.swap();
                self.root = Some(Rc::new_cyclic(|weak| {
                    RefCell::new(Node::new(val, weak.clone()))
                }));
                self.size = 1;
            }
            Some(root) => {
                stats.read();
                let mut current = root.clone();
                loop {
                    current = {
                        let mut curr_ref = current.borrow_mut();

                        stats.comp();
                        let child = if val < curr_ref.val {
                            &mut curr_ref.left
                        } else {
                            &mut curr_ref.right
                        };

                        match child {
                            Some(child) => {
                                stats.read();
                                child.clone()
                            }
                            None => {
                                stats.swap();
                                *child = Some(Node::new_pointer(val, Rc::downgrade(&current)));
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn delete(&mut self, val: T, stats: &mut Stats) -> bool {
        if self.root.is_none() {
            return false;
        } else {
            // handling case when root is to be deleted
            let mut comp;
            {
                stats.read();
                let root_ref = self.root.as_ref().unwrap().borrow_mut();

                stats.comp();
                comp = Ord::cmp(&val, &root_ref.val);
                if comp.is_eq() {
                    // delete root
                    let new = get_replacement(root_ref, stats);

                    stats.swap();
                    self.root = new;

                    match &self.root {
                        Some(new) => {
                            stats.swap();
                            new.borrow_mut().parent = Rc::downgrade(&self.root.as_ref().unwrap());
                        }
                        _ => {}
                    }
                    self.size -= 1;
                    return true;
                }
            }

            //when root is not the one to be deleted
            stats.read();
            let mut current = self.root.clone().unwrap();
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
                            let child_ref = child_ptr.borrow_mut();

                            stats.comp();
                            //comparing values
                            comp = Ord::cmp(&val, &child_ref.val);

                            if comp.is_eq() {
                                // delete
                                let new = get_replacement(child_ref, stats);
                                stats.swap();
                                *child = new;

                                self.size -= 1;
                                return true;

                            } else {
                                stats.read();
                                child_ptr.clone()
                            }
                        }

                    }
                }
            }
        }
    }

    // #[deprecated]
    // fn delete3(&mut self, val: T) -> bool {
    //     if self.root.is_none() {
    //         return false;
    //     } else {
    //         let mut current_ptr = self.root.clone();
    //         let mut height = 2;
    //
    //         let mut fin = false;
    //         loop {
    //             current_ptr = {
    //                 let new = match &current_ptr {
    //                     None => {
    //                         return false;
    //                     }
    //                     Some(node) => {
    //                         let mut node_ref = node.borrow_mut();
    //                         if node_ref.val == val {
    //                             // delete
    //                             let mut new = get_replacement(node_ref, stats);
    //
    //                             fin = true;
    //                             new
    //                         } else if val < node_ref.val {
    //                             node_ref.left.clone()
    //                         } else {
    //                             node_ref.right.clone()
    //                         }
    //                     }
    //                 };
    //                 if fin {
    //                     current_ptr.unwrap().swap(&mut new.unwrap());
    //                     return true;
    //                 } else {
    //                     new
    //                 }
    //             };
    //             height += 1;
    //         }
    //     }
    // }
}


fn get_replacement<T: Ord>(mut node: RefMut<Node<T>>, stats: &mut Stats) -> Option<NodePointer<T>> {
    let mut new =
        match (&node.left, &node.right) {
        (Some(_left), Some(right)) => {
            let replacement = min_from(right.clone(), stats);
            {
                let mut replacement_ref = replacement.borrow_mut();

                //connecting replacement's children to replacement's parent
                stats.read();
                stats.swap();
                if Rc::ptr_eq(&replacement, right) {
                    node.right = replacement_ref.right.clone();
                } else {
                    replacement_ref.parent.upgrade().unwrap().borrow_mut().left = replacement_ref.right.clone();
                }

                //setting correct parent in replacement's former child
                if let Some(new_right) = &replacement_ref.right {
                    stats.swap();
                    new_right.borrow_mut().parent = replacement_ref.parent.clone();
                }

                // if let Some(new_left) = &replacement_ref.left {
                //     new_left.borrow_mut().parent = replacement_ref.parent.clone();
                // }

                stats.swap();
                stats.swap();
                //setting replacement's children to be the same as node's children
                replacement_ref.left = node.left.clone();
                replacement_ref.right = node.right.clone();


                //setting correct parent in replacement's children
                if let Some(left) = &replacement_ref.left {
                    stats.swap();
                    left.borrow_mut().parent = Rc::downgrade(&replacement);
                }
                if let Some(right) = &replacement_ref.right {
                    stats.swap();
                    right.borrow_mut().parent = Rc::downgrade(&replacement);
                }
            }

            Some(replacement)
        }
        (Some(left), None) => {
            stats.swap();
            let ret = Some(left.clone());
            node.left = None;
            ret
        }
        (None, Some(right)) => {
            stats.swap();
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

        stats.swap();
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

fn max_from<T: Ord>(start: NodePointer<T>, stats: &mut Stats) -> NodePointer<T> {
    let mut current = start;
    // while let Some(right) = current.clone().borrow().right.clone() {
    //     current = right;
    // }

    loop {
        current = {
            let current_ref = current.borrow();
            stats.read();
            match &current_ref.right {
                Some(right) => right.clone(),
                None => break,
            }
        }
    }
    current
}

fn min_from<T: Ord>(start: NodePointer<T>, stats: &mut Stats) -> NodePointer<T> {
    let mut current = start;
    loop {
        current = {
            let current_ref = current.borrow();
            stats.read();
            match &current_ref.left {
                Some(left) => left.clone(),
                None => break,
            }
        }
    }
    current
}


