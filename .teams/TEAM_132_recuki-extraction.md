# TEAM_132: Extract UKI Builder to Standalone recuki Tool

**Status: COMPLETE**

## Goal
Extract the UKI (Unified Kernel Image) building functionality from `leviso/src/artifact/uki.rs` into a standalone `recuki` tool in `tools/recuki/`.

## Why
- leviso is becoming an orchestrator, not a monolith
- UKI building is self-contained (wraps `ukify`)
- AcornOS needs UKI building too
- Clean separation of concerns

## Current State
- `leviso/src/artifact/uki.rs` - 195 lines
- Depends on `distro_builder::process::Cmd`
- Depends on `distro_spec::levitate::*` constants

## Design

### CLI Interface
```bash
# Single UKI
recuki --kernel vmlinuz --initrd initramfs.img --cmdline "root=LABEL=root rw" -o boot.efi

# With os-release branding
recuki --kernel vmlinuz --initrd initramfs.img --cmdline "..." \
       --os-name "LevitateOS" --os-id levitateos --os-version "1.0" \
       -o levitateos.efi

# Batch mode (from TOML/JSON)
recuki batch entries.toml --kernel vmlinuz --initrd initramfs.img -o output/
```

### Library API
```rust
pub fn build_uki(config: &UkiConfig) -> Result<()>;

pub struct UkiConfig {
    pub kernel: PathBuf,
    pub initrd: PathBuf,
    pub cmdline: String,
    pub output: PathBuf,
    pub os_release: Option<OsRelease>,
}
```

## Implementation Steps
1. [x] Create `tools/recuki/` crate structure
2. [x] Implement core `build_uki()` function
3. [x] Add CLI with clap
4. [x] Update leviso to call `recuki` library (not binary)
5. [x] Add tests

## Files Changed
- NEW: `tools/recuki/Cargo.toml`
- NEW: `tools/recuki/src/lib.rs` - Core UkiConfig and build_uki()
- NEW: `tools/recuki/src/main.rs` - CLI interface
- NEW: `tools/recuki/CLAUDE.md` - Agent instructions
- MODIFY: `leviso/src/artifact/uki.rs` - Now delegates to recuki
- MODIFY: `leviso/Cargo.toml` - Added recuki dependency
- MODIFY: `Cargo.toml` - Added tools/recuki to workspace members

## Result
- `recuki` is a standalone crate usable by both leviso and AcornOS
- leviso's uki.rs is now ~130 lines (down from ~195) and delegates core building
- All tests pass
