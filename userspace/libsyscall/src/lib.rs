//! Userspace Syscall Library for `LevitateOS`
//!
//! `TEAM_118`: Extracted from shell to provide a single source of truth for syscall ABI.
//!
//! ## Usage
//! ```rust
//! use libsyscall::{read, write, exit, println};
//! ```

#![no_std]

use core::panic::PanicInfo;

// ============================================================================
// Syscall Numbers — Linux AArch64 ABI (TEAM_210)
// Reference: https://github.com/torvalds/linux/blob/master/include/uapi/asm-generic/unistd.h
// ============================================================================

// Filesystem
pub const SYS_GETCWD: u64 = 17;
pub const SYS_MKDIRAT: u64 = 34;
pub const SYS_UNLINKAT: u64 = 35;
pub const SYS_SYMLINKAT: u64 = 36;
pub const SYS_LINKAT: u64 = 37;
pub const SYS_RENAMEAT: u64 = 38;
pub const SYS_UMOUNT: u64 = 39;
pub const SYS_MOUNT: u64 = 40;
pub const SYS_OPENAT: u64 = 56;
pub const SYS_CLOSE: u64 = 57;
pub const SYS_GETDENTS: u64 = 61;
pub const SYS_READ: u64 = 63;
pub const SYS_WRITE: u64 = 64;
pub const SYS_READLINKAT: u64 = 78;
pub const SYS_FSTAT: u64 = 80;
pub const SYS_UTIMENSAT: u64 = 88;

// Process
pub const SYS_EXIT: u64 = 93;
pub const SYS_GETPID: u64 = 172;
pub const SYS_GETPPID: u64 = 173; // TEAM_217: Added standard Linux syscall
pub const SYS_SBRK: u64 = 214; // brk
pub const SYS_EXEC: u64 = 221; // execve
pub const SYS_WAITPID: u64 = 260; // wait4
pub const SYS_KILL: u64 = 129;
pub const SYS_RT_SIGACTION: u64 = 134;
pub const SYS_RT_SIGPROCMASK: u64 = 135;
pub const SYS_RT_SIGRETURN: u64 = 139;
pub const SYS_PAUSE: u64 = 236;

// Synchronization
pub const SYS_FUTEX: u64 = 98;

// Time
pub const SYS_NANOSLEEP: u64 = 101;
pub const SYS_CLOCK_GETTIME: u64 = 113;
pub const SYS_SCHED_YIELD: u64 = 124; // TEAM_217: Renamed to match Linux
pub const SYS_SHUTDOWN: u64 = 142; // reboot

// TEAM_228: Memory management syscalls
pub const SYS_MMAP: u64 = 222;
pub const SYS_MUNMAP: u64 = 215;
pub const SYS_MPROTECT: u64 = 226;

// TEAM_228: mmap protection flags
pub const PROT_NONE: u32 = 0;
pub const PROT_READ: u32 = 1;
pub const PROT_WRITE: u32 = 2;
pub const PROT_EXEC: u32 = 4;

// TEAM_228: mmap flags
pub const MAP_SHARED: u32 = 0x01;
pub const MAP_PRIVATE: u32 = 0x02;
pub const MAP_FIXED: u32 = 0x10;
pub const MAP_ANONYMOUS: u32 = 0x20;

// TEAM_228: Threading syscalls
pub const SYS_CLONE: u64 = 220;
pub const SYS_SET_TID_ADDRESS: u64 = 96;

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

// TEAM_233: Pipe and dup syscalls
pub const SYS_DUP: u64 = 23;
pub const SYS_DUP3: u64 = 24;
pub const SYS_PIPE2: u64 = 59;

// TEAM_244: TTY syscalls (POSIX termios)
pub const SYS_IOCTL: u64 = 29;
pub const SYS_ISATTY: u64 = 1010; // Custom - Linux uses ioctl

