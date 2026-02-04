# TEAM_133: Extract ISO Builder to Standalone reciso Tool

**Status: COMPLETE**

## Goal
Extract ISO building functionality into a standalone `reciso` tool that can create bootable ISOs from kernel + initramfs + rootfs inputs.

## Why
- leviso is becoming an orchestrator, not a monolith
- ISO creation is self-contained (wraps xorriso + mtools)
- AcornOS needs ISO building too
- `distro-builder` already has low-level utilities, but no CLI

## Current State
- `leviso/src/artifact/iso.rs` - 424 lines (LevitateOS-specific orchestration)
- `distro-builder/src/artifact/iso_utils.rs` - 284 lines (generic utilities)
- Generic utilities already exist, need CLI wrapper

## Design

### CLI Interface
```bash
# Basic ISO (UEFI boot with UKIs)
reciso --kernel vmlinuz --initrd initramfs.img --rootfs rootfs.erofs \
       --label LEVITATEOS -o output.iso

# With pre-built UKIs
reciso --kernel vmlinuz --initrd initramfs.img --rootfs rootfs.erofs \
       --uki levitateos-live.efi --uki levitateos-emergency.efi \
       --label LEVITATEOS -o output.iso

# Build UKIs inline
reciso --kernel vmlinuz --initrd initramfs.img --rootfs rootfs.erofs \
       --build-uki "LevitateOS::levitateos-live.efi" \
       --build-uki "Emergency:emergency:levitateos-emergency.efi" \
       --os-name LevitateOS --os-id levitateos --os-version 1.0 \
       --label LEVITATEOS -o output.iso

# With live overlay
reciso --kernel vmlinuz --initrd initramfs.img --rootfs rootfs.erofs \
       --overlay live-overlay/ \
       --label LEVITATEOS -o output.iso
```

### Library API
```rust
pub fn create_iso(config: &IsoConfig) -> Result<PathBuf>;

pub struct IsoConfig {
    pub kernel: PathBuf,
    pub initrd: PathBuf,
    pub rootfs: PathBuf,
    pub label: String,
    pub output: PathBuf,
    pub ukis: Vec<UkiSource>,
    pub overlay: Option<PathBuf>,
    pub os_release: Option<OsRelease>,
}

pub enum UkiSource {
    Prebuilt(PathBuf),
    Build { name: String, extra_cmdline: String, filename: String },
}
```

## Implementation Steps
1. [x] Create `tools/reciso/` crate structure
2. [x] Implement core `IsoConfig` and `create_iso()`
3. [x] Add CLI with clap
4. [x] Update leviso to call `reciso` library
5. [x] Add tests

## Files Changed
- NEW: `tools/reciso/Cargo.toml`
- NEW: `tools/reciso/src/lib.rs` - Core IsoConfig and create_iso()
- NEW: `tools/reciso/src/main.rs` - CLI interface
- NEW: `tools/reciso/CLAUDE.md` - Agent instructions
- MODIFY: `leviso/src/artifact/iso.rs` - Now delegates to reciso::create_iso()
- MODIFY: `leviso/Cargo.toml` - Added reciso dependency
- MODIFY: `Cargo.toml` - Added tools/reciso to workspace members

## Result
- `reciso` is a standalone crate usable by both leviso and AcornOS
- leviso's iso.rs is now ~285 lines (down from ~424) and delegates core ISO building
- LevitateOS-specific logic (UKI entries, live overlay, hardware checks) stays in leviso
- All tests pass
