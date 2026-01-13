# Phase 2: Design - General Purpose OS

**TEAM_400**: General Purpose Unix-Compatible OS
**Created**: 2026-01-10
**Status**: Design Complete

---

## Proposed Solution

### User-Facing Behavior

**Before** (Current):
```bash
# Must modify source code
cd my-app
# Add eyra dependency to Cargo.toml
# Add -nostartfiles to build.rs
cargo build --release
# Copy to LevitateOS
./my-app  # Works only with Eyra modifications
```

**After** (Target):
```bash
# Standard compilation on any Linux system
gcc -static -o my-app my-app.c
# OR
RUSTFLAGS='-C target-feature=+crt-static' cargo build --release

# Copy to LevitateOS (no modification)
./my-app  # Just works
```

### System Behavior

```
┌─────────────────────────────────────────────────────────────┐
│              Static Linux Binary (ELF)                      │
│         Compiled with: gcc -static OR musl-gcc              │
└─────────────────────────────┬───────────────────────────────┘
                              │ syscall instruction
┌─────────────────────────────▼───────────────────────────────┐
│                  LevitateOS Kernel                          │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              Syscall Dispatcher                      │   │
│  │   - Linux syscall numbers (from los_abi)            │   │
│  │   - Linux ABI signatures                            │   │
│  └─────────────────────────────────────────────────────┘   │
│                              │                              │
│  ┌───────────┬───────────┬───────────┬───────────────┐     │
│  │ Process   │ File      │ Memory    │ I/O           │     │
│  │ fork      │ open      │ mmap      │ epoll         │     │
│  │ execve    │ read      │ munmap    │ poll          │     │
│  │ wait      │ write     │ brk       │ select        │     │
│  │ exit      │ chmod     │ mprotect  │ eventfd       │     │
│  └───────────┴───────────┴───────────┴───────────────┘     │
└─────────────────────────────────────────────────────────────┘
```

---

## API Design

### New Syscall Signatures

All syscalls follow Linux ABI exactly (verified against `linux-raw-sys`).

#### Process Management

```rust
/// fork - Create child process with copy of parent's address space
/// Returns: 0 to child, child PID to parent, -errno on error
pub fn sys_fork() -> i64;

/// vfork - Create child that shares parent's address space until exec
/// Returns: Same as fork
/// Note: For simplicity, may implement as fork initially
pub fn sys_vfork() -> i64;

/// execve - Replace current process image
/// Returns: Does not return on success, -errno on error
pub fn sys_execve(
    pathname: usize,  // *const c_char, null-terminated
    argv: usize,      // *const *const c_char, null-terminated array
    envp: usize,      // *const *const c_char, null-terminated array
) -> i64;
```

#### File System

```rust
/// chdir - Change working directory
/// Returns: 0 on success, -errno on error
pub fn sys_chdir(path: usize) -> i64;  // *const c_char

/// fchdir - Change working directory via fd
/// Returns: 0 on success, -errno on error
pub fn sys_fchdir(fd: i32) -> i64;

/// chmod - Change file permissions
/// Returns: 0 on success, -errno on error
pub fn sys_chmod(pathname: usize, mode: u32) -> i64;

/// fchmod - Change file permissions via fd
/// Returns: 0 on success, -errno on error
pub fn sys_fchmod(fd: i32, mode: u32) -> i64;

/// chown - Change file ownership
/// Returns: 0 on success, -errno on error
/// Note: Single-user OS - may be stub that always succeeds
pub fn sys_chown(pathname: usize, owner: u32, group: u32) -> i64;

/// fchown - Change file ownership via fd
pub fn sys_fchown(fd: i32, owner: u32, group: u32) -> i64;

/// umask - Set file creation mask
/// Returns: Previous umask value
pub fn sys_umask(mask: u32) -> i64;
```

#### System Information

```rust
/// uname - Get system identification
/// Returns: 0 on success, -errno on error
pub fn sys_uname(buf: usize) -> i64;  // *mut utsname

/// utsname structure (65 bytes per field on Linux)
#[repr(C)]
pub struct Utsname {
    pub sysname: [u8; 65],    // "LevitateOS"
    pub nodename: [u8; 65],   // hostname
    pub release: [u8; 65],    // version string
    pub version: [u8; 65],    // build info
    pub machine: [u8; 65],    // "x86_64" or "aarch64"
    pub domainname: [u8; 65], // "(none)"
}
```

#### I/O Multiplexing

```rust
/// poll - Wait for events on file descriptors
/// Returns: Number of ready fds, 0 on timeout, -errno on error
pub fn sys_poll(
    fds: usize,      // *mut pollfd
    nfds: usize,     // number of fds
    timeout: i32,    // milliseconds, -1 = infinite
) -> i64;

/// select - Synchronous I/O multiplexing
/// Returns: Number of ready fds, 0 on timeout, -errno on error
pub fn sys_select(
    nfds: i32,
    readfds: usize,   // *mut fd_set (may be null)
    writefds: usize,  // *mut fd_set (may be null)
    exceptfds: usize, // *mut fd_set (may be null)
    timeout: usize,   // *mut timeval (may be null)
) -> i64;
```

