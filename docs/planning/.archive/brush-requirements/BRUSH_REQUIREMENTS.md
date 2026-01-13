# Brush Shell Requirements Analysis

## Overview

This document analyzes what the brush shell (https://github.com/reubeno/brush) requires from the operating system and identifies gaps in LevitateOS.

## Critical Finding: Tokio Multi-Thread Runtime

Brush uses **tokio with multi-thread runtime** (`new_multi_thread()`), which:
1. Spawns worker threads via `clone()` ✅ (we have this)
2. Uses async signal handling via self-pipe trick
3. Requires proper epoll integration with signals

## Current Crash Analysis

Brush crash sequence:
```
clone() -> TID 3 created (success)
getuid() -> 0 (success)
CRASH at ud2 (0x6aa71f) - jumped to, not via syscall return
```

The crash happens in brush's async runtime initialization, likely in tokio's signal handling setup.

## What Tokio Signal Handling Needs

Tokio's `tokio::signal::unix::signal()` uses the **self-pipe trick**:

1. **pipe2() or socketpair()** - Create notification channel ✅ (we have socketpair)
2. **rt_sigaction()** - Install signal handlers that write to pipe
3. **epoll** - Wait on the pipe for signal notifications
4. **signalfd()** (optional, more efficient on Linux)

### Critical Missing Pieces

| Syscall | Status | Notes |
|---------|--------|-------|
| `rt_sigaction` | ⚠️ PARTIAL | Our `sys_sigaction` doesn't use Linux sigaction struct format |
| `signalfd` | ❌ MISSING | Tokio can use self-pipe instead |
| `epoll_pwait` | ❌ MISSING | Atomic epoll + signal mask |
| `waitid` | ❌ MISSING | Brush uses this for job control |
| `tcsetpgrp` | ❌ MISSING | Set foreground process group |
| `tcgetpgrp` | ❌ MISSING | Get foreground process group |
| `ioctl(TIOCSPGRP)` | ❌ MISSING | Alternative for tcsetpgrp |
| `ttyname` | ❌ MISSING | Get terminal device path |
| `/etc/passwd` | ❌ MISSING | User info lookup |
| `/proc/self/fd` | ❌ MISSING | FD enumeration |

## Detailed Requirements

### 1. Signal Handling (CRITICAL)

Brush uses these signal features:
```rust
// From brush-core/src/sys/unix/signal.rs
tokio::signal::unix::signal(SignalKind::child())  // SIGCHLD listener
tokio::signal::unix::signal(SignalKind::from_raw(SIGTSTP))  // SIGTSTP listener
sigaction(SIGTTOU, SigHandler::SigIgn)  // Ignore SIGTTOU
waitid(P_ALL, WUNTRACED | WNOHANG)  // Poll for stopped children
```

**What we need:**
- Proper `rt_sigaction` with Linux-compatible sigaction struct
- Signal delivery that can wake epoll
- `waitid` syscall for job control

### 2. Terminal Control (For Interactive Mode)

```rust
// From brush-core/src/sys/unix/terminal.rs
tcgetattr(fd)  // Get terminal attributes (ioctl TCGETS)
tcsetattr(fd, TCSANOW, &termios)  // Set terminal attributes (ioctl TCSETS)
tcgetpgrp(stdin)  // Get foreground process group
tcsetpgrp(stdin, pgid)  // Set foreground process group
ttyname(stdin)  // Get terminal device path
```

**What we need:**
- ioctl `TCGETS`/`TCSETS` for termios
- ioctl `TIOCGPGRP`/`TIOCSPGRP` for process groups
- `ttyname` functionality

### 3. User/Group Information

```rust
// From brush-core/src/sys/unix/users.rs (uses uzers crate)
uzers::get_current_uid()
uzers::get_current_username()
uzers::get_user_by_name(username)  // Reads /etc/passwd
uzers::get_user_groups()  // Reads /etc/group
```

**What we need:**
- `/etc/passwd` file with user info
- `getpwnam_r` or file parsing

### 4. File Descriptor Enumeration

```rust
// From brush-core/src/sys/unix/fd.rs
std::fs::read_dir("/proc/self/fd")  // Enumerate open FDs
```

**What we need:**
- `/proc/self/fd` filesystem

### 5. Process Group Control

```rust
// From brush-core/src/sys/unix/signal.rs
nix::unistd::setpgid(0, 0)  // Lead new process group
```

**What we need:**
- `setpgid` syscall ✅ (we have this)
- `getpgid` syscall ✅ (we have this)

## Recommended Fix Order

### Phase 1: Fix Signal Handling (Unblocks tokio)
1. Implement proper `rt_sigaction` with Linux sigaction struct
2. Make signals wake epoll waiters
3. Implement `waitid` syscall

### Phase 2: Terminal Control (Unblocks interactive mode)
1. Add ioctl `TCGETS`/`TCSETS`
2. Add ioctl `TIOCGPGRP`/`TIOCSPGRP`

### Phase 3: User Info (For $HOME, username, etc.)
1. Create minimal `/etc/passwd`
2. Stub user lookup functions

### Phase 4: Proc Filesystem
1. Implement `/proc/self/fd`

## Root Cause of Current Crash

**CONFIRMED: `rt_sigaction` syscall format mismatch**

### The Problem

Linux `rt_sigaction` (syscall 13 on x86_64) expects:
```c
int rt_sigaction(int signum, 
                 const struct sigaction *act,    // POINTER to struct
                 struct sigaction *oldact,       // POINTER to struct  
                 size_t sigsetsize);

struct sigaction {
    void (*sa_handler)(int);      // Handler function
    unsigned long sa_flags;       // SA_SIGINFO, SA_RESTORER, etc.
    void (*sa_restorer)(void);    // Signal return trampoline
    sigset_t sa_mask;             // 64-bit signal mask
};
```

Our current implementation:
```rust
pub fn sys_sigaction(sig: i32, handler_addr: usize, restorer_addr: usize)
// WRONG! Treats arg1/arg2 as direct addresses, not struct pointers
```

### The Crash Sequence

1. Brush creates tokio multi-thread runtime ✅
2. Tokio spawns worker threads via clone ✅
3. Tokio calls `rt_sigaction` to set up SIGCHLD handler
4. **Our kernel interprets the sigaction struct pointer as handler address**
5. We store garbage (the pointer value) as the signal handler
6. Tokio's signal setup silently fails or returns unexpected values
7. Brush's async runtime hits an assertion/panic
8. Panic handler jumps to ud2 (0x6aa71f)

## Fix Priority

### Phase 1: Fix rt_sigaction (CRITICAL - unblocks brush)

1. Read `struct sigaction` from userspace pointer
2. Handle `sa_flags` including `SA_RESTORER`
3. Store `sa_mask` (needs 64-bit signal mask support)
4. Return old sigaction in `oldact` if provided

### Phase 2: Signal Integration

1. Make signal delivery work with epoll (self-pipe trick)
2. Implement `waitid` for job control

### Phase 3: Terminal Control

1. Add ioctl TCGETS/TCSETS for termios
2. Add ioctl TIOCGPGRP/TIOCSPGRP

### Phase 4: Filesystem

1. `/proc/self/fd` for FD enumeration
2. `/etc/passwd` for user info
