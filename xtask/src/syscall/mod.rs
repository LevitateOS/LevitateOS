//! Syscall Specification Fetcher - Complete Implementation Details
//!
//! Downloads and consolidates everything needed to correctly implement a syscall:
//! - Syscall numbers per architecture
//! - Register calling conventions
//! - Kernel implementation source
//! - Struct layouts with byte offsets
//! - Flags and constants with values
//! - Error codes and when they apply
//! - musl libc reference implementation
//!
//! Usage:
//!   cargo xtask syscall fetch clone     # Complete spec for `clone()`
//!   cargo xtask syscall numbers         # All syscall number tables

use anyhow::{bail, Context, Result};
use clap::Subcommand;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Syscall specification commands
#[derive(Subcommand)]
pub enum SyscallCommands {
    /// Fetch complete syscall specification (numbers, implementation, structs, errors)
    Fetch {
        /// Syscall name (e.g., read, write, mmap, clone, execve)
        name: String,
        /// Force re-download even if spec exists
        #[arg(long)]
        force: bool,
    },
    /// Fetch syscall number tables for all architectures
    Numbers {
        /// Force re-download
        #[arg(long)]
        force: bool,
    },
    /// List all downloaded syscall specs
    List,
    /// Show spec for a syscall (downloads if missing)
    Show {
        /// Syscall name
        name: String,
    },
}

// =============================================================================
// Source URLs
// =============================================================================

const KERNEL_RAW: &str = "https://raw.githubusercontent.com/torvalds/linux/master";
const MUSL_RAW: &str = "https://git.musl-libc.org/cgit/musl/plain";

// =============================================================================
// Syscall Metadata
// =============================================================================

/// Information about a syscall
struct SyscallInfo {
    /// Kernel source files containing the implementation
    kernel_sources: &'static [&'static str],
    /// Related kernel headers for structs/constants
    kernel_headers: &'static [&'static str],
    /// musl libc source files
    musl_sources: &'static [&'static str],
    /// Brief description
    description: &'static str,
    /// Architecture-specific argument order (if different between arches)
    /// Format: (`x86_64_order`, `aarch64_order`) or None if same on all arches
    arch_args: Option<ArchArgs>,
}

/// Architecture-specific argument information
struct ArchArgs {
    /// `x86_64` argument names in order
    x86_64: &'static [&'static str],
    /// aarch64 argument names in order
    aarch64: &'static [&'static str],
    /// Notes about the difference
    notes: &'static str,
}