### Error Handling

All syscalls return negative errno on failure:

```rust
// Standard errno values from crates/kernel/src/syscall/mod.rs
pub mod errno {
    pub const EPERM: i64 = -1;
    pub const ENOENT: i64 = -2;
    pub const ESRCH: i64 = -3;
    pub const EINTR: i64 = -4;
    pub const EIO: i64 = -5;
    pub const ENOMEM: i64 = -12;
    pub const EACCES: i64 = -13;
    pub const EFAULT: i64 = -14;
    pub const ENOTDIR: i64 = -20;
    pub const EISDIR: i64 = -21;
    pub const EINVAL: i64 = -22;
    pub const ENOSYS: i64 = -38;
    // ... etc
}
```

---

## Data Model Changes

### Task Control Block Extensions

```rust
// crates/kernel/src/task/mod.rs
pub struct TaskControlBlock {
    // Existing fields...

    // New: For fork
    pub parent_pid: AtomicUsize,    // Track parent for wait()

    // New: For umask
    pub umask: AtomicU32,           // File creation mask (default 0o022)

    // Existing but verify:
    pub cwd: IrqSafeLock<String>,   // Already exists for getcwd
}
```

### Process Table Extensions

```rust
// crates/kernel/src/task/process_table.rs
pub struct ProcessEntry {
    // Existing fields...

    // May need: For fork parent tracking
    pub is_zombie: bool,
    pub exit_status: Option<i32>,
}
```

### FD Table Extensions (for FD_CLOEXEC)

```rust
// crates/kernel/src/task/fd_table.rs
pub struct FdEntry {
    pub fd_type: FdType,
    pub flags: FdFlags,  // NEW: Track per-fd flags
}

bitflags! {
    pub struct FdFlags: u32 {
        const CLOEXEC = 1 << 0;  // Close on exec
    }
}
```

**Note**: Fork uses eager page copy (no CoW). Page table cloning copies all mapped pages immediately. CoW optimization deferred to future.

---

## Behavioral Decisions

### fork Behavior

| Scenario | Behavior | Rationale |
|----------|----------|-----------|
| Normal fork | Create child with **eager copied** pages | Simplicity (Rule 20) |
| Fork with threads | Return ENOSYS | Complex, defer to future |
| Fork near memory limit | Return ENOMEM | Fail fast |
| Child inherits FDs | Clone FD table (with flags) | Standard behavior |
| Child inherits CWD | Copy CWD string | Standard behavior |
| Child signal handlers | Copy dispositions | Standard behavior |
| Orphaned child | Reparent to PID 1 | Standard Unix semantics |

**Memory Strategy Decision**: ✅ **RESOLVED**

**Eager Copy** selected per Q1 decision:
- Copy all writable pages immediately on fork
- Simpler implementation, no page fault handler changes
- Rule 20: Simplicity first, optimize to CoW later if memory becomes an issue

### execve Behavior

| Scenario | Behavior | Rationale |
|----------|----------|-----------|
| File not found | Return ENOENT | Standard |
| Not executable | Return EACCES | Standard |
| Invalid ELF | Return ENOEXEC | Standard |
| argv/envp > 256KB | Return E2BIG | Q9: Linux ARG_MAX |
| Successful exec | Replace address space via VFS | Q2: VFS resolution |
| FD_CLOEXEC fds | Close them | Q5: Track per-fd flag |
| Signal handlers | Reset all except SIG_IGN | Q7: Linux behavior |

**Path Resolution Decision**: ✅ **RESOLVED**

**VFS path resolution** selected per Q2 decision:
- execve uses VFS to resolve paths (supports mounted filesystems)
- Shell handles PATH environment variable search
- Kernel only resolves absolute/relative paths

**Stack Layout for execve**:
```
High Address
┌─────────────────────┐
│ envp strings        │
├─────────────────────┤
│ argv strings        │
├─────────────────────┤
│ NULL (envp term)    │
├─────────────────────┤
│ envp[n] pointers    │
├─────────────────────┤
│ NULL (argv term)    │
├─────────────────────┤
│ argv[n] pointers    │
├─────────────────────┤
│ argc                │
├─────────────────────┤
│ auxv (AT_* entries) │ ← Already implemented
├─────────────────────┤
│ 16-byte aligned     │
└─────────────────────┘
Low Address (SP points here)
```

### chdir/fchdir Behavior

| Scenario | Behavior |
|----------|----------|
| Path exists, is directory | Set CWD, return 0 |
| Path not found | Return ENOENT |
| Path is file | Return ENOTDIR |
| No read permission | Return EACCES (or succeed for root) |

### chmod/chown Behavior (Single-User OS)

| Scenario | Behavior | Rationale |
|----------|----------|-----------|
| chmod on any file | Succeed, **no-op** | Q6: Single-user, not enforced |
| chown on any file | Succeed, **no-op** | Single-user, always root |
| Invalid path | Return ENOENT | Standard |

