#[cfg(test)]
mod tests;

use std::ops::{Index, IndexMut};

struct Vector<T> {
    head: *mut T,
    len: usize,
    cap: usize,
}

impl<T> Vector<T> {
    fn new() -> Vector<T> {
        todo!()
    }

    fn push(&mut self, val: T) {
        todo!()
    }

    fn erase(&mut self, ind: usize) -> Vector<T> {
        todo!()
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;
    fn index(&self, ind: usize) -> &T {
        todo!()
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, ind: usize) -> &mut T {
        todo!()
    }
}

#[macro_export]
macro_rules! vector {
    () => ({
        Vector::new()
    });

    ($elem:expr; $num:expr) => ({
        let mut vector = Vector::new();
        for _ in 0..num {
            vector.push(elem);
        }
        vector
    });

    ($($elem:expr),+ $(,)?) => ({
        let mut vector = Vector::new();
        $(
            vector.push($elem);
        )*
    });
}