/// Get metadata for a syscall
fn get_syscall_info(name: &str) -> Option<SyscallInfo> {
    Some(match name {
        // === File I/O ===
        "read" => SyscallInfo {
            kernel_sources: &["fs/read_write.c"],
            kernel_headers: &["include/uapi/asm-generic/errno-base.h"],
            musl_sources: &["src/unistd/read.c"],
            description: "Read from a file descriptor",
            arch_args: None,
        },
        "write" => SyscallInfo {
            kernel_sources: &["fs/read_write.c"],
            kernel_headers: &["include/uapi/asm-generic/errno-base.h"],
            musl_sources: &["src/unistd/write.c"],
            description: "Write to a file descriptor",
            arch_args: None,
        },
        "readv" => SyscallInfo {
            kernel_sources: &["fs/read_write.c"],
            kernel_headers: &["include/uapi/linux/uio.h"],
            musl_sources: &["src/unistd/readv.c"],
            description: "Read into multiple buffers",
            arch_args: None,
        },
        "writev" => SyscallInfo {
            kernel_sources: &["fs/read_write.c"],
            kernel_headers: &["include/uapi/linux/uio.h"],
            musl_sources: &["src/unistd/writev.c"],
            description: "Write from multiple buffers",
            arch_args: None,
        },
        "pread64" => SyscallInfo {
            kernel_sources: &["fs/read_write.c"],
            kernel_headers: &[],
            musl_sources: &["src/unistd/pread.c"],
            description: "Read at offset without changing file position",
            arch_args: None,
        },
        "pwrite64" => SyscallInfo {
            kernel_sources: &["fs/read_write.c"],
            kernel_headers: &[],
            musl_sources: &["src/unistd/pwrite.c"],
            description: "Write at offset without changing file position",
            arch_args: None,
        },
        "open" | "openat" => SyscallInfo {
            kernel_sources: &["fs/open.c"],
            kernel_headers: &["include/uapi/asm-generic/fcntl.h", "include/uapi/linux/fcntl.h"],
            musl_sources: &["src/fcntl/open.c", "src/fcntl/openat.c"],
            description: "Open a file",
            arch_args: None,
        },
        "close" => SyscallInfo {
            kernel_sources: &["fs/open.c"],
            kernel_headers: &[],
            musl_sources: &["src/unistd/close.c"],
            description: "Close a file descriptor",
            arch_args: None,
        },
        "lseek" => SyscallInfo {
            kernel_sources: &["fs/read_write.c"],
            kernel_headers: &["include/uapi/linux/fs.h"],
            musl_sources: &["src/unistd/lseek.c"],
            description: "Reposition file offset",
            arch_args: None,
        },
        "dup" | "dup2" | "dup3" => SyscallInfo {
            kernel_sources: &["fs/file.c"],
            kernel_headers: &[],
            musl_sources: &["src/unistd/dup.c", "src/unistd/dup2.c", "src/unistd/dup3.c"],
            description: "Duplicate file descriptor",
            arch_args: None,
        },
        "fcntl" => SyscallInfo {
            kernel_sources: &["fs/fcntl.c"],
            kernel_headers: &["include/uapi/asm-generic/fcntl.h"],
            musl_sources: &["src/fcntl/fcntl.c"],
            description: "File control operations",
            arch_args: None,
        },
        "ioctl" => SyscallInfo {
            kernel_sources: &["fs/ioctl.c"],
            kernel_headers: &["include/uapi/asm-generic/ioctl.h"],
            musl_sources: &["src/misc/ioctl.c"],
            description: "Device control operations",
            arch_args: None,
        },
        "fstat" | "stat" | "lstat" | "newfstatat" => SyscallInfo {
            kernel_sources: &["fs/stat.c"],
            kernel_headers: &[
                "arch/x86/include/uapi/asm/stat.h",
                "arch/arm64/include/uapi/asm/stat.h",
                "include/uapi/asm-generic/stat.h",
            ],
            musl_sources: &["src/stat/fstat.c", "src/stat/stat.c", "src/stat/lstat.c", "src/stat/fstatat.c"],
            description: "Get file status",
            arch_args: None,
        },
        "statx" => SyscallInfo {
            kernel_sources: &["fs/stat.c"],
            kernel_headers: &["include/uapi/linux/stat.h"],
            musl_sources: &["src/stat/statx.c"],
            description: "Get extended file status",
            arch_args: None,
        },
        "access" | "faccessat" | "faccessat2" => SyscallInfo {
            kernel_sources: &["fs/open.c"],
            kernel_headers: &[],
            musl_sources: &["src/unistd/access.c", "src/unistd/faccessat.c"],
            description: "Check file access permissions",
            arch_args: None,
        },
        "getcwd" => SyscallInfo {
            kernel_sources: &["fs/d_path.c"],
            kernel_headers: &[],
            musl_sources: &["src/unistd/getcwd.c"],
            description: "Get current working directory",
            arch_args: None,
        },
        "chdir" | "fchdir" => SyscallInfo {
            kernel_sources: &["fs/open.c"],
            kernel_headers: &[],
            musl_sources: &["src/unistd/chdir.c", "src/unistd/fchdir.c"],
            description: "Change working directory",
            arch_args: None,
        },
        "mkdir" | "mkdirat" => SyscallInfo {
            kernel_sources: &["fs/namei.c"],
            kernel_headers: &[],
            musl_sources: &["src/stat/mkdir.c", "src/stat/mkdirat.c"],
            description: "Create directory",
            arch_args: None,
        },
        "rmdir" => SyscallInfo {
            kernel_sources: &["fs/namei.c"],
            kernel_headers: &[],
            musl_sources: &["src/unistd/rmdir.c"],
            description: "Remove directory",
            arch_args: None,
        },
        "unlink" | "unlinkat" => SyscallInfo {
            kernel_sources: &["fs/namei.c"],
            kernel_headers: &[],
            musl_sources: &["src/unistd/unlink.c", "src/unistd/unlinkat.c"],
            description: "Remove file",
            arch_args: None,
        },
        "rename" | "renameat" | "renameat2" => SyscallInfo {
            kernel_sources: &["fs/namei.c"],
            kernel_headers: &[],
            musl_sources: &["src/stdio/rename.c", "src/stdio/renameat.c"],
            description: "Rename file",
            arch_args: None,
        },
        "link" | "linkat" => SyscallInfo {
            kernel_sources: &["fs/namei.c"],
            kernel_headers: &[],
            musl_sources: &["src/unistd/link.c", "src/unistd/linkat.c"],
            description: "Create hard link",
            arch_args: None,
        },
        "symlink" | "symlinkat" => SyscallInfo {
            kernel_sources: &["fs/namei.c"],
            kernel_headers: &[],
            musl_sources: &["src/unistd/symlink.c", "src/unistd/symlinkat.c"],
            description: "Create symbolic link",
            arch_args: None,
        },
        "readlink" | "readlinkat" => SyscallInfo {
            kernel_sources: &["fs/stat.c"],
            kernel_headers: &[],
            musl_sources: &["src/unistd/readlink.c", "src/unistd/readlinkat.c"],
            description: "Read symbolic link",
            arch_args: None,
        },
        "truncate" | "ftruncate" => SyscallInfo {
            kernel_sources: &["fs/open.c"],
            kernel_headers: &[],
            musl_sources: &["src/unistd/truncate.c", "src/unistd/ftruncate.c"],
            description: "Truncate file",
            arch_args: None,
        },
        "getdents64" => SyscallInfo {
            kernel_sources: &["fs/readdir.c"],
            kernel_headers: &["include/uapi/linux/dirent.h"],
            musl_sources: &["src/dirent/getdents.c"],
            description: "Read directory entries",
            arch_args: None,
        },
        "pipe" | "pipe2" => SyscallInfo {
            kernel_sources: &["fs/pipe.c"],
            kernel_headers: &[],
            musl_sources: &["src/unistd/pipe.c", "src/unistd/pipe2.c"],
            description: "Create pipe",
            arch_args: None,
        },

        // === Process Management ===
        "clone" => SyscallInfo {
            kernel_sources: &["kernel/fork.c"],
            kernel_headers: &["include/uapi/linux/sched.h"],
            musl_sources: &["src/thread/clone.c", "src/thread/__clone.c"],
            description: "Create child process/thread",
            arch_args: Some(ArchArgs {
                x86_64: &["flags", "stack", "parent_tid", "child_tid", "tls"],
                aarch64: &["flags", "stack", "parent_tid", "tls", "child_tid"],
                notes: "CRITICAL: child_tid and tls are SWAPPED between x86_64 and aarch64! \
                        x86_64 uses default kernel order, aarch64 uses CONFIG_CLONE_BACKWARDS order.",
            }),
        },
        "clone3" => SyscallInfo {
            kernel_sources: &["kernel/fork.c"],
            kernel_headers: &["include/uapi/linux/sched.h"],
            musl_sources: &[],
            description: "Create child process/thread (extended)",
            arch_args: None,
        },
        "fork" => SyscallInfo {
            kernel_sources: &["kernel/fork.c"],
            kernel_headers: &[],
            musl_sources: &["src/process/fork.c"],
            description: "Create child process",
            arch_args: None,
        },
        "vfork" => SyscallInfo {
            kernel_sources: &["kernel/fork.c"],
            kernel_headers: &[],
            musl_sources: &["src/process/vfork.c"],
            description: "Create child process (share memory)",
            arch_args: None,
        },
        "execve" => SyscallInfo {
            kernel_sources: &["fs/exec.c"],
            kernel_headers: &["include/uapi/linux/binfmts.h"],
            musl_sources: &["src/process/execve.c"],
            description: "Execute program",
            arch_args: None,
        },
        "execveat" => SyscallInfo {
            kernel_sources: &["fs/exec.c"],
            kernel_headers: &[],
            musl_sources: &["src/process/execveat.c"],
            description: "Execute program (relative to dirfd)",
            arch_args: None,
        },
        "exit" => SyscallInfo {
            kernel_sources: &["kernel/exit.c"],
            kernel_headers: &[],
            musl_sources: &["src/exit/_Exit.c"],
            description: "Terminate process",
            arch_args: None,
        },
        "exit_group" => SyscallInfo {
            kernel_sources: &["kernel/exit.c"],
            kernel_headers: &[],
            musl_sources: &["src/exit/_Exit.c"],
            description: "Terminate all threads",
            arch_args: None,
        },
        "wait4" => SyscallInfo {
            kernel_sources: &["kernel/exit.c"],
            kernel_headers: &["include/uapi/linux/wait.h", "include/uapi/linux/resource.h"],
            musl_sources: &["src/process/wait4.c"],
            description: "Wait for child process",
            arch_args: None,
        },
        "waitid" => SyscallInfo {
            kernel_sources: &["kernel/exit.c"],
            kernel_headers: &["include/uapi/linux/wait.h"],
            musl_sources: &["src/process/waitid.c"],
            description: "Wait for child process (extended)",
            arch_args: None,
        },
        "getpid" => SyscallInfo {
            kernel_sources: &["kernel/sys.c"],
            kernel_headers: &[],
            musl_sources: &["src/unistd/getpid.c"],
            description: "Get process ID",
            arch_args: None,
        },
        "getppid" => SyscallInfo {
            kernel_sources: &["kernel/sys.c"],
            kernel_headers: &[],
            musl_sources: &["src/unistd/getppid.c"],
            description: "Get parent process ID",
            arch_args: None,
        },
        "gettid" => SyscallInfo {
            kernel_sources: &["kernel/sys.c"],
            kernel_headers: &[],
            musl_sources: &["src/thread/pthread_self.c"],
            description: "Get thread ID",
            arch_args: None,
        },
        "getuid" | "geteuid" | "getgid" | "getegid" => SyscallInfo {
            kernel_sources: &["kernel/sys.c"],
            kernel_headers: &[],
            musl_sources: &["src/unistd/getuid.c", "src/unistd/geteuid.c", "src/unistd/getgid.c", "src/unistd/getegid.c"],
            description: "Get user/group ID",
            arch_args: None,
        },
        "setuid" | "setgid" | "setreuid" | "setregid" => SyscallInfo {
            kernel_sources: &["kernel/sys.c"],
            kernel_headers: &[],
            musl_sources: &["src/unistd/setuid.c", "src/unistd/setgid.c"],
            description: "Set user/group ID",
            arch_args: None,
        },
        "setsid" | "getsid" => SyscallInfo {
            kernel_sources: &["kernel/sys.c"],
            kernel_headers: &[],
            musl_sources: &["src/unistd/setsid.c", "src/unistd/getsid.c"],
            description: "Session ID operations",
            arch_args: None,
        },
        "getpgid" | "setpgid" | "getpgrp" => SyscallInfo {
            kernel_sources: &["kernel/sys.c"],
            kernel_headers: &[],
            musl_sources: &["src/unistd/getpgid.c", "src/unistd/setpgid.c"],
            description: "Process group operations",
            arch_args: None,
        },
        "prctl" => SyscallInfo {
            kernel_sources: &["kernel/sys.c"],
            kernel_headers: &["include/uapi/linux/prctl.h"],
            musl_sources: &["src/misc/prctl.c"],
            description: "Process control operations",
            arch_args: None,
        },
        "arch_prctl" => SyscallInfo {
            kernel_sources: &["arch/x86/kernel/process_64.c"],
            kernel_headers: &["arch/x86/include/uapi/asm/prctl.h"],
            musl_sources: &["arch/x86_64/syscall_arch.h"],
            description: "Architecture-specific process control (x86_64)",
            arch_args: None,
        },

        // === Signals ===
        "rt_sigaction" => SyscallInfo {
            kernel_sources: &["kernel/signal.c"],
            kernel_headers: &[
                "include/uapi/asm-generic/signal.h",
                "arch/x86/include/uapi/asm/signal.h",
                "arch/arm64/include/uapi/asm/signal.h",
                "include/uapi/linux/signal.h",
            ],
            musl_sources: &["src/signal/sigaction.c"],
            description: "Set signal handler",
            arch_args: None,
        },
        "rt_sigprocmask" => SyscallInfo {
            kernel_sources: &["kernel/signal.c"],
            kernel_headers: &["include/uapi/asm-generic/signal.h"],
            musl_sources: &["src/signal/sigprocmask.c"],
            description: "Block/unblock signals",
            arch_args: None,
        },
        "rt_sigreturn" => SyscallInfo {
            kernel_sources: &["kernel/signal.c", "arch/x86/kernel/signal.c", "arch/arm64/kernel/signal.c"],
            kernel_headers: &[],
            musl_sources: &["src/signal/restore.c"],
            description: "Return from signal handler",
            arch_args: None,
        },
        "kill" => SyscallInfo {
            kernel_sources: &["kernel/signal.c"],
            kernel_headers: &["include/uapi/asm-generic/signal.h"],
            musl_sources: &["src/signal/kill.c"],
            description: "Send signal to process",
            arch_args: None,
        },
        "tgkill" => SyscallInfo {
            kernel_sources: &["kernel/signal.c"],
            kernel_headers: &[],
            musl_sources: &["src/signal/raise.c"],
            description: "Send signal to thread",
            arch_args: None,
        },
        "sigaltstack" => SyscallInfo {
            kernel_sources: &["kernel/signal.c"],
            kernel_headers: &["include/uapi/asm-generic/signal.h"],
            musl_sources: &["src/signal/sigaltstack.c"],
            description: "Set alternate signal stack",
            arch_args: None,
        },

        // === Memory Management ===
        "mmap" => SyscallInfo {
            kernel_sources: &["mm/mmap.c", "arch/x86/kernel/sys_x86_64.c"],
            kernel_headers: &["include/uapi/asm-generic/mman-common.h", "include/uapi/linux/mman.h"],
            musl_sources: &["src/mman/mmap.c"],
            description: "Map memory",
            arch_args: None,
        },
        "munmap" => SyscallInfo {
            kernel_sources: &["mm/mmap.c"],
            kernel_headers: &[],
            musl_sources: &["src/mman/munmap.c"],
            description: "Unmap memory",
            arch_args: None,
        },
        "mprotect" => SyscallInfo {
            kernel_sources: &["mm/mprotect.c"],
            kernel_headers: &["include/uapi/asm-generic/mman-common.h"],
            musl_sources: &["src/mman/mprotect.c"],
            description: "Set memory protection",
            arch_args: None,
        },
        "brk" => SyscallInfo {
            kernel_sources: &["mm/mmap.c"],
            kernel_headers: &[],
            musl_sources: &["src/mman/brk.c"],
            description: "Change data segment size",
            arch_args: None,
        },
        "mremap" => SyscallInfo {
            kernel_sources: &["mm/mremap.c"],
            kernel_headers: &["include/uapi/linux/mman.h"],
            musl_sources: &["src/mman/mremap.c"],
            description: "Remap memory",
            arch_args: None,
        },
        "madvise" => SyscallInfo {
            kernel_sources: &["mm/madvise.c"],
            kernel_headers: &["include/uapi/asm-generic/mman-common.h"],
            musl_sources: &["src/mman/madvise.c"],
            description: "Memory advice",
            arch_args: None,
        },
        "mlock" | "munlock" | "mlockall" | "munlockall" => SyscallInfo {
            kernel_sources: &["mm/mlock.c"],
            kernel_headers: &[],
            musl_sources: &["src/mman/mlock.c", "src/mman/munlock.c"],
            description: "Lock/unlock memory",
            arch_args: None,
        },

        // === Time ===
        "clock_gettime" => SyscallInfo {
            kernel_sources: &["kernel/time/posix-timers.c"],
            kernel_headers: &["include/uapi/linux/time.h"],
            musl_sources: &["src/time/clock_gettime.c"],
            description: "Get clock time",
            arch_args: None,
        },
        "clock_settime" => SyscallInfo {
            kernel_sources: &["kernel/time/posix-timers.c"],
            kernel_headers: &["include/uapi/linux/time.h"],
            musl_sources: &["src/time/clock_settime.c"],
            description: "Set clock time",
            arch_args: None,
        },
        "clock_getres" => SyscallInfo {
            kernel_sources: &["kernel/time/posix-timers.c"],
            kernel_headers: &["include/uapi/linux/time.h"],
            musl_sources: &["src/time/clock_getres.c"],
            description: "Get clock resolution",
            arch_args: None,
        },
        "clock_nanosleep" => SyscallInfo {
            kernel_sources: &["kernel/time/posix-timers.c"],
            kernel_headers: &["include/uapi/linux/time.h"],
            musl_sources: &["src/time/clock_nanosleep.c"],
            description: "High-resolution sleep",
            arch_args: None,
        },
        "nanosleep" => SyscallInfo {
            kernel_sources: &["kernel/time/hrtimer.c"],
            kernel_headers: &["include/uapi/linux/time.h"],
            musl_sources: &["src/time/nanosleep.c"],
            description: "Sleep for nanoseconds",
            arch_args: None,
        },
        "gettimeofday" => SyscallInfo {
            kernel_sources: &["kernel/time/time.c"],
            kernel_headers: &["include/uapi/linux/time.h"],
            musl_sources: &["src/time/gettimeofday.c"],
            description: "Get time of day",
            arch_args: None,
        },
        "times" => SyscallInfo {
            kernel_sources: &["kernel/sys.c"],
            kernel_headers: &["include/uapi/linux/times.h"],
            musl_sources: &["src/time/times.c"],
            description: "Get process times",
            arch_args: None,
        },

        // === Polling ===
        "poll" | "ppoll" => SyscallInfo {
            kernel_sources: &["fs/select.c"],
            kernel_headers: &["include/uapi/linux/poll.h"],
            musl_sources: &["src/select/poll.c", "src/select/ppoll.c"],
            description: "Wait for events on file descriptors",
            arch_args: None,
        },
        "select" | "pselect6" => SyscallInfo {
            kernel_sources: &["fs/select.c"],
            kernel_headers: &["include/uapi/linux/time.h"],
            musl_sources: &["src/select/select.c", "src/select/pselect.c"],
            description: "Synchronous I/O multiplexing",
            arch_args: None,
        },
        "epoll_create" | "epoll_create1" => SyscallInfo {
            kernel_sources: &["fs/eventpoll.c"],
            kernel_headers: &["include/uapi/linux/eventpoll.h"],
            musl_sources: &["src/epoll/epoll_create.c", "src/epoll/epoll_create1.c"],
            description: "Create epoll instance",
            arch_args: None,
        },
        "epoll_ctl" => SyscallInfo {
            kernel_sources: &["fs/eventpoll.c"],
            kernel_headers: &["include/uapi/linux/eventpoll.h"],
            musl_sources: &["src/epoll/epoll_ctl.c"],
            description: "Control epoll instance",
            arch_args: None,
        },
        "epoll_wait" | "epoll_pwait" | "epoll_pwait2" => SyscallInfo {
            kernel_sources: &["fs/eventpoll.c"],
            kernel_headers: &["include/uapi/linux/eventpoll.h"],
            musl_sources: &["src/epoll/epoll_wait.c", "src/epoll/epoll_pwait.c"],
            description: "Wait for epoll events",
            arch_args: None,
        },

        // === Futex ===
        "futex" => SyscallInfo {
            kernel_sources: &["kernel/futex/syscalls.c", "kernel/futex/core.c"],
            kernel_headers: &["include/uapi/linux/futex.h"],
            musl_sources: &["src/thread/__futex.c"],
            description: "Fast userspace mutex",
            arch_args: None,
        },

        // === Sockets ===
        "socket" => SyscallInfo {
            kernel_sources: &["net/socket.c"],
            kernel_headers: &["include/uapi/linux/socket.h", "include/uapi/linux/net.h"],
            musl_sources: &["src/network/socket.c"],
            description: "Create socket",
            arch_args: None,
        },
        "bind" => SyscallInfo {
            kernel_sources: &["net/socket.c"],
            kernel_headers: &["include/uapi/linux/socket.h"],
            musl_sources: &["src/network/bind.c"],
            description: "Bind socket to address",
            arch_args: None,
        },
        "listen" => SyscallInfo {
            kernel_sources: &["net/socket.c"],
            kernel_headers: &[],
            musl_sources: &["src/network/listen.c"],
            description: "Listen for connections",
            arch_args: None,
        },
        "accept" | "accept4" => SyscallInfo {
            kernel_sources: &["net/socket.c"],
            kernel_headers: &[],
            musl_sources: &["src/network/accept.c", "src/network/accept4.c"],
            description: "Accept connection",
            arch_args: None,
        },
        "connect" => SyscallInfo {
            kernel_sources: &["net/socket.c"],
            kernel_headers: &[],
            musl_sources: &["src/network/connect.c"],
            description: "Connect socket",
            arch_args: None,
        },
        "sendto" | "sendmsg" => SyscallInfo {
            kernel_sources: &["net/socket.c"],
            kernel_headers: &["include/uapi/linux/socket.h"],
            musl_sources: &["src/network/sendto.c", "src/network/sendmsg.c"],
            description: "Send message",
            arch_args: None,
        },
        "recvfrom" | "recvmsg" => SyscallInfo {
            kernel_sources: &["net/socket.c"],
            kernel_headers: &["include/uapi/linux/socket.h"],
            musl_sources: &["src/network/recvfrom.c", "src/network/recvmsg.c"],
            description: "Receive message",
            arch_args: None,
        },
        "shutdown" => SyscallInfo {
            kernel_sources: &["net/socket.c"],
            kernel_headers: &[],
            musl_sources: &["src/network/shutdown.c"],
            description: "Shutdown socket",
            arch_args: None,
        },
        "getsockopt" | "setsockopt" => SyscallInfo {
            kernel_sources: &["net/socket.c"],
            kernel_headers: &["include/uapi/linux/socket.h"],
            musl_sources: &["src/network/getsockopt.c", "src/network/setsockopt.c"],
            description: "Socket options",
            arch_args: None,
        },
        "getsockname" | "getpeername" => SyscallInfo {
            kernel_sources: &["net/socket.c"],
            kernel_headers: &[],
            musl_sources: &["src/network/getsockname.c", "src/network/getpeername.c"],
            description: "Get socket address",
            arch_args: None,
        },
        "socketpair" => SyscallInfo {
            kernel_sources: &["net/socket.c"],
            kernel_headers: &[],
            musl_sources: &["src/network/socketpair.c"],
            description: "Create socket pair",
            arch_args: None,
        },

        // === Misc ===
        "uname" => SyscallInfo {
            kernel_sources: &["kernel/sys.c"],
            kernel_headers: &["include/uapi/linux/utsname.h"],
            musl_sources: &["src/misc/uname.c"],
            description: "Get system information",
            arch_args: None,
        },
        "sysinfo" => SyscallInfo {
            kernel_sources: &["kernel/sys.c"],
            kernel_headers: &["include/uapi/linux/sysinfo.h"],
            musl_sources: &["src/misc/sysinfo.c"],
            description: "Get system statistics",
            arch_args: None,
        },
        "getrandom" => SyscallInfo {
            kernel_sources: &["drivers/char/random.c"],
            kernel_headers: &["include/uapi/linux/random.h"],
            musl_sources: &["src/misc/getrandom.c"],
            description: "Get random bytes",
            arch_args: None,
        },
        "memfd_create" => SyscallInfo {
            kernel_sources: &["mm/memfd.c"],
            kernel_headers: &["include/uapi/linux/memfd.h"],
            musl_sources: &["src/linux/memfd_create.c"],
            description: "Create anonymous file",
            arch_args: None,
        },
        "set_tid_address" => SyscallInfo {
            kernel_sources: &["kernel/fork.c"],
            kernel_headers: &[],
            musl_sources: &["src/env/__init_tls.c"],
            description: "Set pointer to thread ID",
            arch_args: None,
        },
        "set_robust_list" | "get_robust_list" => SyscallInfo {
            kernel_sources: &["kernel/futex/core.c"],
            kernel_headers: &["include/uapi/linux/futex.h"],
            musl_sources: &["src/thread/pthread_create.c"],
            description: "Robust futex list operations",
            arch_args: None,
        },
        "rseq" => SyscallInfo {
            kernel_sources: &["kernel/rseq.c"],
            kernel_headers: &["include/uapi/linux/rseq.h"],
            musl_sources: &[],
            description: "Restartable sequences",
            arch_args: None,
        },
        "prlimit64" | "getrlimit" | "setrlimit" => SyscallInfo {
            kernel_sources: &["kernel/sys.c"],
            kernel_headers: &["include/uapi/linux/resource.h"],
            musl_sources: &["src/misc/getrlimit.c", "src/misc/setrlimit.c", "src/misc/prlimit.c"],
            description: "Resource limits",
            arch_args: None,
        },
        "umask" => SyscallInfo {
            kernel_sources: &["kernel/sys.c"],
            kernel_headers: &[],
            musl_sources: &["src/stat/umask.c"],
            description: "Set file creation mask",
            arch_args: None,
        },
        "sched_yield" => SyscallInfo {
            kernel_sources: &["kernel/sched/core.c"],
            kernel_headers: &[],
            musl_sources: &["src/sched/sched_yield.c"],
            description: "Yield processor",
            arch_args: None,
        },
        "sched_getaffinity" | "sched_setaffinity" => SyscallInfo {
            kernel_sources: &["kernel/sched/core.c"],
            kernel_headers: &["include/uapi/linux/sched.h"],
            musl_sources: &["src/sched/sched_getcpu.c"],
            description: "CPU affinity",
            arch_args: None,
        },
        "ioprio_get" | "ioprio_set" => SyscallInfo {
            kernel_sources: &["block/ioprio.c"],
            kernel_headers: &["include/uapi/linux/ioprio.h"],
            musl_sources: &[],
            description: "I/O priority",
            arch_args: None,
        },

        _ => return None,
    })
}

