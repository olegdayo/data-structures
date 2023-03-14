use crate::linked_list::{LinkedList};

#[test]
fn check_constructors() {
    let list = LinkedList::<i32>::new();
    assert!(list.head.is_none());

    let list = LinkedList::new_with(10);
    assert!(list.head.as_ref().unwrap().value == 10);
    assert!(list.head.as_ref().unwrap().next.is_none());
}

#[test]
fn check_insert() {
    let mut list = LinkedList::new_with(10);
    list.push(4);
    list.push(6);
    list.push(8);

    assert!(list.head.as_ref().unwrap().value == 10);

    assert!(list.head.as_ref().unwrap().next.as_ref().unwrap().value == 4);
    assert!(list.head.as_ref().unwrap().next.as_ref().unwrap().next.as_ref().unwrap().value == 6);
    assert!(list.head.as_ref().unwrap().next.as_ref().unwrap().next.as_ref().unwrap().next.as_ref().unwrap().value == 8);
    assert!(list.head.as_ref().unwrap().next.as_ref().unwrap().next.as_ref().unwrap().next.as_ref().unwrap().next.is_none());
}

#[test]
fn check_length() {
    let mut list = LinkedList::<i32>::new();
    assert!(list.len() == 0);
    list.push(0);
    assert!(list.len() == 1);

    let mut list = LinkedList::new_with(10);
    assert!(list.len() == 1);
    list.push(8);
    assert!(list.len() == 2);
    list.push(20);
    assert!(list.len() == 3);
    list.push(6);
    assert!(list.len() == 4);
}
