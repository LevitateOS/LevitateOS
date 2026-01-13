# Feature Plan: General Purpose Unix-Compatible OS

**TEAM_397**: General Purpose OS Feature Plan  
**Created**: 2026-01-10  
**Status**: Planning

---

## Mission

**Run any Unix program without modification.**

A user should be able to download a Linux binary and run it on LevitateOS without recompiling or modifying the source.

---

## The Test

> Can a user download a Linux ELF binary and run it?

If YES → We are General Purpose  
If NO → We are not there yet

---

## Current State

```
User wants to run a program
        ↓
Must modify source code (add Eyra dependency)
        ↓
Rebuild with special flags
        ↓
Run on LevitateOS

❌ NOT General Purpose
```

## Target State

```
User downloads Linux binary (or compiles with gcc/rustc for Linux)
        ↓
Copy to LevitateOS filesystem
        ↓
./program

✅ General Purpose
```

---

## Architecture Layers

```
┌─────────────────────────────────────────────────────────────┐
│                    User Applications                         │
│         (unmodified Linux binaries, Rust std programs)       │
└─────────────────────────────────┬───────────────────────────┘
                                  │
┌─────────────────────────────────▼───────────────────────────┐
│                    Dynamic Linker                            │
│                    (ld-linux.so.2)                           │
│         Loads shared libraries, resolves symbols             │
└─────────────────────────────────┬───────────────────────────┘
                                  │
┌─────────────────────────────────▼───────────────────────────┐
│                    libc.so.6                                 │
│              (c-gull / c-ward in Rust)                       │
│    Provides: printf, malloc, fopen, pthread, etc.            │
└─────────────────────────────────┬───────────────────────────┘
                                  │
┌─────────────────────────────────▼───────────────────────────┐
│                    Linux Syscall ABI                         │
│         (syscall instruction with Linux numbers)             │
└─────────────────────────────────┬───────────────────────────┘
                                  │
┌─────────────────────────────────▼───────────────────────────┐
│                    LevitateOS Kernel                         │
│              (implements Linux syscall handlers)             │
└─────────────────────────────────────────────────────────────┘
```

---

## Feature Breakdown

### Phase A: Complete Syscall ABI (Foundation)

**Goal**: Implement all syscalls needed by libc and common programs.

| Category | Syscalls | Status | Priority |
|----------|----------|--------|----------|
| **Process** | fork, vfork, clone3, execve, wait4, exit_group | 🟡 Partial | P0 |
| **Memory** | mmap, munmap, mprotect, mremap, brk | ✅ Done | - |
| **Files** | open, read, write, close, lseek, fstat, ftruncate | ✅ Done | - |
| **Directories** | openat, mkdirat, unlinkat, renameat, getdents64 | ✅ Done | - |
| **Links** | linkat, symlinkat, readlinkat | ✅ Done | - |
| **Permissions** | chmod, chown, access, umask | 🔴 Missing | P1 |
| **Users** | getuid, geteuid, getgid, getegid, setuid, setgid | 🔴 Missing | P1 |
| **Signals** | rt_sigaction, rt_sigprocmask, kill, sigreturn | ✅ Done | - |
| **Time** | clock_gettime, nanosleep, gettimeofday | ✅ Done | - |
| **Threads** | clone, futex, set_tid_address, set_robust_list | ✅ Done | - |
| **I/O Multiplex** | poll, select, epoll_create, epoll_ctl, epoll_wait | 🟡 Partial | P0 |
| **Sockets** | socket, bind, listen, accept, connect, send, recv | 🔴 Missing | P2 |
| **IPC** | shmget, shmat, semget, msgget | 🔴 Missing | P3 |
| **Misc** | ioctl, fcntl, getcwd, chdir, uname | 🟡 Partial | P1 |

#### Syscall Implementation Tasks

- [ ] **fork/vfork** - Copy-on-Write process creation
- [ ] **execve** - Replace current process with new program
- [ ] **poll/select** - I/O multiplexing (needed by many programs)
- [ ] **epoll** - Scalable I/O multiplexing (needed by async runtimes)
- [ ] **getuid/setuid family** - User identity syscalls
- [ ] **chmod/chown/access** - Permission syscalls
- [ ] **umask** - File creation mask
- [ ] **chdir/fchdir** - Change working directory
- [ ] **uname** - System information
- [ ] **fcntl** - File descriptor control (F_DUPFD, F_GETFL, F_SETFL, etc.)

