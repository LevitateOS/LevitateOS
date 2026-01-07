//! Signal handling
use crate::sysno::{SYS_KILL, SYS_PAUSE, SYS_RT_SIGACTION, SYS_RT_SIGPROCMASK, SYS_RT_SIGRETURN};

// TEAM_216: Signal constants
pub const SIGINT: i32 = 2;
pub const SIGKILL: i32 = 9;
pub const SIGCHLD: i32 = 17;

/// TEAM_216: Send a signal to a process.
#[inline]
pub fn kill(pid: i32, sig: i32) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_KILL,
            in("x0") pid,
            in("x1") sig,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_216: Wait for a signal.
#[inline]
pub fn pause() -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_PAUSE,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_216: Examine and change a signal action.
#[inline]
pub fn sigaction(sig: i32, handler: usize, restorer: usize) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_RT_SIGACTION,
            in("x0") sig,
            in("x1") handler,
            in("x2") restorer,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_216: Examine and change blocked signals.
#[inline]
pub fn sigprocmask(how: i32, set: usize, oldset: usize) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_RT_SIGPROCMASK,
            in("x0") how,
            in("x1") set,
            in("x2") oldset,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_216: Return from signal handler and cleanup stack frame.
#[inline]
pub fn sigreturn() -> ! {
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_RT_SIGRETURN,
            options(noreturn, nostack)
        );
    }
}
