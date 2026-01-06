# File Read Implementation — Phase 1: Discovery

**Team:** TEAM_177  
**Created:** 2026-01-06  
**Status:** Draft  
**Parent Phase:** Phase 10 (ulib)

---

## 1. Feature Summary

### 1.1 Short Description

Implement `File::read()` for initramfs files, enabling userspace programs to read file contents via the `Read` trait.

### 1.2 Problem Statement

Currently, `File::open()` works but `file.read()` returns `NotImplemented`. Users can open files but cannot read their contents. This blocks:

- `cat` utility (Phase 11)
- Any application that needs to read configuration or data files
- Script interpreters that need to read source files

### 1.3 Who Benefits

- **Application developers** - Can read files from initramfs
- **Shell users** - Can use `cat` and similar utilities
- **System utilities** - Can read configuration files

---

## 2. Success Criteria

### 2.1 Acceptance Criteria

1. **AC1**: `file.read(&mut buf)` returns bytes read from initramfs file
2. **AC2**: Sequential reads advance file position correctly
3. **AC3**: Reading past EOF returns 0 (not error)
4. **AC4**: `read_to_end()` / `read_to_string()` work via `Read` trait default impls
5. **AC5**: Multiple files can be read concurrently (different fds)

### 2.2 Definition of Done

- [ ] Kernel `sys_read` handles `InitramfsFile` fds
- [ ] Offset tracking works correctly
- [ ] ulib `File::read()` calls syscall
- [ ] Basic smoke test works (read known file)

---

## 3. Current State Analysis

### 3.1 How the System Works Today

**Kernel side:**
- `sys_read(fd, buf, len)` only handles fd 0 (stdin)
- Returns `EBADF` for any other fd
- `InitramfsFile` FdType already has `offset` field (unused!)

**Userspace side:**
- `File::open()` works - opens file, gets fd
- `File::read()` stub returns `NotImplemented`
- `File::metadata()` works - can get file size

### 3.2 Existing Infrastructure

**Already implemented:**
- `FdType::InitramfsFile { file_index, offset }` - tracks file and position
- `sys_openat` - creates InitramfsFile fd with offset=0
- `INITRAMFS` global - CpioArchive with file data
- `write_to_user_buf` - helper to write bytes to userspace

**Missing:**
- `sys_read` dispatch for `InitramfsFile`
- Offset update after read
- ulib calling the syscall

---

## 4. Codebase Reconnaissance

### 4.1 Code Areas to Touch

| Component | File | Changes |
|-----------|------|---------|
| Kernel sys_read | `kernel/src/syscall/fs.rs` | Add InitramfsFile handling |
| FdTable | `kernel/src/task/fd_table.rs` | Add method to update offset |
| ulib File | `userspace/ulib/src/fs.rs` | Implement Read trait |

### 4.2 Key Code References

**FdType with offset (already exists):**
```rust
InitramfsFile {
    file_index: usize,
    offset: usize,  // <-- Already there!
}
```

**Current sys_read (stdin only):**
```rust
pub fn sys_read(fd: usize, buf: usize, len: usize) -> i64 {
    if fd != 0 {
        return errno::EBADF;  // <-- Need to handle other fds
    }
    // ... stdin handling
}
```

### 4.3 Estimated Complexity

**Low complexity** - Most infrastructure exists:
- ~30 lines kernel code (sys_read dispatch + initramfs read)
- ~5 lines ulib code (call syscall)
- ~10 lines fd_table (offset update helper)

---

## 5. Constraints

### 5.1 Initramfs is Read-Only

Files in initramfs are immutable. No write support needed (or possible).

### 5.2 No Seek Syscall Yet

Position tracking is internal to the fd. No `lseek()` syscall exists.
For MVP, sequential read-only access is sufficient.

---

## 6. Phase 1 Outputs

### 6.1 Problem Understanding

✅ Problem clear: `File::read()` doesn't work, blocking file access.

### 6.2 Solution Path Identified

The infrastructure exists! Just need to:
1. Extend `sys_read` to check fd type
2. For `InitramfsFile`, read from INITRAMFS and update offset
3. Have ulib call the syscall

### 6.3 Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Offset not updating correctly | Low | Medium | Careful testing with sequential reads |
| Concurrent access issues | Low | Low | FdTable is per-task, already locked |

---

## Next Steps

→ Proceed to **Phase 2: Design** for API details and behavioral questions.
