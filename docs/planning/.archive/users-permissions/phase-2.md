# Phase 2: Design - Users, Authentication & Permissions

**TEAM_405**: Users, Authentication & Permissions
**Created**: 2026-01-10
**Status**: Design

---

## Proposed Solution

### Process Credentials Structure

```rust
// crates/kernel/src/task/credentials.rs

/// Process credentials - who is this process?
#[derive(Clone)]
pub struct Credentials {
    /// Real UID - who started the process
    pub ruid: u32,
    /// Effective UID - what permissions apply
    pub euid: u32,
    /// Saved UID - can restore to this
    pub suid: u32,
    /// Filesystem UID - for file access
    pub fsuid: u32,

    /// Real GID
    pub rgid: u32,
    /// Effective GID
    pub egid: u32,
    /// Saved GID
    pub sgid: u32,
    /// Filesystem GID
    pub fsgid: u32,

    /// Supplementary groups
    pub groups: Vec<u32>,

    /// File creation mask
    pub umask: u16,
}

impl Credentials {
    /// Create root credentials
    pub fn root() -> Self {
        Self {
            ruid: 0, euid: 0, suid: 0, fsuid: 0,
            rgid: 0, egid: 0, sgid: 0, fsgid: 0,
            groups: vec![0],
            umask: 0o022,
        }
    }

    /// Create credentials for a regular user
    pub fn user(uid: u32, gid: u32) -> Self {
        Self {
            ruid: uid, euid: uid, suid: uid, fsuid: uid,
            rgid: gid, egid: gid, sgid: gid, fsgid: gid,
            groups: vec![gid],
            umask: 0o022,
        }
    }

    /// Is this process running as root?
    pub fn is_root(&self) -> bool {
        self.euid == 0
    }

    /// Can this process access a file with given ownership and mode?
    pub fn can_access(&self, file_uid: u32, file_gid: u32, mode: u16, access: Access) -> bool {
        // Root can do anything
        if self.euid == 0 {
            return true;
        }

        let perms = if self.fsuid == file_uid {
            (mode >> 6) & 0o7  // Owner bits
        } else if self.fsgid == file_gid || self.groups.contains(&file_gid) {
            (mode >> 3) & 0o7  // Group bits
        } else {
            mode & 0o7         // Other bits
        };

        match access {
            Access::Read => perms & 0o4 != 0,
            Access::Write => perms & 0o2 != 0,
            Access::Execute => perms & 0o1 != 0,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Access {
    Read,
    Write,
    Execute,
}
```

### Task Struct Integration

```rust
// crates/kernel/src/task/mod.rs

pub struct Task {
    pub pid: Pid,
    pub state: TaskState,
    pub credentials: Credentials,  // NEW
    // ... existing fields
}

impl Task {
    pub fn fork(&self) -> Task {
        Task {
            pid: allocate_pid(),
            state: TaskState::Ready,
            credentials: self.credentials.clone(),  // Inherit credentials
            // ...
        }
    }

    pub fn exec(&mut self, path: &str) -> Result<(), Error> {
        // Check execute permission
        let inode = vfs::lookup(path)?;
        if !self.credentials.can_access(
            inode.uid, inode.gid, inode.mode, Access::Execute
        ) {
            return Err(Error::PermissionDenied);  // EACCES
        }

        // Handle setuid/setgid
        if inode.mode & 0o4000 != 0 {  // setuid bit
            self.credentials.euid = inode.uid;
            self.credentials.fsuid = inode.uid;
        }
        if inode.mode & 0o2000 != 0 {  // setgid bit
            self.credentials.egid = inode.gid;
            self.credentials.fsgid = inode.gid;
        }

        // ... rest of exec
    }
}
```

### VFS Permission Enforcement

```rust
// crates/kernel/src/fs/vfs/operations.rs

pub fn open(path: &str, flags: i32, mode: u16) -> Result<FileDescriptor, VfsError> {
    let creds = current_task().credentials();
    let inode = lookup(path)?;

    // Check permissions based on flags
    if flags & O_RDONLY != 0 || flags & O_RDWR != 0 {
        if !creds.can_access(inode.uid, inode.gid, inode.mode, Access::Read) {
            return Err(VfsError::PermissionDenied);
        }
    }
    if flags & O_WRONLY != 0 || flags & O_RDWR != 0 {
        if !creds.can_access(inode.uid, inode.gid, inode.mode, Access::Write) {
            return Err(VfsError::PermissionDenied);
        }
    }

    // ... rest of open
}

pub fn unlink(path: &str) -> Result<(), VfsError> {
    let creds = current_task().credentials();
    let parent = lookup_parent(path)?;
    let entry = lookup(path)?;

    // Need write permission on parent directory
    if !creds.can_access(parent.uid, parent.gid, parent.mode, Access::Write) {
        return Err(VfsError::PermissionDenied);
    }

    // Sticky bit check: only owner can delete
    if parent.mode & 0o1000 != 0 {
        if creds.euid != 0 && creds.euid != entry.uid && creds.euid != parent.uid {
            return Err(VfsError::PermissionDenied);
        }
    }

    // ... rest of unlink
}
```