// =============================================================================
// Calling Conventions
// =============================================================================

const X86_64_CALLING_CONVENTION: &str = r"
## x86_64 Calling Convention

Syscall instruction: `syscall`

| Register | Purpose           |
|----------|-------------------|
| rax      | Syscall number    |
| rdi      | Argument 1        |
| rsi      | Argument 2        |
| rdx      | Argument 3        |
| r10      | Argument 4        |
| r8       | Argument 5        |
| r9       | Argument 6        |
| rax      | Return value      |

Return: rax contains result or negative errno on error.
Clobbered: rcx, r11 (used by syscall instruction)
";

const AARCH64_CALLING_CONVENTION: &str = r"
## AArch64 Calling Convention

Syscall instruction: `svc #0`

| Register | Purpose           |
|----------|-------------------|
| x8       | Syscall number    |
| x0       | Argument 1        |
| x1       | Argument 2        |
| x2       | Argument 3        |
| x3       | Argument 4        |
| x4       | Argument 5        |
| x5       | Argument 6        |
| x0       | Return value      |

Return: x0 contains result or negative errno on error.
";

// =============================================================================
// Errno Values
// =============================================================================

const ERRNO_TABLE: &str = r"
## Error Codes (errno)

Errors are returned as negative values in the return register.
For example, -EINVAL means the syscall returned -22.

