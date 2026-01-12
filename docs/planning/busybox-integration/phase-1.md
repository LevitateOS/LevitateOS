# Phase 1: Discovery - BusyBox Integration

**Feature:** Replace uutils-coreutils, dash, and custom init with BusyBox  
**Team:** TEAM_449  
**Status:** Complete

---

## 1. Feature Summary

### Problem Statement

LevitateOS currently uses multiple separate components for userspace utilities:
- **uutils-coreutils** (Rust): Only 8 utilities enabled due to musl issues
- **dash** (C): Separate shell binary
- **Custom init** (Rust): Hand-written init process

This creates:
- Complex build system (Rust + C toolchains)
- Limited utility coverage (only 8 coreutils)
- Maintenance burden across multiple codebases
- Large combined binary size (~2.2MB for limited functionality)

### Solution

Replace all three with **BusyBox** - a single ~1MB binary that provides:
- 300+ utilities (coreutils, text processing, system tools)
- Built-in shell (ash - POSIX compliant)
- Built-in init system
- Built-in editor (vi)

### Who Benefits

- **Users**: More utilities available (grep, sed, awk, find, tar, etc.)
- **Developers**: Simpler build, single codebase to understand
- **Future teams**: Standard Linux tooling, well-documented

---

## 2. Success Criteria

### Acceptance Criteria

1. [ ] BusyBox binary builds successfully with musl (static)
2. [ ] BusyBox init starts and spawns shell
3. [ ] Shell (ash) works interactively with job control
4. [ ] All current utilities work: cat, echo, ls, pwd, mkdir, rm, touch
5. [ ] Additional utilities work: grep, sed, find, vi, tar
6. [ ] Boot time is not significantly degraded
7. [ ] Binary size is ≤1.5MB (vs current ~2.2MB)
8. [ ] Golden boot logs updated and passing

### Definition of Done

- BusyBox is the ONLY userspace binary in initramfs (plus symlinks)
- Custom Rust init removed
- uutils-coreutils removed from build
- dash removed from build
- Documentation updated

---

## 3. Current State Analysis

### Current Architecture

```
Kernel boots
    ↓
Spawns /init (Rust binary, ~150KB)
    ↓
init spawns /dash (C binary, ~150KB)
    ↓
dash uses /cat, /ls, /pwd... (symlinks to coreutils, ~2MB)
```

**Total: 3 separate builds, ~2.3MB**

### Current Utilities Available

From uutils-coreutils (8 only):
- cat, echo, head, mkdir, pwd, rm, tail, touch

From dash:
- sh (shell)

**Missing common utilities:**
- grep, sed, awk, find, sort, uniq, wc, cut
- tar, gzip, gunzip
- vi (editor)
- ps, kill, mount, umount
- wget, nc (networking)

### Current Build Process

1. `cargo xtask build coreutils` - Builds uutils with Rust/musl
2. `cargo xtask build dash` - Builds dash with musl-gcc
3. `cargo xtask build userspace` - Builds Rust init
4. Initramfs combines all three

---

## 4. Codebase Reconnaissance

### Files to Modify

| File | Change |
|------|--------|
| `xtask/src/build/apps.rs` | Remove coreutils entry |
| `xtask/src/build/c_apps.rs` | Add BusyBox, remove dash |
| `xtask/src/build/mod.rs` | Update exports |
| `scripts/make_initramfs.sh` | Use BusyBox + symlinks |
| `crates/userspace/init/` | **DELETE** (BusyBox init replaces) |

### Files to Create

| File | Purpose |
|------|---------|
| `xtask/src/build/busybox.rs` | BusyBox build logic |
| `toolchain/busybox/` | BusyBox source (git clone) |
| `toolchain/busybox-config` | Custom BusyBox .config |

### Tests Impacted

| Test | Impact |
|------|--------|
| `tests/golden_boot.txt` | Init messages will change |
| `tests/golden_boot_x86_64.txt` | Init messages will change |
| Behavior tests | May need updates for new output format |

### APIs Involved

- Kernel syscalls (no changes needed - BusyBox uses standard POSIX)
- `spawn` / `execve` (already working)
- TTY/termios (already working)

---

## 5. Constraints

### Technical Constraints

1. **Static linking required** - No dynamic linker in LevitateOS
2. **musl libc** - Must build with musl, not glibc
3. **x86_64 first** - Primary target; aarch64 later
4. **No kernel changes** - BusyBox must work with current syscalls

### BusyBox Configuration Constraints

Must disable features that don't work with musl or LevitateOS:
- CONFIG_SELINUX (no SELinux)
- CONFIG_PAM (no PAM)
- CONFIG_FEATURE_HAVE_RPC (no RPC)
- CONFIG_FEATURE_SYSTEMD (no systemd)
- CONFIG_FEATURE_MOUNT_NFS (no NFS)

### Performance Constraints

- Boot time should not increase significantly
- Memory usage should not increase significantly

---

## 6. BusyBox Capabilities to Enable

### Priority 0 - Must Have (Core Functionality)

| Category | Applets |
|----------|---------|
| **Init** | init, halt, poweroff, reboot |
| **Shell** | ash, sh |
| **Coreutils** | cat, cp, echo, ls, mkdir, mv, pwd, rm, rmdir, touch, ln, chmod |
| **File Info** | stat, file, head, tail, wc |

### Priority 1 - Should Have (Developer Tools)

| Category | Applets |
|----------|---------|
| **Text Processing** | grep, sed, awk, sort, uniq, cut, tr |
| **File Search** | find, xargs |
| **Archives** | tar, gzip, gunzip |
| **Editor** | vi |

### Priority 2 - Nice to Have (System Tools)

| Category | Applets |
|----------|---------|
| **Process** | ps, kill, killall, top |
| **Filesystem** | mount, umount, df, du |
| **Network** | ping, wget, nc, ifconfig |
| **Misc** | date, cal, clear, reset, sleep |

### Priority 3 - Future (Advanced)

| Category | Applets |
|----------|---------|
| **Services** | crond, syslogd |
| **Users** | adduser, passwd, su |
| **Disk** | fdisk, mkfs |

---

## 7. Phase 1 Outputs

- [x] Problem statement documented
- [x] Success criteria defined
- [x] Current state analyzed
- [x] Files to modify identified
- [x] BusyBox capabilities prioritized
- [x] Constraints documented

**Phase 1 Complete - Proceed to Phase 2 (Design)**
