#[cfg(test)]
mod tests;

use core::fmt::Formatter;
use std::fmt::Debug;

pub struct BinaryTree<T: PartialEq + PartialOrd> {
    head: Option<Node<T>>,
}

struct Node<T: PartialEq + PartialOrd> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: PartialEq + PartialOrd> Node<T> {
    fn new(val: T) -> Node<T> {
        Node {
            value: val,
            left: None,
            right: None,
        }
    }

    fn get_height(&self) -> usize {
        let mut height = 1;

        if self.left.is_some() {
            height = std::cmp::max(height, self.left.as_ref().unwrap().get_height() + 1);
        }

        if self.right.is_some() {
            height = std::cmp::max(height, self.right.as_ref().unwrap().get_height() + 1);
        }

        height
    }
}

impl<T: PartialEq + PartialOrd> BinaryTree<T> {
    pub fn new() -> BinaryTree<T> {
        BinaryTree {
            head: None,
        }
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
        todo!()
    }
}

impl<T: Debug + PartialEq + PartialOrd> Debug for BinaryTree<T> {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}
