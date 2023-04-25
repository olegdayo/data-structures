use super::{Vector, DEFAULT_CAPACITY};
use crate::vector;

#[test]
fn check_constructors() {
    let v = Vector::<i32>::new();
    assert!(v.len == 0);
    assert!(v.cap == DEFAULT_CAPACITY);

    let v = Vector::<i32>::new_with_length(4, 0);
    assert!(v.len == 4);
    assert!(v.cap == 8);

    let v = Vector::<i32>::new_with_capacity(4);
    assert!(v.len == 0);
    assert!(v.cap == 4);
}

#[test]
fn check_push() {}

#[test]
fn check_resize() {}

#[test]
fn check_index() {
    let vector = vector![1, 2, 3, 4, 5, 6];
    let vec = vec![1, 2, 3, 4, 5, 6];
    println!("{:?}", vector);
    for i in 0..vector.len() {
        assert!(vector[i] == vec[i]);
    }
}

#[test]
fn check_index_mut() {
    let vector = vector![1, 2, 3, 4, 5, 6];
    let vec = vec![1, 2, 3, 4, 5, 6];
    for i in 0..vector.len() {
        assert!(vector[i] == vec[i]);
    }
}

#[test]
fn check_macro() {
    let vector = vector![1, 2, 3, 4, 5, 6];
    let vec = vec![1, 2, 3, 4, 5, 6];
    for i in 0..vector.len() {
        assert!(vector[i] == vec[i]);
    }
}