### Inode Metadata Extension

```rust
// crates/kernel/src/fs/vfs/inode.rs

pub struct InodeMetadata {
    pub ino: u64,
    pub mode: u16,      // File type + permissions
    pub nlink: u32,
    pub uid: u32,       // Owner
    pub gid: u32,       // Group
    pub size: u64,
    pub atime: Timespec,
    pub mtime: Timespec,
    pub ctime: Timespec,
    // ...
}

// crates/kernel/src/fs/tmpfs/inode.rs

pub struct TmpfsInode {
    pub ino: u64,
    pub mode: u16,
    pub uid: u32,       // NEW
    pub gid: u32,       // NEW
    pub data: TmpfsData,
    // ...
}

impl TmpfsInode {
    pub fn new(mode: u16, creds: &Credentials) -> Self {
        Self {
            ino: allocate_ino(),
            mode: mode & !creds.umask,  // Apply umask
            uid: creds.euid,
            gid: creds.egid,
            data: TmpfsData::Empty,
        }
    }
}
```

---

## Syscall Implementations

### Identity Query Syscalls

```rust
// crates/kernel/src/syscall/creds.rs

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

pub fn sys_getgroups(size: i32, list: *mut u32) -> i64 {
    let creds = current_task().credentials();
    let groups = &creds.groups;

    if size == 0 {
        return groups.len() as i64;
    }

    if (size as usize) < groups.len() {
        return -EINVAL;
    }

    // Copy to userspace
    for (i, &gid) in groups.iter().enumerate() {
        unsafe { *list.add(i) = gid; }
    }

    groups.len() as i64
}
```

### Identity Change Syscalls

```rust
pub fn sys_setuid(uid: u32) -> i64 {
    let task = current_task_mut();
    let creds = &mut task.credentials;

    if creds.euid == 0 {
        // Root can set all UIDs
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

pub fn sys_setgid(gid: u32) -> i64 {
    let task = current_task_mut();
    let creds = &mut task.credentials;

    if creds.euid == 0 {
        creds.rgid = gid;
        creds.egid = gid;
        creds.sgid = gid;
        creds.fsgid = gid;
    } else if gid == creds.rgid || gid == creds.sgid {
        creds.egid = gid;
        creds.fsgid = gid;
    } else {
        return -EPERM;
    }

    0
}

pub fn sys_setresuid(ruid: u32, euid: u32, suid: u32) -> i64 {
    let task = current_task_mut();
    let creds = &mut task.credentials;

    // -1 means "don't change"
    let new_ruid = if ruid == u32::MAX { creds.ruid } else { ruid };
    let new_euid = if euid == u32::MAX { creds.euid } else { euid };
    let new_suid = if suid == u32::MAX { creds.suid } else { suid };

    if creds.euid != 0 {
        // Non-root can only set to current ruid, euid, or suid
        let allowed = [creds.ruid, creds.euid, creds.suid];
        if !allowed.contains(&new_ruid) ||
           !allowed.contains(&new_euid) ||
           !allowed.contains(&new_suid) {
            return -EPERM;
        }
    }

    creds.ruid = new_ruid;
    creds.euid = new_euid;
    creds.suid = new_suid;
    creds.fsuid = new_euid;

    0
}

pub fn sys_setgroups(size: i32, list: *const u32) -> i64 {
    let creds = current_task().credentials();

    // Only root can set groups
    if creds.euid != 0 {
        return -EPERM;
    }

    let mut groups = Vec::with_capacity(size as usize);
    for i in 0..size as usize {
        groups.push(unsafe { *list.add(i) });
    }

    current_task_mut().credentials.groups = groups;
    0
}
```

### File Permission Syscalls

