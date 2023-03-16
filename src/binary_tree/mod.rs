mod node;
#[cfg(test)]
mod tests;

use core::fmt::Formatter;
use std::fmt::Debug;

use node::Node;

pub struct BinaryTree<T: Default + PartialEq + PartialOrd> {
    head: Option<node::Node<T>>,
}

impl<T: Default + PartialEq + PartialOrd> BinaryTree<T> {
    pub fn new() -> BinaryTree<T> {
        BinaryTree { head: None }
    }

    pub fn new_with(val: T) -> BinaryTree<T> {
        BinaryTree {
            head: Some(Node {
                value: val,
                left: None,
                right: None,
            }),
        }
    }

    pub fn insert(&mut self, val: T) {
        let mut curr_node = match &mut self.head {
            Some(head) => head,
            None => {
                self.head = Some(Node {
                    value: val,
                    left: None,
                    right: None,
                });
                return;
            }
        };

        loop {
            if curr_node.value > val {
                match &mut curr_node.left {
                    Some(left_child) => {
                        curr_node = left_child;
                    }
                    left_child => {
                        *left_child = Some(Box::new(Node::new(val)));
                        return;
                    }
                }
                continue;
            }

            match &mut curr_node.right {
                Some(right_child) => {
                    curr_node = right_child;
                }
                right_child => {
                    *right_child = Some(Box::new(Node::new(val)));
                    return;
                }
            }
        }
    }

    pub fn get_height(&self) -> usize {
        match &self.head {
            Some(head) => head.get_height(),
            None => 0,
        }
    }

    pub fn is_present(&self, val: T) -> bool {
        let mut curr_node = match &self.head {
            Some(head) => head,
            None => {
                return false;
            }
        };

        loop {
            if curr_node.value == val {
                return true;
            }

            if curr_node.value > val {
                curr_node = match &curr_node.left {
                    Some(left) => &left,
                    None => {
                        return false;
                    }
                };
                continue;
            }

            curr_node = match &curr_node.right {
                Some(right) => &right,
                None => {
                    return false;
                }
            };
        }
    }
}

impl<T: Debug + Default + PartialEq + PartialOrd> Debug for BinaryTree<T> {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}
