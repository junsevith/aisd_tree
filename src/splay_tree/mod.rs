mod tree_node;

use std::fmt::{Debug, Formatter};
use itertools::Itertools;
use crate::experiment::Stats;
use crate::splay_tree::tree_node::SplayNode;

#[derive(Clone)]
pub struct SplayTree<T: Ord + Clone> {
    root: Option<Box<SplayNode<T>>>,
}

impl<T: Ord + Clone> Default for SplayTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Clone> SplayTree<T> {
    pub fn new() -> Self {
        SplayTree { root: None }
    }

    pub fn insert(&mut self, value: T, stats: &mut Stats) {
        if let Some(root) = self.root.take() {
            self.root = Some(root.insert(value, stats));
        } else {
            self.root = Some(Box::new(SplayNode::new(value)));
        }
    }

    pub fn height(&self) -> u32 {
        self.root.as_ref().map_or(0, |node| node.height())
    }

    // pub fn print_tree(&self) {
    //     if let Some(root) = self.root.as_ref() {
    //         root.print_tree_helper("", false);
    //     }
    // }

    pub fn delete(&mut self, value: T, stats: &mut Stats) {
        if let Some(root) = self.root.take() {
            self.root = root.delete(value, stats);
        }
    }
}

impl<T: Debug + Ord + std::clone::Clone> Debug for SplayTree<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if let Some(root) = &self.root {
            root.print_node(f, String::new());
        } else {
            write!(f, "Empty tree")?;
        }
        Ok(())
    }
}