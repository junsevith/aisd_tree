use std::cell::{RefCell, RefMut};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::experiment::Stats;
use crate::rb_tree::node_pointer::{color, new_pointer, parent};
use crate::rb_tree::tree_node::{Node, NodePointer};
use crate::rb_tree::tree_node::Color::{Black, Red};

mod tree_node;
mod node_pointer;

pub struct BinTree<T: Ord> {
    root: Option<NodePointer<T>>,
    nil: Option<NodePointer<T>>,
    size: usize,
}

impl<T: Ord + Debug> BinTree<T> {
    pub fn new() -> Self {
        BinTree { root: None, nil: None, size: 0 }
    }

    pub fn height(&self) -> usize {
        match &self.root {
            None => 0,
            Some(root) => root.borrow().measure_height()
        }
    }
    pub fn insert(&mut self, val: T, stats: &mut Stats) {
        let mut x = self.insert_helper(val, stats);

        while !self.check_root(&x)
            && parent(&x).borrow().color == Red {
            stats.read();
            stats.read();

            stats.read();
            if self.comp_ptr(&parent(&x), &parent(&parent(&x)).borrow().left) {
                stats.read();
                let y = parent(&parent(&x)).borrow().right.clone();

                if color(&y) == Red {
                    parent(&x).borrow_mut().color = Black;

                    y.unwrap().borrow_mut().color = Black;

                    parent(&parent(&x)).borrow_mut().color = Red;

                    x = parent(&parent(&x));
                } else {
                    stats.read();
                    if self.comp_ptr(&x, &parent(&x).borrow().right) {
                        x = parent(&x);

                        self.left_rotate(&x, stats);
                    }
                    parent(&x).borrow_mut().color = Black;

                    parent(&parent(&x)).borrow_mut().color = Red;

                    self.right_rotate(&parent(&parent(&x)), stats);
                }
            } else {
                stats.read();
                let y = parent(&parent(&x)).borrow().left.clone();

                if color(&y) == Red {
                    stats.read();
                    parent(&x).borrow_mut().color = Black;

                    y.unwrap().borrow_mut().color = Black;

                    parent(&parent(&x)).borrow_mut().color = Red;

                    x = parent(&parent(&x));
                } else {
                    stats.read();
                    if self.comp_ptr(&x, &parent(&x).borrow().left) {

                        x = parent(&x);

                        self.right_rotate(&x, stats);
                    }
                    parent(&x).borrow_mut().color = Black;

                    parent(&parent(&x)).borrow_mut().color = Red;

                    self.left_rotate(&parent(&parent(&x)), stats);
                }
            }
        }
        self.root.as_ref().unwrap().borrow_mut().color = Black;
    }

