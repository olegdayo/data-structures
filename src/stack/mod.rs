#[cfg(test)]
mod tests;

use crate::vector::Vector;

pub struct Stack<T> {
    size: usize,
    buffer: Vector<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            size: 0,
            buffer: Vector::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, val: T) {
        self.size += 1;
        self.buffer.push(val);
    }

    pub fn pop(&mut self) {
        if self.size == 0 {
            return;
        }

        self.size -= 1;
    }

    pub fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            return None;
        }
        Some(&self.buffer[self.size - 1])
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.size == 0 {
            return None;
        }
        Some(&mut self.buffer[self.size - 1])
    }
}
