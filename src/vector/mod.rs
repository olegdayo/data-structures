#[cfg(test)]
mod tests;

use std::{
    mem::size_of,
    ops::{Index, IndexMut},
};

pub struct Vector<T: Default> {
    buf: *mut T,
    len: usize,
    cap: usize,
}

const DEFAULT_CAPACITY: usize = 10;

impl<T: Default> Vector<T> {
    pub fn new() -> Vector<T> {
        let v = Vector {
            buf: &mut T::default(),
            len: 0,
            cap: DEFAULT_CAPACITY,
        };

        for i in 0..v.cap {
            unsafe {
                v.buf
                    .add(i * size_of::<T>())
                    .copy_from(&mut T::default() as *mut T, size_of::<T>());
            }
        }

        v
    }

    pub fn new_with_length(len: usize) -> Vector<T> {
        let v = Vector {
            buf: &mut T::default(),
            len: len,
            cap: len * 2,
        };

        for i in 0..v.cap {
            unsafe {
                v.buf
                    .add(i * size_of::<T>())
                    .copy_from(&mut T::default() as *mut T, size_of::<T>());
            }
        }

        v
    }

    pub fn new_with_capacity(cap: usize) -> Vector<T> {
        let v = Vector {
            buf: &mut T::default(),
            len: 0,
            cap: cap,
        };

        for i in 0..v.cap {
            unsafe {
                v.buf
                    .add(i * size_of::<T>())
                    .copy_from(&mut T::default() as *mut T, size_of::<T>());
            }
        }

        v
    }

    pub fn push(&mut self, val: &mut T) {
        let ind = self.len;
        self.len += 1;

        if self.len < self.cap {
            unsafe {
                self.buf
                    .add(ind * size_of::<T>())
                    .copy_from(val as *mut T, size_of::<T>());
            }
            return;
        }

        self.resize(self.len);
        unsafe {
            self.buf
                .add(ind * size_of::<T>())
                .copy_from(val as *mut T, size_of::<T>());
        }
    }

    pub fn resize(&mut self, new_size: usize) {
        for i in self.cap..new_size {
            unsafe {
                self.buf
                    .add(i * size_of::<T>())
                    .copy_from(&mut T::default() as *mut T, size_of::<T>());
            }
        }

        self.len = std::cmp::min(self.len, new_size);
        self.cap = new_size;
    }
}

impl<T: Default> Index<usize> for Vector<T> {
    type Output = T;
    fn index(&self, ind: usize) -> &T {
        if ind < self.len {
            unsafe {
                return &*self.buf.add(ind * size_of::<T>());
            }
        }

        panic!(
            "Index i: {} is outside the bounds of vector l: {}",
            ind, self.len,
        );
    }
}

impl<T: Default> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, ind: usize) -> &mut T {
        if ind < self.len {
            unsafe {
                return &mut *self.buf.add(ind * size_of::<T>());
            }
        }

        panic!(
            "Index i: {} is outside the bounds of vector l: {}",
            ind, self.len,
        );
    }
}

#[macro_export]
macro_rules! vector {
    () => ({
        Vector::new()
    });

    ($t:ty) => ({
        Vector::<$t>::new()
    });

    ($elem:expr; $num:expr) => ({
        let mut vector = Vector::new();
        for _ in 0..num {
            vector.push($elem);
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
