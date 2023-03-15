use super::{Vector, DEFAULT_CAPACITY};

#[test]
fn check_constructors() {
    let v = Vector::<i32>::new();
    assert!(v.len == 0);
    assert!(v.cap == DEFAULT_CAPACITY);

    // let v = Vector::<i32>::new_with_length(4);
    // assert!(v.len == 4);
    // assert!(v.cap == 8);

    // let v = Vector::<i32>::new_with_capacity(4);
    // assert!(v.len == 0);
    // assert!(v.cap == 4);
}
