use super::{Vector, DEFAULT_CAPACITY};
use crate::vector;

#[test]
fn check_constructors() {
    let v = Vector::<i32>::new();
    assert!(v.len() == 0);
    assert!(v.cap() == DEFAULT_CAPACITY);

    let v = Vector::<i32>::new_with_length(4, 0);
    assert!(v.len() == 4);
    assert!(v.cap() == 8);

    let v = Vector::<i32>::new_with_capacity(4);
    assert!(v.len() == 0);
    assert!(v.cap() == 4);
}

#[test]
fn check_push() {
    let mut v = Vector::<i32>::new();
    v.push(1);
    assert!(v.len() == 1);
    assert!(v.cap() == DEFAULT_CAPACITY);
    assert!(v[0] == 1);

    v.push(2);
    assert!(v.len == 2);
    assert!(v.cap == DEFAULT_CAPACITY);
    assert!(v[1] == 2);

    let v = vector![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    assert!(v.len() == 15);
    assert!(v.cap() == DEFAULT_CAPACITY * 2);
}

#[test]
fn check_resize() {
    let mut vector = vector![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    assert!(vector.len() == vec.len());
    assert!(vector.cap() == DEFAULT_CAPACITY * 2);
    for i in 0..vector.len() {
        assert!(vector[i] == vec[i]);
    }

    vector.resize(8, 0);
    vec.resize(8, 0);
    assert!(vector.len() == vec.len());
    assert!(vector.cap() == vec.len() * 2);
    for i in 0..vector.len() {
        assert!(vector[i] == vec[i]);
    }

    vector.resize(30, 0);
    vec.resize(30, 0);
    assert!(vector.len() == vec.len());
    assert!(vector.cap() == vec.len() * 2);
    for i in 0..vector.len() {
        assert!(vector[i] == vec[i]);
    }
}

#[test]
fn check_index() {
    let vector = vector![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    println!("{:?}", vector);
    for i in 0..vector.len() {
        assert!(vector[i] == vec[i]);
    }
}

#[test]
fn check_index_mut() {
    let vector = vector![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    for i in 0..vector.len() {
        assert!(vector[i] == vec[i]);
    }
}

#[test]
fn check_macro() {
    let vector = vector![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    for i in 0..vector.len() {
        assert!(vector[i] == vec[i]);
    }
}
