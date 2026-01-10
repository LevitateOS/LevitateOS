# Phase 1: Discovery and Safeguards

**TEAM_411** | Syscall Abstractions Refactor
**Status**: Planning

---

## 1. Refactor Summary

**What**: Introduce common abstractions to reduce boilerplate across syscall implementations.

**Pain Points**:
- Repetitive user-space memory validation (validate → translate → unsafe copy)
- Duplicate fd lookup and type dispatch patterns
- Inconsistent VfsError → errno mapping across syscalls
- Every `*at()` syscall duplicates dirfd handling (currently stubbed)
- Task/ttbr0 retrieval repeated in every function

**Motivation**: Reducing ~30% boilerplate per syscall, improving consistency, and enabling proper dirfd support.

---

## 2. Success Criteria

| Criterion | Before | After |
|-----------|--------|-------|
| User buffer handling | ~8 lines per syscall | ~1 line (`ctx.user_buf()`) |
| Fd lookup | ~6 lines per syscall | ~1 line (`with_fd()` or `ctx.get_fd()`) |
| Error mapping | Duplicate match arms | Single `impl From<VfsError>` |
| dirfd support | Stub (returns EBADF) | Working resolution |
| Linux ABI | Compatible | **Must remain compatible** |

---

## 3. Linux ABI Compatibility Contract

**CRITICAL**: All abstractions are internal. The syscall interface visible to userspace must remain **identical**.

### 3.1 Syscall Numbers (Immutable)
- x86_64: Per `crates/kernel/src/arch/x86_64/syscall_numbers.rs`
- aarch64: Per `crates/kernel/src/arch/aarch64/syscall_numbers.rs`

### 3.2 Syscall Signatures (Immutable)
All argument counts and types must match Linux. Examples:
```
openat(dirfd: i32, pathname: *const c_char, flags: i32, mode: u32) -> i32
fstat(fd: i32, statbuf: *mut stat) -> i32
read(fd: i32, buf: *mut c_void, count: size_t) -> ssize_t
```

### 3.3 Return Values (Must Match Linux)
- Success: Non-negative (often 0, or bytes read/written, or new fd)
- Error: Negative errno values per Linux convention

### 3.4 Struct Layouts (Immutable)
- `struct stat` / `struct stat64`
- `struct statx`
- `struct dirent64`
- `struct iovec`
- `struct timespec`

These are architecture-specific and **must not change**.

### 3.5 errno Values (Immutable)
All errno constants must match Linux values exactly:
- ENOENT = -2
- EBADF = -9
- EFAULT = -14
- etc.

---

## 4. Behavioral Contracts

### 4.1 File Descriptor Operations
- `read(fd, ...)` on invalid fd → EBADF
- `read(fd, ...)` on non-readable fd → EBADF  
- `write(fd, ...)` on non-writable fd → EBADF
- `lseek(fd, ...)` on pipe → ESPIPE
- `ftruncate(fd, ...)` on directory → EISDIR

### 4.2 Path Operations
- Null pathname pointer → EFAULT
- Path too long → ENAMETOOLONG
- Path not found → ENOENT
- dirfd = AT_FDCWD → use current working directory
- dirfd != AT_FDCWD with relative path → resolve relative to dirfd

### 4.3 Memory Operations  
- Invalid user buffer → EFAULT
- Buffer overflow attempts → EFAULT (not kernel crash)

---

## 5. Golden/Regression Tests

### 5.1 Eyra Behavior Tests
**File**: `tests/eyra_behavior_test.sh`
**Golden**: `tests/eyra_output_golden.txt`

This captures syscall behavior for Eyra coreutils. Must pass unchanged.

### 5.2 Build Verification
```bash
cargo build  # Must succeed for all targets
```

### 5.3 Manual Syscall Tests
Run userspace binaries that exercise syscalls:
```bash
cargo xtask run  # Boot and run shell commands
```

---

## 6. Current Architecture Notes

