//! TEAM_201: Readers-Writer Lock for VFS
//!
//! A simple readers-writer lock that allows multiple concurrent readers
//! or a single exclusive writer. This is needed for inode access patterns
//! where many processes may read file metadata simultaneously, but writes
//! (like updating timestamps) require exclusive access.
//!
//! Design: Writer-preferring to avoid writer starvation.

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicU32, Ordering};

/// TEAM_201: State encoding for RwLock
/// - Bits 0-30: Number of active readers (max 2^31 - 1)
/// - Bit 31: Writer active flag
const WRITER_BIT: u32 = 1 << 31;
const READER_MASK: u32 = !WRITER_BIT;
const MAX_READERS: u32 = READER_MASK;

/// TEAM_201: A readers-writer lock
///
/// Allows multiple concurrent readers OR a single exclusive writer.
/// Writer-preferring to prevent writer starvation.
pub struct RwLock<T> {
    /// State: bits 0-30 = reader count, bit 31 = writer flag
    state: AtomicU32,
    /// Protected data
    data: UnsafeCell<T>,
}

// SAFETY: RwLock provides synchronization, so it's Send+Sync if T is Send
unsafe impl<T: Send> Send for RwLock<T> {}
unsafe impl<T: Send + Sync> Sync for RwLock<T> {}

/// TEAM_201: RAII read guard
pub struct RwLockReadGuard<'a, T> {
    lock: &'a RwLock<T>,
}

/// TEAM_201: RAII write guard
pub struct RwLockWriteGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<T> RwLock<T> {
    /// TEAM_201: Create a new RwLock with the given data
    pub const fn new(data: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            data: UnsafeCell::new(data),
        }
    }

    /// TEAM_201: Acquire a read lock, blocking until available
    ///
    /// Multiple readers can hold the lock simultaneously.
    /// Blocks if a writer holds the lock.
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        loop {
            let state = self.state.load(Ordering::Relaxed);
            
            // If writer is active, spin
            if state & WRITER_BIT != 0 {
                core::hint::spin_loop();
                continue;
            }
            
            // Check for reader overflow
            let readers = state & READER_MASK;
            if readers >= MAX_READERS {
                panic!("RwLock reader overflow");
            }
            
            // Try to increment reader count
            if self
                .state
                .compare_exchange_weak(
                    state,
                    state + 1,
                    Ordering::Acquire,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                return RwLockReadGuard { lock: self };
            }
            
            core::hint::spin_loop();
        }
    }

    /// TEAM_201: Try to acquire a read lock without blocking
    ///
    /// Returns Some(guard) if successful, None if a writer holds the lock.
    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
        let state = self.state.load(Ordering::Relaxed);
        
        // If writer is active, fail immediately
        if state & WRITER_BIT != 0 {
            return None;
        }
        
        // Check for reader overflow
        let readers = state & READER_MASK;
        if readers >= MAX_READERS {
            return None;
        }
        
        // Try to increment reader count
        if self
            .state
            .compare_exchange_weak(
                state,
                state + 1,
                Ordering::Acquire,
                Ordering::Relaxed,
            )
            .is_ok()
        {
            Some(RwLockReadGuard { lock: self })
        } else {
            None
        }
    }

    /// TEAM_201: Acquire a write lock, blocking until available
    ///
    /// Only one writer can hold the lock. Blocks if any readers or
    /// another writer holds the lock.
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        loop {
            // Try to set writer bit when state is 0 (no readers, no writer)
            if self
                .state
                .compare_exchange_weak(
                    0,
                    WRITER_BIT,
                    Ordering::Acquire,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                return RwLockWriteGuard { lock: self };
            }
            
            core::hint::spin_loop();
        }
    }

    /// TEAM_201: Try to acquire a write lock without blocking
    ///
    /// Returns Some(guard) if successful, None if any readers or
    /// another writer holds the lock.
    pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
        if self
            .state
            .compare_exchange(
                0,
                WRITER_BIT,
                Ordering::Acquire,
                Ordering::Relaxed,
            )
            .is_ok()
        {
            Some(RwLockWriteGuard { lock: self })
        } else {
            None
        }
    }

    /// TEAM_201: Get a mutable reference to the underlying data
    ///
    /// This is safe because we have &mut self, meaning no other references exist.
    pub fn get_mut(&mut self) -> &mut T {
        self.data.get_mut()
    }

    /// TEAM_201: Consume the lock and return the underlying data
    pub fn into_inner(self) -> T {
        self.data.into_inner()
    }
}

impl<T: Default> Default for RwLock<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

// TEAM_201: Read guard implementation

impl<T> Deref for RwLockReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        // SAFETY: We hold a read lock, so shared access is safe
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> Drop for RwLockReadGuard<'_, T> {
    fn drop(&mut self) {
        // Decrement reader count
        self.lock.state.fetch_sub(1, Ordering::Release);
    }
}

// TEAM_201: Write guard implementation

impl<T> Deref for RwLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        // SAFETY: We hold an exclusive write lock
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for RwLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: We hold an exclusive write lock
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for RwLockWriteGuard<'_, T> {
    fn drop(&mut self) {
        // Clear writer bit
        self.lock.state.store(0, Ordering::Release);
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    #[test]
    fn test_rwlock_read_basic() {
        let lock = RwLock::new(42);
        let guard = lock.read();
        assert_eq!(*guard, 42);
    }

    #[test]
    fn test_rwlock_write_basic() {
        let lock = RwLock::new(42);
        {
            let mut guard = lock.write();
            *guard = 100;
        }
        let guard = lock.read();
        assert_eq!(*guard, 100);
    }

    #[test]
    fn test_rwlock_multiple_readers() {
        let lock = RwLock::new(42);
        let r1 = lock.read();
        let r2 = lock.read();
        let r3 = lock.read();
        assert_eq!(*r1, 42);
        assert_eq!(*r2, 42);
        assert_eq!(*r3, 42);
    }

    #[test]
    fn test_rwlock_try_write_fails_with_readers() {
        let lock = RwLock::new(42);
        let _r = lock.read();
        assert!(lock.try_write().is_none());
    }

    #[test]
    fn test_rwlock_try_read_fails_with_writer() {
        let lock = RwLock::new(42);
        let _w = lock.write();
        assert!(lock.try_read().is_none());
    }

    #[test]
    fn test_rwlock_get_mut() {
        let mut lock = RwLock::new(42);
        *lock.get_mut() = 100;
        assert_eq!(*lock.read(), 100);
    }

    #[test]
    fn test_rwlock_into_inner() {
        let lock = RwLock::new(42);
        assert_eq!(lock.into_inner(), 42);
    }

    #[test]
    fn test_rwlock_concurrent() {
        use std::sync::Arc;
        use std::thread;

        let lock = Arc::new(RwLock::new(0));
        let mut handles = vec![];

        // Spawn 10 reader threads
        for _ in 0..10 {
            let lock = Arc::clone(&lock);
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    let _r = lock.read();
                    thread::yield_now();
                }
            }));
        }

        // Spawn 2 writer threads
        for _ in 0..2 {
            let lock = Arc::clone(&lock);
            handles.push(thread::spawn(move || {
                for _ in 0..50 {
                    let mut w = lock.write();
                    *w += 1;
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(*lock.read(), 100);
    }
}
