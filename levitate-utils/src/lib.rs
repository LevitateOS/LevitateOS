#![cfg_attr(not(feature = "std"), no_std)]

use core::cell::UnsafeCell;
use core::marker::{Send, Sync};
use core::ops::{Deref, DerefMut, Drop};
use core::sync::atomic::{AtomicBool, Ordering};

pub struct Spinlock<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for Spinlock<T> {}
unsafe impl<T: Send> Send for Spinlock<T> {}

pub struct SpinlockGuard<'a, T> {
    lock: &'a Spinlock<T>,
    data: &'a mut T,
}

impl<T> Spinlock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> SpinlockGuard<'_, T> {
        while self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }
        SpinlockGuard {
            lock: self,
            data: unsafe { &mut *self.data.get() },
        }
    }
}

impl<'a, T> Drop for SpinlockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.lock.store(false, Ordering::Release);
    }
}

impl<'a, T> Deref for SpinlockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.data
    }
}

impl<'a, T> DerefMut for SpinlockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}

pub struct RingBuffer<const N: usize> {
    buffer: [u8; N],
    head: usize,
    tail: usize,
    full: bool,
}

impl<const N: usize> RingBuffer<N> {
    pub const fn new() -> Self {
        Self {
            buffer: [0; N],
            head: 0,
            tail: 0,
            full: false,
        }
    }

    pub fn push(&mut self, byte: u8) -> bool {
        if self.full {
            return false;
        }

        self.buffer[self.head] = byte;
        self.head = (self.head + 1) % N;
        self.full = self.head == self.tail;
        true
    }

    pub fn pop(&mut self) -> Option<u8> {
        if !self.full && self.head == self.tail {
            return None;
        }

        let byte = self.buffer[self.tail];
        self.tail = (self.tail + 1) % N;
        self.full = false;
        Some(byte)
    }

    pub fn is_empty(&self) -> bool {
        !self.full && self.head == self.tail
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    #[test]
    fn test_spinlock_basic() {
        let lock = Spinlock::new(42);
        {
            let mut guard = lock.lock();
            assert_eq!(*guard, 42);
            *guard = 43;
        }
        assert_eq!(*lock.lock(), 43);
    }

    #[test]
    fn test_ring_buffer_fifo() {
        let mut rb = RingBuffer::<4>::new();
        assert!(rb.is_empty());

        assert!(rb.push(1));
        assert!(rb.push(2));
        assert!(rb.push(3));
        assert!(rb.push(4));
        assert!(!rb.push(5)); // Full

        assert_eq!(rb.pop(), Some(1));
        assert_eq!(rb.pop(), Some(2));
        assert_eq!(rb.pop(), Some(3));
        assert_eq!(rb.pop(), Some(4));
        assert_eq!(rb.pop(), None);
        assert!(rb.is_empty());
    }

    #[test]
    fn test_ring_buffer_wrap_around() {
        let mut rb = RingBuffer::<2>::new();
        rb.push(1);
        rb.push(2);
        rb.pop();
        rb.push(3);
        assert_eq!(rb.pop(), Some(2));
        assert_eq!(rb.pop(), Some(3));
        assert!(rb.is_empty());
    }
}
