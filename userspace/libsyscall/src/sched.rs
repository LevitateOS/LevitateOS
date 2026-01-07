//! Scheduling
use crate::sysno::SYS_SCHED_YIELD;

/// Yield execution to another thread.
#[inline]
pub fn sched_yield() {
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_SCHED_YIELD,
            options(nostack)
        );
    }
}

#[inline]
pub fn yield_cpu() {
    sched_yield();
}
