# Shell Prerequisites Checklist

This document lists ALL OS-level features needed for a POSIX shell (like dash, bash, or brush).
Learned the hard way during TEAM_444/TEAM_445.

**Legend:** ✅ = Implemented | ⚠️ = Partial/Stub | ❌ = Not Implemented

---

## Status Summary

| Category | Status | Notes |
|----------|--------|-------|
| Process Management | ✅ Done | fork, execve, waitpid, exit all working |
| File Descriptors | ✅ Done | open, dup2, pipe, fcntl all working |
| File System | ✅ Done | stat, getcwd, chdir, getdents working |
| Memory | ✅ Done | brk, mmap, munmap, mprotect working |
| Signals | ✅ Done | TEAM_447: Signal delivery now implemented |
| Terminal | ✅ Done | termios, TIOCGPGRP/TIOCSPGRP working |
| Environment | ✅ Done | execve stack setup with argv, envp, auxv |

---

## Signal Delivery - IMPLEMENTED ✅

**TEAM_447: `check_and_deliver_signals()` now fully implemented!**

Signals can be:
- ✅ Registered via `sigaction()`
- ✅ Sent via `kill()` (sets pending bit)
- ✅ Masked via `sigprocmask()`
- ✅ **DELIVERED** to userspace handlers

Signal delivery:
- Pushes signal frame to user stack
- Redirects PC to handler address
- Sets up restorer trampoline for sigreturn

---

## 1. Process Management

### 1.1 Process Creation & Execution (CRITICAL)

| Syscall | Status | Notes |
|---------|--------|-------|
| `fork()` / `clone()` | ✅ Done | Full address space copy, CLONE_VM/CLONE_FILES/CLONE_THREAD |
| `execve()` | ✅ Done | argv, envp, auxv stack setup, ELF loading |
| `exit()` / `exit_group()` | ✅ Done | Wakes waiters, closes FDs |
| `waitpid()` / `wait4()` | ✅ Done | Zombie tracking, WNOHANG support |

### 1.2 Process Identity

| Syscall | Status | Notes |
|---------|--------|-------|
| `getpid()` | ✅ Done | |
| `getppid()` | ✅ Done | |
| `getpgid()` / `getpgrp()` | ✅ Done | |
| `setpgid()` | ✅ Done | TEAM_447: Now works for child processes too |
| `setsid()` | ✅ Done | Creates new session |

---

## 2. File Descriptors

### 2.1 Basic I/O (CRITICAL)

| Syscall | Status | Notes |
|---------|--------|-------|
| `open()` / `openat()` | ✅ Done | Both legacy open() and openat() |
| `close()` | ✅ Done | |
| `read()` / `write()` | ✅ Done | |
| `readv()` / `writev()` | ✅ Done | Scatter-gather I/O |
| `lseek()` | ✅ Done | SEEK_SET/CUR/END |
| `pread64()` / `pwrite64()` | ✅ Done | |

### 2.2 File Descriptor Manipulation

| Syscall | Status | Notes |
|---------|--------|-------|
| `dup()` | ✅ Done | |
| `dup2()` / `dup3()` | ✅ Done | |
| `pipe()` / `pipe2()` | ✅ Done | |
| `fcntl()` | ✅ Done | F_DUPFD, F_GETFD/SETFD, F_GETFL/SETFL, F_*PIPE_SZ |

---

## 3. File System

### 3.1 File Information

| Syscall | Status | Notes |
|---------|--------|-------|
| `stat()` / `fstat()` / `lstat()` | ✅ Done | Via fstatat |
| `statx()` | ✅ Done | Extended stat |
| `access()` / `faccessat()` | ⚠️ Stub | Returns success, doesn't check permissions |
| `readlink()` / `readlinkat()` | ✅ Done | |

### 3.2 Directory Operations

| Syscall | Status | Notes |
|---------|--------|-------|
| `getcwd()` | ✅ Done | |
| `chdir()` | ✅ Done | |
| `fchdir()` | ❌ ENOSYS | Requires VFS path tracking (low priority) |
| `getdents()` / `getdents64()` | ✅ Done | |
| `mkdir()` / `mkdirat()` | ✅ Done | |
| `unlinkat()` | ✅ Done | |
| `renameat()` | ✅ Done | |

