//! Time operations
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
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_NANOSLEEP,
            in("x0") seconds,
            in("x1") nanoseconds,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_170: Get current monotonic time.
#[inline]
pub fn clock_gettime(ts: &mut Timespec) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_CLOCK_GETTIME,
            in("x0") ts as *mut Timespec,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}