// TEAM_244: ioctl requests for TTY
pub const TCGETS: u64 = 0x5401;  // tcgetattr
pub const TCSETS: u64 = 0x5402;  // tcsetattr TCSANOW
pub const TCSETSW: u64 = 0x5403; // tcsetattr TCSADRAIN
pub const TCSETSF: u64 = 0x5404; // tcsetattr TCSAFLUSH

// Custom LevitateOS (temporary, until clone/execve work)
pub const SYS_SPAWN: u64 = 1000;
pub const SYS_SPAWN_ARGS: u64 = 1001;
pub const SYS_SET_FOREGROUND: u64 = 1002;
pub const SYS_GET_FOREGROUND: u64 = 1003; // TEAM_244: Get foreground PID

/// TEAM_208: Futex operations
pub mod futex_ops {
    pub const FUTEX_WAIT: usize = 0;
    pub const FUTEX_WAKE: usize = 1;
}

pub mod errno {
    pub const EPERM: i64 = -1;
    pub const ENOENT: i64 = -2;
    pub const ESRCH: i64 = -3;
    pub const EINTR: i64 = -4;
    pub const EIO: i64 = -5;
    pub const ENXIO: i64 = -6;
    pub const E2BIG: i64 = -7;
    pub const ENOEXEC: i64 = -8;
    pub const EBADF: i64 = -9;
    pub const ECHILD: i64 = -10;
    pub const EAGAIN: i64 = -11;
    pub const ENOMEM: i64 = -12;
    pub const EACCES: i64 = -13;
    pub const EFAULT: i64 = -14;
    pub const EBUSY: i64 = -16;
    pub const EEXIST: i64 = -17;
    pub const EXDEV: i64 = -18;
    pub const ENODEV: i64 = -19;
    pub const ENOTDIR: i64 = -20;
    pub const EISDIR: i64 = -21;
    pub const EINVAL: i64 = -22;
    pub const ENFILE: i64 = -23;
    pub const EMFILE: i64 = -24;
    pub const ENOTTY: i64 = -25;
    pub const ETXTBSY: i64 = -26;
    pub const EFBIG: i64 = -27;
    pub const ENOSPC: i64 = -28;
    pub const ESPIPE: i64 = -29;
    pub const EROFS: i64 = -30;
    pub const EMLINK: i64 = -31;
    pub const EPIPE: i64 = -32;
    pub const ENOSYS: i64 = -38;
    pub const ENOTEMPTY: i64 = -39;
    pub const ELOOP: i64 = -40;
}

// TEAM_244: Re-export commonly used errno values at module level
pub use errno::ENOSYS;

// ============================================================================
// Syscall Wrappers
// ============================================================================

/// TEAM_217: Vectored write.
#[inline]
pub fn writev(fd: usize, iov: &[IoVec]) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_WRITEV,
            in("x0") fd,
            in("x1") iov.as_ptr(),
            in("x2") iov.len(),
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_217: Vectored read.
#[inline]
pub fn readv(fd: usize, iov: &mut [IoVec]) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_READV,
            in("x0") fd,
            in("x1") iov.as_mut_ptr(),
            in("x2") iov.len(),
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_217: sys_writev constant
pub const SYS_WRITEV: u64 = 66;
/// TEAM_217: sys_readv constant
pub const SYS_READV: u64 = 65;

/// TEAM_217: struct iovec for writev/readv
#[repr(C)]
pub struct IoVec {
    pub base: *const u8,
    pub len: usize,
}

/// Read from a file descriptor.
///
/// # Arguments
/// * `fd` - File descriptor (0 = stdin)
/// * `buf` - Buffer to read into
///
/// # Returns
/// Number of bytes read, or negative error code.
#[inline]
pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_READ,
            in("x0") fd,
            in("x1") buf.as_mut_ptr(),
            in("x2") buf.len(),
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// Write to a file descriptor.
///
/// # Arguments
/// * `fd` - File descriptor (1 = stdout, 2 = stderr)
/// * `buf` - Buffer to write from
///
/// # Returns
/// Number of bytes written, or negative error code.
#[inline]
pub fn write(fd: usize, buf: &[u8]) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_WRITE,
            in("x0") fd,
            in("x1") buf.as_ptr(),
            in("x2") buf.len(),
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
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