```rust
pub fn sys_chmod(path_ptr: usize, mode: u16) -> i64 {
    let path = read_user_string(path_ptr)?;
    let creds = current_task().credentials();

    let inode = vfs::lookup(&path)?;

    // Only owner or root can chmod
    if creds.euid != 0 && creds.euid != inode.uid {
        return -EPERM;
    }

    // Clear setgid if not in group (unless root)
    let mut new_mode = mode & 0o7777;
    if creds.euid != 0 && !creds.groups.contains(&inode.gid) && creds.egid != inode.gid {
        new_mode &= !0o2000;  // Clear setgid
    }

    vfs::set_mode(&path, new_mode)?;
    0
}

pub fn sys_chown(path_ptr: usize, owner: u32, group: u32) -> i64 {
    let path = read_user_string(path_ptr)?;
    let creds = current_task().credentials();

    // Only root can change owner
    if owner != u32::MAX && creds.euid != 0 {
        return -EPERM;
    }

    // Non-root can only chgrp to a group they're in
    if group != u32::MAX && creds.euid != 0 {
        if creds.egid != group && !creds.groups.contains(&group) {
            return -EPERM;
        }
    }

    let inode = vfs::lookup(&path)?;

    // Non-root must own the file to chgrp
    if creds.euid != 0 && creds.euid != inode.uid {
        return -EPERM;
    }

    vfs::set_owner(&path, owner, group)?;

    // Clear setuid/setgid on chown (security)
    if owner != u32::MAX {
        vfs::clear_suid(&path)?;
    }

    0
}

pub fn sys_umask(mask: u16) -> i64 {
    let old = current_task().credentials.umask;
    current_task_mut().credentials.umask = mask & 0o777;
    old as i64
}

pub fn sys_access(path_ptr: usize, mode: i32) -> i64 {
    let path = read_user_string(path_ptr)?;
    let creds = current_task().credentials();

    let inode = vfs::lookup(&path)?;

    // Use REAL uid/gid, not effective (that's the point of access())
    let check_creds = Credentials {
        fsuid: creds.ruid,
        fsgid: creds.rgid,
        ..creds.clone()
    };

    if mode & R_OK != 0 && !check_creds.can_access(inode.uid, inode.gid, inode.mode, Access::Read) {
        return -EACCES;
    }
    if mode & W_OK != 0 && !check_creds.can_access(inode.uid, inode.gid, inode.mode, Access::Write) {
        return -EACCES;
    }
    if mode & X_OK != 0 && !check_creds.can_access(inode.uid, inode.gid, inode.mode, Access::Execute) {
        return -EACCES;
    }

    0
}
```

---

## User Database Parsing

```rust
// crates/kernel/src/auth/passwd.rs

pub struct PasswdEntry {
    pub name: String,
    pub uid: u32,
    pub gid: u32,
    pub gecos: String,
    pub home: String,
    pub shell: String,
}

pub fn parse_passwd(content: &str) -> Vec<PasswdEntry> {
    content.lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 7 {
                Some(PasswdEntry {
                    name: parts[0].to_string(),
                    uid: parts[2].parse().ok()?,
                    gid: parts[3].parse().ok()?,
                    gecos: parts[4].to_string(),
                    home: parts[5].to_string(),
                    shell: parts[6].to_string(),
                })
            } else {
                None
            }
        })
        .collect()
}

pub fn lookup_user(name: &str) -> Option<PasswdEntry> {
    let content = vfs::read_to_string("/etc/passwd").ok()?;
    parse_passwd(&content).into_iter().find(|e| e.name == name)
}

pub fn lookup_uid(uid: u32) -> Option<PasswdEntry> {
    let content = vfs::read_to_string("/etc/passwd").ok()?;
    parse_passwd(&content).into_iter().find(|e| e.uid == uid)
}
```

---

## Boot Process Changes

```rust
// Init starts as root
fn init_process() {
    let mut task = Task::new();
    task.credentials = Credentials::root();
    task.exec("/sbin/init");
}

// Login spawns shell with user credentials
fn login(username: &str, password: &str) -> Result<(), Error> {
    let user = lookup_user(username)?;

    // Verify password (if shadow file exists)
    verify_password(username, password)?;

    // Fork and set credentials
    let pid = fork();
    if pid == 0 {
        // Child: become the user
        let creds = Credentials::user(user.uid, user.gid);
        current_task_mut().credentials = creds;

        // Set supplementary groups from /etc/group
        let groups = get_user_groups(username);
        current_task_mut().credentials.groups = groups;

        // Change to home directory
        chdir(&user.home);

        // Execute shell
        exec(&user.shell);
    }
}
```