| Name          | Value | Description                                |
|---------------|-------|--------------------------------------------|
| EPERM         | 1     | Operation not permitted                    |
| ENOENT        | 2     | No such file or directory                  |
| ESRCH         | 3     | No such process                            |
| EINTR         | 4     | Interrupted system call                    |
| EIO           | 5     | I/O error                                  |
| ENXIO         | 6     | No such device or address                  |
| E2BIG         | 7     | Argument list too long                     |
| ENOEXEC       | 8     | Exec format error                          |
| EBADF         | 9     | Bad file descriptor                        |
| ECHILD        | 10    | No child processes                         |
| EAGAIN        | 11    | Try again / Would block                    |
| ENOMEM        | 12    | Out of memory                              |
| EACCES        | 13    | Permission denied                          |
| EFAULT        | 14    | Bad address                                |
| ENOTBLK       | 15    | Block device required                      |
| EBUSY         | 16    | Device or resource busy                    |
| EEXIST        | 17    | File exists                                |
| EXDEV         | 18    | Cross-device link                          |
| ENODEV        | 19    | No such device                             |
| ENOTDIR       | 20    | Not a directory                            |
| EISDIR        | 21    | Is a directory                             |
| EINVAL        | 22    | Invalid argument                           |
| ENFILE        | 23    | File table overflow                        |
| EMFILE        | 24    | Too many open files                        |
| ENOTTY        | 25    | Not a typewriter                           |
| ETXTBSY       | 26    | Text file busy                             |
| EFBIG         | 27    | File too large                             |
| ENOSPC        | 28    | No space left on device                    |
| ESPIPE        | 29    | Illegal seek                               |
| EROFS         | 30    | Read-only file system                      |
| EMLINK        | 31    | Too many links                             |
| EPIPE         | 32    | Broken pipe                                |
| EDOM          | 33    | Math argument out of domain                |
| ERANGE        | 34    | Math result not representable              |
| EDEADLK       | 35    | Resource deadlock would occur              |
| ENAMETOOLONG  | 36    | File name too long                         |
| ENOLCK        | 37    | No record locks available                  |
| ENOSYS        | 38    | Function not implemented                   |
| ENOTEMPTY     | 39    | Directory not empty                        |
| ELOOP         | 40    | Too many symbolic links                    |
| EWOULDBLOCK   | 11    | Same as EAGAIN                             |
";

