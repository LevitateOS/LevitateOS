# Phase 4: Integration - Users, Authentication & Permissions

**TEAM_405**: Users, Authentication & Permissions
**Created**: 2026-01-10
**Status**: Pending

---

## Integration Tests

### Test 1: Basic Permission Denial

```bash
# Setup
touch /test-file
chmod 600 /test-file
chown root:root /test-file

# Test as non-root
su user -c "cat /test-file"
# Expected: cat: /test-file: Permission denied

su user -c "echo x > /test-file"
# Expected: Permission denied
```

### Test 2: Group Permission

```bash
# Setup
touch /group-file
chmod 060 /group-file
chown root:users /group-file

# Test user in group
su user -c "cat /group-file"
# Expected: Success (user is in users group)

# Test user not in group
su nobody -c "cat /group-file"
# Expected: Permission denied
```

### Test 3: Other Permission

```bash
# Setup
touch /world-readable
chmod 004 /world-readable

# Test
su nobody -c "cat /world-readable"
# Expected: Success
```

### Test 4: setuid Binary

```bash
# Create setuid test program
cat > /tmp/showuid.c << 'EOF'
#include <stdio.h>
#include <unistd.h>
int main() {
    printf("ruid=%d euid=%d\n", getuid(), geteuid());
    return 0;
}
EOF

# Compile and set setuid
gcc -o /bin/showuid /tmp/showuid.c
chmod u+s /bin/showuid
chown root /bin/showuid

# Test
su user -c "/bin/showuid"
# Expected: ruid=1000 euid=0
```

### Test 5: Directory Permissions

```bash
# Setup
mkdir /restricted
chmod 700 /restricted
touch /restricted/file

# Test
su user -c "ls /restricted"
# Expected: Permission denied

su user -c "touch /restricted/newfile"
# Expected: Permission denied
```

### Test 6: Sticky Bit

```bash
# Setup
mkdir /tmp/sticky
chmod 1777 /tmp/sticky
touch /tmp/sticky/root-file
chown root /tmp/sticky/root-file

# Test - user can't delete root's file
su user -c "rm /tmp/sticky/root-file"
# Expected: Permission denied

# But can delete own files
su user -c "touch /tmp/sticky/user-file"
su user -c "rm /tmp/sticky/user-file"
# Expected: Success
```

### Test 7: umask

```bash
# Test default umask
umask
# Expected: 0022

# Test file creation with umask
umask 077
touch /tmp/umask-test
ls -l /tmp/umask-test
# Expected: -rw------- (600)
```

### Test 8: Identity Syscalls

```bash
# Test getuid/geteuid
id
# Expected: uid=0(root) gid=0(root) groups=0(root)

su user -c "id"
# Expected: uid=1000(user) gid=1000(user) groups=1000(user),10(wheel)
```

### Test 9: setuid Syscall Rules

```bash
# Root can setuid to anyone
su -c "id"  # as user
# Expected: uid=1000

# Non-root can only setuid to saved uid
# (Tested via setuid binary that drops and tries to regain)
```

### Test 10: chown Clears setuid

```bash
# Setup
cp /bin/showuid /tmp/test-setuid
chmod u+s /tmp/test-setuid
ls -l /tmp/test-setuid
# Expected: -rwsr-xr-x

# chown should clear setuid
chown user /tmp/test-setuid
ls -l /tmp/test-setuid
# Expected: -rwxr-xr-x (no 's')
```

---

## Syscall Test Matrix

| Syscall | Test | Expected |
|---------|------|----------|
| getuid | id command | Correct ruid |
| geteuid | setuid binary | Shows euid=0 |
| getgid | id command | Correct rgid |
| getegid | setgid binary | Shows egid |
| getgroups | id command | All groups |
| setuid (root) | su to user | Changes all uids |
| setuid (user) | Limited | Only to saved uid |
| setgid | Similar to setuid | Works |
| chmod | chmod 755 | Mode changes |
| chown | chown user:group | Owner changes |
| umask | umask 077 | Affects new files |
| access | test -r | Checks real uid |

---

## Regression Tests

### Existing Functionality

- [ ] fork() inherits credentials correctly
- [ ] exec() checks execute permission
- [ ] exec() applies setuid/setgid bits
- [ ] open() checks read/write permissions
- [ ] VFS operations all check permissions
- [ ] Root bypasses all checks

### Edge Cases

- [ ] UID 0 always succeeds
- [ ] GID match via supplementary groups
- [ ] Sticky bit on directories
- [ ] setgid on directories (new files inherit group)
- [ ] umask applied to mkdir too
- [ ] chown clears setuid/setgid

---

## Performance Tests

| Operation | Target | Notes |
|-----------|--------|-------|
| Permission check | <1µs | Hot path |
| getuid | <100ns | Simple read |
| setuid | <1µs | Privilege check + update |
| open with check | <5µs overhead | Compared to no-check |

---

## Security Audit Checklist

- [ ] No permission bypass for non-root
- [ ] setuid bit cannot be set by non-owner
- [ ] chown requires root for owner change
- [ ] Sticky bit prevents unauthorized deletion
- [ ] Supplementary groups checked correctly
- [ ] fsuid used for file access, not euid
- [ ] Saved uid logic matches POSIX
- [ ] No TOCTOU in permission checks
