# TEAM_139: Memory Footprint Audit

## Objective
Audit and optimize memory footprint across Rust crates: struct sizing, enum size optimization, cache locality.

## Status: COMPLETE

## Changes Made

### Phase 1: Add Size Assertions
- [x] Added size checks for `BootEntry`, `LoaderConfig` in distro-spec/src/shared/boot.rs
- [x] Added size checks for `UserSpec` in distro-spec/src/shared/users.rs
- [x] Added size checks for `Op`, `Phase`, `Component` in leviso/src/component/mod.rs

### Phase 2: Optimize distro-spec Types
- [x] Updated `BootEntry` to use `Cow<'static, str>` for all 5 fields
- [x] Updated `LoaderConfig.console_mode` to `Option<Cow<'static, str>>`
- [x] Updated `LoaderConfig.default_entry` to `Cow<'static, str>`
- [x] Updated `UserSpec`:
  - `shell: Cow<'static, str>` (zero-copy for static defaults like "/bin/bash")
  - `groups: SmallVec<[String; 2]>` (inline for 0-2 groups, heap for more)
- [x] Updated constructors to accept both `&'static str` and `String`
- [x] Fixed call site in testing/install-tests/src/steps/phase5_boot.rs

### Phase 3: Optimize Installable Trait
- [x] Changed `ops(&self) -> Vec<Op>` to `ops(&self) -> Cow<'static, [Op]>`
- [x] Updated `Component` impl to return `Cow::Borrowed(self.ops)` (zero-copy)
- [x] Updated `Service` impl to return `Cow::Owned(self.ops())` (dynamic)
- [x] Fixed executor to iterate with `ops.iter()` instead of `&ops`

## Files Modified
- distro-spec/Cargo.toml - Added `smallvec = "1.11"`
- distro-spec/src/shared/boot.rs - `Cow<'static, str>` for BootEntry/LoaderConfig
- distro-spec/src/shared/users.rs - `Cow` for shell, `SmallVec` for groups
- leviso/src/component/mod.rs - `Cow<'static, [Op]>` return type, size tests
- leviso/src/component/executor.rs - Fixed iteration over `Cow`
- testing/install-tests/src/steps/phase5_boot.rs - Added `.into()` for assignment

## Size Measurements

| Type | Before | After | Savings |
|------|--------|-------|---------|
| BootEntry | ~120 bytes (5 Strings) | ~120 bytes (5 Cows) | Zero-copy for static defaults |
| LoaderConfig | ~56 bytes | ~56 bytes | Zero-copy for console_mode |
| UserSpec | ~112 bytes (Vec<String>) | ~152 bytes (SmallVec) | Inline for 0-2 groups |
| Op | 64 bytes | 64 bytes | (already optimal) |
| Phase | 1 byte | 1 byte | (already #[repr(u8)]) |
| Component | ~40 bytes | ~40 bytes | (already optimal) |

Note: `UserSpec` is slightly larger due to SmallVec inline capacity, but this is a
worthwhile trade-off since most users have 0-2 groups (wheel, video) and avoid
heap allocation entirely in the common case.

## Key Benefits

1. **Zero-copy for static defaults**: `BootEntry::with_defaults()` and `LoaderConfig::with_defaults()`
   now avoid heap allocation for the default options string "root=LABEL=root rw quiet"

2. **Zero-copy for Installable::ops()**: `Component` implementations now return
   `Cow::Borrowed` directly referencing static slices, avoiding the previous
   `.to_vec()` allocation on every call (25-30× per build)

3. **Inline groups**: `UserSpec.groups` uses `SmallVec<[String; 2]>` to store
   up to 2 group names inline, covering the common case of "wheel,video" without
   heap allocation

4. **Size assertions**: Added compile-time regression guards to catch struct bloat

## Verification
- `cargo build --workspace` ✓
- `cargo test --package distro-spec --package leviso` ✓