---

## 4. Memory Management

| Syscall | Status | Notes |
|---------|--------|-------|
| `brk()` / `sbrk()` | ✅ Done | |
| `mmap()` | ✅ Done | |
| `munmap()` | ✅ Done | |
| `mprotect()` | ✅ Done | |
| `mremap()` | ❌ Not implemented | Not critical for shells |
| `madvise()` | ⚠️ Stub | Returns success |

---

## 5. Signals - **NEEDS WORK**

### 5.1 Signal Syscalls

| Syscall | Status | Notes |
|---------|--------|-------|
| `rt_sigaction()` | ✅ Done | Arch-specific struct parsing, stores handlers |
| `rt_sigprocmask()` | ✅ Done | 32-bit mask (TODO: 64-bit) |
| `kill()` | ✅ Done | Sets pending bit, wakes blocked tasks |
| `tkill()` | ✅ Done | Thread-specific |
| `pause()` | ✅ Done | Blocks until signal |
| `rt_sigreturn()` | ✅ Done | Restores signal frame |
| `rt_sigaltstack()` | ⚠️ Stub | Returns success, not functional |

### 5.2 Signal Delivery - ✅ IMPLEMENTED (TEAM_447)

```rust
// levitate/src/main.rs - check_and_deliver_signals()
// - Checks pending signals vs blocked signals
// - Handles SIG_DFL (default action)
// - Handles SIG_IGN (ignore)
// - Custom handlers: pushes frame, redirects PC
```

**Now working:**
- ✅ Push signal frame to user stack
- ✅ Redirect PC to signal handler
- ✅ Actually invoke registered handlers
- ⚠️ SA_SIGINFO, SA_RESTART (partial - basic support)

### 5.3 TTY Signal Generation

| Feature | Status | Notes |
|---------|--------|-------|
| Ctrl+C → SIGINT | ✅ Done | TTY driver calls `signal_foreground_process()` |
| Ctrl+Z → SIGTSTP | ✅ Done | Sets pending bit |
| Ctrl+\\ → SIGQUIT | ✅ Done | Sets pending bit |

**Note:** These work for termination because the kernel checks pending signals and terminates,
but custom handlers registered via `sigaction()` will never be invoked.

---

## 6. Terminal / TTY

### 6.1 Terminal Control

| Operation | Status | Notes |
|-----------|--------|-------|
| `ioctl(TCGETS)` | ✅ Done | Get termios |
| `ioctl(TCSETS/TCSETSW/TCSETSF)` | ✅ Done | Set termios |
| `ioctl(TIOCGPGRP)` | ✅ Done | Get foreground pgrp |
| `ioctl(TIOCSPGRP)` | ✅ Done | Set foreground pgrp |
| `ioctl(TIOCGWINSZ)` | ✅ Done | Returns 80x24 (hardcoded) |
| `ioctl(TIOCSWINSZ)` | ✅ Done | TEAM_447: Set window size (accepted, no-op) |
| `ioctl(TIOCSCTTY)` | ⚠️ Stub | Set controlling terminal |
| `isatty()` | ✅ Done | Via TCGETS success |

### 6.2 Termios / Line Discipline

| Feature | Status | Notes |
|---------|--------|-------|
| INITIAL_TERMIOS | ✅ Done | Properly initialized (TEAM_445 fix) |
| ICANON (canonical mode) | ✅ Done | Line buffering works |
| ECHO | ✅ Done | |
| ICRNL (CR→LF) | ✅ Done | |
| ISIG (signal chars) | ✅ Done | Ctrl+C/Z/\\ generate signals |
| Control chars (VINTR, VEOF, etc.) | ✅ Done | All initialized |

### 6.3 PTY Support

| Feature | Status | Notes |
|---------|--------|-------|
| PTY allocation | ✅ Done | Master/slave pairs |
| `ioctl(TIOCGPTN)` | ✅ Done | Get PTY number |
| `ioctl(TIOCSPTLCK)` | ✅ Done | Lock/unlock PTY |

