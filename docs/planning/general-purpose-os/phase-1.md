# Phase 1: Discovery - General Purpose OS

**TEAM_400**: General Purpose Unix-Compatible OS
**Created**: 2026-01-10
**Status**: Discovery Complete

---

## Feature Summary

### Problem Statement

LevitateOS currently requires **source code modification** to run applications:
- Apps must add Eyra as a dependency
- Apps must be rebuilt with special flags
- Pre-compiled Linux binaries cannot run

This prevents LevitateOS from being a **general purpose** operating system.

### Who Benefits

| User Type | Benefit |
|-----------|---------|
| End Users | Download and run any Linux binary |
| Developers | Use standard toolchains (gcc, rustc for Linux target) |
| Package Maintainers | Port existing packages without modification |
| LevitateOS Project | Broader adoption, easier testing |

### The Definition

**General Purpose** = Can run any Unix program without modification.

**The Test**: Download a Linux binary → Run it → It works.

---

## Success Criteria

### Milestone 1: Static Binary Compatibility

```bash
# User compiles on any Linux system:
gcc -static -o hello hello.c

# Or with Rust:
RUSTFLAGS='-C target-feature=+crt-static' cargo build --release

# Copy to LevitateOS and run:
./hello
# Output: Hello, World!
```

**Acceptance Tests**:
- [ ] `hello.c` compiled with `gcc -static` runs
- [ ] `hello.c` compiled with `musl-gcc -static` runs
- [ ] Rust binary with `+crt-static` runs
- [ ] No Eyra dependency required
- [ ] No source modification required

### Milestone 2: Dynamic Binary Compatibility (Future)

```bash
# User downloads pre-compiled binary:
curl -O https://example.com/program

# Run directly:
./program
```

**Acceptance Tests** (deferred):
- [ ] Dynamically-linked ELF loads libc.so
- [ ] Standard Linux binary runs unmodified

---

## Current State Analysis

### What Works Today

| Feature | Status | Evidence |
|---------|--------|----------|
| Linux syscall ABI | ✅ Working | `los_abi` crate, syscall dispatcher |
| Eyra-modified apps | ✅ Working | uutils coreutils running |
| ELF loading | ✅ Working | `spawn_from_elf()` |
| Thread creation | ✅ Working | `sys_clone()` with CLONE_THREAD |
| File I/O | ✅ Working | read, write, openat, etc. |
| Memory management | ✅ Working | mmap, munmap, brk |
| Signals | ✅ Working | sigaction, kill, sigreturn |

### What's Missing for Static Binaries

| Gap | Impact | Effort |
|-----|--------|--------|
| **execve** (stub) | Cannot execute programs | High |
| **fork/vfork** | Cannot create processes | High |
| **chdir/fchdir** | Programs can't change directory | Low |
| **chmod/chown** | No permission management | Low |
| **uname** | System identification | Low |
| **poll/select** | Some programs need these | Medium |
| libc.a integration | Need c-gull as staticlib | Medium |

### What's Missing for Dynamic Binaries (Future)

| Gap | Impact | Effort |
|-----|--------|--------|
| libc.so.6 | Shared library loading | Very High |
| ld-linux.so.2 | Dynamic linker | Very High |
| /proc filesystem | Runtime introspection | High |
| Symbol versioning | glibc compatibility | High |

---

## Codebase Reconnaissance

### Core Modules Affected

| Module | Location | Changes Needed |
|--------|----------|----------------|
| Syscall dispatcher | `crates/kernel/src/syscall/mod.rs` | Add new syscalls |
| Process syscalls | `crates/kernel/src/syscall/process.rs` | Implement fork, execve |
| FS syscalls | `crates/kernel/src/syscall/fs/` | Add chmod, chown, chdir |
| Task management | `crates/kernel/src/task/` | Page table duplication for fork |
| ELF loader | `crates/kernel/src/task/process.rs` | execve integration |
| Memory management | `crates/kernel/src/memory/` | Eager page copy for fork |

### APIs to Extend

```rust
// New syscalls needed:
pub fn sys_fork() -> i64;
pub fn sys_execve(pathname: usize, argv: usize, envp: usize) -> i64;
pub fn sys_chdir(path: usize) -> i64;
pub fn sys_fchdir(fd: i32) -> i64;
pub fn sys_chmod(pathname: usize, mode: u32) -> i64;
pub fn sys_fchmod(fd: i32, mode: u32) -> i64;
pub fn sys_chown(pathname: usize, owner: u32, group: u32) -> i64;
pub fn sys_uname(buf: usize) -> i64;
pub fn sys_poll(fds: usize, nfds: usize, timeout: i32) -> i64;
pub fn sys_select(nfds: i32, readfds: usize, writefds: usize, exceptfds: usize, timeout: usize) -> i64;
```

### Tests Affected

| Test Suite | Impact |
|------------|--------|
| Behavior tests | New golden files for syscall traces |
| Unit tests | New syscall unit tests |
| Integration tests | Static binary execution tests |

### Existing Work to Leverage

| Component | Team | Status |
|-----------|------|--------|
| clone (threads) | TEAM_230 | Complete |
| waitpid | TEAM_188 | Complete |
| epoll | TEAM_394 | Complete |
| ppoll | TEAM_360 | Complete |
| Linux ABI signatures | TEAM_339-345 | Complete |
| ELF loader | TEAM_354 | Complete |

---

## Constraints

### Technical Constraints

1. **No std in kernel** - All syscall implementations must be `no_std`
2. **Single address space per process** - Current MMU design assumption
3. **Single-user OS** - UID/GID always 0 (root)
4. **No swap** - All memory must be physical

### Resource Constraints

1. **c-gull limitations** - Cannot build as cdylib (TEAM_399 investigation)
2. **Upstream dependencies** - c-gull/rustix versions must be compatible

### Design Constraints

1. **Rule 20: Simplicity** - Avoid over-engineering
2. **Rule 4: Silence is Golden** - No verbose output on success
3. **Rule 5: Memory Safety** - Minimize unsafe, document all unsafe blocks

---

## Resolved Questions

| ID | Question | Decision | Rationale |
|----|----------|----------|-----------|
| Q1 | Should fork use CoW or eager copy? | **Eager Copy** | Rule 20: Simplicity first, optimize to CoW later |
| Q2 | What's the minimum libc function set? | **Defer to c-gull** | c-gull provides comprehensive coverage |

See `docs/questions/TEAM_400_general_purpose_os.md` for full question resolution.

---

## References

- TEAM_397: Initial feature plan
- TEAM_398: Plan review (identified c-gull blocker)
- TEAM_399: c-gull cdylib investigation (found not feasible)
- `docs/planning/general-purpose-os/FEATURE_PLAN.md`: Original vision document