// =============================================================================
// Directory and URL helpers
// =============================================================================

fn specs_dir() -> PathBuf {
    PathBuf::from("docs/specs/syscalls")
}

/// Convert `snake_case` to `PascalCase`
fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect()
}

fn fetch_url(url: &str) -> Result<String> {
    let output = Command::new("curl")
        .args(["-fsSL", "--connect-timeout", "10", url])
        .output()
        .context("Failed to run curl - is it installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Failed to fetch {url}: {stderr}");
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

// =============================================================================
// Syscall Number Lookup
// =============================================================================

fn load_syscall_numbers() -> Result<(HashMap<String, u32>, HashMap<String, u32>)> {
    let specs_dir = specs_dir();
    let mut x86_64_numbers = HashMap::new();
    let mut aarch64_numbers = HashMap::new();

    // Parse x86_64 table
    let x86_path = specs_dir.join("numbers_x86_64.txt");
    if x86_path.exists() {
        let content = fs::read_to_string(&x86_path)?;
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                if let Ok(num) = parts[0].parse::<u32>() {
                    x86_64_numbers.insert(parts[2].to_string(), num);
                }
            }
        }
    }

    // Parse aarch64 table (from unistd.h - different format)
    let aarch64_path = specs_dir.join("numbers_aarch64.txt");
    if aarch64_path.exists() {
        let content = fs::read_to_string(&aarch64_path)?;
        for line in content.lines() {
            // Format: #define __NR_xxx yyy or #define __NR3264_xxx yyy
            if line.contains("#define __NR") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let name = parts[1]
                        .trim_start_matches("__NR_")
                        .trim_start_matches("__NR3264_");
                    if let Ok(num) = parts[2].parse::<u32>() {
                        aarch64_numbers.insert(name.to_string(), num);
                    }
                }
            }
        }
    }

    Ok((x86_64_numbers, aarch64_numbers))
}

