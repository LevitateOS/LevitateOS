# TEAM_103: AcornOS Preparation and Code Sharing

**Started:** 2026-01-24
**Status:** COMPLETED

## Goal

Prepare AcornOS as a sibling distribution to LevitateOS with:
- OpenRC (instead of systemd)
- musl (instead of glibc)
- busybox (instead of GNU coreutils)
- Alpine packages (instead of Rocky RPMs)

## Scope

This task creates a **foundation, not a complete system**. Resolution:
- distro-spec/acorn: HIGH (small surface, can be complete)
- distro-builder extraction: LOW (structural only)
- AcornOS crate: SKELETON (directory structure only)

## Phase 1: Perfect distro-spec/acorn/

### Changes Made
- [x] boot.rs - Added 4 kernel modules (sd_mod, nvme, ahci, virtio_pci) with .ko.gz extension
- [x] paths.rs - Fixed QEMU settings (4GB RAM, 20GB disk) to match LevitateOS
- [x] services.rs - No changes needed (boot essentials only, desktop services are future work)

## Phase 2: Extract Shared Library (distro-builder)

- [x] Create distro-builder crate structure
- [x] Define component system (Op enum, Installable trait, Phase enum)
- [x] Create artifact builder interfaces (squashfs, initramfs, iso - placeholder)
- [x] Extract build utilities (context traits, filesystem operations)
- [x] Add preflight checks (host tool validation)
- [x] Ensure leviso still builds

## Phase 3: Create AcornOS Skeleton

- [x] Create AcornOS crate with CLI stub (status command works)
- [x] Create placeholder modules (config, extract, component)
- [x] Implement DistroConfig trait using distro-spec::acorn
- [x] Create AcornOS CLAUDE.md

## Summary

### What Was Created

1. **distro-spec/acorn/** perfected:
   - boot.rs: 12 kernel modules (was 8)
   - paths.rs: QEMU 4GB RAM, 20GB disk (was 2GB/10GB)

2. **distro-builder crate** (structural skeleton):
   - `component/mod.rs`: Installable trait, Phase enum, generic Op variants
   - `build/context.rs`: DistroConfig and BuildContext traits
   - `build/filesystem.rs`: FHS structure utilities (with tests)
   - `artifact/`: Placeholder interfaces for squashfs/initramfs/iso
   - `preflight/mod.rs`: Host tool checking

3. **AcornOS crate** (skeleton):
   - CLI with build/initramfs/iso/run/download/extract/status commands
   - config.rs: AcornConfig implementing DistroConfig
   - extract.rs: Placeholder for Alpine APK extraction
   - component/mod.rs: OpenRC-specific operation types

### What's Still Needed (Future Work)

- Alpine APK extraction implementation
- OpenRC service setup during build
- mdev vs eudev decision and implementation
- Desktop services (dbus, elogind)
- Complete component definitions
- Testing infrastructure for AcornOS

## Log

- 2026-01-24: Created team file, starting Phase 1
- 2026-01-24: Phase 1 complete - distro-spec/acorn updated
- 2026-01-24: Phase 2 complete - distro-builder crate created
- 2026-01-24: Phase 3 complete - AcornOS skeleton created
- 2026-01-24: All phases complete, workspace builds successfully