---

### Phase B: libc Implementation (Critical Milestone)

**Goal**: Provide libc.so.6 that programs can link against.

#### Option 1: c-gull as libc (Recommended)

[c-ward/c-gull](https://github.com/sunfishcode/c-ward) is a Rust implementation of libc.

```
Advantages:
- Pure Rust (memory safe)
- Already designed for this use case
- Maintained by sunfishcode (same as Eyra)
- "take-charge" mode handles program startup

Challenges:
- Experimental
- May need patches for LevitateOS-specific behavior
- Dynamic linking support unclear
```

#### Option 2: musl-libc

[musl](https://musl.libc.org/) is a lightweight libc written in C.

```
Advantages:
- Production-ready
- Well-tested
- Static linking works great

Challenges:
- Written in C (not Rust)
- Would need to cross-compile for LevitateOS
```

#### libc Implementation Tasks

- [ ] **Evaluate c-gull feasibility** - Can it be built as libc.so?
- [ ] **Build c-gull static library** - libc.a for static linking first
- [ ] **Test static linking** - Compile hello.c with our libc.a
- [ ] **Build c-gull shared library** - libc.so.6
- [ ] **Implement missing functions** - Whatever c-gull lacks
- [ ] **Create sysroot** - /lib/libc.so.6, /usr/include/*, etc.

---

### Phase C: Dynamic Linker (Full Compatibility)

**Goal**: Load and run dynamically-linked ELF binaries.

#### What is ld-linux.so?

The dynamic linker is the first thing that runs when you execute a dynamically-linked program. It:

1. Loads the program into memory
2. Finds and loads all shared libraries (.so files)
3. Resolves symbol references between them
4. Calls the program's entry point

#### Dynamic Linker Tasks

- [ ] **Study existing implementations** - musl, glibc, Redox
- [ ] **Implement ELF interpreter** - Parse PT_INTERP, load libraries
- [ ] **Symbol resolution** - GOT/PLT handling
- [ ] **Lazy binding** - RTLD_LAZY support
- [ ] **LD_PRELOAD support** - Library injection
- [ ] **LD_LIBRARY_PATH** - Library search paths

#### Alternative: Static-Only First

We could defer dynamic linking and focus on statically-linked binaries first:

```bash
# User compiles with:
gcc -static -o program program.c

# Or Rust:
RUSTFLAGS='-C target-feature=+crt-static' cargo build
```

This gives us "general purpose for static binaries" faster.

---

### Phase D: POSIX Filesystem Layout

**Goal**: Standard Unix filesystem hierarchy.

| Path | Purpose | Status |
|------|---------|--------|
| `/bin` | Essential binaries | 🟡 Have initramfs |
| `/lib` | Shared libraries (libc.so.6) | 🔴 Missing |
| `/usr/bin` | User binaries | 🔴 Missing |
| `/usr/lib` | User libraries | 🔴 Missing |
| `/usr/include` | Header files | 🔴 Missing |
| `/etc` | Configuration files | 🔴 Missing |
| `/tmp` | Temporary files | ✅ tmpfs |
| `/dev` | Device files | 🟡 Partial |
| `/proc` | Process information | 🔴 Missing |
| `/sys` | System information | 🔴 Missing |

#### Filesystem Tasks

- [ ] **Implement /proc filesystem** - /proc/self/*, /proc/[pid]/*
- [ ] **Implement /dev properly** - /dev/null, /dev/zero, /dev/random, /dev/tty
- [ ] **Create FHS layout** - Standard directory structure
- [ ] **Implement /etc/passwd** - User database (even if single user)
- [ ] **Environment variables** - PATH, HOME, USER, etc.

---

### Phase E: Terminal/TTY Completion

**Goal**: Full POSIX terminal support for interactive programs.

| Feature | Status | Notes |
|---------|--------|-------|
| Basic read/write | ✅ Done | |
| isatty() | ✅ Done | |
| termios (TCGETS/TCSETS) | 🔴 Missing | Needed for raw mode |
| Canonical mode (line editing) | 🔴 Missing | |
| Echo, ICANON flags | 🔴 Missing | |
| Signal characters (Ctrl+C, Ctrl+Z) | ✅ Done | |
| Job control (fg, bg) | 🔴 Missing | |
| Pseudo-terminals (PTY) | 🔴 Missing | Needed for ssh, screen, tmux |

#### TTY Tasks

- [ ] **Full termios implementation** - All flags and special characters
- [ ] **Canonical mode** - Line editing with VERASE, VKILL
- [ ] **PTY support** - /dev/ptmx, /dev/pts/*
- [ ] **Session/process group** - setsid, setpgid, tcsetpgrp

---

### Phase F: Networking (Optional for v1)

**Goal**: Basic TCP/IP networking for client applications.

| Feature | Status |
|---------|--------|
| socket() | 🔴 Missing |
| TCP client | 🔴 Missing |
| TCP server | 🔴 Missing |
| UDP | 🔴 Missing |
| DNS resolution | 🔴 Missing |
| /etc/resolv.conf | 🔴 Missing |

This is lower priority - many useful programs work without networking.

---

## Implementation Order

### Milestone 1: Static Binary Compatibility

**Goal**: Run statically-linked Linux binaries.

1. Complete missing syscalls (fork, execve, poll, chmod, etc.)
2. Build c-gull as libc.a
3. Create minimal sysroot with headers
4. Test: compile and run `hello.c` with our libc

**Success Criteria**: `gcc -static -o hello hello.c && ./hello` works

### Milestone 2: Dynamic Binary Compatibility  

**Goal**: Run dynamically-linked Linux binaries.

1. Build c-gull as libc.so.6
2. Implement dynamic linker (ld-linux.so.2)
3. Set up /lib with shared libraries
4. Test: run standard dynamically-linked binary

**Success Criteria**: Download a Linux binary, copy it to LevitateOS, run it

### Milestone 3: Development Environment

**Goal**: Compile programs ON LevitateOS.

1. Port GCC or LLVM/Clang
2. Port binutils (as, ld)
3. Port make
4. Set up /usr/include with headers

**Success Criteria**: Write, compile, and run a C program entirely on LevitateOS

### Milestone 4: Package Management

**Goal**: Install software from packages.

1. Implement a simple package format
2. Port or create package manager
3. Create package repository

**Success Criteria**: `pkg install vim` works

---

## Priority Matrix

| Feature | Impact | Effort | Priority |
|---------|--------|--------|----------|
| fork/execve | Critical | High | P0 |
| poll/epoll | Critical | Medium | P0 |
| libc.a (static) | Critical | Medium | P0 |
| User syscalls (uid/gid) | High | Low | P1 |
| Permission syscalls | High | Low | P1 |
| termios/PTY | High | Medium | P1 |
| /proc filesystem | Medium | Medium | P2 |
| libc.so (dynamic) | High | High | P2 |
| Dynamic linker | High | Very High | P2 |
| Networking | Medium | High | P3 |
| Package management | Nice | Very High | P4 |

---

## Success Metrics

### Level 1: Eyra Apps Work (Current)
- [x] Apps modified to use Eyra run successfully
- [x] coreutils with Eyra works

### Level 2: Static Binaries Work
- [ ] `gcc -static` programs run
- [ ] No source modification needed
- [ ] musl-compiled binaries work

### Level 3: Dynamic Binaries Work
- [ ] Standard Linux binaries run
- [ ] libc.so.6 loads correctly
- [ ] ld-linux.so.2 resolves symbols

### Level 4: Self-Hosting
- [ ] Can compile programs on LevitateOS
- [ ] Development tools work natively
- [ ] Kernel can be built on LevitateOS

---

## References

- [c-ward/c-gull](https://github.com/sunfishcode/c-ward) - Rust libc implementation
- [musl-libc](https://musl.libc.org/) - Lightweight C libc
- [Redox relibc](https://gitlab.redox-os.org/redox-os/relibc) - Redox OS libc
- [Linux syscall table (x86_64)](https://filippo.io/linux-syscall-table/)
- [Linux syscall table (aarch64)](https://arm64.syscall.sh/)
- [ELF specification](https://refspecs.linuxfoundation.org/elf/elf.pdf)
- [POSIX.1-2017](https://pubs.opengroup.org/onlinepubs/9699919799/)

---

## Team Assignments

| Phase | Suggested Team Range |
|-------|---------------------|
| Phase A (Syscalls) | TEAM_400-420 |
| Phase B (libc) | TEAM_421-440 |
| Phase C (Dynamic Linker) | TEAM_441-460 |
| Phase D (Filesystem) | TEAM_461-480 |
| Phase E (TTY) | TEAM_481-500 |
| Phase F (Networking) | TEAM_501+ |
