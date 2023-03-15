mod node;
#[cfg(test)]
mod tests;

use std::fmt::{Debug, Formatter};

use node::Node;

pub struct LinkedList<T: PartialEq> {
    head: Option<Node<T>>,
}

impl<T: PartialEq> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None }
    }

    pub fn new_with(val: T) -> LinkedList<T> {
        LinkedList {
            head: Some(Node::new(val)),
        }
    }

    pub fn push(&mut self, val: T) {
        let mut curr_node = match &mut self.head {
            Some(head) => head,
            None => {
                self.head = Some(Node::new(val));
                return;
            }
        };

        loop {
            match &mut curr_node.next {
                Some(next) => {
                    curr_node = next;
                }
                next => {
                    *next = Some(Box::new(Node::new(val)));
                    return;
                }
            }
        }
    }

    pub fn erase(&mut self, val: T) -> Result<(), String> {
        let mut curr_node = match &mut self.head {
            Some(head) => head,
            None => {
                return Err("Cannot erase from empty list".to_string());
            }
        };

        if curr_node.value == val {
            if curr_node.next.is_none() {
                self.head = None;
                return Ok(());
            }

            let next = curr_node.next.take().unwrap();
            *curr_node = *next;
            return Ok(());
        }

        loop {
            match &mut curr_node.next {
                None => {
                    return Err("Didn't find given value".to_string());
                }

                next => {
                    if next.as_ref().unwrap().value.eq(&val) {
                        let new_next = next.as_mut().unwrap().next.take();
                        *next = new_next;
                        return Ok(());
                    }
                    curr_node = next.as_mut().unwrap();
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        match &self.head {
            Some(head) => head.len(),
            None => 0,
        }
    }
}

impl<T: Debug + PartialEq> ToString for LinkedList<T> {
    fn to_string(&self) -> String {
        let mut curr_node = match &self.head {
            Some(head) => head,
            None => {
                return "empty".to_string();
            }
        };

        let mut s = format!("{:?}->", curr_node.value);

        loop {
            match &curr_node.next {
                Some(next) => {
                    curr_node = next;
                    s += &format!("{:?}->", curr_node.value);
                }
                None => break,
            }
        }
        s
    }
}

impl<T: Debug + PartialEq> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinkedList")
            .field("list:", &self.to_string())
            .finish()
    }
}
