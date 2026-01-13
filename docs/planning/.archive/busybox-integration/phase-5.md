# Phase 5: Polish & Cleanup - BusyBox Integration

**Feature:** Replace uutils-coreutils, dash, and custom init with BusyBox  
**Team:** TEAM_449  
**Status:** Ready (after Phase 4)

---

## Code Cleanup

### 1. Remove Old Code

**Delete:**
- `crates/userspace/init/` - Entire directory (custom Rust init)
- `toolchain/coreutils/` - Can keep for reference or delete
- `toolchain/unmodified-coreutils/` - Delete
- `toolchain/dash/` - Can keep for reference or delete

**Modify:**
- `crates/userspace/Cargo.toml` - Remove init from workspace members
- `xtask/src/build/apps.rs` - Remove coreutils entry

### 2. Clean Up Build System

**Remove from `apps.rs`:**
```rust
// DELETE this entry
ExternalApp {
    name: "coreutils",
    repo: "https://github.com/uutils/coreutils",
    ...
}
```

**Update `APPS` array:**
```rust
pub static APPS: &[ExternalApp] = &[
    // coreutils removed - using BusyBox
];
```

### 3. Update .gitignore

Add:
```
toolchain/busybox/
toolchain/busybox-out/
```

---

## Documentation Updates

### 1. Update CLAUDE.md

```markdown
## Building

### Quick Start
cargo xtask build all        # Kernel + BusyBox + initramfs
cargo xtask run              # Run in QEMU

### Individual Builds
cargo xtask build kernel     # Kernel only
cargo xtask build busybox    # BusyBox userspace
cargo xtask build initramfs  # Create initramfs

## Userspace

LevitateOS uses BusyBox for all userspace utilities:
- **Shell**: ash (POSIX-compliant)
- **Init**: BusyBox init with /etc/inittab
- **Coreutils**: cat, ls, cp, mv, rm, mkdir, etc.
- **Text tools**: grep, sed, awk, sort, etc.
- **Editor**: vi

BusyBox is built statically with musl libc.
```

### 2. Update README.md

Add to features:
```markdown
## Features

- Custom kernel (Rust, no_std)
- BusyBox userspace (300+ utilities)
- Interactive ash shell
- POSIX-compatible init system
```

### 3. Update ROADMAP.md

```markdown
## ✅ Phase 11: Core Utilities (BusyBox)

**Status:** Complete (TEAM_449)

Replaced uutils-coreutils + dash + custom init with BusyBox:
- Single ~1MB binary provides 300+ utilities
- Built-in ash shell with job control
- BusyBox init system
- Simplified build (one C build vs Rust + C)

Available utilities:
- Shell: sh, ash
- Coreutils: cat, ls, cp, mv, rm, mkdir, touch, etc.
- Text: grep, sed, awk, sort, uniq, cut, tr
- Files: find, tar, gzip
- System: ps, kill, mount, umount
- Editor: vi
```

### 4. Create BusyBox-specific Documentation

**File:** `docs/BUSYBOX.md`

```markdown
# BusyBox in LevitateOS

## Overview

LevitateOS uses BusyBox as its userspace toolkit, providing:
- Init system
- Shell (ash)
- 300+ Unix utilities

## Building

```bash
cargo xtask build busybox
```

## Configuration

BusyBox is configured for LevitateOS with:
- Static linking (musl)
- Disabled: SELinux, PAM, RPC, systemd, NFS
- Enabled: init, ash, all common utilities

To modify configuration:
1. Edit `toolchain/busybox-levitateos.config`
2. Rebuild: `cargo xtask build busybox --clean`

## Init System

BusyBox init reads `/etc/inittab`:

```
::sysinit:/bin/echo "LevitateOS (BusyBox) starting..."
::respawn:-/bin/ash
::ctrlaltdel:/sbin/reboot
::shutdown:/bin/echo "System shutting down..."
```

## Available Commands

Run `busybox --list` for full list. Key utilities:

### Shell & Core
sh, ash, echo, test, true, false

### File Management
ls, cp, mv, rm, mkdir, rmdir, touch, ln, chmod, chown

### Text Processing
cat, head, tail, grep, sed, awk, sort, uniq, cut, tr, wc

### File Search
find, xargs, which

### Archives
tar, gzip, gunzip, zcat

### System
ps, kill, mount, umount, df, du, date, uname

### Editor
vi
```

---

## Handoff Notes

### What Changed

1. **Removed:**
   - `crates/userspace/init/` (custom Rust init)
   - uutils-coreutils dependency
   - dash dependency

2. **Added:**
   - `xtask/src/build/busybox.rs`
   - `toolchain/busybox-levitateos.config`
   - Updated `scripts/make_initramfs.sh`

3. **Modified:**
   - `xtask/src/build/apps.rs` (removed coreutils)
   - `xtask/src/build/mod.rs` (added busybox module)
   - Golden boot logs

### Build Dependencies

**Required (same as before):**
- musl-gcc (x86_64)
- Standard build tools (make, git)

**No longer required:**
- Rust nightly for userspace
- autoconf/automake (was for dash)

### Testing

```bash
# Full build and test
cargo xtask build all
cargo xtask run --arch x86_64

# Verify in shell
echo "test"
ls -la /
ps
```

### Known Limitations

1. **No procfs/sysfs yet** - `ps` may not show all info
2. **No /dev population** - Some utilities may not work fully
3. **Single user** - No multi-user support

### Future Enhancements

1. Mount procfs/sysfs in init
2. Add devtmpfs support
3. Enable networking utilities (wget, nc)
4. Add cron/syslog for services

---

## Phase 5 Checklist

- [ ] Old code removed
- [ ] Build system cleaned up
- [ ] CLAUDE.md updated
- [ ] README.md updated
- [ ] ROADMAP.md updated
- [ ] BUSYBOX.md created
- [ ] Team file finalized
- [ ] All tests passing
- [ ] No dead code remaining