---

## Default Users

### /etc/passwd

```
root:x:0:0:System Administrator:/root:/bin/sh
user:x:1000:1000:Default User:/home/user:/bin/sh
nobody:x:65534:65534:Nobody:/nonexistent:/bin/false
```

### /etc/group

```
root:x:0:root
wheel:x:10:root,user
users:x:100:user
nobody:x:65534:
```

---

## Behavioral Decisions

### Initial Boot User

| Option | Description | Decision |
|--------|-------------|----------|
| A) Start as root | Init runs as UID 0 | **Selected** |
| B) Start as user | Safer but less useful | Rejected |

Boot as root, login creates user sessions.

### Password Authentication

| Option | Description | Decision |
|--------|-------------|----------|
| A) No passwords | Everyone is trusted | For development |
| B) Shadow file | SHA-512 hashed | **For production** |
| C) PAM | Pluggable auth | Future |

Start with no passwords (auto-login as root), add shadow file authentication for multi-user.

### Supplementary Groups Limit

| Option | Description | Decision |
|--------|-------------|----------|
| A) 16 groups | Traditional limit | Rejected |
| B) 65536 groups | Linux NGROUPS_MAX | **Selected** |

No artificial limits.

### Capabilities

| Option | Description | Decision |
|--------|-------------|----------|
| A) UID 0 only | Traditional root | **For M1** |
| B) Full CAP_* | Fine-grained | Future |

UID 0 has all privileges. Capabilities are future enhancement.

---

## Test Strategy

### Unit Tests

```rust
#[test]
fn test_permission_check_owner() {
    let creds = Credentials::user(1000, 1000);
    assert!(creds.can_access(1000, 1000, 0o600, Access::Read));
    assert!(creds.can_access(1000, 1000, 0o600, Access::Write));
    assert!(!creds.can_access(1000, 1000, 0o600, Access::Execute));
}

#[test]
fn test_permission_check_group() {
    let creds = Credentials::user(1000, 1000);
    assert!(creds.can_access(0, 1000, 0o060, Access::Read));
    assert!(!creds.can_access(0, 2000, 0o060, Access::Read));
}

#[test]
fn test_permission_check_other() {
    let creds = Credentials::user(1000, 1000);
    assert!(creds.can_access(0, 0, 0o004, Access::Read));
    assert!(!creds.can_access(0, 0, 0o004, Access::Write));
}

#[test]
fn test_root_bypasses_all() {
    let creds = Credentials::root();
    assert!(creds.can_access(1000, 1000, 0o000, Access::Read));
    assert!(creds.can_access(1000, 1000, 0o000, Access::Write));
    assert!(creds.can_access(1000, 1000, 0o000, Access::Execute));
}
```

### Integration Tests

```bash
# Test permission denied
$ touch /root-only
$ chmod 600 /root-only
$ su user -c "cat /root-only"
cat: /root-only: Permission denied

# Test setuid
$ cat > /tmp/whoami.c << 'EOF'
#include <stdio.h>
#include <unistd.h>
int main() { printf("uid=%d euid=%d\n", getuid(), geteuid()); }
EOF
$ gcc -o /tmp/whoami /tmp/whoami.c
$ chmod u+s /tmp/whoami
$ chown root /tmp/whoami
$ su user -c "/tmp/whoami"
uid=1000 euid=0
```

---

## Implementation Order

1. Add Credentials struct to task
2. Add uid/gid/mode to all inode types
3. Implement getuid/geteuid/getgid/getegid
4. Implement permission checks in VFS open()
5. Implement chmod/chown/umask
6. Implement setuid/setgid syscalls
7. Handle setuid bit in exec
8. Parse /etc/passwd and /etc/group
9. Implement access() syscall
10. Add login utility (optional for M1)

---

## Dependencies

| Dependency | Status | Notes |
|------------|--------|-------|
| Task struct | ✅ Ready | Add credentials field |
| VFS layer | ✅ Ready | Add permission hooks |
| ext4 read | ✅ Ready | Has uid/gid/mode |
| tmpfs | ✅ Ready | Add uid/gid/mode |
| fork/exec | Pending TEAM_400 | Credential inheritance |

---

## References

- `credentials(7)` - Linux credentials
- `capabilities(7)` - Linux capabilities
- POSIX.1-2017 Chapter 4 - File permissions
- `setuid(2)`, `setgid(2)` man pages