/// Adjust program break (heap allocation).
#[inline]
pub fn sbrk(increment: isize) -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_SBRK,
            in("x0") increment,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret
}

/// TEAM_228: Map memory into process address space.
///
/// # Arguments
/// * `addr` - Hint address (can be 0 for system to choose)
/// * `len` - Length of mapping
/// * `prot` - Protection flags (PROT_READ | PROT_WRITE | PROT_EXEC)
/// * `flags` - Mapping flags (must include MAP_ANONYMOUS | MAP_PRIVATE)
/// * `fd` - File descriptor (-1 for anonymous)
/// * `offset` - File offset (0 for anonymous)
///
/// # Returns
/// Virtual address of mapping, or negative error code.
#[inline]
pub fn mmap(addr: usize, len: usize, prot: u32, flags: u32, fd: i32, offset: usize) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_MMAP,
            in("x0") addr,
            in("x1") len,
            in("x2") prot,
            in("x3") flags,
            in("x4") fd,
            in("x5") offset,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_228: Unmap memory from process address space.
///
/// # Arguments
/// * `addr` - Start address of mapping (must be page-aligned)
/// * `len` - Length to unmap
///
/// # Returns
/// 0 on success, negative error code on failure.
#[inline]
pub fn munmap(addr: usize, len: usize) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_MUNMAP,
            in("x0") addr,
            in("x1") len,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_228: Change protection on memory region.
///
/// # Arguments
/// * `addr` - Start address (must be page-aligned)
/// * `len` - Length of region
/// * `prot` - New protection flags
///
/// # Returns
/// 0 on success, negative error code on failure.
#[inline]
pub fn mprotect(addr: usize, len: usize, prot: u32) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_MPROTECT,
            in("x0") addr,
            in("x1") len,
            in("x2") prot,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_228: Create a new thread (clone syscall).
///
/// # Arguments
/// * `flags` - Clone flags (CLONE_VM | CLONE_THREAD etc.)
/// * `stack` - New stack pointer for child
/// * `parent_tid` - Address to write parent TID
/// * `tls` - TLS pointer for child
/// * `child_tid` - Address for child TID operations
///
/// # Returns
/// Child TID to parent, 0 to child, or negative error code.
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
///
/// # Arguments
/// * `tidptr` - Address to store TID, cleared when thread exits
///
/// # Returns
/// Current thread ID.
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

/// TEAM_233: Create a pipe.
///
/// # Arguments
/// * `pipefd` - Array to store read and write fd [read_fd, write_fd]
/// * `flags` - Pipe flags (O_CLOEXEC, O_NONBLOCK)
///
/// # Returns
/// 0 on success, negative error code on failure.
#[inline]
pub fn pipe2(pipefd: &mut [i32; 2], flags: u32) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_PIPE2,
            in("x0") pipefd.as_mut_ptr(),
            in("x1") flags,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_233: Duplicate a file descriptor to lowest available.
///
/// # Arguments
/// * `oldfd` - File descriptor to duplicate
///
/// # Returns
/// New fd on success, negative error code on failure.
#[inline]
pub fn dup(oldfd: usize) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_DUP,
            in("x0") oldfd,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_233: Duplicate a file descriptor to a specific number.
///
/// # Arguments
/// * `oldfd` - File descriptor to duplicate
/// * `newfd` - Target fd number (closed if already open)
///
/// # Returns
/// newfd on success, negative error code on failure.
#[inline]
pub fn dup2(oldfd: usize, newfd: usize) -> isize {
    dup3(oldfd, newfd, 0)
}

/// TEAM_233: Duplicate a file descriptor with flags.
///
/// # Arguments
/// * `oldfd` - File descriptor to duplicate
/// * `newfd` - Target fd number
/// * `flags` - Flags (O_CLOEXEC)
///
/// # Returns
/// newfd on success, negative error code on failure.
#[inline]
pub fn dup3(oldfd: usize, newfd: usize, flags: u32) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_DUP3,
            in("x0") oldfd,
            in("x1") newfd,
            in("x2") flags,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// Spawn a new process from a path.
