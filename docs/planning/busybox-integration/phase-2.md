# Phase 2: Design - BusyBox Integration

**Feature:** Replace uutils-coreutils, dash, and custom init with BusyBox  
**Team:** TEAM_449  
**Status:** Complete

---

## 1. Proposed Solution

### High-Level Architecture

```
Kernel boots
    ↓
Spawns /init → symlink to /bin/busybox
    ↓
BusyBox init runs /etc/init.d/rcS (or spawns shell directly)
    ↓
BusyBox ash shell (interactive)
    ↓
All commands (ls, cat, grep...) → symlinks to /bin/busybox
```

**Single binary (~1MB) replaces everything.**

### Directory Structure in Initramfs

```
/
├── init -> bin/busybox          # Kernel starts this
├── bin/
│   ├── busybox                  # The actual binary
│   ├── sh -> busybox
│   ├── ash -> busybox
│   ├── cat -> busybox
│   ├── ls -> busybox
│   ├── grep -> busybox
│   └── ... (all applets)
├── sbin/
│   ├── init -> ../bin/busybox
│   ├── halt -> ../bin/busybox
│   ├── reboot -> ../bin/busybox
│   └── ...
├── etc/
│   ├── inittab                  # BusyBox init config (optional)
│   ├── passwd                   # User database (minimal)
│   └── profile                  # Shell profile
├── tmp/                         # Writable tmpfs
├── proc/                        # procfs mount point
└── sys/                         # sysfs mount point
```

---

## 2. BusyBox Init Configuration

### Option A: Minimal Init (Recommended for Now)

BusyBox init with simple `/etc/inittab`:

```
# /etc/inittab - BusyBox init configuration
::sysinit:/bin/echo "LevitateOS booting..."
::sysinit:/bin/mount -t proc proc /proc
::sysinit:/bin/mount -t sysfs sysfs /sys
::respawn:/bin/ash
::ctrlaltdel:/sbin/reboot
::shutdown:/bin/echo "System shutting down..."
```

This:
- Prints boot message
- Spawns ash shell (respawns if it exits)
- Handles Ctrl+Alt+Del
- Clean shutdown message

### Option B: Direct Shell Spawn

Simplest possible - `/init` symlink directly runs shell:

```bash
#!/bin/ash
exec /bin/ash
```

Or configure BusyBox init to just spawn shell without inittab.

### Option C: Full Init with Services (Future)

```
::sysinit:/etc/init.d/rcS
::respawn:/sbin/getty 38400 tty1
::ctrlaltdel:/sbin/reboot
```

---

## 3. Build System Design

### New File: `xtask/src/build/busybox.rs`

```rust
//! BusyBox build support
//! TEAM_449: Single binary replaces coreutils + dash + init

use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use std::process::Command;

pub struct BusyBox;

impl BusyBox {
    pub const REPO: &'static str = "https://git.busybox.net/busybox";
    
    pub fn clone_dir() -> PathBuf {
        PathBuf::from("toolchain/busybox")
    }
    
    pub fn config_file() -> PathBuf {
        PathBuf::from("toolchain/busybox-levitateos.config")
    }
    
    pub fn output_dir(arch: &str) -> PathBuf {
        PathBuf::from(format!("toolchain/busybox-out/{}", arch))
    }
    
    pub fn output_path(arch: &str) -> PathBuf {
        Self::output_dir(arch).join("busybox")
    }
    
    /// All applets to create symlinks for
    pub fn applets() -> &'static [&'static str] {
        &[
            // Init
            "init", "halt", "poweroff", "reboot",
            // Shell
            "sh", "ash",
            // Coreutils
            "cat", "cp", "echo", "ls", "mkdir", "mv", "pwd", "rm", 
            "rmdir", "touch", "ln", "chmod", "chown", "head", "tail",
            "true", "false", "yes", "test", "[",
            // File info
            "stat", "wc", "du", "df",
            // Text processing
            "grep", "sed", "awk", "sort", "uniq", "cut", "tr", 
            "head", "tail", "tee",
            // Search
            "find", "xargs", "which",
            // Archives
            "tar", "gzip", "gunzip", "zcat",
            // Editor
            "vi",
            // Process
            "ps", "kill", "killall", "sleep",
            // Filesystem
            "mount", "umount",
            // Misc
            "date", "clear", "reset", "env", "printenv", "uname",
        ]
    }
    
    pub fn clone_repo() -> Result<()> { /* ... */ }
    pub fn apply_config() -> Result<()> { /* ... */ }
    pub fn build(arch: &str) -> Result<()> { /* ... */ }
}
```

### BusyBox Configuration File

Create `toolchain/busybox-levitateos.config`:

```
# LevitateOS BusyBox Configuration
# TEAM_449: Minimal config for static musl build

# Build options
CONFIG_STATIC=y
CONFIG_CROSS_COMPILER_PREFIX=""

# Disable features incompatible with musl/LevitateOS
# CONFIG_SELINUX is not set
# CONFIG_PAM is not set
# CONFIG_FEATURE_HAVE_RPC is not set
# CONFIG_FEATURE_SYSTEMD is not set
# CONFIG_FEATURE_MOUNT_NFS is not set
# CONFIG_FEATURE_INETD_RPC is not set

# Init
CONFIG_INIT=y
CONFIG_HALT=y
CONFIG_POWEROFF=y
CONFIG_REBOOT=y

# Shell
CONFIG_ASH=y
CONFIG_ASH_JOB_CONTROL=y
CONFIG_ASH_ALIAS=y
CONFIG_ASH_BUILTIN_ECHO=y
CONFIG_ASH_BUILTIN_TEST=y
CONFIG_ASH_CMDCMD=y

# Coreutils - all enabled
CONFIG_CAT=y
CONFIG_CP=y
# ... etc
```

