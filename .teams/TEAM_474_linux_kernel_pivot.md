# TEAM_474: Linux Kernel Pivot

## Objective

Pivot LevitateOS from a custom educational kernel to a production-ready lightweight/embedded OS using the Linux kernel. Compete with Alpine Linux.

## Strategic Context

- **Old mode**: Educational - learn by building from scratch
- **New mode**: Race mode - ship fast, compete with others
- **Target market**: Lightweight/embedded (Alpine competitor)
- **Timeline**: Weeks, not months

---

## Linux Prerequisites Checklist

This is the complete checklist for running Linux on LevitateOS. Future teams should ensure all items are complete.

### Core Kernel Requirements

- [x] **Linux kernel submodule** - `linux/` directory with shallow clone
- [x] **Kernel config** - `linux/arch/x86/configs/levitate_defconfig`
- [x] **Build system** - `cargo xtask build linux`
- [x] **Run integration** - `cargo xtask run --linux`

### Critical Kernel Config Options

| Option | Purpose | Status |
|--------|---------|--------|
| `CONFIG_BLK_DEV_INITRD=y` | **Load initrd from bootloader** | ✅ CRITICAL |
| `CONFIG_RD_GZIP=y` | Decompress gzipped initramfs | ✅ |
| `CONFIG_VIRTIO_*=y` | QEMU device support | ✅ |
| `CONFIG_DEVTMPFS_MOUNT=y` | Auto-mount /dev | ✅ |
| `CONFIG_TMPFS=y` | tmpfs for /tmp, /run | ✅ |
| `CONFIG_SERIAL_8250_CONSOLE=y` | Serial console output | ✅ |
| `CONFIG_NAMESPACES=y` | Container support | ✅ |
| `CONFIG_CGROUPS=y` | Resource limits | ✅ |

### Initramfs Requirements

- [x] `/init` - Entry point (symlink or copy of BusyBox)
- [x] `/bin/busybox` - Multi-call binary with init applet
- [x] `/bin/sh` -> `/bin/busybox` - Shell symlink
- [x] `/dev/console` - Character device (5,1)
- [x] `/dev/null` - Character device (1,3)
- [x] `/etc/inittab` - Init configuration
- [x] Proper CPIO newc format

### Phase 2: Init System (Next)

- [ ] **OpenRC** - Alpine's init system, service management
- [ ] **apk** - Alpine package manager
- [ ] **musl libc** - Already have for static builds

### Phase 3: Desktop Readiness

- [ ] **aarch64 support** - Cross-compile Linux for ARM64
- [ ] **Limine ISO boot** - Boot Linux from ISO (not just direct kernel)
- [ ] **Graphics** - virtio-gpu with DRM/KMS
- [ ] **Networking** - DHCP, DNS resolver
- [ ] **Storage** - Mount root filesystem from disk

---

## Progress Log

### Session 1 (2026-01-13)

**Completed:**
1. Added Linux kernel as git submodule (`linux/`)
2. Created `levitate_defconfig` for x86_64 with embedded/container focus
3. Integrated with xtask build system (`cargo xtask build linux`)
4. Added `--linux` flag to run command
5. Fixed initramfs builder bugs (duplicate `/init`, unnecessary musl linker)
6. **CRITICAL FIX**: Added `CONFIG_BLK_DEV_INITRD=y` to kernel config
7. Successfully booted Linux with BusyBox shell!

**Boot verified:**
```
Run /init as init process
LevitateOS (BusyBox) starting...
LevitateOS#
```

**Kernel size:** 7.2 MB (target was ~20MB, we're well under!)

---

## Gotchas Discovered

### 1. CONFIG_BLK_DEV_INITRD is CRITICAL

The defconfig initially didn't have `CONFIG_BLK_DEV_INITRD=y`. Without this, the kernel cannot load an external initrd from the bootloader. It only supports built-in initramfs via `CONFIG_INITRAMFS_SOURCE`.

**Symptoms:** Kernel boots but reports:
```
check access for rdinit=/init failed: -2, ignoring
VFS: Cannot open root device "" or unknown-block(0,0)
```

**Fix:** Add to defconfig:
```
CONFIG_BLK_DEV_INITRD=y
CONFIG_RD_GZIP=y
CONFIG_RD_BZIP2=y
CONFIG_RD_XZ=y
```

### 2. Initramfs /init Can Be Symlink

BusyBox is a multi-call binary. `/init -> /bin/busybox` works because BusyBox checks `argv[0]` to decide which applet to run. A symlink is preferred over copying (saves ~1.2MB).

### 3. Don't Include Host musl Linker

The alpha initramfs builder was copying `/lib/ld-musl-x86_64.so.1` from the host system. This is unnecessary since BusyBox is statically linked.

### 4. CPIO Format: newc (SVR4 with no CRC)

Linux expects CPIO archives in "newc" format (ASCII headers, 110 bytes each). The command to create manually:
```bash
find . | cpio -o -H newc > initramfs.cpio
```

---

## Key Decisions

- Use Linux 6.x mainline (6.19-rc5 currently)
- Start with x86_64, add aarch64 after
- Keep custom initramfs builder (it works, just needed bug fixes)
- Limine bootloader stays (works with Linux)
- BusyBox for init + shell + utilities

---

## Files Modified

| File | Changes |
|------|---------|
| `linux/` | Git submodule for Linux kernel |
| `linux/arch/x86/configs/levitate_defconfig` | Custom kernel config |
| `xtask/src/build/linux.rs` | Linux build module |
| `xtask/src/build/mod.rs` | Export linux module |
| `xtask/src/build/commands.rs` | Added `Linux` build command |
| `xtask/src/main.rs` | Added `--linux` run flag |
| `xtask/src/run.rs` | Added `linux` parameter |
| `xtask/src/qemu/builder.rs` | Linux kernel path + command line |
| `xtask/src/build/initramfs/builder.rs` | Fixed duplicate `/init` bug |

---

## Remaining Work

- [x] Add Linux kernel submodule
- [x] Create kernel config
- [x] Build system integration
- [x] Test boot
- [ ] OpenRC init system
- [ ] Alpine package manager (apk)
- [ ] aarch64 cross-compilation
- [ ] Limine ISO integration

---

## Commands Reference

```bash
# Build Linux kernel
cargo xtask build linux

# Run with Linux kernel (direct boot, no ISO)
cargo xtask run --linux

# Run with Linux kernel, headless
cargo xtask run --linux --headless

# Build everything including Linux
cargo xtask build all  # Currently builds custom kernel
```
