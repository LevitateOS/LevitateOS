# Open Questions: Users, Authentication & Permissions (TEAM_405)

**Created**: 2026-01-10
**Status**: All Questions Answered
**Plan**: `docs/planning/users-permissions/`

---

## Critical Questions (Block Implementation)

### Q1: Password Authentication Scope

**Question**: How should user authentication work?

**Options**:
- **A) No passwords for M1**
  - Auto-login as root
  - Single-user development mode
  - Add passwords later

- **B) Shadow file from start** (Recommended)
  - /etc/shadow with SHA-512 hashed passwords
  - Proper multi-user support
  - login utility required
  - Standard Unix behavior

- **C) Simple plaintext (dev only)**
  - Passwords in /etc/passwd
  - Insecure but simple
  - NOT for any real use

**Recommendation**: B) Shadow file—we're building a real OS, not a toy.

**Answer**: **B) Shadow file from start**

**Rationale**: A general-purpose OS needs real authentication. Plaintext passwords are unacceptable. No-password mode is fine for single-user development but won't scale. SHA-512 crypt is the Linux standard. Implementing it now means we don't retrofit security later.

---

### Q2: Default Boot Identity

**Question**: What identity should init start with?

**Options**:
- **A) Root (UID 0)** (Recommended)
  - Standard Unix behavior
  - Init needs full privileges
  - Login spawns user sessions

- **B) Unprivileged user**
  - More secure by default
  - But init can't do system tasks
  - Non-standard

**Recommendation**: A) Root—init needs to mount filesystems, start services, etc.

**Answer**: **A) Root (UID 0)**

**Rationale**: Standard Unix behavior. Init must mount filesystems, create device nodes, start daemons—all privileged operations. Starting unprivileged would require immediate privilege escalation anyway.

---

### Q3: Capabilities vs Traditional Root

**Question**: Should we implement Linux capabilities?

**Options**:
- **A) Traditional UID 0 only** (Recommended for M1)
  - euid==0 has all privileges
  - Simple to implement
  - Matches classic Unix

- **B) Full capabilities system**
  - CAP_CHOWN, CAP_SETUID, CAP_NET_ADMIN, etc.
  - Fine-grained privilege control
  - Significant complexity
  - ~40 capability bits to track

- **C) Subset of capabilities**
  - Just the critical ones (CAP_SETUID, CAP_CHOWN)
  - Middle ground

**Recommendation**: A) Traditional—capabilities are complex and can be added later without breaking compatibility.

**Answer**: **A) Traditional UID 0 only**

**Rationale**: Linux capabilities are 40+ bits with complex inheritance rules (effective, permitted, inheritable, ambient, bounding sets). The traditional model (euid==0 has all power) covers all use cases. Capabilities can be added later—they're additive and don't break existing binaries.

---

## Important Questions (Should Answer Before Implementation)

### Q4: Default User Account

**Question**: What non-root user should exist by default?

**Options**:
- **A) "user" (UID 1000)**
  - Standard first-user UID
  - /home/user
  - Simple name

- **B) "levitate" (UID 1000)**
  - OS-branded name
  - /home/levitate
  - More distinctive

- **C) No default user**
  - Only root exists
  - User creates accounts manually
  - Minimal

**Recommendation**: A) "user"—generic, standard UID, easy to remember.

**Answer**: **A) "user" (UID 1000)**

**Rationale**: UID 1000 is the de facto standard for the first human user on Linux systems. Generic name "user" is memorable. Branded names feel forced. Users can rename or add accounts as needed.

---

### Q5: Supplementary Groups Implementation

**Question**: How should supplementary groups be handled?

**Options**:
- **A) Full implementation** (Recommended)
  - Parse /etc/group
  - setgroups() syscall
  - NGROUPS_MAX = 65536

- **B) Primary group only**
  - Skip supplementary groups
  - Simpler but less compatible
  - Many programs expect groups

**Recommendation**: A) Full implementation—programs expect getgroups() to work.

**Answer**: **A) Full implementation**

**Rationale**: Supplementary groups are how Unix handles multiple group memberships. Programs use getgroups() and check group membership for access control. Skipping this breaks su, sudo, and any group-based permissions. The implementation cost is a Vec<u32> per task—trivial.

---

### Q6: Privilege Escalation Utilities

