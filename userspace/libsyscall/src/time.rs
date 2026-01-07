//! Time operations
//! TEAM_275: Refactored to use arch::syscallN

use crate::arch;
use crate::sysno::{SYS_CLOCK_GETTIME, SYS_NANOSLEEP};

/// TEAM_217: Linux-compatible Timespec.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Timespec {
    /// Seconds
    pub tv_sec: i64,
    /// Nanoseconds
    pub tv_nsec: i64,
}

/// TEAM_170: Sleep for specified duration.
#[inline]
pub fn nanosleep(seconds: u64, nanoseconds: u64) -> isize {
    arch::syscall2(SYS_NANOSLEEP, seconds, nanoseconds) as isize
}

/// TEAM_170: Get current monotonic time.
#[inline]
pub fn clock_gettime(ts: &mut Timespec) -> isize {
    arch::syscall1(SYS_CLOCK_GETTIME, ts as *mut Timespec as u64) as isize
}