///
/// # Arguments
/// * `path` - Path to the executable
///
/// # Returns
/// PID of the new process, or negative error code.
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

/// Replace current process with a new one from a path.
///
/// # Arguments
/// * `path` - Path to the executable
///
/// # Returns
/// Does not return on success. Negative error code on failure.
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

/// TEAM_186: Argv entry for spawn_args syscall.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ArgvEntry {
    /// Pointer to argument string
    pub ptr: *const u8,
    /// Length of argument string
    pub len: usize,
}

/// TEAM_186: Spawn a process with command-line arguments.
///
/// # Arguments
/// * `path` - Path to the executable
/// * `argv` - Command-line arguments (including program name as argv[0])
///
/// # Returns
/// PID of the new process, or negative error code.
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

/// TEAM_188: Wait for a child process to exit.
///
/// # Arguments
/// * `pid` - PID of child to wait for (must be > 0)
/// * `status` - Optional pointer to store exit status
///
/// # Returns
/// PID of exited child on success, negative error code on failure.
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

/// TEAM_142: Shutdown flags
pub mod shutdown_flags {
    /// Normal shutdown (minimal output)
    pub const NORMAL: u32 = 0;
    /// Verbose shutdown (for golden file testing)
    pub const VERBOSE: u32 = 1;
}

#[inline]
pub fn yield_cpu() {
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_SCHED_YIELD,
            options(nostack)
        );
    }
}

/// TEAM_142: Graceful system shutdown.
///
/// # Arguments
/// * `flags` - Shutdown flags (see `shutdown_flags` module)
///   - `NORMAL` (0): Minimal output
///   - `VERBOSE` (1): Detailed output for golden file testing
///
/// # Returns
/// Does not return on success. Halts the system.
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

/// TEAM_208: sys_futex syscall wrapper.
///
/// # Arguments
/// * `uaddr` - Address of the futex word
/// * `op` - Operation (FUTEX_WAIT=0, FUTEX_WAKE=1)
/// * `val` - Expected value (for WAIT) or number of waiters to wake (for WAKE)
/// * `timeout` - Timeout (not implemented yet)
/// * `uaddr2` - Second futex address (not implemented yet)
/// * `val3` - Third value (not implemented yet)
///
/// # Returns
/// 0 on success, or negative error code.
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

// ============================================================================
// Signal Syscalls (TEAM_216)
// ============================================================================

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

// ============================================================================
// File Syscalls (TEAM_168: Phase 10 Step 3)
// ============================================================================

/// TEAM_168: Open a file.
///
/// # Arguments
/// * `path` - Path to the file
/// * `flags` - Open flags (0 for read-only)
///
/// # Returns
/// File descriptor on success, or negative error code.
#[inline]
pub fn openat(path: &str, flags: u32) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_OPENAT,
            in("x0") path.as_ptr(),
            in("x1") path.len(),
            in("x2") flags,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_168: Close a file descriptor.
///
/// # Arguments
/// * `fd` - File descriptor to close
///
/// # Returns
/// 0 on success, or negative error code.
#[inline]
pub fn close(fd: usize) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_CLOSE,
            in("x0") fd,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_217: Linux-compatible Stat structure (128 bytes).
