# Phase 5: Polish - Users, Authentication & Permissions

**TEAM_405**: Users, Authentication & Permissions
**Created**: 2026-01-10
**Status**: Pending

---

## Documentation

### Man Pages

| Page | Description |
|------|-------------|
| chmod(1) | Change file mode |
| chown(1) | Change file owner |
| id(1) | Print user identity |
| su(1) | Switch user |
| passwd(5) | Password file format |
| group(5) | Group file format |
| shadow(5) | Shadow password file |

### Error Messages

| Error | Message |
|-------|---------|
| EACCES | "Permission denied" |
| EPERM | "Operation not permitted" |

Clear distinction:
- EACCES: File permissions block access
- EPERM: Not allowed to perform operation (e.g., non-root chown)

---

## Default Configuration

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

### /etc/shadow

```
root:$6$rounds=5000$salt$hash:19000:0:99999:7:::
user:$6$rounds=5000$salt$hash:19000:0:99999:7:::
```

---

## Utilities Included

| Utility | Purpose |
|---------|---------|
| chmod | Change file permissions |
| chown | Change file owner/group |
| chgrp | Change file group |
| id | Show user/group info |
| whoami | Print effective username |
| groups | Print group memberships |
| su | Switch user identity |
| login | Authenticate user (optional) |

---

## Known Limitations

| Limitation | Impact | Future Work |
|------------|--------|-------------|
| No capabilities | Root is all-or-nothing | Add CAP_* system |
| No ACLs | Mode bits only | Add POSIX ACLs |
| No PAM | Direct file auth only | Add PAM support |
| No sudo | su only | Add sudo |
| No user management | Manual /etc editing | Add useradd/userdel |

---

## Behavior Documentation

### Permission Check Order

1. If euid == 0: ALLOW
2. If fsuid == file owner: check owner bits
3. If fsgid == file group OR file group in supplementary: check group bits
4. Check other bits

### setuid Rules (POSIX)

**Root (euid == 0)**:
- setuid(X) sets ruid, euid, suid all to X

**Non-root**:
- setuid(X) only allowed if X == ruid or X == suid
- Only sets euid (and fsuid)

### Credential Inheritance

- fork(): Child gets exact copy of parent credentials
- exec(): Credentials preserved, except setuid/setgid bits may change euid/egid
- exec() on setuid: suid set to new euid

---

## Future Enhancements

### Capabilities (TEAM_406?)

```rust
pub struct Capabilities {
    effective: u64,
    permitted: u64,
    inheritable: u64,
}

const CAP_CHOWN: u64 = 1 << 0;
const CAP_SETUID: u64 = 1 << 7;
const CAP_NET_BIND_SERVICE: u64 = 1 << 10;
// ... 40+ capabilities
```

### POSIX ACLs (TEAM_407?)

```
# setfacl -m u:user:rw file
# getfacl file
user::rw-
user:user:rw-
group::r--
mask::rw-
other::r--
```

### User Management (TEAM_408?)

```bash
useradd -m -s /bin/sh newuser
userdel -r olduser
passwd username
groupadd developers
usermod -aG developers user
```

---

## Success Metrics

| Metric | Target |
|--------|--------|
| Permission checks | 100% enforced |
| setuid binaries | Work correctly |
| Multi-user support | Users isolated |
| Root privilege | Complete access |
| Test coverage | All syscalls tested |

---

## Sign-off Criteria

- [ ] All syscalls implemented and tested
- [ ] Permission enforcement complete
- [ ] su utility works
- [ ] /etc/passwd,group parsed correctly
- [ ] setuid/setgid bits work
- [ ] Documentation complete
- [ ] No permission bypasses found
- [ ] Integration tests pass
