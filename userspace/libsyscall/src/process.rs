//! Process management
//! TEAM_275: Refactored to use arch::syscallN

use crate::arch;
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
    arch::syscall_exit(SYS_EXIT, code as u64)
}

/// Get current process ID.
#[inline]
pub fn getpid() -> i64 {
    arch::syscall0(SYS_GETPID)
}

/// TEAM_217: Get parent process ID.
#[inline]
pub fn getppid() -> i64 {
    arch::syscall0(SYS_GETPPID)
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
    arch::syscall5(
        SYS_CLONE,
        flags,
        stack as u64,
        parent_tid as u64,
        tls as u64,
        child_tid as u64,
    ) as isize
}

/// TEAM_228: Set pointer to thread ID (cleared on exit).
#[inline]
pub fn set_tid_address(tidptr: *mut i32) -> isize {
    arch::syscall1(SYS_SET_TID_ADDRESS, tidptr as u64) as isize
}

/// Spawn a new process from a path.
#[inline]
pub fn spawn(path: &str) -> isize {
    arch::syscall2(SYS_SPAWN, path.as_ptr() as u64, path.len() as u64) as isize
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

    arch::syscall4(
        SYS_SPAWN_ARGS,
        path.as_ptr() as u64,
        path.len() as u64,
        entries.as_ptr() as u64,
        argc as u64,
    ) as isize
}

/// Replace current process with a new one from a path.
#[inline]
pub fn exec(path: &str) -> isize {
    arch::syscall2(SYS_EXEC, path.as_ptr() as u64, path.len() as u64) as isize
}

/// TEAM_188: Wait for a child process to exit.
#[inline]
pub fn waitpid(pid: i32, status: Option<&mut i32>) -> isize {
    let status_ptr = match status {
        Some(s) => s as *mut i32 as u64,
        None => 0,
    };
    arch::syscall2(SYS_WAITPID, pid as u64, status_ptr) as isize
}

/// TEAM_220: Set the foreground process for shell control.
#[inline]
pub fn set_foreground(pid: usize) -> isize {
    arch::syscall1(SYS_SET_FOREGROUND, pid as u64) as isize
}

/// TEAM_244: Get the foreground process PID.
#[inline]
pub fn get_foreground() -> isize {
    arch::syscall0(SYS_GET_FOREGROUND) as isize
}

/// TEAM_142: Graceful system shutdown.
#[inline]
pub fn shutdown(flags: u32) -> ! {
    arch::syscall_exit(SYS_SHUTDOWN, flags as u64)
}
