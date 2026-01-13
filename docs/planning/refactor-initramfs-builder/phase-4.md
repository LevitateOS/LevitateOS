# Phase 4: Cleanup - Initramfs Builder Refactor

## Dead Code Removal (Rule 6)

### Files to Delete

| File | Reason | Size |
|------|--------|------|
| `scripts/make_initramfs.sh` | References non-existent paths, unused since TEAM_198 | 37 lines |
| `xtask/src/build/apps.rs` | `APPS` array empty, no external apps used | 288 lines |
| `xtask/src/build/initramfs_old.rs` | Old implementation after migration verified | 184 lines |
| `xtask/initrd_resources/` | Scripts moved to `initramfs/scripts/` | directory |
| `initramfs/lib/` | Unknown purpose, orphaned directory | directory |

### Embedded Strings to Remove

After files are externalized, remove:
```rust
// xtask/src/build/initramfs.rs (old file, being deleted)
const TEST_SH: &str = include_str!("../../initrd_resources/test.sh");
const TEST_CORE_SH: &str = include_str!("../../initrd_resources/test-core.sh");
```

### Build Artifacts to Clean Up

Update `xtask/src/support/clean.rs` to handle new locations:

```rust
// Remove these from ARTIFACTS list (legacy locations)
// "initramfs_aarch64.cpio",
// "initramfs_x86_64.cpio",
// "initramfs_test.cpio",

// Add cleanup for new location
// "target/initramfs/",

// Still clean staging if it exists
// "initrd_root/",
```

## Temporary Adapters to Remove

### Legacy Path Copy

In `xtask/src/build/initramfs/mod.rs`, remove the backward-compat copy:

**Before (Phase 3):**
```rust
pub fn create_busybox_initramfs(arch: &str) -> Result<()> {
    let output = build_initramfs(arch)?;

    // REMOVE THIS: Copy to legacy location at repo root
    let legacy_path = format!("initramfs_{arch}.cpio");
    std::fs::copy(&output, &legacy_path)?;

    Ok(())
}
```

**After (Phase 4):**
```rust
pub fn create_busybox_initramfs(arch: &str) -> Result<()> {
    build_initramfs(arch)?;
    Ok(())
}
```

### Update All Call Sites to Use New Path

Instead of copying to legacy location, update callers to use `target/initramfs/{arch}.cpio`:

| File | Change |
|------|--------|
| `xtask/src/tests/behavior.rs:122` | `format!("target/initramfs/{arch}.cpio")` |
| `xtask/src/tests/serial_input.rs:60` | `"target/initramfs/aarch64.cpio"` |
| `xtask/src/tests/shutdown.rs:67` | `"target/initramfs/aarch64.cpio"` |
| `xtask/src/tests/keyboard_input.rs:65` | `"target/initramfs/aarch64.cpio"` |
| `xtask/src/qemu/builder.rs:120-121` | Use new paths |
| `xtask/src/build/iso.rs:60` | `format!("target/initramfs/{arch}.cpio")` |
| `xtask/src/vm/exec.rs:156` | `"target/initramfs/aarch64.cpio"` |
| `xtask/src/support/clean.rs:35-37` | Update cleanup paths |

## Encapsulation Tightening

### Make Internal Types Private

```rust
// cpio.rs - entry struct is implementation detail
pub(crate) struct CpioEntry { ... }

// manifest.rs - only Manifest needs to be public
pub struct Manifest { ... }
pub(crate) struct Meta { ... }
pub(crate) struct Layout { ... }
pub(crate) struct Binary { ... }
pub(crate) enum FileEntry { ... }
```

### Seal the Public API

The only public exports from `initramfs/mod.rs`:

```rust
//! Initramfs builder module

mod builder;
mod cpio;
mod manifest;

// Public API - just two functions
pub fn build_initramfs(arch: &str) -> anyhow::Result<std::path::PathBuf>;
pub fn create_busybox_initramfs(arch: &str) -> anyhow::Result<()>;

// Internal use only
pub(crate) use manifest::Manifest;
pub(crate) use builder::InitramfsBuilder;
```

## File Size Check

| File | Lines | Status |
|------|-------|--------|
| `initramfs/mod.rs` | ~30 | OK |
| `initramfs/cpio.rs` | ~150 | OK |
| `initramfs/manifest.rs` | ~80 | OK |
| `initramfs/builder.rs` | ~120 | OK |
| **Total** | ~380 | Well under 500/file |

## Update Build Module Exports

**`xtask/src/build/mod.rs`:**

```rust
//! Build commands module

mod commands;
mod initramfs;  // Now a directory, not file
mod iso;
mod kernel;
mod orchestration;
mod userspace;

pub mod busybox;    // Keep - not initramfs-specific
pub mod c_apps;     // Keep
pub mod sysroot;    // Keep

// REMOVED: pub mod apps;  (dead code)

pub use commands::BuildCommands;
pub use initramfs::create_busybox_initramfs;  // Same export, new impl
pub use iso::{build_iso, build_iso_test, build_iso_verbose};
pub use orchestration::{build_all, build_kernel_only, build_kernel_verbose};
pub use userspace::build_userspace;
```

## Cleanup Checklist

- [ ] Delete `scripts/make_initramfs.sh`
- [ ] Delete `xtask/src/build/apps.rs`
- [ ] Delete `xtask/src/build/initramfs_old.rs` (if kept during migration)
- [ ] Delete `xtask/initrd_resources/` directory
- [ ] Delete `initramfs/lib/` directory (investigate first)
- [ ] Remove `include_str!` macros from deleted file
- [ ] Update `xtask/src/build/mod.rs` to remove `apps` export
- [ ] Remove legacy path copy from `create_busybox_initramfs`
- [ ] Update all call sites to use `target/initramfs/` path
- [ ] Update `clean.rs` to clean new artifact locations
- [ ] Make internal types `pub(crate)` instead of `pub`
- [ ] Remove old CPIO files from repo root (if present)
- [ ] Update `.gitignore` if needed
- [ ] Verify all tests still pass
- [ ] Verify `cargo xtask build all` works
- [ ] Verify `cargo xtask run` works
