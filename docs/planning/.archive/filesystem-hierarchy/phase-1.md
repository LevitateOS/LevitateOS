# Phase 1: Discovery - Filesystem Hierarchy Standard

**TEAM_401**: Filesystem Hierarchy Standard Compliance
**Created**: 2026-01-10
**Status**: Discovery Complete

---

## Feature Summary

### Problem Statement

LevitateOS currently uses a **flat initramfs structure**:
- All binaries in root directory (`/shell`, `/coreutils`, `/cat`, etc.)
- No standard `/bin`, `/etc`, `/usr` hierarchy
- No `/dev` filesystem (PTY hardcoded in syscalls)
- No `/proc` or `/sys` pseudo-filesystems
- Programs expecting standard paths will fail

This prevents compatibility with:
- Build systems expecting `/usr/bin/env`
- Scripts using `#!/bin/sh`
- Programs looking for config in `/etc/`
- Tools expecting `/dev/null`, `/dev/zero`

### Who Benefits

| User Type | Benefit |
|-----------|---------|
| End Users | Standard paths work as expected |
| Developers | Scripts and build systems just work |
| Package Porters | Less path modification needed |
| LevitateOS Project | Closer to "real" Unix |

### The Goal

**FHS Compliance** = Standard Unix paths resolve correctly.

**The Test**: `#!/usr/bin/env bash` works. `/dev/null` works. `/etc/` exists.

---

## Success Criteria

### Milestone 1: Basic FHS Structure

```bash
# Standard directories exist and contain expected content
ls /bin /sbin /etc /usr /var /tmp /dev /home

# Essential paths work
/bin/sh              # Shell exists
/usr/bin/env         # env exists
cat /etc/hostname    # Config exists
echo test > /dev/null # /dev/null works
```

**Acceptance Tests**:
- [ ] `/bin/` contains shell and essential utilities
- [ ] `/sbin/` contains system utilities
- [ ] `/etc/` contains hostname, passwd stubs
- [ ] `/usr/bin/` contains additional utilities
- [ ] `/dev/null`, `/dev/zero`, `/dev/urandom` work
- [ ] `/tmp/` is writable (already works)
- [ ] `/home/` directory exists

### Milestone 2: Pseudo-Filesystems (Future)

```bash
# /proc provides process info
cat /proc/self/cmdline
cat /proc/meminfo
ls /proc/1/

# /sys provides kernel info
cat /sys/kernel/version
```

**Acceptance Tests** (deferred):
- [ ] `/proc/self/` links to current process
- [ ] `/proc/{pid}/` directories exist
- [ ] `/sys/` exposes kernel parameters

---

## Current State Analysis

### What Works Today

| Feature | Status | Evidence |
|---------|--------|----------|
| VFS layer | ✅ Working | Full inode/dentry/superblock abstraction |
| Mount table | ✅ Working | Longest-prefix matching, overlay support |
| Initramfs | ✅ Working | CPIO-based, read-only |
| Tmpfs | ✅ Working | /tmp mounted read-write |
| Path resolution | ✅ Working | Normalizes `.`, `..`, handles symlinks |
| PTY devices | ✅ Working | /dev/ptmx, /dev/pts/{n} (hardcoded) |

### What's Missing

| Gap | Impact | Effort |
|-----|--------|--------|
| **Directory structure** | No /bin, /usr, etc. | Low |
| **devfs/devtmpfs** | No /dev filesystem | Medium |
| **Device nodes** | No /dev/null, /dev/zero | Medium |
| **/etc content** | No config files | Low |
| **procfs** | No /proc filesystem | High |
| **sysfs** | No /sys filesystem | High |
| **Symlink compatibility** | /bin -> /usr/bin? | Low |

### Filesystem Support Status

| Filesystem | Mount Point | Status | Notes |
|------------|-------------|--------|-------|
| Initramfs | `/` | ✅ Ready | Can restructure CPIO contents |
| Tmpfs | `/tmp` | ✅ Ready | Already working |
| Tmpfs | `/var/tmp` | 🔲 TODO | Needs mount |
| Devtmpfs | `/dev` | 🔲 TODO | New filesystem needed |
| Procfs | `/proc` | 🔲 TODO | New filesystem needed |
| Sysfs | `/sys` | 🔲 TODO | New filesystem needed |

