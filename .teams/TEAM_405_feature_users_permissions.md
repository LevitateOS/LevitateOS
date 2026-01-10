# TEAM_405: Feature - Users, Authentication & Permissions

**Date**: 2026-01-10
**Status**: Design Complete - Ready for Implementation
**Type**: Feature

---

## Summary

Implement Unix user/group identity, file permissions, and authentication for LevitateOS.

After this feature, files have owners and permissions, processes run as specific users, and unauthorized access is denied.

---

## Goal

**Real Unix permissions work correctly.**

```bash
# File permissions enforced
$ chmod 600 /secret
$ su user -c "cat /secret"
cat: /secret: Permission denied

# User identity works
$ id
uid=1000(user) gid=1000(user) groups=1000(user),10(wheel)

# setuid works
$ ls -l /bin/su
-rwsr-xr-x 1 root root ... /bin/su
```

---

## Scope

### In Scope

| Component | Description |
|-----------|-------------|
| Process credentials | UID/GID/groups in task struct |
| File permissions | Mode bits (rwxrwxrwx) |
| Permission enforcement | VFS checks on open/read/write/exec |
| Identity syscalls | getuid, setuid, getgroups, etc. |
| File syscalls | chmod, chown, umask, access |
| setuid/setgid | Execute as file owner |
| /etc/passwd parsing | User database |
| /etc/shadow parsing | Password hashes |
| /etc/group parsing | Group database |
| su utility | Switch user |

### Out of Scope (Future)

| Component | Notes |
|-----------|-------|
| Linux capabilities | CAP_* system |
| POSIX ACLs | Extended permissions |
| SELinux/AppArmor | Mandatory access control |
| PAM | Pluggable authentication |
| NSS | Name service switch |
| sudo | Complex config, su is enough |

---

## Plan Documents

| Phase | Document | Status |
|-------|----------|--------|
| Discovery | `docs/planning/users-permissions/phase-1.md` | Complete |
| Design | `docs/planning/users-permissions/phase-2.md` | Complete |
| Implementation | `docs/planning/users-permissions/phase-3.md` | Pending |
| Integration | `docs/planning/users-permissions/phase-4.md` | Pending |
| Polish | `docs/planning/users-permissions/phase-5.md` | Pending |

---

## Resolved Design Questions

See `docs/questions/TEAM_405_users_permissions.md` for full rationales.

| ID | Question | Decision | Rationale |
|----|----------|----------|-----------|
| Q1 | Password auth | **Shadow file** | Real auth from start, not retrofitting security later |
| Q2 | Boot identity | **Root (UID 0)** | Init needs full privileges for system tasks |
| Q3 | Capabilities | **Traditional UID 0** | 40+ capability bits are complex; add later if needed |
| Q4 | Default user | **"user" (UID 1000)** | Standard first-user UID, generic name |
| Q5 | Supplementary groups | **Full implementation** | Programs expect getgroups(); trivial cost |
| Q6 | Privilege escalation | **su only** | sudo's /etc/sudoers is complex; su covers 90% of needs |
| Q7 | ACL support | **No** | Mode bits cover 99% of cases |
| Q8 | NSS/PAM | **No** | Enterprise abstractions; direct file parsing is fine |
| Q9 | Login sessions | **utmp/wtmp** | Standard Unix format; who/w/last expect it |

---

## Implementation Priority

| Priority | Component | Effort |
|----------|-----------|--------|
| P0 | Credentials in task struct | Low |
| P0 | uid/gid/mode in inodes | Medium |
| P0 | Permission checks in VFS | Medium |
| P1 | getuid/geteuid/getgid/getegid | Low |
| P1 | chmod/chown/umask | Medium |
| P1 | setuid/setgid bit in exec | Medium |
| P2 | setuid/setgid/setgroups syscalls | Medium |
| P2 | /etc/passwd,shadow,group parsing | Medium |
| P3 | su utility | Medium |
| P3 | login utility | Medium |

---

## Success Criteria

- [ ] Processes have UID/GID credentials
- [ ] Files have owner/group/mode metadata
- [ ] Permission checks enforced on file operations
- [ ] chmod/chown syscalls work
- [ ] Root (UID 0) bypasses all permission checks
- [ ] Non-root cannot access mode 0600 files owned by others
- [ ] setuid binaries execute with owner's effective UID
- [ ] getuid/geteuid return correct values
- [ ] /etc/passwd parsed correctly
- [ ] su changes effective user

---

## Dependencies

| Dependency | Team | Status | Required For |
|------------|------|--------|--------------|
| VFS layer | - | ✅ Ready | Permission hooks |
| Task struct | - | ✅ Ready | Credentials |
| fork/exec | TEAM_400 | Planning | Credential inheritance |
| ext4 write | TEAM_402 | Planning | Store permissions |
| /etc files | TEAM_401 | Planning | User database |

---

## Related Teams

| Team | Relation |
|------|----------|
| TEAM_400 | General Purpose OS (fork/exec) |
| TEAM_401 | FHS (/etc files) |
| TEAM_402 | Disk root (ext4 stores perms) |

---

## Required Syscalls

### Identity Query
- getuid (102/174)
- geteuid (107/175)
- getgid (104/176)
- getegid (108/177)
- getgroups (115/80)

### Identity Change
- setuid (105/146)
- setgid (106/144)
- setreuid (113/145)
- setregid (114/143)
- setresuid (117/147)
- setresgid (119/149)
- setgroups (116/81)

### File Permissions
- chmod (90/52)
- fchmod (91/52)
- fchmodat (268/53)
- chown (92/55)
- fchown (93/55)
- fchownat (260/54)
- access (21/faccessat)
- faccessat (269/48)
- umask (95/166)

---

## Security Notes

- Passwords stored as SHA-512 crypt hashes in /etc/shadow
- /etc/shadow must be mode 0600, owned by root
- setuid/setgid bits cleared on chown (security)
- Sticky bit on directories: only owner can delete files
- Root bypasses ALL permission checks (euid == 0)

---

## Implementation Order

1. Add Credentials struct to task
2. Add uid/gid/mode to tmpfs inodes
3. Add permission checks in VFS open()
4. Implement getuid/geteuid/getgid/getegid
5. Implement chmod/chown/umask
6. Handle setuid bit in exec
7. Implement setuid/setgid syscalls
8. Parse /etc/passwd and /etc/group
9. Implement su utility
10. Add /etc/shadow and login (optional)

---

## Notes

- This is fundamental infrastructure for a real multi-user OS
- Must work correctly before any real deployment
- setuid handling is security-critical—get it right
- Test permission denied cases thoroughly
