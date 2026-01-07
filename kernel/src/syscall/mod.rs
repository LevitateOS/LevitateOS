use crate::memory::user as mm_user;

pub mod fs;
pub mod mm;
pub mod process;
pub mod signal;
pub mod sync;
pub mod sys;
pub mod time;

use crate::arch::SyscallFrame;
use los_hal::println;

/// TEAM_073: Error codes for syscalls.
pub mod errno {
    pub const ENOENT: i64 = -2;
    pub const EBADF: i64 = -9;
    pub const EFAULT: i64 = -14;
    pub const EEXIST: i64 = -17;
    pub const EINVAL: i64 = -22;
    pub const ENOSYS: i64 = -38;
    pub const EIO: i64 = -5;
}

pub mod errno_file {
    pub const ENOENT: i64 = -2;
    pub const EMFILE: i64 = -24;
    pub const ENOTDIR: i64 = -20;
    #[allow(dead_code)]
    pub const EACCES: i64 = -13;
    #[allow(dead_code)]
    pub const EEXIST: i64 = -17;
    #[allow(dead_code)]
    pub const EIO: i64 = -5;
}
/// TEAM_210: Linux AArch64 compatible syscall numbers
/// Reference: https://github.com/torvalds/linux/blob/master/include/uapi/asm-generic/unistd.h
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallNumber {
    // === Standard Linux AArch64 syscalls ===
    Getcwd = 17,
    Mkdirat = 34,
    Unlinkat = 35,
    Symlinkat = 36,
    Linkat = 37,
    Renameat = 38,
    Umount = 39,
    Mount = 40,
    Openat = 56,
    Close = 57,
    Getdents = 61,
    Read = 63,
    Write = 64,
    Readlinkat = 78,
    Fstat = 80,
    Utimensat = 88,
    Exit = 93,
    Futex = 98,
    Nanosleep = 101,
    ClockGettime = 113,
    Yield = 124,    // sched_yield
    Shutdown = 142, // reboot
    Kill = 129,
    SigAction = 134,
    SigProcMask = 135,
    SigReturn = 139,
    GetPid = 172,
    GetPpid = 173,  // TEAM_217: Added standard Linux syscall
    Sbrk = 214,    // brk
    Exec = 221,    // execve
    Waitpid = 260, // wait4
    Pause = 236,
    Writev = 66,   // TEAM_217: Added for std println!
    Readv = 65,    // TEAM_217: Added for completeness

    // === Custom LevitateOS syscalls (temporary, until clone/execve work) ===
    /// TEAM_120: Spawn process (custom, will be replaced by clone+execve)
    Spawn = 1000,
    /// TEAM_186: Spawn with args (custom, will be replaced by clone+execve)
    SpawnArgs = 1001,
    /// TEAM_220: Set foreground process (custom)
    SetForeground = 1002,
}

