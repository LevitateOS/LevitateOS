# TEAM_113: AcornOS Build Pipeline

**Status:** COMPLETE (code written, needs end-to-end testing)
**Started:** 2026-01-24
**Goal:** Implement squashfs → initramfs → ISO → QEMU pipeline for AcornOS

## What We Built

Complete AcornOS build pipeline to create a bootable ISO:
- `acornos build squashfs` - Compress rootfs into filesystem.squashfs
- `acornos initramfs` - Create tiny initramfs (~5MB) that mounts squashfs
- `acornos iso` - Package kernel + initramfs + squashfs into bootable ISO
- `acornos run` - Boot ISO in QEMU

## Files Changed

### New Files
- `AcornOS/src/artifact/mod.rs` - Artifact module
- `AcornOS/src/artifact/squashfs.rs` - Squashfs builder
- `AcornOS/src/artifact/initramfs.rs` - Initramfs builder
- `AcornOS/src/artifact/iso.rs` - ISO builder
- `AcornOS/src/qemu.rs` - QEMU runner
- `AcornOS/src/cache.rs` - Hash-based caching (SHA256)
- `AcornOS/src/rebuild.rs` - Rebuild detection logic
- `AcornOS/src/timing.rs` - Build timing utilities
- `AcornOS/profile/init_tiny.template` - Init script for initramfs (OpenRC)

### Modified Files
- `AcornOS/src/lib.rs` - Add artifact, qemu, cache, rebuild, timing modules
- `AcornOS/src/main.rs` - Wire up commands with rebuild detection + timing
- `AcornOS/Cargo.toml` - Add sha2 dependency
- `AcornOS/ROADMAP.md` - Update implementation status

## Key Differences from leviso

| Aspect | leviso | AcornOS |
|--------|--------|---------|
| Init system | systemd | OpenRC |
| Init binary | `/lib/systemd/systemd` | `/sbin/openrc-init` |
| Boot modules | `.ko.xz` (Rocky) | `.ko.gz` (Alpine) |
| ISO label | `LEVITATEOS` | `ACORNOS` |
| Kernel | Custom built | Alpine `linux-lts` from rootfs |

## Progress

- [x] Read existing leviso patterns
- [x] Read distro-spec/acorn constants
- [x] Create artifact/squashfs.rs
- [x] Create artifact/initramfs.rs
- [x] Create profile/init_tiny.template
- [x] Create artifact/iso.rs
- [x] Create qemu.rs
- [x] Wire up commands in main.rs
- [x] Compilation passes clean
- [x] Test: `acornos download` (requires network)
- [x] Test: `acornos extract` (packages installed, chroot warning non-critical)
- [x] Test: `acornos build squashfs` (677 MB)
- [x] Test: `acornos initramfs` (1.1 MB)
- [x] Test: `acornos iso` (708 MB)
- [x] Test: `acornos run` (QEMU launches, GRUB menu displays)
- [x] Add: `acornos test` (automated boot verification)

## Usage

```bash
cd AcornOS
cargo run -- status           # Check current state
cargo run -- download         # Download Alpine ISO (~1GB)
cargo run -- extract          # Extract ISO, create rootfs
cargo run -- build squashfs   # Create filesystem.squashfs
cargo run -- initramfs        # Create initramfs
cargo run -- iso              # Create bootable ISO
cargo run -- run              # Boot in QEMU (GUI)
cargo run -- test             # Boot in QEMU (headless, verify login prompt)
```

Or do a full build:
```bash
cargo run -- build  # Does squashfs + initramfs + iso
```

## Automated Testing

`acornos test` boots the ISO headless via serial console and watches for:

**Success patterns** (test passes if any match):
- `login:` - Getty prompt reached
- `Welcome to Alpine Linux`
- `openrc-init`

**Failure patterns** (test fails immediately):
- `Kernel panic`
- `VFS: Cannot open root device`
- `SQUASHFS error`
- etc.

**Stall detection**: Fails if no output for 30s (system hung).

### Shared Testing Infrastructure

The `testing/install-tests/` crate provides the same pattern-based boot verification
for LevitateOS. Both distros use:

| Component | Location | Purpose |
|-----------|----------|---------|
| Serial console | QEMU `-nographic -serial mon:stdio` | Capture boot output |
| Pattern matching | Success/failure pattern lists | Detect boot result |
| Stall detection | No output timeout | Catch hung boots |
| Stage tracking | UEFI→kernel→init | Better error context |

Future: The pattern matching could be extracted to `distro-builder/` and shared.
