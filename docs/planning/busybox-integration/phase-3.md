# Phase 3: Implementation - BusyBox Integration

**Feature:** Replace uutils-coreutils, dash, and custom init with BusyBox  
**Team:** TEAM_449  
**Status:** Ready (after Phase 2 questions answered)

---

## Prerequisites

- [x] Phase 2 questions answered by USER
- [ ] All tests currently passing

---

## Implementation Steps

### Step 1: Add BusyBox Build Infrastructure

**File:** `xtask/src/build/busybox.rs`

**Tasks:**
1. Create `busybox.rs` module
2. Implement `clone_repo()` - git clone busybox
3. Implement `apply_config()` - apply LevitateOS config
4. Implement `build()` - build with musl-gcc
5. Implement `applets()` - list of symlinks to create

**Estimated time:** 1 session

---

### Step 2: Create BusyBox Configuration

**File:** `toolchain/busybox-levitateos.config`

**Tasks:**
1. Start from `defconfig`
2. Enable static linking
3. Disable musl-incompatible features
4. Enable all P0/P1 applets
5. Enable init system
6. Enable ash shell with job control

**Estimated time:** 1 session

---

### Step 3: Integrate BusyBox Build into xtask

**Files:**
- `xtask/src/build/mod.rs`
- `xtask/src/main.rs`

**Tasks:**
1. Add `mod busybox` to build module
2. Add `Build Busybox` command to xtask
3. Update `build all` to include busybox
4. Remove coreutils from required apps

**Estimated time:** 1 session

---

### Step 4: Update Initramfs Creation

**File:** `scripts/make_initramfs.sh`

**Tasks:**
1. Remove uutils-coreutils copying
2. Remove dash copying  
3. Remove custom init copying
4. Add BusyBox binary copy
5. Create symlinks for all applets
6. Create /etc/inittab
7. Create /etc/passwd
8. Create /etc/profile
9. Create directory structure (/proc, /sys, /tmp, /dev)

**Estimated time:** 1 session

---

### Step 5: Remove Old Code

**Tasks:**
1. Remove `coreutils` entry from `apps.rs` APPS array
2. Remove `dash` entry from `c_apps.rs` (if present)
3. Delete `crates/userspace/init/` directory
4. Update `crates/userspace/Cargo.toml` to remove init
5. Clean up any references to old init

**Estimated time:** 1 session

---

### Step 6: Test BusyBox Build

**Tasks:**
1. Run `cargo xtask build busybox`
2. Verify binary exists and is static
3. Check binary size
4. Verify applets work: `./busybox ls`, `./busybox ash`

**Verification:**
```bash
file toolchain/busybox-out/x86_64/busybox
# Should show: "statically linked"

ls -la toolchain/busybox-out/x86_64/busybox
# Should be ~1MB

./toolchain/busybox-out/x86_64/busybox --list
# Should list all enabled applets
```

**Estimated time:** 1 session

---

## Detailed UoW Breakdown

### UoW 3.1: Create busybox.rs

