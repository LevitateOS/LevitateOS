# TEAM_072: Stage3 → Rootfs Migration

## Mission
Migrate stage3 tarball builder into leviso as the `rootfs` module, eliminating the separate stage3 git submodule.

## Status: COMPLETE

---

## What Changed

### Before
```
LevitateOS/
├── leviso/              # ISO builder
├── stage3/              # Separate git submodule
└── ...
```

### After
```
LevitateOS/
├── leviso/
│   ├── src/
│   │   ├── rootfs/      # Base tarball builder now lives here
│   │   │   ├── mod.rs
│   │   │   ├── builder.rs
│   │   │   ├── binary.rs
│   │   │   ├── context.rs
│   │   │   └── parts/
│   │   │       ├── binaries.rs
│   │   │       ├── etc.rs
│   │   │       ├── filesystem.rs
│   │   │       ├── pam.rs
│   │   │       ├── recipe.rs
│   │   │       └── systemd.rs
│   │   └── ...
│   └── downloads/       # Rocky ISO and rootfs live here
└── ...
```

---

## New Commands

```bash
cd leviso

# Build base tarball
cargo run -- rootfs

# With custom recipe binary
cargo run -- rootfs --recipe /path/to/recipe

# List tarball contents
cargo run -- rootfs-list ./output/levitateos-base.tar.xz

# Verify tarball
cargo run -- rootfs-verify ./output/levitateos-base.tar.xz
```

---

## Naming Decision

Originally named "catalyst" and "stage3" after Gentoo's terminology, but we renamed because:

- **Gentoo stage3** = bootstrapped from source through stage1/2/3
- **LevitateOS** = copies pre-built binaries from Rocky Linux

We're NOT doing Gentoo-style source bootstrapping, so using their terminology was misleading.

**Final naming:**
- Module: `rootfs` (builds a root filesystem tarball)
- Output: `levitateos-base.tar.xz` (not stage3, just a base system)

---

## Decisions Made

1. **Named "rootfs"** - Accurately describes what it builds
2. **Output named "levitateos-base.tar.xz"** - Not stage3 since we're not bootstrapping
3. **Rocky rootfs stays in leviso/downloads** - Single location for downloads
4. **No separate crate** - Rootfs is a module within leviso, not a separate crate
5. **Removed stage3 submodule** - No longer needed

---

## Bug Fixes Included

1. **Host library fallback removed** - `binary.rs` now fails loudly if library not found in rootfs
2. **Recipe path handling improved** - Explicit path must exist, better search for default location
3. **Base tarball included in ISO** - `iso.rs` now copies `levitateos-base.tar.xz` to ISO root

---

## Files Created

- `leviso/src/rootfs/mod.rs`
- `leviso/src/rootfs/builder.rs`
- `leviso/src/rootfs/binary.rs`
- `leviso/src/rootfs/context.rs`
- `leviso/src/rootfs/parts/mod.rs`
- `leviso/src/rootfs/parts/binaries.rs`
- `leviso/src/rootfs/parts/etc.rs`
- `leviso/src/rootfs/parts/filesystem.rs`
- `leviso/src/rootfs/parts/pam.rs`
- `leviso/src/rootfs/parts/recipe.rs`
- `leviso/src/rootfs/parts/systemd.rs`

## Files Removed

- `stage3/` (entire git submodule)
- `rocky-source/` (temporary crate, never committed)

---

## Testing

```bash
cd leviso
cargo check   # Compiles successfully
cargo run -- --help  # Shows rootfs commands
```
