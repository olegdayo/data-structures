#[cfg(test)]
mod tests;

use std::ops::{Index, IndexMut};

use std::alloc::{alloc, Layout};
use std::fmt::{Debug, Formatter};

pub struct Vector<T: ?Sized> {
    buffer: *const T,
    len: usize,
    cap: usize,
}

pub const DEFAULT_CAPACITY: usize = 10;

impl<T: Sized> Vector<T> {
    pub fn new() -> Vector<T> {
        let layout = Layout::array::<T>(DEFAULT_CAPACITY).unwrap();
        unsafe {
            Vector {
                buffer: alloc(layout) as *const T,
                len: 0,
                cap: DEFAULT_CAPACITY,
            }
        }
    }

    pub fn new_with_capacity(cap: usize) -> Vector<T> {
        let layout = Layout::array::<T>(cap).unwrap();

        unsafe {
            Vector {
                buffer: alloc(layout) as *const T,
                len: 0,
                cap: cap,
            }
        }
    }

    pub fn push(&mut self, val: T) {
        let ind = self.len;
        self.len += 1;

        if self.len < self.cap {
            unsafe {
                let elem = self.buffer.add(ind) as *mut T;
                elem.write(val);
            }
            return;
        }

        self.set_cap(self.len);
        unsafe {
            let elem = self.buffer.add(ind) as *mut T;
            elem.write(val);
        }
    }

    pub fn set_cap(&mut self, new_cap: usize) {
        let v: Vector<T> = Vector::new_with_capacity(new_cap);

        for i in 0..self.len() {
            unsafe {
                let old_elem = self.buffer.add(i) as *mut T;
                let new_elem = v.buffer.add(i) as *mut T;
                new_elem.write(old_elem.read());
            }
        }

        *self = v;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn cap(&self) -> usize {
        self.cap
    }
}

impl<T: Clone> Vector<T> {
    pub fn new_with_length(len: usize, val: T) -> Vector<T> {
        let layout = Layout::array::<T>(len * 2).unwrap();

        let v = unsafe {
            Vector {
                buffer: alloc(layout) as *const T,
                len: len,
                cap: len * 2,
            }
        };

        for i in 0..len {
            unsafe {
                let elem = v.buffer.add(i) as *mut T;
                elem.write(val.clone());
            }
        }

        v
    }

    pub fn resize(&mut self, new_len: usize, val: T) {
        let v: Vector<T> = Vector::new_with_length(new_len, val);

        for i in 0..self.len() {
            unsafe {
                let old_elem = self.buffer.add(i) as *mut T;
                let new_elem = v.buffer.add(i) as *mut T;
                new_elem.write(old_elem.read());
            }
        }

        *self = v;
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;
    fn index(&self, ind: usize) -> &Self::Output {
        if ind < self.len {
            unsafe {
                return &*self.buffer.add(ind);
            }
        }

        panic!(
            "Index i: {} is outside the bounds of vector l: {}",
            ind, self.len,
        );
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, ind: usize) -> &mut T {
        if ind < self.len {
            unsafe {
                return &mut *(self.buffer.add(ind) as *mut T);
            }
        }

        panic!(
            "Index i: {} is outside the bounds of vector l: {}",
            ind, self.len,
        );
    }
}

impl<T: Debug> ToString for Vector<T> {
    fn to_string(&self) -> String {
        let mut s = "[".to_string();
        for i in 0..self.len() - 1 {
            s += &format!("{:?}, ", self[i]);
        }

        s = s + &format!("{:?}", self[self.len() - 1]) + "]";

        s
    }
}

impl<T: Debug> Debug for Vector<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector")
            .field("Length:", &self.len())
            .field("Capacity:", &self.len())
            .field("Elements:", &self.to_string())
            .finish()
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
        vector
    });
}
