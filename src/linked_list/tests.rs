use super::LinkedList;

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
    let mut list = LinkedList::<i32>::new();
    assert!(list.head.is_none());
    list.push(10);
    assert!(list.head.as_ref().unwrap().value == 10);

    let mut list = LinkedList::new_with(10);
    list.push(4);
    list.push(6);
    list.push(8);

    assert!(list.head.as_ref().unwrap().value == 10);

    assert!(list.head.as_ref().unwrap().next.as_ref().unwrap().value == 4);
    assert!(
        list.head
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .value
            == 6
    );
    assert!(
        list.head
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .value
            == 8
    );
    assert!(list
        .head
        .as_ref()
        .unwrap()
        .next
        .as_ref()
        .unwrap()
        .next
        .as_ref()
        .unwrap()
        .next
        .as_ref()
        .unwrap()
        .next
        .is_none());
}

#[test]
fn check_to_string() {
    let mut list = LinkedList::<i32>::new();
    println!("{:?}", list);
    assert!(list.to_string() == "empty");
    list.push(100);
    println!("{:?}", list);
    assert!(list.to_string() == "100->");

    let mut list = LinkedList::new_with(10);
    println!("{:?}", list);
    assert!(list.to_string() == "10->");
    list.push(8);
    println!("{:?}", list);
    assert!(list.to_string() == "10->8->");
    list.push(8);
    println!("{:?}", list);
    assert!(list.to_string() == "10->8->8->");
    list.push(6);
    println!("{:?}", list);
    assert!(list.to_string() == "10->8->8->6->");
}

#[test]
fn check_erase() {
    let mut list = LinkedList::<i32>::new();
    assert!(list.erase(0) == Err("Cannot erase from empty list".to_string()));

    let mut list = LinkedList::new_with(10);
    assert!(list.erase(8) == Err("Didn't find given value".to_string()));
    assert!(list.erase(10).is_ok());

    list.push(10);
    list.push(8);
    list.push(8);
    assert!(list.erase(8).is_ok());

    list.push(8);
    assert!(list.erase(10).is_ok());

    list.push(10);
    list.push(6);
    assert!(list.erase(6).is_ok());
    assert!(list.erase(100) == Err("Didn't find given value".to_string()));
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