impl SyscallNumber {
    pub fn from_u64(n: u64) -> Option<Self> {
        match n {
            // Linux AArch64 numbers
            17 => Some(Self::Getcwd),
            34 => Some(Self::Mkdirat),
            35 => Some(Self::Unlinkat),
            36 => Some(Self::Symlinkat),
            37 => Some(Self::Linkat),
            38 => Some(Self::Renameat),
            39 => Some(Self::Umount),
            40 => Some(Self::Mount),
            56 => Some(Self::Openat),
            57 => Some(Self::Close),
            61 => Some(Self::Getdents),
            63 => Some(Self::Read),
            64 => Some(Self::Write),
            78 => Some(Self::Readlinkat),
            80 => Some(Self::Fstat),
            88 => Some(Self::Utimensat),
            93 => Some(Self::Exit),
            98 => Some(Self::Futex),
            101 => Some(Self::Nanosleep),
            113 => Some(Self::ClockGettime),
            124 => Some(Self::Yield),
            142 => Some(Self::Shutdown),
            129 => Some(Self::Kill),
            134 => Some(Self::SigAction),
            135 => Some(Self::SigProcMask),
            139 => Some(Self::SigReturn),
            172 => Some(Self::GetPid),
            173 => Some(Self::GetPpid),
            236 => Some(Self::Pause),
            214 => Some(Self::Sbrk),
            221 => Some(Self::Exec),
            260 => Some(Self::Waitpid),
            66 => Some(Self::Writev),
            65 => Some(Self::Readv),
            // Custom LevitateOS
            1000 => Some(Self::Spawn),
            1001 => Some(Self::SpawnArgs),
            1002 => Some(Self::SetForeground),
            _ => None,
        }
    }
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

/// TEAM_217: Linux-compatible Timespec.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

pub fn syscall_dispatch(frame: &mut SyscallFrame) {
    let nr = frame.syscall_number();
    let result = match SyscallNumber::from_u64(nr) {
        Some(SyscallNumber::Read) => fs::sys_read(
            frame.arg0() as usize,
            frame.arg1() as usize,
            frame.arg2() as usize,
        ),
        Some(SyscallNumber::Write) => fs::sys_write(
            frame.arg0() as usize,
            frame.arg1() as usize,
            frame.arg2() as usize,
        ),
        Some(SyscallNumber::Exit) => process::sys_exit(frame.arg0() as i32),
        Some(SyscallNumber::GetPid) => process::sys_getpid(),
        Some(SyscallNumber::Sbrk) => mm::sys_sbrk(frame.arg0() as isize),
        Some(SyscallNumber::Spawn) => {
            process::sys_spawn(frame.arg0() as usize, frame.arg1() as usize)
        }
        Some(SyscallNumber::Exec) => {
            process::sys_exec(frame.arg0() as usize, frame.arg1() as usize)
        }
        Some(SyscallNumber::Yield) => process::sys_yield(),
        Some(SyscallNumber::Shutdown) => sys::sys_shutdown(frame.arg0() as u32),
        Some(SyscallNumber::Openat) => fs::sys_openat(
            frame.arg0() as usize,
            frame.arg1() as usize,
            frame.arg2() as u32,
        ),
        Some(SyscallNumber::Close) => fs::sys_close(frame.arg0() as usize),
        Some(SyscallNumber::Fstat) => fs::sys_fstat(frame.arg0() as usize, frame.arg1() as usize),
        Some(SyscallNumber::Nanosleep) => {
            time::sys_nanosleep(frame.arg0() as u64, frame.arg1() as u64)
        }
        Some(SyscallNumber::ClockGettime) => time::sys_clock_gettime(frame.arg0() as usize),
        // TEAM_176: Directory listing syscall
        Some(SyscallNumber::Getdents) => fs::sys_getdents(
            frame.arg0() as usize,
            frame.arg1() as usize,
            frame.arg2() as usize,
        ),
        // TEAM_186: Spawn process with arguments
        Some(SyscallNumber::SpawnArgs) => process::sys_spawn_args(
            frame.arg0() as usize,
            frame.arg1() as usize,
            frame.arg2() as usize,
            frame.arg3() as usize,
        ),
        // TEAM_188: Wait for child process
        Some(SyscallNumber::Waitpid) => {
            process::sys_waitpid(frame.arg0() as i32, frame.arg1() as usize)
        }
        Some(SyscallNumber::Getcwd) => fs::sys_getcwd(frame.arg0() as usize, frame.arg1() as usize),
        Some(SyscallNumber::Mkdirat) => fs::sys_mkdirat(
            frame.arg0() as i32,
            frame.arg1() as usize,
            frame.arg2() as usize,
            frame.arg3() as u32,
        ),
        Some(SyscallNumber::Unlinkat) => fs::sys_unlinkat(
            frame.arg0() as i32,
            frame.arg1() as usize,
            frame.arg2() as usize,
            frame.arg3() as u32,
        ),
        Some(SyscallNumber::Renameat) => fs::sys_renameat(
            frame.arg0() as i32,
            frame.arg1() as usize,
            frame.arg2() as usize,
            frame.arg3() as i32,
            frame.arg4() as usize,
            frame.arg5() as usize,
        ),
        // TEAM_198: Set file timestamps
        Some(SyscallNumber::Utimensat) => fs::sys_utimensat(
            frame.arg0() as i32,
            frame.arg1() as usize,
            frame.arg2() as usize,
            frame.arg3() as usize,
            frame.arg4() as u32,
        ),
        // TEAM_198: Create symbolic link
        Some(SyscallNumber::Symlinkat) => fs::sys_symlinkat(
            frame.arg0() as usize,
            frame.arg1() as usize,
            frame.arg2() as i32,
            frame.arg3() as usize,
            frame.arg4() as usize,
        ),
        Some(SyscallNumber::Readlinkat) => fs::sys_readlinkat(
            frame.arg0() as i32,
            frame.arg1() as usize,
            frame.arg2() as usize,
            frame.arg3() as usize,
            frame.arg4() as usize,
        ),
        // TEAM_206: Mount/Umount
        Some(SyscallNumber::Mount) => fs::sys_mount(
            frame.arg0() as usize,
            frame.arg1() as usize,
            frame.arg2() as usize,
            frame.arg3() as usize,
            frame.arg4() as usize,
        ),
        Some(SyscallNumber::Umount) => fs::sys_umount(frame.arg0() as usize, frame.arg1() as usize),
        // TEAM_208: Futex syscall
        Some(SyscallNumber::Futex) => {
            let addr = frame.arg0() as usize;
            let op = frame.arg1() as usize;
            let val = frame.arg2() as usize;
            let timeout = frame.arg3() as usize;
            let addr2 = frame.arg4() as usize;
            crate::syscall::sync::sys_futex(addr, op, val, timeout, addr2)
        }
        Some(SyscallNumber::GetPpid) => process::sys_getppid(),
        Some(SyscallNumber::Writev) => fs::sys_writev(
            frame.arg0() as usize,
            frame.arg1() as usize,
            frame.arg2() as usize,
        ),
        Some(SyscallNumber::Readv) => fs::sys_readv(
            frame.arg0() as usize,
            frame.arg1() as usize,
            frame.arg2() as usize,
        ),
        Some(SyscallNumber::Linkat) => fs::sys_linkat(
            frame.arg0() as i32,
            frame.arg1() as usize,
            frame.arg2() as usize,
            frame.arg3() as i32,
            frame.arg4() as usize,
            frame.arg5() as usize,
            frame.arg6() as u32,
        ),
        // TEAM_216: Signal Handling syscalls
        Some(SyscallNumber::Kill) => signal::sys_kill(frame.arg0() as i32, frame.arg1() as i32),
        Some(SyscallNumber::Pause) => signal::sys_pause(),
        Some(SyscallNumber::SigAction) => signal::sys_sigaction(
            frame.arg0() as i32,
            frame.arg1() as usize,
            frame.arg2() as usize,
        ),
        Some(SyscallNumber::SigReturn) => signal::sys_sigreturn(frame),
        Some(SyscallNumber::SigProcMask) => signal::sys_sigprocmask(
            frame.arg0() as i32,
            frame.arg1() as usize,
            frame.arg2() as usize,
        ),
        Some(SyscallNumber::SetForeground) => process::sys_set_foreground(frame.arg0() as usize),
        None => {
            println!("[SYSCALL] Unknown syscall number: {}", nr);
            errno::ENOSYS
        }
    };

    frame.set_return(result);
}

pub const EC_SVC_AARCH64: u64 = 0b010101;

#[inline]
pub fn esr_exception_class(esr: u64) -> u64 {
    (esr >> 26) & 0x3F
}

#[inline]
pub fn is_svc_exception(esr: u64) -> bool {
    esr_exception_class(esr) == EC_SVC_AARCH64
}

pub(crate) fn write_to_user_buf(
    ttbr0: usize,
    user_buf_base: usize,
    offset: usize,
    byte: u8,
) -> bool {
    let user_va = user_buf_base + offset;
    if let Some(kernel_ptr) = mm_user::user_va_to_kernel_ptr(ttbr0, user_va) {
        unsafe {
            *kernel_ptr = byte;
        }
        true
    } else {
        false
    }
}

pub(crate) fn read_from_user(ttbr0: usize, user_va: usize) -> Option<u8> {
    if let Some(kernel_ptr) = mm_user::user_va_to_kernel_ptr(ttbr0, user_va) {
        Some(unsafe { *kernel_ptr })
    } else {
        None
    }
}

/// TEAM_226: Copy a string from user space into a kernel buffer.
///
/// Validates the user buffer and copies bytes through kernel-accessible pointers.
/// This is the safe pattern for reading user memory from syscalls.
///
/// # Arguments
/// * `ttbr0` - User page table physical address
/// * `user_ptr` - User virtual address of string
/// * `len` - Length of string to copy
/// * `buf` - Kernel buffer to copy into
///
/// # Returns
/// * `Ok(&str)` - Valid UTF-8 string slice from buffer
/// * `Err(errno)` - EFAULT if copy fails, EINVAL if not valid UTF-8
pub fn copy_user_string<'a>(
    ttbr0: usize,
    user_ptr: usize,
    len: usize,
    buf: &'a mut [u8],
) -> Result<&'a str, i64> {
    let len = len.min(buf.len());
    if mm_user::validate_user_buffer(ttbr0, user_ptr, len, false).is_err() {
        return Err(errno::EFAULT);
    }
    for i in 0..len {
        if let Some(ptr) = mm_user::user_va_to_kernel_ptr(ttbr0, user_ptr + i) {
            buf[i] = unsafe { *ptr };
        } else {
            return Err(errno::EFAULT);
        }
    }
    core::str::from_utf8(&buf[..len]).map_err(|_| errno::EINVAL)
}
