# BusyBox Integration Investigation

**Date:** 2026-01-12  
**Status:** Investigation Complete  
**Goal:** Replace uutils-coreutils with BusyBox

---

## Executive Summary

BusyBox is a better fit for LevitateOS than uutils-coreutils because:

1. **Single binary** - One ~1MB executable provides shell + 300+ utilities
2. **Includes dash/ash shell** - No separate shell binary needed
3. **Battle-tested** - Used by Alpine Linux, embedded systems, initramfs everywhere
4. **Smaller footprint** - ~1MB vs ~10MB for equivalent functionality
5. **Simpler integration** - One C build vs Rust toolchain complexity

---

## Current State: uutils-coreutils

### How It Works Now

```
@xtask/src/build/apps.rs
```

- Uses `x86_64-unknown-linux-musl` target
- Builds with cargo, limited features: `cat,echo,head,mkdir,pwd,rm,tail,touch`
- Creates symlinks for multi-call binary pattern
- Separate dash shell needed for actual shell functionality

### Problems with uutils-coreutils

1. **Limited features** - Only 8 utilities enabled (musl compatibility issues)
2. **No shell** - Need separate dash build
3. **Two build systems** - Rust (coreutils) + C (dash)
4. **Large binary** - Rust binaries are bigger than equivalent C
5. **Complexity** - Rust ecosystem complexity for simple utils

---

## BusyBox Overview

### What BusyBox Provides

**Single binary** that acts as:
- **Shell**: ash (POSIX sh), hush
- **Coreutils**: cat, ls, cp, mv, rm, mkdir, touch, etc.
- **File tools**: find, grep, sed, awk, tar, gzip
- **System tools**: mount, umount, init, ps, kill
- **Network tools**: wget, nc, ping, ifconfig
- **Editors**: vi

### Size Comparison

| Solution | Utilities | Binary Size | Notes |
|----------|-----------|-------------|-------|
| BusyBox (static musl) | 300+ | ~1MB | Single binary |
| uutils-coreutils | 8 | ~2MB | Limited features |
| dash (separate) | 1 (shell) | ~150KB | Needs coreutils too |
| **Total current** | 9 | ~2.2MB | Two builds needed |

---

## Build Requirements

### BusyBox + musl Static Build

```bash
# Prerequisites (already have for dash)
# Fedora: sudo dnf install musl-gcc musl-devel
# Ubuntu: sudo apt install musl-tools musl-dev

# Clone BusyBox
git clone --depth=1 https://git.busybox.net/busybox toolchain/busybox

# Configure
cd toolchain/busybox
make defconfig

# Enable static linking (via menuconfig or sed)
# Settings → Build Options → Build as static binary
sed -i 's/# CONFIG_STATIC is not set/CONFIG_STATIC=y/' .config

# Disable features that don't work with musl
sed -i 's/CONFIG_FEATURE_HAVE_RPC=y/# CONFIG_FEATURE_HAVE_RPC is not set/' .config
sed -i 's/CONFIG_FEATURE_MOUNT_NFS=y/# CONFIG_FEATURE_MOUNT_NFS is not set/' .config
sed -i 's/CONFIG_FEATURE_INETD_RPC=y/# CONFIG_FEATURE_INETD_RPC is not set/' .config
sed -i 's/CONFIG_SELINUX=y/# CONFIG_SELINUX is not set/' .config
sed -i 's/CONFIG_FEATURE_SYSTEMD=y/# CONFIG_FEATURE_SYSTEMD is not set/' .config
sed -i 's/CONFIG_PAM=y/# CONFIG_PAM is not set/' .config

# Build with musl
make CC=musl-gcc LDFLAGS="-static" -j$(nproc)

# Result: busybox binary (~1MB static)
```

### Cross-Compilation for aarch64

```bash
# Requires aarch64 musl cross-compiler
# Fedora: sudo dnf install musl-gcc  # x86_64 only :(
# Need to build musl cross-toolchain or use musl.cc prebuilts

# With cross-compiler:
make ARCH=arm64 CROSS_COMPILE=aarch64-linux-musl- CC=aarch64-linux-musl-gcc -j$(nproc)
```

---

## Integration Approach

### Option A: Replace coreutils entirely (Recommended)

1. **Remove uutils-coreutils** from `apps.rs`
2. **Add BusyBox** as new C app in `c_apps.rs`
3. **Remove dash** (BusyBox includes ash shell)
4. **Update initramfs** to use busybox symlinks

### Option B: Add BusyBox alongside (gradual migration)

1. Add BusyBox build
2. Keep uutils-coreutils for now
3. Test both work
4. Remove uutils-coreutils later

---

## Implementation Plan

### Phase 1: Add BusyBox Build

Modify `@xtask/src/build/c_apps.rs` (or create new file):

