#[cfg(test)]
mod tests;

use std::ops::{Index, IndexMut};

struct Vector<T: Default> {
    buf: Box<[T]>,
    len: usize,
    cap: usize,
}

const DEFAULT_CAPACITY: usize = 10;

impl<T: Copy + Default> Vector<T> {
    fn new() -> Vector<T> {
        Vector {
            buf: Box::new([T::default(); DEFAULT_CAPACITY]),
            len: 0,
            cap: DEFAULT_CAPACITY,
        }
    }

    fn push(&mut self, val: T) {
        let ind = self.len;
        self.len += 1;

        if self.len < self.cap {
             self.buf[ind] = val;
             return;
        }
        
        self.resize(self.len);
        self.buf[ind] = val;
        return;
    }

    fn resize(&mut self, new_size: usize) {
        todo!()
    }

    fn erase(&mut self, ind: usize) -> Vector<T> {
        todo!()
    }
}

impl<T: Copy + Default> Index<usize> for Vector<T> {
    type Output = T;
    fn index(&self, ind: usize) -> &T {
        todo!()
    }
}

impl<T: Copy + Default> IndexMut<usize> for Vector<T> {
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