**Question**: What privilege escalation should be included?

**Options**:
- **A) su only** (Recommended for M1)
  - Classic Unix utility
  - Switch to any user
  - Requires target user's password

- **B) sudo from start**
  - Modern privilege escalation
  - Complex config (/etc/sudoers)
  - More flexible

- **C) Both su and sudo**
  - Maximum compatibility
  - More utilities to implement

- **D) Neither**
  - Login/logout only
  - No runtime privilege change

**Recommendation**: A) su only—simpler, covers basic needs. sudo can come later.

**Answer**: **A) su only**

**Rationale**: su is the classic Unix privilege escalation: authenticate as target user, spawn shell. sudo requires /etc/sudoers parsing, complex permission matching, and per-command authorization. su covers "become root" and "become user" which is 90% of the need. Add sudo when we need fine-grained "user X can run command Y".

---

## Nice to Have Questions (Can Defer)

### Q7: ACL Support

**Question**: Should extended ACLs be supported?

**Options**:
- **A) No ACLs** (Recommended for M1)
  - Traditional mode bits only
  - Simpler implementation
  - Compatible with all programs

- **B) POSIX ACLs**
  - getfacl/setfacl
  - Fine-grained permissions
  - Requires xattr support
  - Complex

**Recommendation**: A) No ACLs—mode bits cover 99% of use cases.

**Answer**: **A) No ACLs**

**Rationale**: Traditional Unix mode bits (owner/group/other × read/write/execute) handle almost all real-world permission needs. ACLs require extended attributes (xattr) support in all filesystems, getfacl/setfacl utilities, and complex permission merging logic. Add when someone has a use case that mode bits can't handle.

---

### Q8: NSS/PAM Integration

**Question**: Should we support pluggable authentication?

**Options**:
- **A) No** (Recommended for M1)
  - Direct file parsing only
  - /etc/passwd, /etc/shadow, /etc/group
  - Simple and sufficient

- **B) Basic NSS**
  - Name Service Switch
  - Support for LDAP, NIS later
  - Complex

- **C) Full PAM**
  - Pluggable Authentication Modules
  - Maximum flexibility
  - Very complex

**Recommendation**: A) No—NSS/PAM are complex infrastructure. Direct file parsing is fine.

**Answer**: **A) No NSS/PAM**

**Rationale**: NSS (Name Service Switch) and PAM (Pluggable Authentication Modules) are abstraction layers for enterprise environments (LDAP, Kerberos, NIS). We're building a standalone OS that reads /etc/passwd directly. When we need LDAP integration, we can add NSS. PAM is even more complex with module stacking and conversation APIs.

---

### Q9: Login Session Management

**Question**: How should login sessions be tracked?

**Options**:
- **A) utmp/wtmp files** (Recommended)
  - Traditional Unix login records
  - /var/run/utmp, /var/log/wtmp
  - who/w/last commands use these

- **B) systemd-logind style**
  - D-Bus based
  - Modern but complex
  - Overkill for M1

- **C) No tracking**
  - No login records
  - Simpler
  - Less visibility

**Recommendation**: A) utmp/wtmp—standard, simple, expected by tools.

**Answer**: **A) utmp/wtmp files**

**Rationale**: Traditional Unix login tracking. who, w, last, users commands all read these files. The format is a fixed-size struct per entry—simple to implement. systemd-logind is D-Bus based and requires a service manager. No tracking means no audit trail of logins.

---

## Summary

| Q | Question | Recommendation |
|---|----------|----------------|
| Q1 | Password auth scope | Shadow file from start |
| Q2 | Boot identity | Root (UID 0) |
| Q3 | Capabilities | Traditional UID 0 only |
| Q4 | Default user | "user" (UID 1000) |
| Q5 | Supplementary groups | Full implementation |
| Q6 | Privilege escalation | su only |
| Q7 | ACL support | No ACLs |
| Q8 | NSS/PAM | No |
| Q9 | Login sessions | utmp/wtmp |

---

## Dependencies

| Question | Depends On | Notes |
|----------|------------|-------|
| Q1 (shadow) | VFS read | Need to read /etc/shadow |
| Q2 (init) | TEAM_400 | Init process creation |
| Q6 (su) | Q1 | Password verification |
| Q9 (utmp) | TEAM_401 | /var/run exists |
