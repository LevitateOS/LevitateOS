// TEAM_210: Syscall constants
// TEAM_296: Added x86_64 support with correct Linux syscall numbers

// Filesystem
#[cfg(target_arch = "aarch64")]
pub const SYS_GETCWD: u64 = 17;
#[cfg(target_arch = "x86_64")]
pub const SYS_GETCWD: u64 = 79;

#[cfg(target_arch = "aarch64")]
pub const SYS_MKDIRAT: u64 = 34;
#[cfg(target_arch = "x86_64")]
pub const SYS_MKDIRAT: u64 = 258;

#[cfg(target_arch = "aarch64")]
pub const SYS_UNLINKAT: u64 = 35;
#[cfg(target_arch = "x86_64")]
pub const SYS_UNLINKAT: u64 = 263;

#[cfg(target_arch = "aarch64")]
pub const SYS_SYMLINKAT: u64 = 36;
#[cfg(target_arch = "x86_64")]
pub const SYS_SYMLINKAT: u64 = 266;

#[cfg(target_arch = "aarch64")]
pub const SYS_LINKAT: u64 = 37;
#[cfg(target_arch = "x86_64")]
pub const SYS_LINKAT: u64 = 265;

#[cfg(target_arch = "aarch64")]
pub const SYS_RENAMEAT: u64 = 38;
#[cfg(target_arch = "x86_64")]
pub const SYS_RENAMEAT: u64 = 264;

#[cfg(target_arch = "aarch64")]
pub const SYS_UMOUNT: u64 = 39;
#[cfg(target_arch = "x86_64")]
pub const SYS_UMOUNT: u64 = 166;

#[cfg(target_arch = "aarch64")]
pub const SYS_MOUNT: u64 = 40;
#[cfg(target_arch = "x86_64")]
pub const SYS_MOUNT: u64 = 165;

#[cfg(target_arch = "aarch64")]
pub const SYS_OPENAT: u64 = 56;
#[cfg(target_arch = "x86_64")]
pub const SYS_OPENAT: u64 = 257;

#[cfg(target_arch = "aarch64")]
pub const SYS_CLOSE: u64 = 57;
#[cfg(target_arch = "x86_64")]
pub const SYS_CLOSE: u64 = 3;

#[cfg(target_arch = "aarch64")]
pub const SYS_GETDENTS: u64 = 61;
#[cfg(target_arch = "x86_64")]
pub const SYS_GETDENTS: u64 = 78;

#[cfg(target_arch = "aarch64")]
pub const SYS_READ: u64 = 63;
#[cfg(target_arch = "x86_64")]
pub const SYS_READ: u64 = 0;

#[cfg(target_arch = "aarch64")]
pub const SYS_WRITE: u64 = 64;
#[cfg(target_arch = "x86_64")]
pub const SYS_WRITE: u64 = 1;

#[cfg(target_arch = "aarch64")]
pub const SYS_READLINKAT: u64 = 78;
#[cfg(target_arch = "x86_64")]
pub const SYS_READLINKAT: u64 = 267;

#[cfg(target_arch = "aarch64")]
pub const SYS_FSTAT: u64 = 80;
#[cfg(target_arch = "x86_64")]
pub const SYS_FSTAT: u64 = 5;

#[cfg(target_arch = "aarch64")]
pub const SYS_UTIMENSAT: u64 = 88;
#[cfg(target_arch = "x86_64")]
pub const SYS_UTIMENSAT: u64 = 280;

// Process
#[cfg(target_arch = "aarch64")]
pub const SYS_EXIT: u64 = 93;
#[cfg(target_arch = "x86_64")]
pub const SYS_EXIT: u64 = 60;

#[cfg(target_arch = "aarch64")]
pub const SYS_GETPID: u64 = 172;
#[cfg(target_arch = "x86_64")]
pub const SYS_GETPID: u64 = 39;

#[cfg(target_arch = "aarch64")]
pub const SYS_GETPPID: u64 = 173;
#[cfg(target_arch = "x86_64")]
pub const SYS_GETPPID: u64 = 110;

#[cfg(target_arch = "aarch64")]
pub const SYS_SBRK: u64 = 214;
#[cfg(target_arch = "x86_64")]
pub const SYS_SBRK: u64 = 12;

#[cfg(target_arch = "aarch64")]
pub const SYS_EXEC: u64 = 221;
#[cfg(target_arch = "x86_64")]
pub const SYS_EXEC: u64 = 59;

#[cfg(target_arch = "aarch64")]
pub const SYS_WAITPID: u64 = 260;
#[cfg(target_arch = "x86_64")]
pub const SYS_WAITPID: u64 = 61;