/// Matches AArch64 asm-generic layout used by Rust std and musl/glibc.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Stat {
    /// Device ID containing file
    pub st_dev: u64,
    /// Inode number
    pub st_ino: u64,
    /// File type and permissions (S_IFMT | mode bits)
    pub st_mode: u32,
    /// Number of hard links
    pub st_nlink: u32,
    /// Owner user ID
    pub st_uid: u32,
    /// Owner group ID
    pub st_gid: u32,
    /// Device ID (if special file)
    pub st_rdev: u64,
    /// Padding for alignment
    pub __pad1: u64,
    /// File size in bytes
    pub st_size: i64,
    /// Block size for filesystem I/O
    pub st_blksize: i32,
    /// Padding for alignment
    pub __pad2: i32,
    /// Number of 512-byte blocks allocated
    pub st_blocks: i64,
    /// Access time (seconds)
    pub st_atime: i64,
    /// Access time (nanoseconds)
    pub st_atime_nsec: u64,
    /// Modification time (seconds)
    pub st_mtime: i64,
    /// Modification time (nanoseconds)
    pub st_mtime_nsec: u64,
    /// Status change time (seconds)
    pub st_ctime: i64,
    /// Status change time (nanoseconds)
    pub st_ctime_nsec: u64,
    /// Unused padding
    pub __unused: [u32; 2],
}

/// TEAM_168: Get file status.
///
/// # Arguments
/// * `fd` - File descriptor
/// * `stat` - Output buffer for file status
///
/// # Returns
/// 0 on success, or negative error code.
#[inline]
pub fn fstat(fd: usize, stat: &mut Stat) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_FSTAT,
            in("x0") fd,
            in("x1") stat as *mut Stat,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

// ============================================================================
// Time Syscalls (TEAM_170: Phase 10 Step 7)
// ============================================================================

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
///
/// # Arguments
/// * `seconds` - Number of seconds to sleep
/// * `nanoseconds` - Additional nanoseconds to sleep
///
/// # Returns
/// 0 on success, or negative error code.
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
///
/// # Arguments
/// * `ts` - Output buffer for timespec
///
/// # Returns
/// 0 on success, or negative error code.
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

// ============================================================================
// Directory Syscalls (TEAM_176: Directory Iteration)
// ============================================================================

// SYS_GETDENTS is defined above with Linux AArch64 number (61)

/// TEAM_176: Dirent64 structure for directory entries.
/// Matches Linux ABI layout.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Dirent64 {
    /// Inode number
    pub d_ino: u64,
    /// Offset to next entry
    pub d_off: i64,
    /// Length of this record
    pub d_reclen: u16,
    /// File type
    pub d_type: u8,
    // d_name follows (null-terminated, variable length)
}

/// TEAM_176: File type constants for d_type field.
pub mod d_type {
    pub const DT_UNKNOWN: u8 = 0;
    pub const DT_FIFO: u8 = 1;
    pub const DT_CHR: u8 = 2;
    pub const DT_DIR: u8 = 4;
    pub const DT_BLK: u8 = 6;
    pub const DT_REG: u8 = 8;
    pub const DT_LNK: u8 = 10;
    pub const DT_SOCK: u8 = 12;
}

/// TEAM_176: Read directory entries.
///
/// # Arguments
/// * `fd` - Directory file descriptor
/// * `buf` - Buffer to read entries into
///
/// # Returns
/// Number of bytes read on success, 0 at end of directory, or negative error code.
#[inline]
pub fn getdents(fd: usize, buf: &mut [u8]) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_GETDENTS,
            in("x0") fd,
            in("x1") buf.as_mut_ptr(),
            in("x2") buf.len(),
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_192: Get current working directory.
///
/// # Arguments
/// * `buf` - Buffer to write CWD string
///
/// # Returns
/// * Length of the CWD string (including NUL) on success, or negative error code.
#[inline]
pub fn getcwd(buf: &mut [u8]) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_GETCWD,
            in("x0") buf.as_mut_ptr(),
            in("x1") buf.len(),
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_192: Create directory.
#[inline]
pub fn mkdirat(dfd: i32, path: &str, mode: u32) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_MKDIRAT,
            in("x0") dfd,
            in("x1") path.as_ptr(),
            in("x2") path.len(),
            in("x3") mode,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_192: Remove file or directory.
#[inline]
pub fn unlinkat(dfd: i32, path: &str, flags: u32) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_UNLINKAT,
            in("x0") dfd,
            in("x1") path.as_ptr(),
            in("x2") path.len(),
            in("x3") flags,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_192: Rename/move file or directory.