**Metadata Storage Decision**: ✅ **RESOLVED**

**No-op** selected per Q6 decision:
- LevitateOS is single-user (always root)
- Permission checks not enforced anyway
- Storing modes adds complexity with no functional benefit
- Programs see success, which is all they need

### poll/select Behavior

| Scenario | Behavior |
|----------|----------|
| timeout = 0 | Non-blocking check |
| timeout = -1 | Block forever |
| timeout > 0 | Block up to timeout ms |
| Invalid fd | Set POLLNVAL in revents |
| Signal interrupts | Return EINTR |

**I/O Multiplexing Decision**: ✅ **RESOLVED**

**Both poll() and ppoll()** implemented per Q4 decision:
- poll() is thin wrapper around existing ppoll()
- Maximum compatibility with programs that use poll() directly
- Low additional effort

### Edge Cases and Defaults

| Situation | Default/Behavior |
|-----------|------------------|
| umask not set | Default 0o022 |
| uname nodename | "levitate" |
| uname release | Kernel version from build |
| fork() after exec() | Works (standard) |
| Double fork() | Works (orphan reparented to PID 1) |
| exec() non-existent file | ENOENT |
| poll() on epoll fd | EPOLLIN if events ready |
| getppid() for orphan | Returns 1 (Q10: standard Unix) |
| argv+envp size | Max 256KB, else E2BIG (Q9) |

---

## Design Alternatives Considered

### Alternative 1: Skip fork, Only Support spawn

**Rejected because**:
- Many programs use fork/exec pattern
- Shell job control requires fork
- Would break too many programs

### Alternative 2: Implement vfork Only

**Rejected because**:
- vfork is deprecated/discouraged
- Modern programs expect fork
- Complexity savings minimal

### Alternative 3: Full CoW from Day 1

**Deferred because**:
- Adds significant complexity to page fault handler
- Eager copy is "correct enough" for v1
- Can optimize later without API changes

### Alternative 4: Use musl-libc Instead of c-gull

**Under consideration**:
- musl is production-ready
- Would require C compilation in build
- Could run in parallel with c-gull investigation

---

## Resolved Design Decisions

All questions answered. See `docs/questions/TEAM_400_general_purpose_os.md` for full rationale.

### Critical Decisions

| ID | Question | Decision | Rationale |
|----|----------|----------|-----------|
| **Q1** | Fork memory strategy | **Eager Copy** | Rule 20: Simplicity, optimize later |
| **Q2** | execve path resolution | **VFS resolution** | Standard behavior, shell handles PATH |
| **Q3** | Orphan reparenting | **Yes, to PID 1** | Standard Unix semantics |
| **Q4** | poll() implementation | **Both poll and ppoll** | Maximum compatibility |

### Important Decisions

| ID | Question | Decision | Rationale |
|----|----------|----------|-----------|
| **Q5** | FD_CLOEXEC handling | **Track per-fd flag** | Prevents FD leaks, standard |
| **Q6** | chmod metadata storage | **No-op (just succeed)** | Single-user, not enforced |
| **Q7** | Signal reset on exec | **All except SIG_IGN** | Linux behavior, nohup compat |

### Deferred Decisions

| ID | Question | Decision | Rationale |
|----|----------|----------|-----------|
| **Q8** | pselect6 | **Defer** | YAGNI, add when needed |
| **Q9** | argv+envp limit | **256KB** | Matches Linux ARG_MAX |
| **Q10** | getppid for orphans | **Return 1** | Consistent with Q3 |

---

## Dependencies

### External Dependencies

| Dependency | Purpose | Status |
|------------|---------|--------|
| c-gull (staticlib) | libc.a for static linking | Requires testing |
| linux-raw-sys | Syscall number verification | Already used |

### Internal Dependencies

| Component | Depends On | Status |
|-----------|------------|--------|
| sys_fork | Page table duplication | Needs implementation |
| sys_execve | ELF loader | Exists (spawn_from_elf) |
| sys_chdir | VFS path resolution | Exists |
| sys_poll | FD polling infrastructure | Exists (ppoll) |

---

## Test Strategy Preview

### Unit Tests

```rust
#[test]
fn test_fork_returns_different_pids() { ... }

#[test]
fn test_execve_replaces_address_space() { ... }

#[test]
fn test_chdir_updates_cwd() { ... }

#[test]
fn test_uname_returns_valid_struct() { ... }
```

### Integration Tests

```bash
# Static binary compatibility test
gcc -static -o /tmp/hello hello.c
# Copy to LevitateOS initramfs
# Run and verify output
```

### Behavior Tests

New golden files for:
- fork/exec lifecycle traces
- Process tree verification
- CWD propagation

---

## References

- Phase 1 Discovery: `docs/planning/general-purpose-os/phase-1.md`
- TEAM_399: c-gull investigation
- Linux man pages: fork(2), execve(2), poll(2), select(2)
- `docs/specs/LINUX_ABI_GUIDE.md`