// =============================================================================
// Content Extraction
// =============================================================================

fn extract_syscall_impl(source: &str, syscall_name: &str) -> String {
    let mut result = String::new();
    let mut in_function = false;
    let mut brace_depth = 0;
    let mut found_any = false;

    // Patterns that indicate a syscall definition
    let patterns = [
        "SYSCALL_DEFINE".to_string(),
        format!("__do_{syscall_name}"),
        format!("do_{syscall_name}"),
        format!("ksys_{syscall_name}"),
        format!("__x64_sys_{syscall_name}"),
        format!("__arm64_sys_{syscall_name}"),
        format!("__se_sys_{syscall_name}"),
    ];

    for line in source.lines() {
        let starts_function = patterns.iter().any(|p| line.contains(p.as_str()))
            && (line.to_lowercase().contains(syscall_name)
                || line.contains(&syscall_name.to_uppercase()));

        if starts_function && !in_function {
            in_function = true;
            brace_depth = 0;
            found_any = true;
            if !result.is_empty() {
                result.push_str("\n// ---\n\n");
            }
        }

        if in_function {
            result.push_str(line);
            result.push('\n');

            for ch in line.chars() {
                match ch {
                    '{' => brace_depth += 1,
                    '}' => {
                        brace_depth -= 1;
                        if brace_depth == 0 {
                            in_function = false;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    if !found_any {
        result.push_str("// Implementation not found with pattern matching.\n");
        result.push_str("// Check the full source file for the implementation.\n");
    }

    result
}

// =============================================================================
// Main Fetch Function
// =============================================================================

pub fn fetch(name: &str, force: bool) -> Result<PathBuf> {
    let specs_dir = specs_dir();
    fs::create_dir_all(&specs_dir)?;

    let spec_path = specs_dir.join(format!("{name}.md"));

    if spec_path.exists() && !force {
        println!("Spec already exists: {}", spec_path.display());
        println!("Use --force to re-download");
        return Ok(spec_path);
    }

    let info = get_syscall_info(name);
    if info.is_none() {
        println!("Warning: Unknown syscall '{name}', will attempt to fetch anyway");
    }
    let info = info.unwrap_or(SyscallInfo {
        kernel_sources: &["kernel/sys.c"],
        kernel_headers: &[],
        musl_sources: &[],
        description: "Unknown syscall",
        arch_args: None,
    });

    println!("Fetching complete spec for '{name}'...");

    let mut content = String::new();

    // =================================
    // Header
    // =================================
    content.push_str(&format!(
        "# {} - Complete Syscall Specification\n\n",
        name.to_uppercase()
    ));
    content.push_str(&format!("**Description**: {}\n\n", info.description));
    content.push_str("---\n\n");

    // =================================
    // Syscall Numbers
    // =================================
    content.push_str("## Syscall Numbers\n\n");

    // Try to load numbers from cached tables
    let numbers_exist = specs_dir.join("numbers_x86_64.txt").exists();
    if numbers_exist {
        match load_syscall_numbers() {
            Ok((x86_64, aarch64)) => {
                if let Some(num) = x86_64.get(name) {
                    content.push_str(&format!("- **x86_64**: {num} (0x{num:x})\n"));
                } else {
                    content.push_str("- **x86_64**: Not found in table\n");
                }
                if let Some(num) = aarch64.get(name) {
                    content.push_str(&format!("- **aarch64**: {num} (0x{num:x})\n"));
                } else {
                    content.push_str("- **aarch64**: Not found in table\n");
                }
            }
            Err(_) => {
                content
                    .push_str("Run `cargo xtask syscall numbers` first to fetch syscall tables.\n");
            }
        }
    } else {
        content.push_str("Run `cargo xtask syscall numbers` first to fetch syscall tables.\n");
    }
    content.push('\n');

    // =================================
    // Calling Conventions
    // =================================
    content.push_str(X86_64_CALLING_CONVENTION);
    content.push('\n');
    content.push_str(AARCH64_CALLING_CONVENTION);
    content.push('\n');

    // =================================
    // Architecture-Specific Arguments (if different)
    // =================================
    if let Some(ref arch_args) = info.arch_args {
        content.push_str("## ⚠️ ARCHITECTURE-SPECIFIC ARGUMENT ORDER\n\n");
        content.push_str(
            "**This syscall has DIFFERENT argument orders on different architectures!**\n\n",
        );

        content.push_str("### x86_64 Argument Order\n\n");
        content.push_str("| Arg # | Register | Parameter |\n");
        content.push_str("|-------|----------|----------|\n");
        let x86_regs = ["rdi", "rsi", "rdx", "r10", "r8", "r9"];
        for (i, arg) in arch_args.x86_64.iter().enumerate() {
            if i < x86_regs.len() {
                content.push_str(&format!("| {} | {} | {} |\n", i + 1, x86_regs[i], arg));
            }
        }
        content.push('\n');

        content.push_str("### aarch64 Argument Order\n\n");
        content.push_str("| Arg # | Register | Parameter |\n");
        content.push_str("|-------|----------|----------|\n");
        for (i, arg) in arch_args.aarch64.iter().enumerate() {
            content.push_str(&format!("| {} | x{} | {} |\n", i + 1, i, arg));
        }
        content.push('\n');

        content.push_str(&format!("**Notes**: {}\n\n", arch_args.notes));

        // Add implementation guidance
        content.push_str("### LevitateOS Implementation Pattern\n\n");
        content.push_str("```rust\n");
        content.push_str("// In syscall dispatcher (lib.rs):\n");
        content.push_str("#[cfg(target_arch = \"x86_64\")]\n");
        content.push_str(&format!(
            "Some(SyscallNumber::{}) => sys_{}(\n",
            to_pascal_case(name),
            name
        ));
        for (i, arg) in arch_args.x86_64.iter().enumerate() {
            content.push_str(&format!("    frame.arg{i}() as _, // {arg}\n"));
        }
        content.push_str("),\n");
        content.push_str("#[cfg(target_arch = \"aarch64\")]\n");
        content.push_str(&format!(
            "Some(SyscallNumber::{}) => sys_{}(\n",
            to_pascal_case(name),
            name
        ));
        for (i, arg) in arch_args.aarch64.iter().enumerate() {
            content.push_str(&format!("    frame.arg{i}() as _, // {arg}\n"));
        }
        content.push_str("),\n");
        content.push_str("```\n\n");
        content.push_str("---\n\n");
    }

    // =================================
    // Error Codes
    // =================================
    content.push_str(ERRNO_TABLE);
    content.push_str("\n---\n\n");

    // =================================
    // Kernel Headers (struct definitions)
    // =================================
    if !info.kernel_headers.is_empty() {
        content.push_str("## Kernel Headers (Struct Definitions)\n\n");

        for header_path in info.kernel_headers {
            let url = format!("{KERNEL_RAW}/{header_path}");
            print!("  Fetching {header_path}... ");

            match fetch_url(&url) {
                Ok(header_content) => {
                    println!("ok");
                    content.push_str(&format!("### {header_path}\n\n"));
                    content.push_str(&format!("Source: {url}\n\n"));
                    content.push_str("```c\n");
                    // Truncate very large headers
                    let lines: Vec<&str> = header_content.lines().collect();
                    if lines.len() > 300 {
                        for line in lines.iter().take(300) {
                            content.push_str(line);
                            content.push('\n');
                        }
                        content.push_str(&format!(
                            "\n// ... truncated ({} more lines)\n",
                            lines.len() - 300
                        ));
                    } else {
                        content.push_str(&header_content);
                    }
                    content.push_str("```\n\n");
                }
                Err(e) => {
                    println!("failed: {e}");
                    content.push_str(&format!("### {header_path} (FAILED)\n\n"));
                }
            }
        }
    }

    // =================================
    // Kernel Implementation
    // =================================
    content.push_str("## Kernel Implementation\n\n");

    for source_path in info.kernel_sources {
        let url = format!("{KERNEL_RAW}/{source_path}");
        print!("  Fetching {source_path}... ");

        match fetch_url(&url) {
            Ok(source) => {
                println!("ok");
                content.push_str(&format!("### {source_path}\n\n"));
                content.push_str(&format!("Source: {url}\n\n"));
                content.push_str("```c\n");
                content.push_str(&extract_syscall_impl(&source, name));
                content.push_str("```\n\n");
            }
            Err(e) => {
                println!("failed: {e}");
                content.push_str(&format!("### {source_path} (FAILED: {e})\n\n"));
            }
        }
    }

    // =================================
    // musl libc Reference
    // =================================
    if !info.musl_sources.is_empty() {
        content.push_str("## musl libc Reference Implementation\n\n");
        content.push_str("musl provides clean, readable implementations that show the userspace perspective.\n\n");

        for source_path in info.musl_sources {
            let url = format!("{MUSL_RAW}/{source_path}");
            print!("  Fetching musl {source_path}... ");

            match fetch_url(&url) {
                Ok(source) => {
                    println!("ok");
                    content.push_str(&format!("### {source_path}\n\n"));
                    content.push_str(&format!("Source: {url}\n\n"));
                    content.push_str("```c\n");
                    content.push_str(&source);
                    content.push_str("```\n\n");
                }
                Err(_) => {
                    println!("not found");
                    // Don't include failed musl fetches, they may not exist
                }
            }
        }
    }

    // =================================
    // Write output
    // =================================
    fs::write(&spec_path, &content)?;

    println!("\nSaved: {}", spec_path.display());
    println!("Lines: {}", content.lines().count());

    Ok(spec_path)
}

// =============================================================================
// Fetch Numbers
// =============================================================================

pub fn fetch_numbers(force: bool) -> Result<()> {
    let specs_dir = specs_dir();
    fs::create_dir_all(&specs_dir)?;

    println!("Fetching syscall number tables...\n");

    // x86_64 syscall table
    let x86_path = specs_dir.join("numbers_x86_64.txt");
    if !x86_path.exists() || force {
        let url = format!("{KERNEL_RAW}/arch/x86/entry/syscalls/syscall_64.tbl");
        print!("x86_64: fetching... ");
        match fetch_url(&url) {
            Ok(content) => {
                let mut output = format!("# Syscall Numbers - x86_64\n# Source: {url}\n\n");
                output.push_str(&content);
                fs::write(&x86_path, &output)?;
                println!("ok");
            }
            Err(e) => println!("failed: {e}"),
        }
    } else {
        println!("x86_64: exists (use --force to re-download)");
    }

    // aarch64 syscall table
    let aarch64_path = specs_dir.join("numbers_aarch64.txt");
    if !aarch64_path.exists() || force {
        let url = format!("{KERNEL_RAW}/include/uapi/asm-generic/unistd.h");
        print!("aarch64: fetching... ");
        match fetch_url(&url) {
            Ok(content) => {
                let mut output = format!("# Syscall Numbers - aarch64\n# Source: {url}\n\n");
                output.push_str(&content);
                fs::write(&aarch64_path, &output)?;
                println!("ok");
            }
            Err(e) => println!("failed: {e}"),
        }
    } else {
        println!("aarch64: exists (use --force to re-download)");
    }

    // Errno values
    let errno_path = specs_dir.join("errno.txt");
    if !errno_path.exists() || force {
        print!("errno: fetching... ");
        let url1 = format!("{KERNEL_RAW}/include/uapi/asm-generic/errno-base.h");
        let url2 = format!("{KERNEL_RAW}/include/uapi/asm-generic/errno.h");

        let mut errno_content = String::from("# Linux Error Numbers\n\n");

        if let Ok(content) = fetch_url(&url1) {
            errno_content.push_str("## errno-base.h\n\n```c\n");
            errno_content.push_str(&content);
            errno_content.push_str("```\n\n");
        }
        if let Ok(content) = fetch_url(&url2) {
            errno_content.push_str("## errno.h\n\n```c\n");
            errno_content.push_str(&content);
            errno_content.push_str("```\n\n");
        }

        fs::write(&errno_path, &errno_content)?;
        println!("ok");
    } else {
        println!("errno: exists");
    }

    println!("\nDone! Files saved to {}", specs_dir.display());
    Ok(())
}

// =============================================================================
// List and Show
// =============================================================================

pub fn list() -> Result<()> {
    let specs_dir = specs_dir();

    if !specs_dir.exists() {
        println!("No syscall specs downloaded yet.");
        println!("Use: cargo xtask syscall numbers");
        println!("     cargo xtask syscall fetch <name>");
        return Ok(());
    }

    let mut entries: Vec<_> = fs::read_dir(&specs_dir)?
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().is_file())
        .collect();

    entries.sort_by_key(std::fs::DirEntry::file_name);

    let mut tables = Vec::new();
    let mut specs = Vec::new();

    for entry in entries {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("numbers_") || name == "errno.txt" {
            tables.push(name);
        } else if name.ends_with(".md") {
            specs.push(name.trim_end_matches(".md").to_string());
        }
    }

    if !tables.is_empty() {
        println!("Reference tables:");
        for t in &tables {
            println!("  {t}");
        }
        println!();
    }

    if !specs.is_empty() {
        println!("Syscall specs ({}):", specs.len());
        for s in &specs {
            println!("  {s}");
        }
        println!();
    }

    if tables.is_empty() && specs.is_empty() {
        println!("No specs downloaded yet.");
        println!("Use: cargo xtask syscall numbers");
        println!("     cargo xtask syscall fetch <name>");
    }

    println!("Location: {}", specs_dir.display());
    Ok(())
}

pub fn show(name: &str) -> Result<()> {
    let spec_path = specs_dir().join(format!("{name}.md"));

    if !spec_path.exists() {
        fetch(name, false)?;
    }

    let content = fs::read_to_string(&spec_path)?;
    println!("{content}");
    Ok(())
}