#[inline]
pub fn renameat(old_dfd: i32, old_path: &str, new_dfd: i32, new_path: &str) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_RENAMEAT,
            in("x0") old_dfd,
            in("x1") old_path.as_ptr(),
            in("x2") old_path.len(),
            in("x3") new_dfd,
            in("x4") new_path.as_ptr(),
            in("x5") new_path.len(),
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_198: Create a symbolic link.
///
/// # Arguments
/// * `target` - Target path the symlink points to
/// * `linkdirfd` - Directory fd for link path (use AT_FDCWD)
/// * `linkpath` - Path for the new symlink
#[inline]
pub fn symlinkat(target: &str, linkdirfd: i32, linkpath: &str) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_SYMLINKAT,
            in("x0") target.as_ptr(),
            in("x1") target.len(),
            in("x2") linkdirfd,
            in("x3") linkpath.as_ptr(),
            in("x4") linkpath.len(),
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_209: Create a hard link.
#[inline]
pub fn linkat(olddfd: i32, oldpath: &str, newdfd: i32, newpath: &str, flags: u32) -> isize {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_LINKAT,
            in("x0") olddfd,
            in("x1") oldpath.as_ptr(),
            in("x2") oldpath.len(),
            in("x3") newdfd,
            in("x4") newpath.as_ptr(),
            in("x5") newpath.len(),
            in("x6") flags,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

/// TEAM_198: UTIME_NOW - set to current time
pub const UTIME_NOW: u64 = 0x3FFFFFFF;
/// TEAM_198: UTIME_OMIT - don't change
pub const UTIME_OMIT: u64 = 0x3FFFFFFE;

/// TEAM_198: Set file access and modification times.
///
/// # Arguments
/// * `dirfd` - Directory fd (use AT_FDCWD for cwd)
/// * `path` - Path to file
/// * `times` - Optional pointer to [atime, mtime] Timespec array. None = now.
/// * `flags` - AT_SYMLINK_NOFOLLOW to not follow symlinks
#[inline]
pub fn utimensat(dirfd: i32, path: &str, times: Option<&[Timespec; 2]>, flags: u32) -> isize {
    let ret: i64;
    let times_ptr = match times {
        Some(t) => t.as_ptr() as usize,
        None => 0,
    };
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") SYS_UTIMENSAT,
            in("x0") dirfd,
            in("x1") path.as_ptr(),
            in("x2") path.len(),
            in("x3") times_ptr,
            in("x4") flags,
            lateout("x0") ret,
            options(nostack)
        );
    }
    ret as isize
}

// ============================================================================
// Synchronization Syscalls (TEAM_208: Futex)
// ============================================================================

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

// ============================================================================
// Process Control (TEAM_220)
// ============================================================================

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

// ============================================================================
// TTY Syscalls (TEAM_244)
// ============================================================================

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
        0 => TCSETS,   // TCSANOW
        1 => TCSETSW,  // TCSADRAIN
        2 => TCSETSF,  // TCSAFLUSH
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

// ============================================================================
// Panic Handler (shared logic)
// ============================================================================

/// Common panic handler logic.
///
/// Call this from `#[panic_handler]` in each binary crate.
///
/// # Example
/// ```rust
/// #[panic_handler]
/// fn panic(info: &PanicInfo) -> ! {
///     libsyscall::common_panic_handler(info)
/// }
/// ```
pub fn common_panic_handler(_info: &PanicInfo) -> ! {
    // Use write() directly to avoid recursion through print! macros
    let msg = b"PANIC!\n";
    write(2, msg);
    exit(1);
}

// ============================================================================
// Print Macros
// ============================================================================

/// Print to stdout without newline.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let mut writer = $crate::Stdout;
        let _ = write!(writer, $($arg)*);
    }};
}

/// Print to stdout with newline.
#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}

/// Stdout writer for print! macro.
pub struct Stdout;

impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write(1, s.as_bytes());
        Ok(())
    }
}
