# Phase 1: Discovery - Users, Authentication & Permissions

**TEAM_405**: Users, Authentication & Permissions
**Created**: 2026-01-10
**Status**: Discovery

---

## Problem Statement

LevitateOS currently has no concept of users or permissions. Everything runs as an implicit "root" with full access. A general-purpose Unix OS requires:

1. **User identity** - UIDs, GIDs, user accounts
2. **File permissions** - rwx bits, owner/group
3. **Process credentials** - real/effective/saved UIDs
4. **Authentication** - login, password verification
5. **Privilege management** - su, sudo, setuid binaries

---

## Current State Analysis

### What Exists

| Component | Status | Location |
|-----------|--------|----------|
| /etc/passwd | Stub file | initramfs (TEAM_401) |
| /etc/group | Stub file | initramfs (TEAM_401) |
| File metadata | No owner/group/mode | VFS layer |
| Process credentials | None | Task struct |
| Syscalls | No permission checks | syscall handlers |

### What's Missing

| Component | Impact |
|-----------|--------|
| UID/GID in task struct | Can't identify process owner |
| File mode bits | Can't enforce permissions |
| chmod/chown syscalls | Can't change permissions |
| setuid/setgid/setgroups | Can't change identity |
| getuid/geteuid/getgid/getegid | Can't query identity |
| Permission checks in VFS | All files accessible |
| Password hashing | Can't verify credentials |
| Login process | No authentication |

---

## Unix Permission Model

### Process Credentials

Every Unix process has:

```
Real UID (ruid)      - Who started the process
Effective UID (euid) - What permissions apply NOW
Saved UID (suid)     - Can restore to this UID
Filesystem UID (fsuid) - Used for file access (usually == euid)

Real GID (rgid)      - Primary group of starter
Effective GID (egid) - What group permissions apply
Saved GID (sgid)     - Can restore to this GID
Supplementary groups - Additional group memberships
```

### File Permissions

```
Mode bits: rwxrwxrwx (9 bits)
           ^^^        - Owner permissions
              ^^^     - Group permissions
                 ^^^  - Other permissions

Special bits:
  setuid (4000) - Execute as file owner
  setgid (2000) - Execute as file group
  sticky (1000) - Only owner can delete in directory
```

### Permission Check Algorithm

```
if (euid == 0):
    return ALLOW  # root bypasses everything

if (euid == file.owner):
    check owner bits
elif (egid == file.group or file.group in supplementary_groups):
    check group bits
else:
    check other bits
```

---

## Required Syscalls

### Identity Query (Read-only)

| Syscall | Number (x86_64) | Number (aarch64) | Purpose |
|---------|-----------------|------------------|---------|
| getuid | 102 | 174 | Get real UID |
| geteuid | 107 | 175 | Get effective UID |
| getgid | 104 | 176 | Get real GID |
| getegid | 108 | 177 | Get effective GID |
| getgroups | 115 | 80 | Get supplementary groups |

### Identity Change (Privileged)

| Syscall | Number (x86_64) | Number (aarch64) | Purpose |
|---------|-----------------|------------------|---------|
| setuid | 105 | 146 | Set UID (complex rules) |
| setgid | 106 | 144 | Set GID |
| setreuid | 113 | 145 | Set real and effective UID |
| setregid | 114 | 143 | Set real and effective GID |
| setresuid | 117 | 147 | Set real, effective, saved UID |
| setresgid | 119 | 149 | Set real, effective, saved GID |
| setgroups | 116 | 81 | Set supplementary groups |
| setfsuid | 122 | 151 | Set filesystem UID |
| setfsgid | 123 | 152 | Set filesystem GID |

### File Permission Syscalls

| Syscall | Number (x86_64) | Number (aarch64) | Purpose |
|---------|-----------------|------------------|---------|
| chmod | 90 | 52 | Change file mode |
| fchmod | 91 | 52 | Change mode by fd |
| fchmodat | 268 | 53 | Change mode relative to dir |
| chown | 92 | 55 | Change owner/group |
| fchown | 93 | 55 | Change owner by fd |
| fchownat | 260 | 54 | Change owner relative to dir |
| access | 21 | 33 (faccessat) | Check permissions |
| faccessat | 269 | 48 | Check relative to dir |
| umask | 95 | 166 | Set file creation mask |

---

## User Database Files

### /etc/passwd

```
root:x:0:0:root:/root:/bin/sh
user:x:1000:1000:Default User:/home/user:/bin/sh
nobody:x:65534:65534:Nobody:/nonexistent:/bin/false
```

Format: `name:password:uid:gid:gecos:home:shell`

### /etc/shadow (Optional for M1)

```
root:$6$...:19000:0:99999:7:::
user:$6$...:19000:0:99999:7:::
```

Format: `name:hash:lastchange:min:max:warn:inactive:expire:`

