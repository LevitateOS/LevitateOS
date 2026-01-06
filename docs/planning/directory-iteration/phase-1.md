# Directory Iteration Feature — Phase 1: Discovery

**Team:** TEAM_175  
**Created:** 2026-01-06  
**Status:** Draft  
**Parent Phase:** Phase 10 (ulib)

---

## 1. Feature Summary

### 1.1 Short Description

Implement a `ReadDir` iterator type in `ulib` that allows userspace programs to enumerate the contents of directories, powered by a new `sys_getdents` kernel syscall.

### 1.2 Problem Statement

Currently, userspace programs have no way to list the contents of a directory. The `ls` utility (Phase 11) and any file browser or shell tab-completion functionality requires directory enumeration. Without this, users cannot:

- List files in the initramfs
- Implement `ls`, `find`, or similar utilities
- Build interactive file selection in applications

### 1.3 Who Benefits

- **Shell users** - Can run `ls` to see available files
- **Application developers** - Can build file browsers, backup tools, etc.
- **System utilities** - Can implement recursive operations (`rm -r`, `cp -r`)

---

## 2. Success Criteria

### 2.1 Acceptance Criteria

1. **AC1**: `ReadDir::new("/path")` returns an iterator over directory entries
2. **AC2**: Each `DirEntry` provides: filename, file type (file/dir), and inode number
3. **AC3**: Iterator handles empty directories (returns `None` immediately)
4. **AC4**: Iterator handles non-existent paths (returns appropriate error)
5. **AC5**: Iterator handles non-directory paths (returns error, not panic)
6. **AC6**: Works with initramfs directories

### 2.2 Definition of Done

- [ ] `SYS_GETDENTS` syscall implemented in kernel
- [ ] `getdents()` wrapper in libsyscall
- [ ] `ReadDir` and `DirEntry` types in ulib/fs.rs
- [ ] Unit tests pass (if test harness available)
- [ ] `ls` command can list `/` directory contents

---

## 3. Current State Analysis

### 3.1 How the System Works Today

**Without directory iteration:**
- Users must know exact file paths to open files
- No discovery mechanism for initramfs contents
- Shell cannot provide file listing functionality

**Workarounds:**
- Hard-code known file paths
- Document initramfs contents externally

### 3.2 Existing Infrastructure

**Available:**
- `sys_openat` - Can open directories (with `O_DIRECTORY` flag)
- `sys_fstat` - Can check if fd is directory
- `sys_close` - Can close directory fd
- `Dirent64` struct defined in `docs/specs/userspace-abi.md`

**Missing:**
- `sys_getdents` syscall (kernel)
- `getdents()` wrapper (libsyscall)
- `ReadDir` / `DirEntry` types (ulib)

---

## 4. Codebase Reconnaissance

### 4.1 Code Areas to Touch

| Component | File | Changes |
|-----------|------|---------|
| Kernel syscall table | `kernel/src/syscall/mod.rs` | Add `SYS_GETDENTS` |
| Kernel syscall impl | `kernel/src/syscall/sys.rs` | Implement handler |
| Kernel initramfs | `kernel/src/fs/initramfs.rs` | Add directory enumeration |
| libsyscall | `userspace/libsyscall/src/lib.rs` | Add `getdents()` wrapper |
| ulib fs module | `userspace/ulib/src/fs.rs` | Add `ReadDir`, `DirEntry` |

### 4.2 Public APIs Involved

**New kernel syscall:**
```rust
// NR 14 (next available after clock_gettime)
pub const SYS_GETDENTS: u64 = 14;

fn sys_getdents(fd: usize, buf: *mut u8, buf_len: usize) -> isize;
```

**New libsyscall wrapper:**
```rust
pub fn getdents(fd: usize, buf: &mut [u8]) -> isize;
```

**New ulib types:**
```rust
pub struct ReadDir { /* internal state */ }
pub struct DirEntry { /* entry data */ }

impl Iterator for ReadDir {
    type Item = Result<DirEntry>;
}
```

### 4.3 Tests That May Be Impacted

- Golden boot test (if syscall table changes affect output)
- Any future `ls` tests
- File system integration tests

### 4.4 Non-Obvious Constraints

1. **Dirent64 variable-length records**: Each directory entry has variable size due to filename. Buffer parsing is non-trivial.

2. **Initramfs structure**: Current initramfs may be flat (no subdirectories). Need to verify structure.

3. **Syscall numbering**: Must use next available number (14) to avoid conflicts.

4. **Buffer sizing**: Caller must provide buffer; kernel fills with as many entries as fit.

---

## 5. Constraints

### 5.1 Performance Requirements

- Directory iteration should be O(n) in number of entries
- Single syscall should return multiple entries (batch efficiency)

### 5.2 Compatibility Requirements

- `Dirent64` struct layout must match Linux ABI (per userspace-abi.md)
- Error codes should use Linux errno values

### 5.3 Platform Requirements

- Must work on AArch64 (primary platform)
- x86_64 support deferred (kernel arch incomplete)

---

## 6. Phase 1 Outputs

### 6.1 Problem Understanding

✅ Problem clearly defined: Users cannot enumerate directory contents.

### 6.2 Scope Identified

- Kernel: 1 new syscall + initramfs directory support
- libsyscall: 1 new wrapper function
- ulib: 2 new types (`ReadDir`, `DirEntry`)

### 6.3 Risks Identified

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Initramfs has no directory structure | Medium | High | Verify initramfs format, may need to enhance |
| Buffer parsing bugs | Medium | Medium | Careful Dirent64 parsing, test edge cases |
| Syscall number conflicts | Low | Low | Check kernel syscall table before assigning |

---

## Next Steps

→ Proceed to **Phase 2: Design** to define API contracts and behavioral questions.
