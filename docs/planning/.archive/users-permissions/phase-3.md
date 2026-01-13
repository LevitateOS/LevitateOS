# Phase 3: Implementation - Users, Authentication & Permissions

**TEAM_405**: Users, Authentication & Permissions
**Created**: 2026-01-10
**Status**: Pending

---

## Implementation Units

### Unit 1: Credentials Structure (P0)

**File**: `crates/kernel/src/task/credentials.rs`

```rust
#[derive(Clone)]
pub struct Credentials {
    pub ruid: u32,
    pub euid: u32,
    pub suid: u32,
    pub fsuid: u32,
    pub rgid: u32,
    pub egid: u32,
    pub sgid: u32,
    pub fsgid: u32,
    pub groups: Vec<u32>,
    pub umask: u16,
}

impl Default for Credentials {
    fn default() -> Self {
        Self::root()
    }
}

impl Credentials {
    pub fn root() -> Self {
        Self {
            ruid: 0, euid: 0, suid: 0, fsuid: 0,
            rgid: 0, egid: 0, sgid: 0, fsgid: 0,
            groups: vec![0],
            umask: 0o022,
        }
    }

    pub fn is_root(&self) -> bool {
        self.euid == 0
    }
}
```

**Task**: Add `credentials: Credentials` to Task struct.

---

### Unit 2: Inode Metadata (P0)

**Files**:
- `crates/kernel/src/fs/vfs/inode.rs`
- `crates/kernel/src/fs/tmpfs/inode.rs`
- `crates/kernel/src/fs/devtmpfs/inode.rs`

Add to all inode types:
```rust
pub uid: u32,
pub gid: u32,
pub mode: u16,
```

Update inode creation to use current task's credentials:
```rust
impl TmpfsInode {
    pub fn new(mode: u16) -> Self {
        let creds = current_task().credentials();
        Self {
            uid: creds.euid,
            gid: creds.egid,
            mode: mode & !creds.umask,
            // ...
        }
    }
}
```

---

### Unit 3: Permission Check Function (P0)

**File**: `crates/kernel/src/task/credentials.rs`

```rust
pub enum AccessMode {
    Read,
    Write,
    Execute,
}

impl Credentials {
    pub fn can_access(&self, file_uid: u32, file_gid: u32, mode: u16, access: AccessMode) -> bool {
        // Root bypasses all
        if self.euid == 0 {
            return true;
        }

        let perm_bits = if self.fsuid == file_uid {
            (mode >> 6) & 0o7
        } else if self.fsgid == file_gid || self.groups.contains(&file_gid) {
            (mode >> 3) & 0o7
        } else {
            mode & 0o7
        };

        match access {
            AccessMode::Read => perm_bits & 0o4 != 0,
            AccessMode::Write => perm_bits & 0o2 != 0,
            AccessMode::Execute => perm_bits & 0o1 != 0,
        }
    }
}
```

---

### Unit 4: VFS Permission Enforcement (P0)

**File**: `crates/kernel/src/fs/vfs/operations.rs`

Add to `open()`:
```rust
pub fn open(path: &str, flags: i32, mode: u16) -> Result<FileDescriptor, VfsError> {
    let creds = current_task().credentials();
    let inode = lookup(path)?;

    // Check read permission
    if (flags & O_RDONLY != 0 || flags & O_RDWR != 0) &&
       !creds.can_access(inode.uid, inode.gid, inode.mode, AccessMode::Read) {
        return Err(VfsError::PermissionDenied);
    }

    // Check write permission
    if (flags & O_WRONLY != 0 || flags & O_RDWR != 0) &&
       !creds.can_access(inode.uid, inode.gid, inode.mode, AccessMode::Write) {
        return Err(VfsError::PermissionDenied);
    }

    // ... existing open logic
}
```

Add checks to: `unlink()`, `rmdir()`, `mkdir()`, `rename()`, `link()`, `symlink()`

---

### Unit 5: Identity Query Syscalls (P1)

**File**: `crates/kernel/src/syscall/creds.rs`

```rust
pub fn sys_getuid() -> i64 {
    current_task().credentials.ruid as i64
}

pub fn sys_geteuid() -> i64 {
    current_task().credentials.euid as i64
}

pub fn sys_getgid() -> i64 {
    current_task().credentials.rgid as i64
}

pub fn sys_getegid() -> i64 {
    current_task().credentials.egid as i64
}

pub fn sys_getgroups(size: i32, list_ptr: usize) -> i64 {
    let groups = &current_task().credentials.groups;

    if size == 0 {
        return groups.len() as i64;
    }

    if (size as usize) < groups.len() {
        return -EINVAL;
    }

    // Copy to userspace
    copy_to_user(list_ptr, groups)?;
    groups.len() as i64
}
```

Register in syscall table.

---

### Unit 6: chmod/chown/umask Syscalls (P1)

**File**: `crates/kernel/src/syscall/fs.rs`

