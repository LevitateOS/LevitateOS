//! TTY and Terminal operations
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
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_IOCTL,
            in("x0") fd,
            in("x1") TCGETS,
            in("x2") termios_p,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
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
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_IOCTL,
            in("x0") fd,
            in("x1") request,
            in("x2") termios_p,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_244: Check if fd refers to a terminal.
/// Returns 1 if tty, 0 if not, negative error on failure.
#[inline]
pub fn isatty(fd: i32) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_ISATTY,
            in("x0") fd,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}
