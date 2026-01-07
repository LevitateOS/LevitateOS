//! TTY and Terminal operations
//! TEAM_275: Refactored to use arch::syscallN

use crate::arch;
use crate::sysno::{SYS_IOCTL, SYS_ISATTY};

// TEAM_244: ioctl requests for TTY
pub const TCGETS: u64 = 0x5401; // tcgetattr
pub const TCSETS: u64 = 0x5402; // tcsetattr TCSANOW
pub const TCSETSW: u64 = 0x5403; // tcsetattr TCSADRAIN
pub const TCSETSF: u64 = 0x5404; // tcsetattr TCSAFLUSH

/// TEAM_244: Get terminal attributes (POSIX tcgetattr).
/// Returns 0 on success, negative error on failure.
#[inline]
pub fn tcgetattr(fd: i32, termios_p: *mut u8) -> isize {
    arch::syscall3(SYS_IOCTL, fd as u64, TCGETS, termios_p as u64) as isize
}

/// TEAM_244: Set terminal attributes (POSIX tcsetattr).
/// Returns 0 on success, negative error on failure.
#[inline]
pub fn tcsetattr(fd: i32, optional_actions: i32, termios_p: *const u8) -> isize {
    let request = match optional_actions {
        0 => TCSETS,  // TCSANOW
        1 => TCSETSW, // TCSADRAIN
        2 => TCSETSF, // TCSAFLUSH
        _ => TCSETS,
    };
    arch::syscall3(SYS_IOCTL, fd as u64, request, termios_p as u64) as isize
}

/// TEAM_244: Check if fd refers to a terminal.
/// Returns 1 if tty, 0 if not, negative error on failure.
#[inline]
pub fn isatty(fd: i32) -> isize {
    arch::syscall1(SYS_ISATTY, fd as u64) as isize
}
