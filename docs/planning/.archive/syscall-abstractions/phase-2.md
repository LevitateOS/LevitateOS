# Phase 2: Structural Extraction

**TEAM_411** | Syscall Abstractions Refactor
**Parent**: `phase-1.md`
**Status**: Planning

---

## 1. Target Design

### 1.1 New Module: `crates/kernel/src/syscall/helpers.rs`

Central location for syscall helper abstractions. Does **not** change the syscall dispatch or public API.

```
crates/kernel/src/syscall/
├── mod.rs          # Existing dispatch + errno
├── helpers.rs      # NEW: Abstractions
├── fs/             # Existing (will use helpers)
├── process.rs      # Existing (will use helpers)
└── ...
```

### 1.2 Abstractions to Implement

**Priority 1: VfsError → errno (Zero-effort win)**
```rust
// In syscall/mod.rs or helpers.rs
impl From<VfsError> for i64 {
    fn from(e: VfsError) -> i64 {
        match e {
            VfsError::NotFound => errno::ENOENT,
            VfsError::AlreadyExists => errno::EEXIST,
            VfsError::NotADirectory => errno::ENOTDIR,
            VfsError::IsADirectory => errno::EISDIR,
            VfsError::NotSupported => errno::ENOSYS,
            VfsError::BadFd => errno::EBADF,
            VfsError::NoSpace => errno::ENOSPC,
            VfsError::CrossDevice => errno::EXDEV,
            VfsError::DirectoryNotEmpty => errno::ENOTEMPTY,
            VfsError::PermissionDenied => errno::EACCES,
            VfsError::FileTooLarge => errno::EFBIG,
            VfsError::ReadOnlyFs => errno::EROFS,
            VfsError::TooManyLinks => errno::ELOOP,
            _ => errno::EIO,
        }
    }
}
```

**Priority 2: UserPtr / UserSlice (Safe user memory access)**
```rust
/// Validated pointer to user-space data
pub struct UserPtr<T> {
    ttbr0: usize,
    addr: usize,
    _marker: PhantomData<T>,
}

impl<T: Copy> UserPtr<T> {
    /// Create from raw user address. Does NOT validate yet.
    pub fn new(ttbr0: usize, addr: usize) -> Self;
    
    /// Read value from user space
    pub fn read(&self) -> Result<T, i64>;
    
    /// Write value to user space  
    pub fn write(&self, val: T) -> Result<(), i64>;
}

/// Validated slice in user-space
pub struct UserSlice<T> {
    ttbr0: usize,
    addr: usize,
    len: usize,
    _marker: PhantomData<T>,
}

impl<T: Copy> UserSlice<T> {
    pub fn new(ttbr0: usize, addr: usize, len: usize) -> Self;
    
    /// Copy from user space into kernel buffer
    pub fn read_to(&self, buf: &mut [T]) -> Result<usize, i64>;
    
    /// Copy from kernel buffer to user space
    pub fn write_from(&self, buf: &[T]) -> Result<usize, i64>;
}
```

**Priority 3: get_fd helper (Reduce fd lookup boilerplate)**
```rust
/// Get a cloned FdEntry for the given fd, or return EBADF
pub fn get_fd(fd: usize) -> Result<FdEntry, i64> {
    let task = current_task();
    let fd_table = task.fd_table.lock();
    fd_table.get(fd).cloned().ok_or(errno::EBADF)
}

/// Get a VfsFile for the given fd, or return appropriate errno
pub fn get_vfs_file(fd: usize) -> Result<Arc<VfsFile>, i64> {
    match get_fd(fd)?.fd_type {
        FdType::VfsFile(f) => Ok(f),
        _ => Err(errno::EBADF),
    }
}
```

