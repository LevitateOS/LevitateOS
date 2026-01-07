//! Signal handling
//! TEAM_275: Refactored to use arch::syscallN

use crate::arch;
use crate::sysno::{SYS_KILL, SYS_PAUSE, SYS_RT_SIGACTION, SYS_RT_SIGPROCMASK, SYS_RT_SIGRETURN};

// TEAM_216: Signal constants
pub const SIGINT: i32 = 2;
pub const SIGKILL: i32 = 9;
pub const SIGCHLD: i32 = 17;

/// TEAM_216: Send a signal to a process.
#[inline]
pub fn kill(pid: i32, sig: i32) -> isize {
    arch::syscall2(SYS_KILL, pid as u64, sig as u64) as isize
}

/// TEAM_216: Wait for a signal.
#[inline]
pub fn pause() -> isize {
    arch::syscall0(SYS_PAUSE) as isize
}

/// TEAM_216: Examine and change a signal action.
#[inline]
pub fn sigaction(sig: i32, handler: usize, restorer: usize) -> isize {
    arch::syscall3(
        SYS_RT_SIGACTION,
        sig as u64,
        handler as u64,
        restorer as u64,
    ) as isize
}

/// TEAM_216: Examine and change blocked signals.
#[inline]
pub fn sigprocmask(how: i32, set: usize, oldset: usize) -> isize {
    arch::syscall3(SYS_RT_SIGPROCMASK, how as u64, set as u64, oldset as u64) as isize
}

/// TEAM_216: Return from signal handler and cleanup stack frame.
#[inline]
pub fn sigreturn() -> ! {
    arch::syscall_noreturn(SYS_RT_SIGRETURN)
}