```rust
pub struct BusyBoxApp;

impl BusyBoxApp {
    pub fn clone_dir() -> PathBuf {
        PathBuf::from("toolchain/busybox")
    }

    pub fn output_path(arch: &str) -> PathBuf {
        PathBuf::from(format!("toolchain/busybox-out/{}/busybox", arch))
    }

    pub fn clone_repo() -> Result<()> {
        let dir = Self::clone_dir();
        if dir.exists() { return Ok(()); }
        
        Command::new("git")
            .args(["clone", "--depth=1", "https://git.busybox.net/busybox", &dir.to_string_lossy()])
            .status()?;
        Ok(())
    }

    pub fn build(arch: &str) -> Result<()> {
        Self::clone_repo()?;
        let dir = Self::clone_dir();
        
        // Generate config
        Command::new("make")
            .current_dir(&dir)
            .arg("defconfig")
            .status()?;
        
        // Apply musl-compatible config
        Self::apply_musl_config(&dir)?;
        
        // Build
        let (cc, cross) = match arch {
            "x86_64" => ("musl-gcc", ""),
            "aarch64" => ("aarch64-linux-musl-gcc", "aarch64-linux-musl-"),
            _ => bail!("Unsupported arch"),
        };
        
        Command::new("make")
            .current_dir(&dir)
            .env("CC", cc)
            .args([&format!("CROSS_COMPILE={}", cross), "LDFLAGS=-static", "-j8"])
            .status()?;
        
        // Copy output
        let out_dir = PathBuf::from(format!("toolchain/busybox-out/{}", arch));
        std::fs::create_dir_all(&out_dir)?;
        std::fs::copy(dir.join("busybox"), out_dir.join("busybox"))?;
        
        Ok(())
    }
    
    pub fn symlinks() -> &'static [&'static str] {
        &[
            // Shell
            "sh", "ash",
            // Coreutils
            "cat", "cp", "echo", "ls", "mkdir", "mv", "pwd", "rm", "rmdir", 
            "head", "tail", "touch", "ln", "chmod", "chown",
            // Text processing
            "grep", "sed", "awk", "sort", "uniq", "wc", "cut",
            // File tools
            "find", "tar", "gzip", "gunzip",
            // System
            "ps", "kill", "mount", "umount",
            // Editors
            "vi",
        ]
    }
}
```

### Phase 2: Update Initramfs Creation

Modify `@scripts/make_initramfs.sh`:

```bash
# Instead of copying multiple binaries, copy busybox + create symlinks
cp toolchain/busybox-out/x86_64/busybox initramfs/bin/busybox

# Create symlinks for all applets
for cmd in sh ash cat cp ls mkdir mv pwd rm ...; do
    ln -s busybox initramfs/bin/$cmd
done
```

### Phase 3: Remove Old Code

1. Remove coreutils from `APPS` array in `apps.rs`
2. Remove dash from `C_APPS` array in `c_apps.rs`
3. Clean up toolchain directories

---

## Syscall Requirements

BusyBox uses standard POSIX syscalls. Key requirements for LevitateOS:

| Syscall | Used By | LevitateOS Status |
|---------|---------|-------------------|
| fork/clone | sh, all | ✅ Done |
| execve | sh | ✅ Done |
| waitpid | sh | ✅ Done |
| pipe | sh pipelines | ✅ Done |
| dup2 | sh redirects | ✅ Done |
| open/read/write | all | ✅ Done |
| stat/fstat | ls, test | ✅ Done |
| getdents | ls | ✅ Done |
| getcwd/chdir | cd, pwd | ✅ Done |
| kill/signal | sh job control | ✅ Done |
| ioctl (termios) | sh, vi | ✅ Done |

**BusyBox should work with current LevitateOS syscall support.**

---

## Risks & Mitigations

### Risk 1: musl Compatibility Issues

**Mitigation**: Use BusyBox defconfig with musl-incompatible features disabled. Well-documented process.

### Risk 2: aarch64 Cross-Compilation

**Mitigation**: 
- x86_64 works with system musl-gcc
- aarch64 needs musl cross-toolchain (can download prebuilt from musl.cc)
- Or use QEMU user-mode for native build

### Risk 3: Missing Syscalls

**Mitigation**: BusyBox uses basic POSIX syscalls that LevitateOS already supports. If issues arise, they'll be specific applets we can disable.

---

## Decision Matrix

| Factor | uutils-coreutils | BusyBox |
|--------|------------------|---------|
| Binary size | Large (~2MB for 8 utils) | Small (~1MB for 300+ utils) |
| Shell included | ❌ No | ✅ Yes (ash) |
| Build complexity | Medium (Rust toolchain) | Low (C + musl-gcc) |
| Feature coverage | Low (8 utils) | High (300+ utils) |
| Battle-tested | Medium | Very High (Alpine, embedded) |
| Maintenance | Active (uutils project) | Active (busybox.net) |
| Language | Rust | C |

**Recommendation: Switch to BusyBox**

---

## Next Steps

1. [ ] Create `xtask/src/build/busybox.rs` with build logic
2. [ ] Add `cargo xtask build busybox` command
3. [ ] Test BusyBox binary runs on LevitateOS
4. [ ] Update initramfs to use BusyBox
5. [ ] Remove uutils-coreutils and dash
6. [ ] Update documentation

---

## References

- BusyBox source: https://git.busybox.net/busybox
- musl wiki - Building BusyBox: https://wiki.musl-libc.org/building-busybox
- BusyBox FAQ: https://busybox.net/FAQ.html
- Alpine Linux (BusyBox + musl): https://alpinelinux.org/