```rust
// xtask/src/build/busybox.rs
// TEAM_449: BusyBox build support

use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use std::process::Command;

pub fn clone_dir() -> PathBuf {
    PathBuf::from("toolchain/busybox")
}

pub fn output_dir(arch: &str) -> PathBuf {
    PathBuf::from(format!("toolchain/busybox-out/{}", arch))
}

pub fn output_path(arch: &str) -> PathBuf {
    output_dir(arch).join("busybox")
}

pub fn exists(arch: &str) -> bool {
    output_path(arch).exists()
}

pub fn clone_repo() -> Result<()> {
    let dir = clone_dir();
    if dir.exists() {
        return Ok(());
    }
    
    println!("📥 Cloning BusyBox...");
    let status = Command::new("git")
        .args(["clone", "--depth=1", "https://git.busybox.net/busybox"])
        .arg(&dir)
        .status()
        .context("Failed to clone BusyBox")?;
    
    if !status.success() {
        bail!("git clone failed");
    }
    Ok(())
}

pub fn build(arch: &str) -> Result<()> {
    clone_repo()?;
    
    let dir = clone_dir();
    
    // Clean previous build
    println!("🧹 Cleaning BusyBox...");
    Command::new("make")
        .current_dir(&dir)
        .arg("clean")
        .status()?;
    
    // Generate defconfig
    println!("⚙️ Configuring BusyBox...");
    Command::new("make")
        .current_dir(&dir)
        .arg("defconfig")
        .status()?;
    
    // Apply our config overrides
    apply_musl_config(&dir)?;
    
    // Build
    println!("🔨 Building BusyBox for {}...", arch);
    let cc = match arch {
        "x86_64" => "musl-gcc",
        "aarch64" => "aarch64-linux-musl-gcc",
        _ => bail!("Unsupported architecture: {}", arch),
    };
    
    let status = Command::new("make")
        .current_dir(&dir)
        .env("CC", cc)
        .args(["LDFLAGS=-static", &format!("-j{}", num_cpus())])
        .status()
        .context("Failed to build BusyBox")?;
    
    if !status.success() {
        bail!("BusyBox build failed");
    }
    
    // Copy to output
    let out_dir = output_dir(arch);
    std::fs::create_dir_all(&out_dir)?;
    std::fs::copy(dir.join("busybox"), output_path(arch))?;
    
    println!("✅ BusyBox built: {}", output_path(arch).display());
    Ok(())
}

fn apply_musl_config(dir: &PathBuf) -> Result<()> {
    let config_path = dir.join(".config");
    let config = std::fs::read_to_string(&config_path)?;
    
    let config = config
        // Enable static
        .replace("# CONFIG_STATIC is not set", "CONFIG_STATIC=y")
        // Disable musl-incompatible features
        .replace("CONFIG_SELINUX=y", "# CONFIG_SELINUX is not set")
        .replace("CONFIG_FEATURE_HAVE_RPC=y", "# CONFIG_FEATURE_HAVE_RPC is not set")
        .replace("CONFIG_FEATURE_MOUNT_NFS=y", "# CONFIG_FEATURE_MOUNT_NFS is not set")
        .replace("CONFIG_FEATURE_INETD_RPC=y", "# CONFIG_FEATURE_INETD_RPC is not set")
        .replace("CONFIG_PAM=y", "# CONFIG_PAM is not set")
        .replace("CONFIG_FEATURE_SYSTEMD=y", "# CONFIG_FEATURE_SYSTEMD is not set");
    
    std::fs::write(&config_path, config)?;
    Ok(())
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(4)
}

/// List of applets to create symlinks for in initramfs
pub fn applets() -> &'static [(&'static str, &'static str)] {
    // (name, directory) - "bin" or "sbin"
    &[
        // Init system
        ("init", "sbin"),
        ("halt", "sbin"),
        ("poweroff", "sbin"),
        ("reboot", "sbin"),
        // Shell
        ("sh", "bin"),
        ("ash", "bin"),
        // Coreutils
        ("cat", "bin"),
        ("cp", "bin"),
        ("echo", "bin"),
        ("ls", "bin"),
        ("mkdir", "bin"),
        ("mv", "bin"),
        ("pwd", "bin"),
        ("rm", "bin"),
        ("rmdir", "bin"),
        ("touch", "bin"),
        ("ln", "bin"),
        ("chmod", "bin"),
        ("chown", "bin"),
        ("head", "bin"),
        ("tail", "bin"),
        ("true", "bin"),
        ("false", "bin"),
        ("test", "bin"),
        ("[", "bin"),
        ("stat", "bin"),
        ("wc", "bin"),
        // Text processing
        ("grep", "bin"),
        ("sed", "bin"),
        ("awk", "bin"),
        ("sort", "bin"),
        ("uniq", "bin"),
        ("cut", "bin"),
        ("tr", "bin"),
        ("tee", "bin"),
        // Search
        ("find", "bin"),
        ("xargs", "bin"),
        ("which", "bin"),
        // Archives
        ("tar", "bin"),
        ("gzip", "bin"),
        ("gunzip", "bin"),
        ("zcat", "bin"),
        // Editor
        ("vi", "bin"),
        // Process
        ("ps", "bin"),
        ("kill", "bin"),
        ("killall", "bin"),
        ("sleep", "bin"),
        // Filesystem
        ("mount", "bin"),
        ("umount", "bin"),
        ("df", "bin"),
        ("du", "bin"),
        // Misc
        ("date", "bin"),
        ("clear", "bin"),
        ("reset", "bin"),
        ("env", "bin"),
        ("printenv", "bin"),
        ("uname", "bin"),
        ("hostname", "bin"),
        ("id", "bin"),
        ("whoami", "bin"),
    ]
}
```