#[cfg(target_arch = "aarch64")]
pub const SYS_KILL: u64 = 129;
#[cfg(target_arch = "x86_64")]
pub const SYS_KILL: u64 = 62;

#[cfg(target_arch = "aarch64")]
pub const SYS_RT_SIGACTION: u64 = 134;
#[cfg(target_arch = "x86_64")]
pub const SYS_RT_SIGACTION: u64 = 13;

#[cfg(target_arch = "aarch64")]
pub const SYS_RT_SIGPROCMASK: u64 = 135;
#[cfg(target_arch = "x86_64")]
pub const SYS_RT_SIGPROCMASK: u64 = 14;

#[cfg(target_arch = "aarch64")]
pub const SYS_RT_SIGRETURN: u64 = 139;
#[cfg(target_arch = "x86_64")]
pub const SYS_RT_SIGRETURN: u64 = 15;

#[cfg(target_arch = "aarch64")]
pub const SYS_PAUSE: u64 = 236;
#[cfg(target_arch = "x86_64")]
pub const SYS_PAUSE: u64 = 34;

// Synchronization
#[cfg(target_arch = "aarch64")]
pub const SYS_FUTEX: u64 = 98;
#[cfg(target_arch = "x86_64")]
pub const SYS_FUTEX: u64 = 202;

// Time
#[cfg(target_arch = "aarch64")]
pub const SYS_NANOSLEEP: u64 = 101;
#[cfg(target_arch = "x86_64")]
pub const SYS_NANOSLEEP: u64 = 35;

#[cfg(target_arch = "aarch64")]
pub const SYS_CLOCK_GETTIME: u64 = 113;
#[cfg(target_arch = "x86_64")]
pub const SYS_CLOCK_GETTIME: u64 = 228;

#[cfg(target_arch = "aarch64")]
pub const SYS_SCHED_YIELD: u64 = 124;
#[cfg(target_arch = "x86_64")]
pub const SYS_SCHED_YIELD: u64 = 24;

#[cfg(target_arch = "aarch64")]
pub const SYS_SHUTDOWN: u64 = 142;
#[cfg(target_arch = "x86_64")]
pub const SYS_SHUTDOWN: u64 = 169;

// Memory management
#[cfg(target_arch = "aarch64")]
pub const SYS_MMAP: u64 = 222;
#[cfg(target_arch = "x86_64")]
pub const SYS_MMAP: u64 = 9;

#[cfg(target_arch = "aarch64")]
pub const SYS_MUNMAP: u64 = 215;
#[cfg(target_arch = "x86_64")]
pub const SYS_MUNMAP: u64 = 11;

#[cfg(target_arch = "aarch64")]
pub const SYS_MPROTECT: u64 = 226;
#[cfg(target_arch = "x86_64")]
pub const SYS_MPROTECT: u64 = 10;

// Threading
#[cfg(target_arch = "aarch64")]
pub const SYS_CLONE: u64 = 220;
#[cfg(target_arch = "x86_64")]
pub const SYS_CLONE: u64 = 56;

#[cfg(target_arch = "aarch64")]
pub const SYS_SET_TID_ADDRESS: u64 = 96;
#[cfg(target_arch = "x86_64")]
pub const SYS_SET_TID_ADDRESS: u64 = 218;

// Pipe and dup
#[cfg(target_arch = "aarch64")]
pub const SYS_DUP: u64 = 23;
#[cfg(target_arch = "x86_64")]
pub const SYS_DUP: u64 = 32;

#[cfg(target_arch = "aarch64")]
pub const SYS_DUP3: u64 = 24;
#[cfg(target_arch = "x86_64")]
pub const SYS_DUP3: u64 = 292;

#[cfg(target_arch = "aarch64")]
pub const SYS_PIPE2: u64 = 59;
#[cfg(target_arch = "x86_64")]
pub const SYS_PIPE2: u64 = 293;

// TTY
#[cfg(target_arch = "aarch64")]
pub const SYS_IOCTL: u64 = 29;
#[cfg(target_arch = "x86_64")]
pub const SYS_IOCTL: u64 = 16;

pub const SYS_ISATTY: u64 = 1010;

// Custom LevitateOS
pub const SYS_SPAWN: u64 = 1000;
pub const SYS_SPAWN_ARGS: u64 = 1001;
pub const SYS_SET_FOREGROUND: u64 = 1002;
pub const SYS_GET_FOREGROUND: u64 = 1003;

#[cfg(target_arch = "aarch64")]
pub const SYS_WRITEV: u64 = 66;
#[cfg(target_arch = "x86_64")]
pub const SYS_WRITEV: u64 = 20;

#[cfg(target_arch = "aarch64")]
pub const SYS_READV: u64 = 65;
#[cfg(target_arch = "x86_64")]
pub const SYS_READV: u64 = 19;
