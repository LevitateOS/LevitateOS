//! Scheduling
//! TEAM_275: Refactored to use arch::syscallN

use crate::arch;
use crate::sysno::SYS_SCHED_YIELD;

/// Yield execution to another thread.
#[inline]
pub fn sched_yield() {
    arch::syscall0(SYS_SCHED_YIELD);
}

#[inline]
pub fn yield_cpu() {
    sched_yield();
}
