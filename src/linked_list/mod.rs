#[cfg(test)]
mod tests;

pub struct LinkedList<T: PartialEq> {
    head: Option<Node<T>>,
}

struct Node<T: PartialEq> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T: PartialEq + PartialOrd> Node<T> {
    fn new(val: T) -> Node<T> {
        Node {
            value: val,
            next: None,
        }
    }

    fn len(&self) -> u32 {
        let mut len = 0;
        let mut curr_node = self;

        loop {
            len += 1;
            match &curr_node.next {
                None => {
                    break;
                }

                next => {
                    curr_node = next.as_ref().unwrap();
                }
            }
        }

        len
    }
}

impl<T: PartialEq + PartialOrd> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
        }
    }

    fn new_with(val: T) -> LinkedList<T> {
        LinkedList {
            head: Some(Node::new(val)),
        }
    }

    fn push(&mut self, val: T) {
        let mut curr_node = match &mut self.head {
            Some(head) => head,
            None => {
                self.head.insert(Node::new(val));
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

    fn erase(&mut self, val: T) -> Result<(), String> {
        todo!()
        // let curr_node = self;

        // if curr_node.value == val {
        //     if curr_node.next.is_none() {
        //         return Err("Cannot delete last element from linked list with len = 1".to_string());
        //     }

        //     curr_node.value = self.next.as_mut().unwrap().value;
        // }

        // loop {
        //     match &mut curr_node.next {
        //         None => {
        //             return Err("Didn't find given value".to_string());
        //         }

        //         next => {                    
        //             if next.as_ref().map(|x| &x.value) == Some(&val) {
        //                 let new_next = next.as_mut().unwrap().next.take();

        //                 *next = new_next;
                        
        //                 return Ok(());
        //             }
        //         }
        //     }
        // }
    }

    fn len(&self) -> u32 {
        match &self.head {
            Some(head) => head.len(),
            None => 0,
        }
    }
}