### /etc/group

```
root:x:0:
users:x:100:user
wheel:x:10:root,user
```

Format: `name:password:gid:members`

---

## Implementation Scope

### Priority 0 - Kernel Infrastructure

| Component | Description |
|-----------|-------------|
| Task credentials | Add uid/gid/groups to task struct |
| VFS permission checks | Check mode bits on open/read/write/exec |
| Inode owner/group/mode | Store in all filesystems |
| Basic syscalls | getuid, geteuid, getgid, getegid |

### Priority 1 - File Operations

| Component | Description |
|-----------|-------------|
| chmod/chown | Change file metadata |
| umask | Default permissions for new files |
| access/faccessat | Permission checking |
| setuid bit handling | Execute as file owner |

### Priority 2 - Identity Management

| Component | Description |
|-----------|-------------|
| setuid/setgid | Change process identity |
| setgroups | Supplementary groups |
| Capability checks | Who can change identity |

### Priority 3 - Authentication

| Component | Description |
|-----------|-------------|
| Password parsing | Read /etc/shadow |
| Password hashing | SHA-512 crypt |
| login utility | Authenticate and spawn shell |
| su utility | Switch user |

---

## Filesystem Impact

### ext4 (Required for TEAM_402)

ext4 stores full Unix permissions:
- i_mode: 16-bit mode (type + permissions)
- i_uid: 32-bit owner UID
- i_gid: 32-bit group GID

Already supported in read path.

### tmpfs/devtmpfs

Need to add owner/group/mode to in-memory inodes.

### initramfs

CPIO format supports mode/uid/gid. Need to preserve during extraction.

---

## Design Considerations

### Root User (UID 0)

- Bypasses all permission checks
- Can change to any UID
- Can access any file
- Required for system administration

### Default User

- UID 1000 (standard first user)
- Home directory /home/user
- Member of wheel/sudo group for privilege escalation

### Nobody User

- UID 65534
- No privileges
- Used for sandboxing

### Capability-Based (Future)

Linux capabilities (CAP_CHOWN, CAP_SETUID, etc.) provide fine-grained privilege control. Consider for future but not M1.

---

## Security Considerations

### Password Storage

- NEVER store plaintext passwords
- Use SHA-512 crypt ($6$) like modern Linux
- Salt each password
- Consider: do we even need passwords for M1?

### Setuid Binaries

- Dangerous if mishandled
- Execute with file owner's privileges
- Must be on a filesystem that supports it

### Race Conditions

- TOCTOU (time-of-check to time-of-use)
- Check permissions at operation time, not open time
- Use fd-based operations (fchmod, fchown)

---

## Codebase Reconnaissance

### Task Struct Location

```
crates/kernel/src/task/mod.rs
```

Need to add:
```rust
pub struct Credentials {
    pub ruid: u32,
    pub euid: u32,
    pub suid: u32,
    pub rgid: u32,
    pub egid: u32,
    pub sgid: u32,
    pub groups: Vec<u32>,
    pub umask: u16,
}
```

### VFS Permission Check Points

```
crates/kernel/src/fs/vfs/operations.rs - open, read, write, etc.
crates/kernel/src/syscall/fs.rs - syscall handlers
```

### Inode Metadata

```
crates/kernel/src/fs/vfs/inode.rs - need uid/gid/mode
crates/kernel/src/fs/tmpfs/ - add to TmpfsInode
crates/kernel/src/fs/ext4/ - already has it
```

---

## Questions to Answer

1. **Password authentication in M1?** - Or just start as root?
2. **Shadow file support?** - Or inline passwords (insecure)?
3. **Supplementary groups limit?** - Linux allows 65536
4. **Capabilities?** - Full CAP_* system or just uid==0?
5. **ACLs?** - Extended permissions or just mode bits?
6. **SELinux/AppArmor?** - Mandatory access control?

---

## Dependencies

| Dependency | Status | Notes |
|------------|--------|-------|
| ext4 write | Pending | Stores permissions |
| VFS layer | ✅ Complete | Needs permission hooks |
| Task management | ✅ Complete | Needs credentials |
| /etc/passwd | ✅ TEAM_401 | Stub exists |
| fork/exec | Pending TEAM_400 | Credential inheritance |

---

## Success Criteria

- [ ] Processes have UID/GID credentials
- [ ] Files have owner/group/mode metadata
- [ ] Permission checks enforced on file access
- [ ] chmod/chown syscalls work
- [ ] Root (UID 0) can access everything
- [ ] Non-root cannot access restricted files
- [ ] setuid binaries execute with file owner's UID
- [ ] getuid/geteuid/getgid/getegid return correct values

---

## References

- Linux `credentials(7)` man page
- Linux `capabilities(7)` man page
- POSIX.1-2017 file permissions
- `crypt(3)` for password hashing
