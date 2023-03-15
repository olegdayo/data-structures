use super::BinaryTree;

#[test]
fn check_new() {
    let tree = BinaryTree::<i32>::new();
    assert!(tree.head.is_none());

    let tree = BinaryTree::new_with(10);
    assert!(tree.head.as_ref().unwrap().value == 10);
    assert!(tree.head.as_ref().unwrap().left.is_none());
    assert!(tree.head.as_ref().unwrap().right.is_none());
}

#[test]
fn check_insert() {
    let mut tree = BinaryTree::new_with(10);
    tree.insert(8);
    tree.insert(20);
    tree.insert(6);

    assert!(tree.head.as_ref().unwrap().value == 10);

    assert!(tree.head.as_ref().unwrap().left.as_ref().unwrap().value == 8);
    assert!(tree.head.as_ref().unwrap().left.as_ref().unwrap().left.as_ref().unwrap().value == 6);
    assert!(tree.head.as_ref().unwrap().left.as_ref().unwrap().right.is_none());

    assert!(tree.head.as_ref().unwrap().right.as_ref().unwrap().value == 20);
    assert!(tree.head.as_ref().unwrap().right.as_ref().unwrap().left.is_none());
    assert!(tree.head.as_ref().unwrap().right.as_ref().unwrap().right.is_none());
}

#[test]
fn check_height() {
    let mut tree = BinaryTree::<i32>::new();
    assert!(tree.get_height() == 0);
    tree.insert(0);
    assert!(tree.get_height() == 1);

    let mut tree = BinaryTree::new_with(10);
    assert!(tree.get_height() == 1);
    tree.insert(8);
    assert!(tree.get_height() == 2);
    tree.insert(20);
    assert!(tree.get_height() == 2);
    tree.insert(6);
    assert!(tree.get_height() == 3);
}