---

## FHS Overview

### Standard Directory Purposes

| Directory | Purpose | LevitateOS Plan |
|-----------|---------|-----------------|
| `/bin` | Essential user commands | Symlinks to coreutils |
| `/sbin` | Essential system commands | Minimal for now |
| `/etc` | Configuration files | hostname, passwd stubs |
| `/dev` | Device files | devtmpfs with null, zero, urandom, tty, ptmx |
| `/home` | User home directories | /home/user |
| `/lib` | Essential libraries | Empty (static binaries only) |
| `/mnt` | Temporary mount point | Empty directory |
| `/opt` | Optional packages | Empty directory |
| `/proc` | Process information | procfs (deferred) |
| `/root` | Root home directory | Empty or minimal |
| `/run` | Runtime data | tmpfs mount |
| `/sys` | Kernel/device info | sysfs (deferred) |
| `/tmp` | Temporary files | tmpfs (already working) |
| `/usr` | Secondary hierarchy | /usr/bin, /usr/lib, /usr/share |
| `/var` | Variable data | /var/log, /var/tmp |

### Modern vs Traditional Layout

**Traditional (pre-2012)**:
```
/bin/         Essential binaries (sh, cat, ls)
/sbin/        System binaries (mount, fsck)
/usr/bin/     User binaries (gcc, vim)
/usr/sbin/    System admin binaries
```

**Modern (merged /usr)**:
```
/bin -> /usr/bin     (symlink)
/sbin -> /usr/sbin   (symlink)
/lib -> /usr/lib     (symlink)
```

**Recommendation**: Use merged /usr layout—modern standard, no legacy to maintain.

---

## Codebase Reconnaissance

### Files to Modify

| Component | Location | Changes Needed |
|-----------|----------|----------------|
| Initramfs build | `xtask/src/build/commands.rs` | Create directory structure |
| Mount init | `crates/kernel/src/fs/mount.rs` | Add devtmpfs, run mounts |
| Device syscalls | `crates/kernel/src/syscall/fs/` | Route /dev/* to devfs |
| New: devtmpfs | `crates/kernel/src/fs/devtmpfs/` | New filesystem |
| New: procfs | `crates/kernel/src/fs/procfs/` | New filesystem (deferred) |

### Existing Code to Leverage

| Component | Status | Reuse |
|-----------|--------|-------|
| Tmpfs | Complete | Model for devtmpfs |
| VFS layer | Complete | Standard integration |
| Mount table | Complete | Standard mounting |
| PTY handling | Complete | Move to devfs |

---

## Constraints

### Technical Constraints

1. **Initramfs is read-only** - Can't create directories at runtime in initramfs
2. **Static binaries only** - /lib will be empty (no shared libraries yet)
3. **Single-user** - /etc/passwd is stub only

### Resource Constraints

1. **Initramfs size** - Must stay reasonable (<50MB uncompressed)
2. **Memory for tmpfs mounts** - Each tmpfs uses RAM

### Design Constraints

1. **Rule 20: Simplicity** - Start with minimal FHS, expand as needed
2. **Compatibility** - Must not break existing programs

---

## Open Questions

| ID | Question | Impact |
|----|----------|--------|
| Q1 | Traditional vs merged /usr layout? | Directory structure |
| Q2 | devtmpfs or static /dev? | Device file implementation |
| Q3 | Which /dev nodes are essential? | Initial device set |
| Q4 | procfs scope for Milestone 1? | /proc implementation |

See `docs/questions/TEAM_401_filesystem_hierarchy.md` for resolution.

---

## References

- [Filesystem Hierarchy Standard 3.0](https://refspecs.linuxfoundation.org/FHS_3.0/fhs/index.html)
- Current VFS: `crates/kernel/src/fs/vfs/`
- Current tmpfs: `crates/kernel/src/fs/tmpfs/`
- Initramfs build: `xtask/src/build/commands.rs`