```rust
pub fn sys_chmod(path_ptr: usize, mode: u16) -> i64 {
    let path = read_user_string(path_ptr)?;
    let creds = current_task().credentials();
    let inode = vfs::lookup(&path)?;

    // Only owner or root can chmod
    if !creds.is_root() && creds.euid != inode.uid {
        return -EPERM;
    }

    vfs::set_mode(&path, mode & 0o7777)?;
    0
}

pub fn sys_chown(path_ptr: usize, owner: u32, group: u32) -> i64 {
    let path = read_user_string(path_ptr)?;
    let creds = current_task().credentials();

    // Only root can change owner
    if owner != u32::MAX && !creds.is_root() {
        return -EPERM;
    }

    // Non-root can only chgrp to own groups
    if group != u32::MAX && !creds.is_root() {
        if creds.egid != group && !creds.groups.contains(&group) {
            return -EPERM;
        }
    }

    vfs::set_owner(&path, owner, group)?;
    0
}

pub fn sys_umask(mask: u16) -> i64 {
    let old = current_task().credentials.umask;
    current_task_mut().credentials.umask = mask & 0o777;
    old as i64
}
```

---

### Unit 7: setuid Bit Handling in exec (P1)

**File**: `crates/kernel/src/syscall/exec.rs`

```rust
pub fn sys_execve(path_ptr: usize, argv_ptr: usize, envp_ptr: usize) -> i64 {
    let path = read_user_string(path_ptr)?;
    let inode = vfs::lookup(&path)?;
    let creds = current_task().credentials();

    // Check execute permission
    if !creds.can_access(inode.uid, inode.gid, inode.mode, AccessMode::Execute) {
        return -EACCES;
    }

    // Handle setuid bit
    if inode.mode & 0o4000 != 0 {
        current_task_mut().credentials.euid = inode.uid;
        current_task_mut().credentials.fsuid = inode.uid;
        current_task_mut().credentials.suid = inode.uid;
    }

    // Handle setgid bit
    if inode.mode & 0o2000 != 0 {
        current_task_mut().credentials.egid = inode.gid;
        current_task_mut().credentials.fsgid = inode.gid;
        current_task_mut().credentials.sgid = inode.gid;
    }

    // ... rest of exec
}
```

---

### Unit 8: setuid/setgid Syscalls (P2)

**File**: `crates/kernel/src/syscall/creds.rs`

```rust
pub fn sys_setuid(uid: u32) -> i64 {
    let creds = &mut current_task_mut().credentials;

    if creds.euid == 0 {
        // Root sets all UIDs
        creds.ruid = uid;
        creds.euid = uid;
        creds.suid = uid;
        creds.fsuid = uid;
    } else if uid == creds.ruid || uid == creds.suid {
        // Non-root can only set euid to ruid or suid
        creds.euid = uid;
        creds.fsuid = uid;
    } else {
        return -EPERM;
    }
    0
}

pub fn sys_setgroups(size: i32, list_ptr: usize) -> i64 {
    if !current_task().credentials.is_root() {
        return -EPERM;
    }

    let groups = read_user_array::<u32>(list_ptr, size as usize)?;
    current_task_mut().credentials.groups = groups;
    0
}
```

---

### Unit 9: Password/User Database (P2)

**File**: `crates/kernel/src/auth/mod.rs`

```rust
pub struct PasswdEntry {
    pub name: String,
    pub uid: u32,
    pub gid: u32,
    pub home: String,
    pub shell: String,
}

pub fn lookup_user(name: &str) -> Option<PasswdEntry> {
    let content = vfs::read_to_string("/etc/passwd").ok()?;
    for line in content.lines() {
        if line.starts_with('#') || line.is_empty() { continue; }
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 7 && parts[0] == name {
            return Some(PasswdEntry {
                name: parts[0].to_string(),
                uid: parts[2].parse().ok()?,
                gid: parts[3].parse().ok()?,
                home: parts[5].to_string(),
                shell: parts[6].to_string(),
            });
        }
    }
    None
}
```

---

### Unit 10: su Utility (P3)

**File**: `crates/userspace/su/src/main.rs`

```rust
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let target_user = args.get(1).map(|s| s.as_str()).unwrap_or("root");

    // Look up user
    let user = lookup_user(target_user).expect("Unknown user");

    // Verify password (if not already root)
    if unsafe { libc::geteuid() } != 0 {
        verify_password(target_user).expect("Authentication failed");
    }

    // Set credentials
    unsafe {
        libc::setgid(user.gid);
        libc::setuid(user.uid);
    }

    // Execute shell
    let shell = std::ffi::CString::new(user.shell).unwrap();
    unsafe { libc::execl(shell.as_ptr(), shell.as_ptr(), std::ptr::null::<i8>()); }
}
```

---

## Estimated Effort

| Unit | Effort | Dependencies |
|------|--------|--------------|
| 1. Credentials struct | 1 day | None |
| 2. Inode metadata | 2 days | Unit 1 |
| 3. Permission check | 1 day | Unit 1 |
| 4. VFS enforcement | 2 days | Units 2, 3 |
| 5. Identity syscalls | 1 day | Unit 1 |
| 6. chmod/chown/umask | 2 days | Unit 2 |
| 7. setuid in exec | 1 day | Units 2, 4 |
| 8. setuid syscalls | 1 day | Unit 1 |
| 9. User database | 1 day | VFS |
| 10. su utility | 2 days | All above |

**Total**: ~14 days

---

## Test Plan

See phase-4.md for integration testing.
