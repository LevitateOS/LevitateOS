//! Process management
use crate::sysno::{
    SYS_CLONE, SYS_EXEC, SYS_EXIT, SYS_GETPID, SYS_GETPPID, SYS_GET_FOREGROUND, SYS_SET_FOREGROUND,
    SYS_SET_TID_ADDRESS, SYS_SHUTDOWN, SYS_SPAWN, SYS_SPAWN_ARGS, SYS_WAITPID,
};

// TEAM_228: Clone flags
pub const CLONE_VM: u64 = 0x00000100;
pub const CLONE_FS: u64 = 0x00000200;
pub const CLONE_FILES: u64 = 0x00000400;
pub const CLONE_SIGHAND: u64 = 0x00000800;
pub const CLONE_THREAD: u64 = 0x00010000;
pub const CLONE_SETTLS: u64 = 0x00080000;
pub const CLONE_PARENT_SETTID: u64 = 0x00100000;
pub const CLONE_CHILD_CLEARTID: u64 = 0x00200000;
pub const CLONE_CHILD_SETTID: u64 = 0x01000000;

/// TEAM_142: Shutdown flags
pub mod shutdown_flags {
    /// Normal shutdown (minimal output)
    pub const NORMAL: u32 = 0;
    /// Verbose shutdown (for golden file testing)
    pub const VERBOSE: u32 = 1;
}

/// TEAM_186: Argv entry for spawn_args syscall.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ArgvEntry {
    /// Pointer to argument string
    pub ptr: *const u8,
    /// Length of argument string
    pub len: usize,
}

/// Exit the process.
///
/// # Arguments
/// * `code` - Exit code (0 = success)
#[inline]
pub fn exit(code: i32) -> ! {
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_EXIT,
            in("x0") code,
            options(noreturn, nostack)
        );
    }
}

/// Get current process ID.
#[inline]
pub fn getpid() -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_GETPID,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret
}

/// TEAM_217: Get parent process ID.
#[inline]
pub fn getppid() -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_GETPPID,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret
}

/// TEAM_228: Create a new thread (clone syscall).
#[inline]
pub fn clone(
    flags: u64,
    stack: usize,
    parent_tid: *mut i32,
    tls: usize,
    child_tid: *mut i32,
) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_CLONE,
            in("x0") flags,
            in("x1") stack,
            in("x2") parent_tid,
            in("x3") tls,
            in("x4") child_tid,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_228: Set pointer to thread ID (cleared on exit).
#[inline]
pub fn set_tid_address(tidptr: *mut i32) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_SET_TID_ADDRESS,
            in("x0") tidptr,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// Spawn a new process from a path.
#[inline]
pub fn spawn(path: &str) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_SPAWN,
            in("x0") path.as_ptr(),
            in("x1") path.len(),
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_186: Spawn a process with command-line arguments.
#[inline]
pub fn spawn_args(path: &str, argv: &[&str]) -> isize {
    // Build ArgvEntry array on stack (max 16 args)
    let mut entries = [ArgvEntry {
        ptr: core::ptr::null(),
        len: 0,
    }; 16];
    let argc = argv.len().min(16);
    for (i, arg) in argv.iter().take(argc).enumerate() {
        entries[i] = ArgvEntry {
            ptr: arg.as_ptr(),
            len: arg.len(),
        };
    }

    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_SPAWN_ARGS,
            in("x0") path.as_ptr(),
            in("x1") path.len(),
            in("x2") entries.as_ptr(),
            in("x3") argc,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// Replace current process with a new one from a path.
#[inline]
pub fn exec(path: &str) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_EXEC,
            in("x0") path.as_ptr(),
            in("x1") path.len(),
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_188: Wait for a child process to exit.
#[inline]
pub fn waitpid(pid: i32, status: Option<&mut i32>) -> isize {
    let status_ptr = match status {
        Some(s) => s as *mut i32 as usize,
        None => 0,
    };

    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_WAITPID,
            in("x0") pid,
            in("x1") status_ptr,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_220: Set the foreground process for shell control.
#[inline]
pub fn set_foreground(pid: usize) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_SET_FOREGROUND,
            in("x0") pid,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_244: Get the foreground process PID.
#[inline]
pub fn get_foreground() -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_GET_FOREGROUND,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_142: Graceful system shutdown.
#[inline]
pub fn shutdown(flags: u32) -> ! {
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_SHUTDOWN,
            in("x0") flags,
            options(noreturn, nostack)
        );
    }
}