---

### UoW 3.2: Update make_initramfs.sh

```bash
#!/bin/bash
# TEAM_449: BusyBox-based initramfs
set -e

ARCH="${1:-x86_64}"
BUSYBOX="toolchain/busybox-out/${ARCH}/busybox"
INITRD="initrd_root"

if [ ! -f "$BUSYBOX" ]; then
    echo "ERROR: BusyBox not found at $BUSYBOX"
    echo "Run: cargo xtask build busybox"
    exit 1
fi

echo "📦 Creating BusyBox initramfs for $ARCH..."

# Clean and create structure
rm -rf "$INITRD"
mkdir -p "$INITRD"/{bin,sbin,etc,proc,sys,tmp,dev,root}

# Copy busybox binary
cp "$BUSYBOX" "$INITRD/bin/busybox"
chmod +x "$INITRD/bin/busybox"

# Create symlinks for applets
# bin applets
for cmd in sh ash cat cp echo ls mkdir mv pwd rm rmdir touch ln chmod \
           head tail true false test grep sed awk sort uniq cut tr tee \
           find xargs which tar gzip gunzip zcat vi ps kill killall sleep \
           mount umount df du date clear reset env printenv uname hostname id whoami; do
    ln -sf busybox "$INITRD/bin/$cmd"
done

# sbin applets
for cmd in init halt poweroff reboot; do
    ln -sf ../bin/busybox "$INITRD/sbin/$cmd"
done

# Create /init symlink (kernel entry point)
ln -sf bin/busybox "$INITRD/init"

# Create /etc/inittab
cat > "$INITRD/etc/inittab" << 'EOF'
# LevitateOS BusyBox init configuration
# Format: <id>:<runlevels>:<action>:<process>

# System initialization
::sysinit:/bin/echo "LevitateOS (BusyBox) starting..."
::sysinit:/bin/mount -t proc proc /proc
::sysinit:/bin/mount -t sysfs sysfs /sys

# Start interactive shell (respawn if it exits)
::respawn:-/bin/ash

# Handle Ctrl+Alt+Del
::ctrlaltdel:/sbin/reboot

# Shutdown hooks
::shutdown:/bin/echo "System shutting down..."
EOF

# Create minimal /etc/passwd
cat > "$INITRD/etc/passwd" << 'EOF'
root:x:0:0:root:/root:/bin/ash
EOF

# Create /etc/group
cat > "$INITRD/etc/group" << 'EOF'
root:x:0:
EOF

# Create /etc/profile
cat > "$INITRD/etc/profile" << 'EOF'
export PATH=/bin:/sbin
export HOME=/root
export PS1='LevitateOS# '
alias ll='ls -la'
EOF

# Create sample files
echo "Welcome to LevitateOS!" > "$INITRD/etc/motd"
echo "Hello from initramfs!" > "$INITRD/root/hello.txt"

# Create CPIO archive
echo "📄 Creating CPIO archive..."
cd "$INITRD"
find . | cpio -o -H newc > ../initramfs.cpio 2>/dev/null
cd ..

SIZE=$(du -h initramfs.cpio | cut -f1)
echo "✅ BusyBox initramfs created: initramfs.cpio ($SIZE)"
```

---

## Rollback Plan

If BusyBox integration fails:

1. Revert changes to `apps.rs` and `c_apps.rs`
2. Restore `crates/userspace/init/`
3. Restore old `make_initramfs.sh`
4. Run `cargo xtask build all`

Keep old code in git history for easy rollback.

---

## Phase 3 Checklist

- [ ] Step 1: busybox.rs created
- [ ] Step 2: BusyBox config tested
- [ ] Step 3: xtask integration complete
- [ ] Step 4: Initramfs script updated
- [ ] Step 5: Old code removed
- [ ] Step 6: Build verified