### 6.1 Syscall Module Structure
```
crates/kernel/src/syscall/
├── mod.rs          # Dispatch, errno, helpers
├── fs/             # Filesystem syscalls
│   ├── fd.rs       # dup, pipe, ioctl, fcntl, lseek, etc.
│   ├── open.rs     # openat, close, faccessat
│   ├── read.rs     # read, readv
│   ├── write.rs    # write, writev
│   ├── dir.rs      # getcwd, getdents, mkdirat, etc.
│   ├── stat.rs     # fstat, fstatat
│   ├── statx.rs    # statx
│   ├── link.rs     # linkat, symlinkat, readlinkat, utimensat
│   └── mount.rs    # mount, umount
├── process.rs      # fork, exec, wait, getpid, etc.
├── signal.rs       # kill, sigaction, etc.
├── mm.rs           # mmap, munmap, mprotect, brk
├── time.rs         # nanosleep, clock_gettime
├── sync.rs         # futex, poll, ppoll
├── epoll.rs        # epoll_*, eventfd
└── sys.rs          # getrandom, shutdown
```

### 6.2 Common Patterns Identified

**Pattern A: User buffer handling** (~40 occurrences)
```rust
if mm_user::validate_user_buffer(ttbr0, buf, len, writable).is_err() {
    return errno::EFAULT;
}
let ptr = mm_user::user_va_to_kernel_ptr(ttbr0, buf).unwrap();
unsafe { core::ptr::copy_nonoverlapping(...) }
```

**Pattern B: Fd lookup** (~25 occurrences)
```rust
let task = current_task();
let fd_table = task.fd_table.lock();
let entry = match fd_table.get(fd) {
    Some(e) => e.clone(),
    None => return errno::EBADF,
};
drop(fd_table);
```

**Pattern C: dirfd check** (~10 occurrences)
```rust
if dirfd != AT_FDCWD && !path_str.starts_with('/') {
    log::warn!("dirfd {} not yet supported", dirfd);
    return errno::EBADF;
}
```

**Pattern D: VfsError mapping** (~15 occurrences)
```rust
match result {
    Ok(v) => v,
    Err(VfsError::NotFound) => errno::ENOENT,
    Err(VfsError::AlreadyExists) => errno::EEXIST,
    // ... duplicated everywhere
}
```

### 6.3 Dependencies
- `crate::memory::user` - User-space memory access
- `crate::task` - Task/process management
- `crate::fs::vfs` - Virtual filesystem
- `los_hal` - Hardware abstraction

---

## 7. Constraints

### 7.1 Absolute Constraints
- **Linux ABI must remain identical** (syscall numbers, signatures, errno values)
- **No behavioral changes** visible to userspace
- **Eyra tests must pass** without golden file updates

### 7.2 Soft Constraints
- Prefer zero-cost abstractions (no runtime overhead)
- Minimize unsafe code surface area
- Keep file sizes <1000 lines

---

## 8. Open Questions

**Q1**: Should `SyscallContext` be passed explicitly or retrieved via `current_task()`?
- Explicit: More testable, clearer dependencies
- Implicit: Less boilerplate at call sites

**Q2**: Should `resolve_at_path` return a `Dentry` or a path string?
- Dentry: More efficient for subsequent operations
- String: Simpler, works with existing VFS functions

**Q3**: How to handle the fd_table lock lifetime with `with_fd()` helpers?
- Clone entry and drop lock (current pattern)
- Pass callback while lock held (more efficient but less flexible)

---

## 9. Phase 1 Steps

### Step 1: Audit Syscall Patterns
- Count exact occurrences of each pattern
- Identify edge cases and variations

### Step 2: Verify Baseline Tests
- Run Eyra behavior tests
- Confirm golden output matches

### Step 3: Document Linux ABI Surface
- Catalog all syscall signatures
- Verify struct layouts match Linux headers

---

## Next Phase

→ `phase-2.md`: Structural Extraction (design and implement abstractions)
