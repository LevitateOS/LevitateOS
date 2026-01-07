pub mod boot;
pub mod cpu;
pub mod exceptions;
pub mod power;
pub mod task;
pub mod time;

pub use self::boot::*;
pub use self::task::*;

pub const ELF_MACHINE: u16 = 183; // EM_AARCH64

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
    GetPpid = 173, // TEAM_217: Added standard Linux syscall
    Sbrk = 214,    // brk
    Exec = 221,    // execve
    Waitpid = 260, // wait4
    Pause = 236,
    Writev = 66, // TEAM_217: Added for std println!
    Readv = 65,  // TEAM_217: Added for completeness
    // TEAM_228: Memory management syscalls for std support
    Mmap = 222,
    Munmap = 215,
    Mprotect = 226,
    // TEAM_228: Threading syscalls for std support
    Clone = 220,
    SetTidAddress = 96,
    // TEAM_233: Pipe and dup syscalls for std support
    Dup = 23,
    Dup3 = 24,
    Pipe2 = 59,

    // === Custom LevitateOS syscalls (temporary, until clone/execve work) ===
    /// TEAM_120: Spawn process (custom, will be replaced by clone+execve)
    Spawn = 1000,
    /// TEAM_186: Spawn with args (custom, will be replaced by clone+execve)
    SpawnArgs = 1001,
    /// TEAM_220: Set foreground process (custom)
    SetForeground = 1002,
    /// TEAM_244: Get foreground process (custom)
    GetForeground = 1003,
    /// TEAM_244: Check if fd is a terminal (custom)
    Isatty = 1010,
    /// TEAM_246: Ioctl (Linux standard)
    Ioctl = 29,
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
            // TEAM_228: Memory management
            222 => Some(Self::Mmap),
            215 => Some(Self::Munmap),
            226 => Some(Self::Mprotect),
            // TEAM_228: Threading
            220 => Some(Self::Clone),
            96 => Some(Self::SetTidAddress),
            // TEAM_233: Pipe and dup
            23 => Some(Self::Dup),
            24 => Some(Self::Dup3),
            59 => Some(Self::Pipe2),
            // Custom LevitateOS
            1000 => Some(Self::Spawn),
            1001 => Some(Self::SpawnArgs),
            1002 => Some(Self::SetForeground),
            1003 => Some(Self::GetForeground),
            1010 => Some(Self::Isatty),
            29 => Some(Self::Ioctl),
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

pub const EC_SVC_AARCH64: u64 = 0b010101;

#[inline]
pub fn esr_exception_class(esr: u64) -> u64 {
    (esr >> 26) & 0x3F
}

#[inline]
pub fn is_svc_exception(esr: u64) -> bool {
    esr_exception_class(esr) == EC_SVC_AARCH64
}

/// TEAM_217: Linux-compatible Timespec.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

/// TEAM_162: Saved user context during syscall.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct SyscallFrame {
    pub regs: [u64; 31],
    pub sp: u64,
    pub pc: u64,
    pub pstate: u64,
    pub ttbr0: u64,
}

impl SyscallFrame {
    pub fn syscall_number(&self) -> u64 {
        self.regs[8]
    }
    pub fn arg0(&self) -> u64 {
        self.regs[0]
    }
    pub fn arg1(&self) -> u64 {
        self.regs[1]
    }
    pub fn arg2(&self) -> u64 {
        self.regs[2]
    }
    // TEAM_163: Part of complete syscall ABI (supports up to 6 args per docs)
    #[allow(dead_code)]
    pub fn arg3(&self) -> u64 {
        self.regs[3]
    }
    #[allow(dead_code)]
    pub fn arg4(&self) -> u64 {
        self.regs[4]
    }
    #[allow(dead_code)]
    pub fn arg5(&self) -> u64 {
        self.regs[5]
    }
    #[allow(dead_code)]
    pub fn arg6(&self) -> u64 {
        self.regs[6]
    }
    pub fn set_return(&mut self, value: i64) {
        self.regs[0] = value as u64;
    }
}
// TEAM_163: Removed dead AArch64EarlyConsole (Rule 6: No Dead Code)
