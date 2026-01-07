//! TEAM_210: Syscall constants

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
pub const SYS_FSTAT: u64 = 80; // fstat
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

// TEAM_228: Threading syscalls
pub const SYS_CLONE: u64 = 220;
pub const SYS_SET_TID_ADDRESS: u64 = 96;

// TEAM_233: Pipe and dup syscalls
pub const SYS_DUP: u64 = 23;
pub const SYS_DUP3: u64 = 24;
pub const SYS_PIPE2: u64 = 59;

// TEAM_244: TTY syscalls (POSIX termios)
pub const SYS_IOCTL: u64 = 29;
pub const SYS_ISATTY: u64 = 1010; // Custom - Linux uses ioctl

// Custom LevitateOS (temporary, until clone/execve work)
pub const SYS_SPAWN: u64 = 1000;
pub const SYS_SPAWN_ARGS: u64 = 1001;
pub const SYS_SET_FOREGROUND: u64 = 1002;
pub const SYS_GET_FOREGROUND: u64 = 1003; // TEAM_244: Get foreground PID

/// TEAM_217: sys_writev constant
pub const SYS_WRITEV: u64 = 66;
/// TEAM_217: sys_readv constant
pub const SYS_READV: u64 = 65;
