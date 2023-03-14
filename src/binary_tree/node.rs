pub(super) struct Node<T: PartialEq + PartialOrd> {
    pub(super) value: T,
    pub(super) left: Option<Box<Node<T>>>,
    pub(super) right: Option<Box<Node<T>>>,
}

impl<T: PartialEq + PartialOrd> Node<T> {
    pub(super) fn new(val: T) -> Node<T> {
        Node {
            value: val,
            left: None,
            right: None,
        }
    }

    pub(super) fn get_height(&self) -> usize {
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