---

## 7. Environment & Arguments

### 7.1 Process Stack Layout

| Feature | Status | Notes |
|---------|--------|-------|
| argc on stack | ✅ Done | |
| argv pointers + strings | ✅ Done | |
| envp pointers + strings | ✅ Done | |
| Auxiliary vector | ✅ Done | AT_PAGESZ, AT_RANDOM, AT_PHDR, etc. |
| AT_RANDOM (16 bytes) | ✅ Done | Required for Rust std |

---

## 8. User & Permissions

| Syscall | Status | Notes |
|---------|--------|-------|
| `getuid()` / `geteuid()` | ✅ Done | Always returns 0 (root) |
| `getgid()` / `getegid()` | ✅ Done | Always returns 0 |
| `setuid()` / `setgid()` | ❌ Not implemented | Single-user OS |
| `getgroups()` | ❌ Not implemented | |

---

## 9. Time

| Syscall | Status | Notes |
|---------|--------|-------|
| `clock_gettime()` | ✅ Done | |
| `clock_getres()` | ✅ Done | |
| `gettimeofday()` | ✅ Done | |
| `nanosleep()` | ✅ Done | |
| `clock_nanosleep()` | ✅ Done | |

---

## 10. Miscellaneous

| Syscall | Status | Notes |
|---------|--------|-------|
| `uname()` | ✅ Done | |
| `umask()` | ✅ Done | |
| `getrandom()` | ✅ Done | |
| `poll()` / `ppoll()` | ✅ Done | |
| `epoll_create1/ctl/wait` | ✅ Done | |
| `eventfd2()` | ✅ Done | |
| `futex()` | ✅ Done | WAIT/WAKE |
| `prlimit64()` | ⚠️ Stub | Returns sensible defaults |
| `getrlimit()` / `setrlimit()` | ⚠️ Stub | |

---

## Remaining TODOs

### Critical (Blocks Job Control) - DONE ✅

1. ~~**Signal Delivery**~~ - ✅ TEAM_447 implemented

### Important (Nice to Have) - Mostly Done

2. ~~**setpgid for other processes**~~ - ✅ TEAM_447 implemented
3. **faccessat** - Actually check permissions (stub OK for now)
4. **64-bit signal mask** - Currently 32-bit (works for common signals)
5. ~~**TIOCSWINSZ**~~ - ✅ TEAM_447 implemented

### Low Priority

6. **fchdir** - Change directory by fd (TEAM_447: skipped - requires tracking paths in VFS file descriptors)
7. **setuid/setgid** - For multi-user support
8. **getgroups** - Supplementary groups
9. **mremap** - Resize mappings
10. **sigaltstack** - Alternate signal stack

---

## Testing Checklist

### Working Now ✅
- [x] `echo $$` prints PID
- [x] Simple command runs: `ls`
- [x] Command with args: `ls -la /`
- [x] Exit status works: `false; echo $?` prints 1
- [x] Backspace works
- [x] Ctrl+U clears line
- [x] Ctrl+D on empty line exits

### Should Work (Not Fully Tested)
- [ ] Output redirect: `echo hi > /tmp/test`
- [ ] Input redirect: `cat < /etc/passwd`
- [ ] Simple pipe: `echo hi | cat`
- [ ] Shebang scripts: `#!/bin/sh`

### Now Enabled by Signal Delivery (TEAM_447) ✅
- [x] Ctrl+C with custom SIGINT handler
- [x] Ctrl+Z suspends foreground (SIGTSTP delivery)
- [x] `fg` resumes stopped job (SIGCONT delivery)
- [x] `bg` runs stopped job in background
- [x] `jobs` lists jobs (SIGCHLD)
- [x] Proper job control (basic)

---

## References

- POSIX.1-2017 Shell & Utilities: https://pubs.opengroup.org/onlinepubs/9699919799/
- Linux man-pages: https://man7.org/linux/man-pages/
- dash source: https://git.kernel.org/pub/scm/utils/dash/dash.git
- musl libc source: https://git.musl-libc.org/cgit/musl/
