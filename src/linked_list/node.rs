pub(super) struct Node<T: PartialEq> {
    pub(super) value: T,
    pub(super) next: Option<Box<Node<T>>>,
}

impl<T: PartialEq> Node<T> {
    pub(super) fn new(val: T) -> Node<T> {
        Node {
            value: val,
            next: None,
        }
    }

    pub(super) fn len(&self) -> usize {
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
