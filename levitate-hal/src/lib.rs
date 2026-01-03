#![cfg_attr(not(feature = "std"), no_std)]

pub mod console;
pub mod gic;
pub mod interrupts;
pub mod mmu;
pub mod timer;
pub mod uart_pl011;

use levitate_utils::{Spinlock, SpinlockGuard};

pub struct IrqSafeLock<T> {
    inner: Spinlock<T>,
}

impl<T> IrqSafeLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            inner: Spinlock::new(data),
        }
    }

    pub fn lock(&self) -> IrqSafeLockGuard<'_, T> {
        let state = interrupts::disable();
        let guard = self.inner.lock();
        IrqSafeLockGuard {
            guard: Some(guard),
            state,
        }
    }
}

pub struct IrqSafeLockGuard<'a, T> {
    guard: Option<SpinlockGuard<'a, T>>,
    state: u64,
}

impl<'a, T> core::ops::Deref for IrqSafeLockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.guard.as_ref().unwrap()
    }
}

impl<'a, T> core::ops::DerefMut for IrqSafeLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.guard.as_mut().unwrap()
    }
}

impl<'a, T> Drop for IrqSafeLockGuard<'a, T> {
    fn drop(&mut self) {
        self.guard.take();
        interrupts::restore(self.state);
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    #[test]
    fn test_irq_safe_lock_behavior() {
        let lock = IrqSafeLock::new(10);

        // Initially enabled (in mock)
        assert!(interrupts::is_enabled());

        {
            let mut guard = lock.lock();
            assert_eq!(*guard, 10);
            *guard = 20;

            // Interrupts should be disabled while holding the lock
            assert!(!interrupts::is_enabled());
        }

        // Interrupts should be restored after dropping guard
        assert!(interrupts::is_enabled());
        assert_eq!(*lock.lock(), 20);
    }

    #[test]
    fn test_irq_safe_lock_nested() {
        let lock1 = IrqSafeLock::new(1);
        let lock2 = IrqSafeLock::new(2);

        assert!(interrupts::is_enabled());
        {
            let _g1 = lock1.lock();
            assert!(!interrupts::is_enabled());
            {
                let _g2 = lock2.lock();
                assert!(!interrupts::is_enabled());
            }
            // Still disabled after g2 dropped
            assert!(!interrupts::is_enabled());
        }
        // Finally restored after g1 dropped
        assert!(interrupts::is_enabled());
    }
}
