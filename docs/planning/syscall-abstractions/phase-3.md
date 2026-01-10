# Phase 3: Migration

**TEAM_411** | Syscall Abstractions Refactor
**Parent**: `phase-2.md`
**Status**: Planning

---

## 1. Migration Strategy

### 1.1 Guiding Principles

1. **One abstraction at a time** - Complete all migrations for one helper before starting the next
2. **One file at a time** - Migrate all uses in a file before moving to the next
3. **Test after each file** - Run build + behavior tests after each file migration
4. **Preserve behavior exactly** - No functional changes during migration

### 1.2 Order of Migration

| Order | Abstraction | Files Affected | Est. Changes |
|-------|-------------|----------------|--------------|
| 1 | VfsError → errno | All fs/*.rs | ~15 match blocks |
| 2 | UserSlice | read.rs, write.rs, dir.rs, stat.rs | ~20 sites |
| 3 | get_fd helpers | fd.rs, read.rs, write.rs, stat.rs | ~25 sites |
| 4 | resolve_at_path | open.rs, dir.rs, link.rs, stat.rs | ~10 sites |

---

## 2. Call Site Inventory

### 2.1 VfsError Mapping Sites

| File | Function | Current Pattern |
|------|----------|-----------------|
| fs/open.rs | sys_openat | 5-case match on VfsError |
| fs/open.rs | sys_faccessat | 3-case match |
| fs/dir.rs | sys_mkdirat | 4-case match |
| fs/dir.rs | sys_unlinkat | 4-case match |
| fs/dir.rs | sys_renameat | 4-case match |
| fs/link.rs | sys_linkat | 3-case match |
| fs/link.rs | sys_symlinkat | 3-case match |
| fs/fd.rs | sys_truncate | 4-case match |
| fs/fd.rs | sys_ftruncate | 4-case match |
| fs/fd.rs | sys_lseek | 2-case match |
| fs/fd.rs | sys_pread64 | 2-case match |
| fs/fd.rs | sys_pwrite64 | 3-case match |
| fs/stat.rs | sys_fstat | 1-case match |
| fs/stat.rs | sys_fstatat | 1-case match |
| fs/statx.rs | sys_statx | 2-case match |

**Migration**: Replace `Err(VfsError::X) => errno::Y` with `Err(e) => e.into()`

### 2.2 User Buffer Validation Sites

| File | Function | Pattern |
|------|----------|---------|
| fs/read.rs | sys_read | validate + copy 3x |
| fs/read.rs | sys_readv | validate iovec + iterate |
| fs/write.rs | sys_write | validate + copy 2x |
| fs/write.rs | sys_writev | validate iovec + iterate |
| fs/dir.rs | sys_getdents | validate + write_to_user_buf loop |
| fs/dir.rs | sys_getcwd | validate + copy |
| fs/stat.rs | sys_fstat | validate + copy struct |
| fs/stat.rs | sys_fstatat | validate + copy struct |
| fs/statx.rs | sys_statx | validate + copy struct |
| fs/fd.rs | sys_pipe2 | validate + write 2 ints |
| fs/fd.rs | sys_ioctl | validate + copy termios |
| fs/fd.rs | sys_pread64 | validate + copy |
| fs/fd.rs | sys_pwrite64 | validate + copy |
| mm.rs | sys_mmap | (may not need change - different pattern) |
| process.rs | sys_clone | validate + copy |
| time.rs | sys_nanosleep | validate + copy timespec |
| time.rs | sys_clock_gettime | validate + copy timespec |

### 2.3 Fd Lookup Sites

| File | Function | Pattern |
|------|----------|---------|
| fs/fd.rs | sys_fcntl | get + match |
| fs/fd.rs | sys_dup | get + dup |
| fs/fd.rs | sys_dup3 | get + dup_to |
| fs/fd.rs | sys_isatty | get + match type |
| fs/fd.rs | sys_ioctl | get + match type |
| fs/fd.rs | sys_lseek | get + match type |
| fs/fd.rs | sys_ftruncate | get + match type |
| fs/fd.rs | sys_pread64 | get + match type |
| fs/fd.rs | sys_pwrite64 | get + match type |
| fs/fd.rs | sys_fchmod | get (validation only) |
| fs/fd.rs | sys_fchown | get (validation only) |
| fs/read.rs | sys_read | get + match type |
| fs/read.rs | sys_readv | delegates to sys_read |
| fs/write.rs | sys_write | get + match type |
| fs/stat.rs | sys_fstat | get + match type |
| fs/dir.rs | sys_getdents | get + match type |

### 2.4 dirfd Resolution Sites

| File | Function | Current Behavior |
|------|----------|------------------|
| fs/open.rs | sys_openat | Stub: warns + returns EBADF |
| fs/open.rs | sys_faccessat | Stub: warns + returns EBADF |
| fs/dir.rs | sys_mkdirat | Stub: warns + returns EBADF |
| fs/dir.rs | sys_unlinkat | Stub: warns + returns EBADF |
| fs/dir.rs | sys_renameat | Stub: warns + returns EBADF |
| fs/link.rs | sys_linkat | Stub: warns + returns EBADF |
| fs/link.rs | sys_symlinkat | Stub: warns + returns EBADF |
| fs/link.rs | sys_readlinkat | Stub: warns + returns EBADF |
| fs/link.rs | sys_utimensat | Stub: warns + returns EBADF |
| fs/stat.rs | sys_fstatat | Stub: warns + returns EBADF |
| fs/statx.rs | sys_statx | Stub: warns + returns EBADF |

---

## 3. Migration Steps

### Step 1: Migrate VfsError Conversions

**UoW 1**: fs/open.rs, fs/dir.rs
**UoW 2**: fs/link.rs, fs/fd.rs  
**UoW 3**: fs/stat.rs, fs/statx.rs

For each file:
```rust
// Before:
match vfs_mkdir(path, mode) {
    Ok(()) => 0,
    Err(VfsError::AlreadyExists) => errno::EEXIST,
    Err(VfsError::NotFound) => errno::ENOENT,
    Err(_) => errno::EINVAL,
}

// After:
match vfs_mkdir(path, mode) {
    Ok(()) => 0,
    Err(e) => e.into(),
}
```

### Step 2: Migrate User Buffer Handling

**UoW 1**: fs/stat.rs (simplest - just struct copy)
**UoW 2**: fs/read.rs, fs/write.rs
**UoW 3**: fs/dir.rs (getdents is complex)
**UoW 4**: fs/fd.rs (ioctl, pread64, pwrite64)
**UoW 5**: time.rs, process.rs

For each:
```rust
// Before:
if mm_user::validate_user_buffer(ttbr0, buf, size, true).is_err() {
    return errno::EFAULT;
}
let dest = mm_user::user_va_to_kernel_ptr(ttbr0, buf).unwrap();
unsafe { core::ptr::copy_nonoverlapping(&stat as *const _, dest, size); }

// After:
let user_buf = UserSlice::new(ttbr0, buf, size);
if user_buf.write_from(stat.as_bytes()).is_err() {
    return errno::EFAULT;
}
```

### Step 3: Migrate Fd Lookups

**UoW 1**: fs/fd.rs (most occurrences)
**UoW 2**: fs/read.rs, fs/write.rs
**UoW 3**: fs/stat.rs, fs/dir.rs

For each:
```rust
// Before:
let task = current_task();
let fd_table = task.fd_table.lock();
let entry = match fd_table.get(fd) {
    Some(e) => e.clone(),
    None => return errno::EBADF,
};
drop(fd_table);

// After:
let entry = match get_fd(fd) {
    Ok(e) => e,
    Err(e) => return e,
};
```

### Step 4: Migrate dirfd Resolution

**UoW 1**: fs/open.rs (openat, faccessat)
**UoW 2**: fs/dir.rs (mkdirat, unlinkat, renameat)
**UoW 3**: fs/link.rs (linkat, symlinkat, readlinkat, utimensat)
**UoW 4**: fs/stat.rs, fs/statx.rs (fstatat, statx)

For each:
```rust
// Before:
let mut path_buf = [0u8; 4096];
let path_str = match read_user_cstring(task.ttbr0, pathname, &mut path_buf) {
    Ok(s) => s,
    Err(e) => return e,
};
if dirfd != AT_FDCWD && !path_str.starts_with('/') {
    log::warn!("dirfd {} not yet supported", dirfd);
    return errno::EBADF;
}

// After:
let path = match resolve_at_path(dirfd, pathname) {
    Ok(p) => p,
    Err(e) => return e,
};
```

---

## 4. Rollback Plan

If migration causes issues:

1. **Per-file rollback**: Git revert the specific file change
2. **Per-abstraction rollback**: Revert all files using that abstraction
3. **Full rollback**: Return to pre-Phase-3 state

Each UoW should be a single commit for easy revert.

---

## 5. Exit Criteria

After all migrations:
- [ ] All syscalls use new abstractions where applicable
- [ ] No duplicate VfsError match blocks remain
- [ ] No inline user buffer validation (use UserSlice)
- [ ] No inline fd lookup (use get_fd helpers)
- [ ] dirfd properly supported (not stubbed)
- [ ] Build passes
- [ ] Eyra behavior tests pass unchanged
- [ ] Manual boot test passes

---

## Next Phase

→ `phase-4.md`: Cleanup (remove dead code, old patterns)
