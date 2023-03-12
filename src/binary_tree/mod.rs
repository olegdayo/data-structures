#[cfg(test)]
mod tests;

use core::fmt::Formatter;
use std::fmt::Debug;

pub struct BinaryTree<T: PartialOrd> {
    pub(crate) value: T,
    pub(crate) left: Option<Box<BinaryTree<T>>>,
    pub(crate) right: Option<Box<BinaryTree<T>>>,
}

impl<'a, T: PartialOrd> BinaryTree<T> {
    pub fn new(val: T) -> BinaryTree<T> {
        BinaryTree {
            value: val,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, val: T) {
        let mut curr_node = self;
        loop {
            if curr_node.value > val {
                if curr_node.left.is_none() {
                    curr_node.left = Some(Box::new(BinaryTree::new(val)));
                    return;
                }

                curr_node = curr_node.left.as_mut().unwrap();
                continue;
            }

            if curr_node.right.is_none() {
                curr_node.right = Some(Box::new(BinaryTree::new(val)));
                return;
            }

            curr_node = curr_node.right.as_mut().unwrap();
        }
    }

    pub fn get_height(&self) -> u32 {
        let mut height = 1u32;
        if self.left.is_some() {
            height = std::cmp::max(height, self.left.as_ref().unwrap().get_height() + 1);
        }

        if self.right.is_some() {
            height = std::cmp::max(height, self.right.as_ref().unwrap().get_height() + 1);
        }

        height
    }
}

impl<'a, T: PartialOrd + Debug> Debug for BinaryTree<T> {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}
