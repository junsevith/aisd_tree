use std::fmt::{Debug, Formatter};
use itertools::Itertools;
use crate::experiment::Stats;

#[derive(Debug, Clone)]
pub(super) struct SplayNode<T: Ord + Clone> {
    value: T,
    left: Option<Box<SplayNode<T>>>,
    right: Option<Box<SplayNode<T>>>,
}


impl<T: Ord + Clone> SplayNode<T> {
    pub(crate) fn new(value: T) -> Self {
        SplayNode {
            value,
            left: None,
            right: None,
        }
    }

    // Perform a right rotation on the node.
    fn rotate_right(mut self: Box<Self>, stats: &mut Stats) -> Box<Self> {
        stats.read();
        if self.left.is_some() {
            stats.swap();
            let mut x = self.left.take().unwrap();
            stats.swap();
            self.left = x.right.take();
            x.right = Some(self);
            return x;
        }
        self
    }

    // Perform a left rotation on the node.
    fn rotate_left(mut self: Box<Self>, stats: &mut Stats) -> Box<Self> {
        stats.read();
        if self.right.is_some() {
            stats.swap();
            let mut x = self.right.take().unwrap();
            stats.swap();
            self.right = x.left.take();
            x.left = Some(self);
            return x;
        }
        self
    }

    // Perform a splay operation on the node.
    fn splay(mut self: Box<Self>, value: T, stats: &mut Stats) -> Box<Self> {
        stats.comp();
        if value < self.value {
            stats.read();
            if let Some(ref mut left) = self.left {
                stats.comp();
                if value < left.value {
                    // Zig-Zig
                    stats.swap();
                    left.left = left.left.take().map(|node| node.splay(value, stats));
                    self = self.rotate_right(stats);
                } else if value > left.value {
                    // Zig-Zag4
                    stats.swap();
                    left.right = left.right.take().map(|node| node.splay(value, stats));
                    self.left = self.left.map(|node| node.rotate_left(stats));
                }
                if let Some(ref mut _left) = self.left {
                    stats.read();
                    self = self.rotate_right(stats);
                }
            }
        } else if value > self.value {
            if let Some(ref mut right) = self.right {
                stats.comp();
                if value > right.value {
                    // Zag-Zag
                    stats.swap();
                    right.right = right.right.take().map(|node| node.splay(value, stats));
                    self = self.rotate_left(stats);
                } else if value < right.value {
                    // Zag-Zig
                    stats.swap();
                    right.left = right.left.take().map(|node| node.splay(value, stats));
                    self.right = self.right.map(|node| node.rotate_right(stats));
                }
                if let Some(ref mut _right) = self.right {
                    stats.read();
                    self = self.rotate_left(stats);
                }
            }
        }
        self
    }

    // Insert a value into the splay tree.
    pub(crate) fn insert(mut self: Box<Self>, value: T, stats: &mut Stats) -> Box<Self> {
        stats.comp();
        if value < self.value {
            stats.read();
            if let Some(left) = self.left.take() {
                stats.swap();
                self.left = Some(left.insert(value.clone(), stats));
            } else {
                stats.swap();
                self.left = Some(Box::new(SplayNode::new(value.clone())));
            }
        } else if value >= self.value {
            stats.read();
            if let Some(right) = self.right.take() {
                stats.swap();
                self.right = Some(right.insert(value.clone(), stats));
            } else {
                stats.swap();
                self.right = Some(Box::new(SplayNode::new(value.clone())));
            }
        }
        self.splay(value, stats)
    }

    // Perform a delete operation on the node.
    pub(crate) fn delete(mut self: Box<Self>, value: T, stats: &mut Stats) -> Option<Box<Self>> {
        self = self.splay(value.clone(), stats);
        stats.swap();
        stats.comp();
        if value != self.value {
            return Some(self);
        }

        stats.read();
        stats.read();
        match (self.left.take(), self.right.take()) {
            (None, right) => right,
            (left, None) => left,
            (Some(left), Some(right)) => {
                let mut x = right.splay(value, stats);
                x.left = Some(left);
                Some(x)
            }
        }
    }

    // Get the height of the splay tree rooted at this node.
    pub(crate) fn height(&self) -> u32 {
        let left_height = self.left.as_ref().map_or(0, |node| node.height());
        let right_height = self.right.as_ref().map_or(0, |node| node.height());
        1 + std::cmp::max(left_height, right_height)
    }

    // // Print the node value with the given prefix and side indication.
    // fn print_node(&self, prefix: &str, is_left: bool) {
    //     let side = if is_left { "└──" } else { "├──" };
    //     println!("{}{} {}", prefix, side, self.value);
    // }
    //
    // // Print the splay tree rooted at this node in a readable format.
    // fn print_tree_helper(&self, prefix: &str, is_left: bool) {
    //     if let Some(right) = self.right.as_ref() {
    //         right.print_tree_helper(&format!("{}{}   ", prefix, if is_left { "│" } else { " " }), false);
    //     }
    //     self.print_node(prefix, is_left);
    //     if let Some(left) = self.left.as_ref() {
    //         left.print_tree_helper(&format!("{}{}   ", prefix, if is_left { " " } else { "│" }), true);
    //     }
    // }
}

impl<T: Ord + Clone + Debug> SplayNode<T> {
    pub(crate) fn print_node(&self, f: &mut Formatter, road: String) {
        if let Some(right) = &self.right {
            right.print_node(f, road.clone() + "u");
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
        writeln!(f, "{:?}", &self.value).unwrap();

        if let Some(left) = &self.left {
            left.print_node(f, road + "d");
        }
    }
}