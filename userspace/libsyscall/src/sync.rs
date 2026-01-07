//! Synchronization (Futex)
use crate::sysno::SYS_FUTEX;

/// TEAM_208: Futex operations
pub mod futex_ops {
    pub const FUTEX_WAIT: usize = 0;
    pub const FUTEX_WAKE: usize = 1;
}

/// TEAM_208: Fast userspace mutex operation.
///
/// # Arguments
/// * `addr` - Pointer to a 4-byte aligned u32 value
/// * `op` - Operation (FUTEX_WAIT or FUTEX_WAKE)
/// * `val` - Expected value (for WAIT) or max waiters to wake (for WAKE)
///
/// # Returns
/// * FUTEX_WAIT: 0 on success, -11 (EAGAIN) if value mismatch
/// * FUTEX_WAKE: Number of tasks woken
#[inline]
pub fn futex(addr: *const u32, op: usize, val: u32) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_FUTEX,
            in("x0") addr as usize,
            in("x1") op,
            in("x2") val as usize,
            in("x3") 0usize, // timeout (unused)
            in("x4") 0usize, // addr2 (unused)
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_208: sys_futex syscall wrapper.
#[inline]
pub fn sys_futex(
    uaddr: usize,
    op: i32,
    val: u32,
    timeout: usize,
    uaddr2: usize,
    val3: u32,
) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_FUTEX,
            in("x0") uaddr,
            in("x1") op,
            in("x2") val,
            in("x3") timeout,
            in("x4") uaddr2,
            in("x5") val3,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}
