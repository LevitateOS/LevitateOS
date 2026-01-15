# TEAM 490: Arch Linux Extraction Support

## Goal
Create `arch.rs` for consistency with `fedora.rs` - automated extraction of Arch Linux ISO to study both distro internals.

## Context
User wants to study both Fedora and Arch rootfs to create an "Arch-like distro with Fedora internals".

Current state:
- Fedora: Has `fedora.rs` with automated extraction
- Arch: Manual extraction, no code support

## Key Differences

| Aspect | Fedora | Arch |
|--------|--------|------|
| Squashfs location | `LiveOS/squashfs.img` | `arch/x86_64/airootfs.sfs` |
| Compression | zstd | xz |
| Output | `vendor/fedora-root/` | `vendor/arch-root/` (new) |

## Tasks
1. Create `arch.rs` mirroring `fedora.rs` structure
2. Adapt extraction for Arch's `airootfs.sfs` path
3. Clean up manual extractions in `vendor/images/arch-*-extracted/`

## Files Changed
- `crates/builder/src/builder/arch.rs` (new)
- `crates/builder/src/builder/mod.rs` (add arch module)