    fn insert_helper(&mut self, val: T, stats: &mut Stats) -> NodePointer<T> {
        self.size += 1;
        match &self.root {
            None => {
                stats.swap();
                self.root = Some(Rc::new_cyclic(|weak| {
                    RefCell::new(Node::new(val, weak.clone()))
                }));
                self.size = 1;
                self.root.as_ref().unwrap().borrow_mut().color = Black;
                self.root.clone().unwrap()
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
                                *child = Some(new_pointer(val, Rc::downgrade(&current)));
                                return child.clone().unwrap();
                            }
                        }
                    }
                }
            }
        }
    }

    fn delete_old(&mut self, val: T, stats: &mut Stats) -> bool {
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
                            new.borrow_mut().color = Black;
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
            let fix;
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
                                let black = child_ref.color == Black;
                                let new = get_replacement(child_ref, stats);

                                if black {
                                    fix = match &new {
                                        Some(new) => Some(new.clone()),
                                        None => Some(child_ptr.clone()),
                                    };
                                } else {
                                    fix = None;
                                }

                                stats.swap();
                                *child = new;

                                self.size -= 1;
                                break;
                            } else {
                                stats.read();
                                child_ptr.clone()
                            }
                        }
                    }
                }
            }
            if let Some(fix) = fix {
                self.delete_fixup(fix, stats);
            }
            return true;
        }
    }

    fn left_rotate(&mut self, node: &NodePointer<T>, stats: &mut Stats) {
        stats.read();
        let y = node.borrow().right.clone().unwrap();

        stats.swap();
        node.borrow_mut().right = y.borrow().left.clone();

        stats.read();
        if let Some(left) = &y.borrow().left {
            stats.swap();
            left.borrow_mut().parent = Rc::downgrade(node);
        };

        stats.swap();
        y.borrow_mut().parent = node.borrow().parent.clone();

        stats.read();
        if self.check_root(node) {
            stats.swap();
            y.borrow_mut().parent = Rc::downgrade(&y);

            stats.swap();
            self.root = Some(y.clone());
        } else if self.comp_ptr(&node, &parent(&node).borrow().left) {
            stats.read();

            stats.swap();
            parent(&node).borrow_mut().left = Some(y.clone());
        } else {
            stats.swap();
            parent(&node).borrow_mut().right = Some(y.clone());
        }

        stats.swap();
        y.borrow_mut().left = Some(node.clone());

        stats.swap();
        node.borrow_mut().parent = Rc::downgrade(&y);
    }

    fn right_rotate(&mut self, node: &NodePointer<T>, stats: &mut Stats) {
        stats.read();
        let y = node.borrow().left.clone().unwrap();

        stats.swap();
        node.borrow_mut().left = y.borrow().right.clone();

        stats.read();
        if let Some(right) = &y.borrow().right {
            stats.swap();
            right.borrow_mut().parent = Rc::downgrade(node);
        };
        stats.swap();
        y.borrow_mut().parent = node.borrow().parent.clone();

        stats.read();
        if self.check_root(node) {
            stats.swap();
            y.borrow_mut().parent = Rc::downgrade(&y);

            stats.swap();
            self.root = Some(y.clone());
        } else if self.comp_ptr(&node, &parent(&node).borrow().right) {
            stats.read();

            stats.swap();
            parent(&node).borrow_mut().right = Some(y.clone());
        } else {
            stats.read();

            stats.swap();
            parent(&node).borrow_mut().left = Some(y.clone());
        }

        y.borrow_mut().right = Some(node.clone());
        node.borrow_mut().parent = Rc::downgrade(&y);
    }


    fn check_root(&self, node: &NodePointer<T>) -> bool {
        Rc::ptr_eq(node, &parent(&node))
    }

    fn transplant(&mut self, u: &NodePointer<T>, v: &Option<NodePointer<T>>, stats: &mut Stats) {
        if let Some(v) = &v {
            stats.swap();
            v.borrow_mut().parent = Rc::downgrade(&parent(&u));
            //println!("transplant {:?} with {:?}", u.borrow().val, v.borrow().val);
        } else {
            //println!("transplant {:?} with nil", u.borrow().val);
        }

        stats.read();
        if self.check_root(&u) {
            if let Some(v) = &v {
                stats.swap();
                v.borrow_mut().parent = Rc::downgrade(&v);
            }
            self.root = v.clone();
        } else if self.comp_ptr(&u, &parent(&u).borrow().left) {
            stats.read();

            parent(&u).borrow_mut().left = v.clone();
        } else {
            stats.read();

            parent(&u).borrow_mut().right = v.clone();
        }
        stats.swap();
    }

    fn search(&self, val: T, stats: &mut Stats) -> Option<NodePointer<T>> {
        let mut current = self.root.clone();
        loop {
            match current {
                None => return None,
                Some(node) => {
                    let node_ref = node.borrow();
                    stats.comp();
                    stats.read();
                    match Ord::cmp(&val, &node_ref.val) {
                        std::cmp::Ordering::Less => current = node_ref.left.clone(),
                        std::cmp::Ordering::Greater => current = node_ref.right.clone(),
                        std::cmp::Ordering::Equal => return Some(node.clone()),
                    }
                }
            }
        }
    }

    pub fn delete(&mut self, val: T, stats: &mut Stats) -> bool {
        let z = self.search(val, stats);
        return match z {
            None => false,
            Some(z) => {
                let mut y = z.clone();
                let mut y_original_color = y.borrow().color.clone();
                let mut x;

                stats.read();
                if z.borrow().left.is_none() {
                    stats.read();
                    self.transplant(&z, &z.borrow().right, stats);

                    stats.read();
                    x = if z.borrow().right.is_some() {
                        z.borrow().right.clone().unwrap()
                    } else {
                        self.set_nil(z.clone(), stats)
                    };
                    // self.transplant(&z, &z.borrow().right);
                } else if z.borrow().right.is_none() {
                    stats.read();

                    stats.read();
                    x = z.borrow().left.clone().unwrap();

                    stats.read();
                    self.transplant(&z, &z.borrow().left, stats);
                } else {
                    stats.read();
                    y = min_from(z.borrow().right.clone().unwrap(), stats);

                    y_original_color = y.borrow().color.clone();

                    let mut fix_x = false;

                    stats.read();
                    x = if y.borrow().right.is_some() {
                        y.borrow().right.clone().unwrap()
                    } else {
                        fix_x = true;
                        parent(&y)
                        //self.set_nil(y.clone())
                    };

                    stats.read();
                    if Rc::ptr_eq(&parent(&y), &z) {
                        if !fix_x {
                            stats.swap();
                            x.borrow_mut().parent = Rc::downgrade(&y);
                        } else {
                            x = y.clone()
                        }
                    } else {
                        stats.read();
                        self.transplant(&y, &y.borrow().right, stats);

                        stats.swap();
                        y.borrow_mut().right = z.borrow().right.clone();

                        stats.swap();
                        y.borrow().right.as_ref().unwrap().borrow_mut().parent = Rc::downgrade(&y);
                    }
                    self.transplant(&z, &Some(y.clone()), stats);

                    stats.swap();
                    y.borrow_mut().left = z.borrow().left.clone();

                    stats.swap();
                    y.borrow().left.as_ref().unwrap().borrow_mut().parent = Rc::downgrade(&y);

                    stats.swap();
                    y.borrow_mut().color = z.borrow().color.clone();

                    if fix_x {
                        let nil = self.set_nil(z, stats);

                        stats.swap();
                        nil.borrow_mut().parent = Rc::downgrade(&x);
                        //println!("nil parent: {:?}", nil.borrow().parent.upgrade().unwrap().borrow().val);
                        x = nil;
                    }
                }

                if y_original_color == Black {
                    self.delete_fixup(x, stats);
                }
                true
            }
        };
    }

    fn delete_fixup(&mut self, mut x: NodePointer<T>, stats: &mut Stats) {
        //println!("delete fixup {:?}", x.borrow().val);

        stats.read();
        while !self.check_root(&x)
            && x.borrow().color == Black {
            //println!("{:?}", self);

            stats.read();
            if self.comp_ptr(&x, &parent(&x).borrow().left) {
                //println!("{:?} is left child", x.borrow().val);

                stats.read();
                let mut w = parent(&x).borrow().right.clone();

                //case 1
                if color(&w) == Red {
                    w.unwrap().borrow_mut().color = Black;

                    stats.read();
                    parent(&x).borrow_mut().color = Red;

                    stats.read();
                    self.left_rotate(&parent(&x), stats);

                    stats.read();
                    w = parent(&x).borrow().right.clone();
                }

                //case 2
                stats.read();
                stats.read();
                if color(&w.as_ref().unwrap().borrow().left) == Black
                    && color(&w.as_ref().unwrap().borrow().right) == Black {
                    w.unwrap().borrow_mut().color = Red;

                    stats.read();
                    x = parent(&x);
                } else {

                    //case 3
                    stats.read();
                    if color(&w.as_ref().unwrap().borrow().right) == Black {
                        stats.read();
                        w.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().color = Black;

                        w.as_ref().unwrap().borrow_mut().color = Red;

                        self.right_rotate(&w.unwrap(), stats);

                        stats.read();
                        w = parent(&x).borrow().right.clone();
                    }
                    //case 4
                    w.as_ref().unwrap().borrow_mut().color = parent(&x).borrow().color.clone();

                    stats.read();
                    parent(&x).borrow_mut().color = Black;

                    stats.read();
                    w.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().color = Black;

                    stats.read();
                    self.left_rotate(&parent(&x), stats);

                    stats.read();
                    x = self.root.clone().unwrap();
                }
            } else {
                //println!("{:?} is right child", x.borrow().val);
                stats.read();
                let mut w = parent(&x).borrow().left.clone();

                if color(&w) == Red {
                    w.unwrap().borrow_mut().color = Black;

                    stats.read();
                    parent(&x).borrow_mut().color = Red;

                    stats.read();
                    self.right_rotate(&parent(&x), stats);

                    stats.read();
                    w = parent(&x).borrow().left.clone();
                }

                stats.read();
                stats.read();
                if color(&w.as_ref().unwrap().borrow().right) == Black
                    && color(&w.as_ref().unwrap().borrow().left) == Black {
                    w.unwrap().borrow_mut().color = Red;

                    stats.read();
                    x = parent(&x);
                } else {
                    if color(&w.as_ref().unwrap().borrow().left) == Black {
                        stats.read();
                        w.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().color = Black;

                        w.as_ref().unwrap().borrow_mut().color = Red;

                        self.left_rotate(&w.unwrap(), stats);

                        stats.read();
                        w = parent(&x).borrow().left.clone();
                    }
                    stats.read();
                    w.as_ref().unwrap().borrow_mut().color = parent(&x).borrow().color.clone();

                    parent(&x).borrow_mut().color = Black;

                    stats.read();
                    w.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().color = Black;

                    stats.read();
                    self.right_rotate(&parent(&x), stats);

                    stats.read();
                    x = self.root.clone().unwrap();
                }
            }
        }
        x.borrow_mut().color = Black;
    }

    fn set_nil(&mut self, node: NodePointer<T>, stats: &mut Stats) -> NodePointer<T> {
        //println!("{:?} is now nil, child of {:?}", node.borrow().val, parent(&node).borrow().val);
        stats.swap();
        self.nil = Some(node.clone());

        node.borrow_mut().color = Black;

        stats.swap();
        node.borrow_mut().left = None;

        stats.swap();
        node.borrow_mut().right = None;
        node
    }

    // fn compare(&self, a: &Option<NodePointer<T>>, b: &Option<NodePointer<T>>) -> bool {
    //     match (a, b) {
    //         (Some(a), Some(b)) => Rc::ptr_eq(a, b),
    //         (Some(a), None) | (None, Some(a)) => {
    //             match &self.nil {
    //                 Some(nil) => Rc::ptr_eq(a, nil),
    //                 None => false,
    //             }
    //         },
    //         _ => false,
    //     }
    // }

    fn comp_ptr(&self, a: &NodePointer<T>, b: &Option<NodePointer<T>>) -> bool {
        match b {
            Some(b) => Rc::ptr_eq(a, b),
            None => {
                match &self.nil {
                    Some(nil) => Rc::ptr_eq(a, nil),
                    None => false,
                }
            }
        }
    }
}


fn get_replacement<T: Ord>(mut node: RefMut<Node<T>>, stats: &mut Stats) -> Option<NodePointer<T>>
{
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
                    replacement_ref.color = node.color.clone();


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

impl<T: Ord + Debug> BinTree<T> {
    pub fn print_road(&self) {
        if let Some(root) = &self.root {
            root.borrow().print_road();
        } else {
            println!("Empty tree")
        }
    }
}

