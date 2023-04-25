use super::Stack;

#[test]
fn check_new() {
    let stack = Stack::<i32>::new();
    assert!(stack.size() == 0);
    assert!(stack.buffer.len() == 0);
    assert!(stack.buffer.cap() == 10);
}

#[test]
fn check_push() {
    let mut stack = Stack::<i32>::new();
    stack.push(1);
    assert!(stack.size() == 1);
    assert!(stack.buffer.len() == 1);
    assert!(stack.buffer.cap() == 10);

    stack.push(10);
    assert!(stack.size() == 2);
    assert!(stack.buffer.len() == 2);
    assert!(stack.buffer.cap() == 10);
}

#[test]
fn check_pop() {
    let mut stack = Stack::<i32>::new();
    stack.pop();
    assert!(stack.size() == 0);
    assert!(stack.buffer.len() == 0);
    assert!(stack.buffer.cap() == 10);

    stack.push(1);
    stack.push(10);
    stack.pop();
    assert!(stack.size() == 1);
    assert!(stack.buffer.len() == 1);
    assert!(stack.buffer.cap() == 10);
}

#[test]
fn check_peek() {
    let mut stack = Stack::<i32>::new();
    assert!(stack.peek().is_none());
    assert!(stack.size() == 0);
    assert!(stack.buffer.len() == 0);
    assert!(stack.buffer.cap() == 10);

    stack.push(1);
    assert!(stack.peek().is_some());
    assert!(stack.size() == 1);
    assert!(stack.buffer.len() == 1);
    assert!(stack.buffer.cap() == 10);

    stack.push(10);
    stack.pop();
    assert!(stack.peek().is_some());
    assert!(stack.size() == 1);
    assert!(stack.buffer.len() == 1);
    assert!(stack.buffer.cap() == 10);
}

#[test]
fn check_peek_mut() {
    let mut stack = Stack::<i32>::new();
    assert!(stack.peek_mut().is_none());
    assert!(stack.size() == 0);
    assert!(stack.buffer.len() == 0);
    assert!(stack.buffer.cap() == 10);

    stack.push(1);
    assert!(stack.peek_mut().is_some());
    assert!(stack.size() == 1);
    assert!(stack.buffer.len() == 1);
    assert!(stack.buffer.cap() == 10);

    stack.push(10);
    stack.pop();
    assert!(stack.peek_mut().is_some());
    assert!(stack.size() == 1);
    assert!(stack.buffer.len() == 1);
    assert!(stack.buffer.cap() == 10);
}
