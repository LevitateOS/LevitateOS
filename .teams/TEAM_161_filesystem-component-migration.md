# TEAM_161: Filesystem Component Migration (Op Enum)

**Date:** 2026-02-10
**Status:** ✅ COMPLETE — First component successfully migrated

## Achievement

Migrated the Filesystem component from custom operations to declarative Op enum pattern, reducing LoC and improving alignment with distro-builder.

## Changes

### Replaced
- `CustomOp::CreateFhsSymlinks` → 7 × `Op::Symlink()` operations
- Removed `src/component/custom/filesystem.rs` (~75 LoC)
- Removed dispatch in `custom/mod.rs` for CreateFhsSymlinks

### New Component Definition (definitions.rs:63-77)
```rust
pub static FILESYSTEM: Component = Component {
    name: "filesystem",
    phase: Phase::Filesystem,
    ops: &[
        dirs(FHS_DIRS),
        // Merged /usr symlinks
        symlink("bin", "usr/bin"),
        symlink("sbin", "usr/sbin"),
        symlink("lib", "usr/lib"),
        symlink("lib64", "usr/lib64"),
        // /var symlinks to /run
        symlink("var/run", "/run"),
        symlink("var/lock", "/run/lock"),
        // Shell
        symlink("usr/bin/sh", "bash"),
    ],
};
```

**Reduction:** 75+ LoC deleted, component definition more readable

## Testing

✅ All 106 unit tests pass
✅ Clippy clean
✅ Leviso builds successfully
✅ ISO boots successfully in QEMU

## Pattern (For Remaining Components)

Following TEAM_160 strategy:
1. **Phase 1: Replace custom ops with Op enum** ← We are here
2. **Phase 2: Gradually migrate remaining components** (Packages, Firmware, Modules, PAM, Services)
3. **Phase 3: Delete custom operation dispatchers** (as components are migrated)

## Next Components (Priority Order)

| Component | LoC | Complexity | Pattern |
|-----------|-----|-----------|---------|
| Packages | 80 | Simple | Recipe calls → Op::Custom or wrapper |
| Firmware | 120 | Medium | CopyTree ops |
| Modules | 90 | Simple | CopyTree + config |
| PAM | 150 | Medium | WriteFile ops for config |
| Services | 200+ | Complex | Multiple operations |
| Binaries | 300+ | Complex | Library deps (keep custom) |

## Alignment with Consolidation Goal

- ✅ Op enum aligns with distro-builder pattern
- ✅ Reduces custom operation count
- ✅ Makes leviso components more declarative
- ✅ Step toward "thin wrapper" architecture

## Files Modified

- `leviso/src/component/definitions.rs` — FILESYSTEM component definition
- `leviso/src/component/mod.rs` — Removed CustomOp::CreateFhsSymlinks variant
- `leviso/src/component/custom/mod.rs` — Removed dispatch case
- `leviso/src/component/custom/filesystem.rs` — DELETED
- Test files fixed for missing fs imports

## Commits

1. `feat: migrate filesystem component from custom ops to Op enum` — 7 file changes, 116 deletions

---

**Status:** Ready to migrate next component (Packages)