---

## 4. Initramfs Creation Design

### Updated `scripts/make_initramfs.sh`

```bash
#!/bin/bash
# TEAM_449: BusyBox-based initramfs
set -e

BUSYBOX="toolchain/busybox-out/x86_64/busybox"
INITRD="initrd_root"

# Clean and create structure
rm -rf "$INITRD"
mkdir -p "$INITRD"/{bin,sbin,etc,proc,sys,tmp,dev}

# Copy busybox
cp "$BUSYBOX" "$INITRD/bin/busybox"
chmod +x "$INITRD/bin/busybox"

# Create symlinks for all applets
APPLETS="init sh ash cat cp ls mkdir mv pwd rm rmdir touch ln chmod
         grep sed awk sort uniq cut tr find xargs tar gzip vi
         ps kill mount umount date clear env uname head tail wc
         true false test halt poweroff reboot"

for applet in $APPLETS; do
    case $applet in
        init|halt|poweroff|reboot)
            ln -s ../bin/busybox "$INITRD/sbin/$applet"
            ;;
        *)
            ln -s busybox "$INITRD/bin/$applet"
            ;;
    esac
done

# Create /init symlink
ln -s bin/busybox "$INITRD/init"

# Create minimal /etc files
cat > "$INITRD/etc/inittab" << 'EOF'
::sysinit:/bin/echo "LevitateOS (BusyBox)"
::respawn:/bin/ash
::ctrlaltdel:/sbin/reboot
::shutdown:/bin/echo "Goodbye!"
EOF

cat > "$INITRD/etc/passwd" << 'EOF'
root:x:0:0:root:/:/bin/ash
EOF

cat > "$INITRD/etc/profile" << 'EOF'
export PATH=/bin:/sbin
export PS1='# '
EOF

# Create CPIO archive
cd "$INITRD"
find . | cpio -o -H newc > ../initramfs.cpio
cd ..

echo "BusyBox initramfs created: $(du -h initramfs.cpio)"
```

---

## 5. Behavioral Decisions

### Q1: How should BusyBox init start?

**Options:**
- A) Use `/etc/inittab` with `::respawn:/bin/ash`
- B) Hardcode to spawn shell directly (no inittab)
- C) Use init script `/etc/init.d/rcS`

**Recommendation:** Option A - Standard BusyBox behavior, configurable.

### Q2: What should happen when shell exits?

**Options:**
- A) Respawn shell (BusyBox default with `::respawn`)
- B) Halt system
- C) Reboot

**Recommendation:** Option A - Matches current behavior where shell can be restarted.

### Q3: Should we mount /proc and /sys automatically?

**Options:**
- A) Yes, in init script
- B) No, leave to user
- C) Kernel mounts them

**Recommendation:** Option A - Standard Linux behavior, needed for `ps`, etc.

### Q4: What shell prompt should we use?

**Options:**
- A) `# ` (root, simple)
- B) `LevitateOS# `
- C) `\u@\h:\w# ` (user@host:path)

**Recommendation:** Option A for now, configurable via `/etc/profile`.

### Q5: Should we keep the test runner in init?

**Current init** runs `eyra-test-runner` if present.

**Options:**
- A) Remove test runner support (use different test approach)
- B) Add test runner check to BusyBox init script
- C) Keep separate test init image

**Recommendation:** Option C - Separate test initramfs for testing.

---

## 6. Open Questions for USER

### Q1: Init Behavior

**Question:** When the shell exits, should the system:
- A) Respawn the shell (keep running)
- B) Halt/shutdown
- C) Configurable via inittab

### Q2: Procfs/Sysfs

**Question:** Should BusyBox init automatically mount `/proc` and `/sys`?
- Needed for: `ps`, `top`, `mount` display, etc.
- Requires: Kernel support for procfs/sysfs (is this implemented?)

### Q3: Test Mode

**Question:** How should we handle test mode (currently `eyra-test-runner`)?
- A) Separate test initramfs image
- B) Check for test binary in BusyBox init script
- C) Remove automated testing (manual only)

### Q4: Which BusyBox Applets?

**Question:** Should we enable ALL BusyBox applets or a curated subset?
- Full: ~300 applets, larger binary (~1.5MB)
- Minimal: ~50 applets, smaller binary (~500KB)

### Q5: Editor

**Question:** Should we include `vi` in the default build?
- Pro: Useful for editing files
- Con: Adds ~50KB to binary

---

## 7. Design Alternatives Considered

### Alternative 1: Keep uutils-coreutils + add BusyBox

**Rejected because:**
- Two implementations of same utilities
- Larger total size
- More complexity

### Alternative 2: Use toybox instead of BusyBox

**Rejected because:**
- Less mature than BusyBox
- Fewer utilities
- Less documentation
- BusyBox is industry standard (Alpine, embedded)

### Alternative 3: Keep custom Rust init + BusyBox utilities only

**Rejected because:**
- BusyBox init is well-tested
- Removes need to maintain custom code
- BusyBox init handles edge cases we haven't

---

## 8. Phase 2 Outputs

- [x] Solution architecture documented
- [x] Directory structure defined
- [x] Build system design complete
- [x] Initramfs creation design complete
- [x] Behavioral decisions documented
- [x] **Open questions answered by USER**

**Phase 2 Status: COMPLETE - Proceed to Phase 3**
