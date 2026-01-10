# TEAM_401: Feature - Filesystem Hierarchy Standard

**Date**: 2026-01-10
**Status**: Design Complete - Ready for Implementation
**Type**: Feature

---

## Summary

Implement the Filesystem Hierarchy Standard (FHS) for LevitateOS, transitioning from a flat initramfs structure to a proper Unix directory hierarchy.

After this feature, LevitateOS will have standard paths like `/bin/sh`, `/etc/hostname`, and `/dev/null`.

---

## Goal

**Standard Unix paths work correctly.**

```bash
# Scripts work
#!/bin/sh
#!/usr/bin/env python

# Devices work
echo test > /dev/null
head -c 10 /dev/urandom | od -x

# Config exists
cat /etc/hostname
```

---

## Scope

### In Scope (Milestone 1)

| Component | Description |
|-----------|-------------|
| FHS directories | /bin, /sbin, /etc, /usr, /var, /home, /tmp |
| devtmpfs | Dynamic device filesystem at /dev |
| Standard devices | null, zero, urandom, tty, console, ptmx |
| /etc content | hostname, passwd, group, shells, profile |
| Mount sequence | devtmpfs, tmpfs mounts at boot |

### Out of Scope (Future Milestones)

| Component | Milestone |
|-----------|-----------|
| procfs (/proc) | Milestone 2 |
| sysfs (/sys) | Milestone 3 |
| Block devices | Milestone 3 |
| Dynamic hotplug | Future |

---

## Plan Documents

| Phase | Document | Status |
|-------|----------|--------|
| Discovery | `docs/planning/filesystem-hierarchy/phase-1.md` | Complete |
| Design | `docs/planning/filesystem-hierarchy/phase-2.md` | Complete |
| Implementation | `docs/planning/filesystem-hierarchy/phase-3.md` | Pending |
| Integration | `docs/planning/filesystem-hierarchy/phase-4.md` | Pending |
| Polish | `docs/planning/filesystem-hierarchy/phase-5.md` | Pending |

---

## Resolved Design Questions

See `docs/questions/TEAM_401_filesystem_hierarchy.md` for full rationales.

### Critical Questions (Resolved)

| ID | Question | Decision | Rationale |
|----|----------|----------|-----------|
| Q1 | Traditional vs merged /usr? | **B) Merged /usr** | Start modern—every major distro has moved to merged |
| Q2 | devtmpfs or static /dev? | **B) devtmpfs** | PTY requires dynamic device creation |
| Q3 | Essential /dev nodes? | **C) Extended (13+)** | /dev/stdin, /dev/stdout, /dev/stderr widely used |

### Important Questions (Resolved)

| ID | Question | Decision | Rationale |
|----|----------|----------|-----------|
| Q4 | procfs scope? | **B) Minimal /proc/self** | /proc/self/exe needed—Rust std::env::current_exe() uses it |
| Q5 | /etc content scope? | **B) Standard (5)** | Functional: getpwuid/getgrgid need passwd/group |
| Q6 | Binary location? | **A) /usr/bin** | Follows from merged /usr—/bin is symlink |

### Nice to Have (Resolved)

| ID | Question | Decision | Rationale |
|----|----------|----------|-----------|
| Q7 | /var structure? | **B) Standard** | /var/run symlink for compatibility |
| Q8 | Random quality? | **B) RDRAND** | One instruction on x86_64, vastly better quality |

---

## Implementation Priority

| Priority | Component | Effort | Blocker |
|----------|-----------|--------|---------|
| P0 | Initramfs FHS structure | Low | Build system |
| P0 | devtmpfs filesystem | Medium | VFS |
| P1 | Device operations | Medium | devtmpfs |
| P1 | Boot mount sequence | Low | devtmpfs |
| P2 | PTY migration | Medium | devtmpfs |
| P2 | /etc files | Low | Build system |

---

## Success Criteria

- [ ] FHS directories exist (/bin, /sbin, /etc, /usr, /var, /home, /tmp)
- [ ] `/bin/sh` exists and works
- [ ] `/usr/bin/env` exists
- [ ] `/dev/null` accepts writes, returns EOF on read
- [ ] `/dev/zero` returns zeros on read
- [ ] `/dev/urandom` returns random bytes
- [ ] `/etc/hostname` contains "levitate"
- [ ] PTY works via `/dev/ptmx`
- [ ] All tests pass

---

## Dependencies

| Dependency | Status | Notes |
|------------|--------|-------|
| VFS layer | ✅ Complete | For devtmpfs integration |
| Tmpfs | ✅ Complete | Model for devtmpfs |
| Mount table | ✅ Complete | For multiple mounts |
| PTY subsystem | ✅ Complete | Migration to devfs |

---

## Related Teams

| Team | Relation |
|------|----------|
| TEAM_202 | VFS core implementation |
| TEAM_194 | Tmpfs design |
| TEAM_247 | PTY implementation |
| TEAM_400 | General Purpose OS (depends on FHS) |

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| PTY regression | Medium | High | Thorough testing |
| Build system complexity | Low | Medium | Incremental changes |
| Path compatibility | Low | Medium | Add symlinks |

---

## Implementation Order

1. Create FHS directories in initramfs build
2. Move binaries to /usr/bin, create /bin symlinks
3. Add /etc configuration files
4. Implement devtmpfs filesystem
5. Add device operations (null, zero, urandom)
6. Update boot mount sequence
7. Migrate PTY to devtmpfs
8. Integration testing

---

## Notes

- This feature is a prerequisite for full TEAM_400 (General Purpose OS)
- Minimal /proc/self included; full procfs and sysfs are future enhancements
- devtmpfs can be modeled closely after existing tmpfs code
- PTY already works; this just formalizes it into the filesystem