**Priority 4: resolve_at_path (Proper dirfd support)**
```rust
/// Resolve a pathname relative to a directory fd
/// 
/// - If `dirfd == AT_FDCWD`, resolves relative to task's cwd
/// - If path is absolute, dirfd is ignored
/// - Otherwise, resolves relative to the directory referred to by dirfd
pub fn resolve_at_path(dirfd: i32, pathname: usize) -> Result<String, i64> {
    let task = current_task();
    
    // Read pathname from user space
    let mut path_buf = [0u8; 4096];
    let path_str = read_user_cstring(task.ttbr0, pathname, &mut path_buf)?;
    
    // Absolute paths ignore dirfd
    if path_str.starts_with('/') {
        return Ok(path_str.to_string());
    }
    
    // AT_FDCWD means use cwd
    if dirfd == AT_FDCWD {
        let cwd = task.cwd.lock();
        return Ok(format!("{}/{}", cwd.trim_end_matches('/'), path_str));
    }
    
    // Otherwise, resolve relative to dirfd
    let entry = get_fd(dirfd as usize)?;
    match &entry.fd_type {
        FdType::VfsFile(file) if file.inode.is_dir() => {
            // Get path from file's dentry
            let base = file.path.as_ref().ok_or(errno::EBADF)?;
            Ok(format!("{}/{}", base.trim_end_matches('/'), path_str))
        }
        _ => Err(errno::ENOTDIR),
    }
}
```

**Priority 5: SyscallContext (Optional ergonomics)**
```rust
/// Context for syscall execution
pub struct SyscallContext {
    task: Arc<Task>,
}

impl SyscallContext {
    pub fn current() -> Self {
        Self { task: current_task() }
    }
    
    pub fn ttbr0(&self) -> usize { self.task.ttbr0 }
    
    pub fn user_ptr<T>(&self, addr: usize) -> UserPtr<T> {
        UserPtr::new(self.ttbr0(), addr)
    }
    
    pub fn user_slice<T>(&self, addr: usize, len: usize) -> UserSlice<T> {
        UserSlice::new(self.ttbr0(), addr, len)
    }
    
    pub fn read_path(&self, addr: usize) -> Result<String, i64> {
        let mut buf = [0u8; 4096];
        read_user_cstring(self.ttbr0(), addr, &mut buf)
            .map(|s| s.to_string())
    }
}
```

---

## 2. Extraction Strategy

### 2.1 Order of Implementation

| Order | Abstraction | Reason |
|-------|-------------|--------|
| 1 | `VfsError → errno` | Zero risk, immediate consistency win |
| 2 | `UserSlice` | Most repeated pattern, biggest LOC reduction |
| 3 | `get_fd` / `get_vfs_file` | Second most repeated pattern |
| 4 | `resolve_at_path` | Enables proper dirfd support (currently stubbed) |
| 5 | `SyscallContext` | Optional convenience wrapper |

### 2.2 Coexistence Strategy

**Old and new coexist temporarily.** During migration:
- New helpers available in `syscall::helpers`
- Old inline patterns still work
- Migration happens syscall-by-syscall
- No breaking changes to syscall dispatch

---

## 3. Linux ABI Preservation

### 3.1 Internal-Only Changes
All abstractions are **internal to the kernel**. No userspace-visible changes:
- Syscall numbers unchanged
- Argument order/types unchanged  
- Return values unchanged
- Struct layouts unchanged

### 3.2 Verification Points

After each abstraction:
1. `cargo build` succeeds
2. Eyra behavior tests pass with unchanged golden output
3. Manual boot test passes

---

## 4. Phase 2 Steps

### Step 1: Create helpers.rs with VfsError conversion
- Add `impl From<VfsError> for i64`
- Export from `syscall/mod.rs`
- **Do not migrate call sites yet**

### Step 2: Implement UserPtr and UserSlice
- Add validation logic
- Add read/write methods
- Include comprehensive error handling
- **Do not migrate call sites yet**

### Step 3: Implement get_fd helpers
- `get_fd(fd) -> Result<FdEntry, i64>`
- `get_vfs_file(fd) -> Result<Arc<VfsFile>, i64>`
- **Do not migrate call sites yet**

### Step 4: Implement resolve_at_path
- Handle AT_FDCWD
- Handle absolute paths
- Handle dirfd-relative paths
- Requires VfsFile to store path (may need VFS change)

### Step 5: (Optional) Implement SyscallContext
- Wrapper for ergonomic access
- Can be skipped if other helpers sufficient

---

## 5. Exit Criteria

- [ ] All new abstractions compile
- [ ] No existing code modified (only additions)
- [ ] Unit tests for new helpers (if applicable)
- [ ] Build passes
- [ ] Ready for Phase 3 migration

---

## Next Phase

→ `phase-3.md`: Migration (update syscalls to use new abstractions)
