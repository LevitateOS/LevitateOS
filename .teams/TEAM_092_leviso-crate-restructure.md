# TEAM_092: Leviso Crate Restructure

## Status: COMPLETED

## Goal
Restructure leviso (9,155 LOC, 30 files) by:
1. Creating workspace Cargo.toml at monorepo root
2. Extracting `leviso-deps` crate (dependency resolution)
3. Extracting `leviso-elf` crate (ELF binary utilities)
4. Splitting 1,283-line `custom.rs` god module into 7 focused files
5. Restructuring artifacts into artifact/ directory
6. Splitting main.rs into commands/ directory

## Phases
- [x] Phase 1: Create leviso-deps crate (~1,500 LOC)
- [x] Phase 2: Create leviso-elf crate (~300 LOC)
- [x] Phase 3: Split custom.rs into 7 files (~1,283 LOC → 7 files)
- [x] Phase 4: Restructure artifacts (initramfs, squashfs, iso → artifact/)
- [x] Phase 5: Split main.rs (commands → commands/)
- [x] Phase 6: Cleanup and verification

## Completed Work

### New Crates Created
```
LevitateOS/
├── Cargo.toml                    # Workspace root
├── leviso-deps/                  # Dependency resolution
│   └── src/
│       ├── lib.rs               # DependencyResolver
│       ├── download.rs          # HTTP/torrent downloads
│       ├── linux.rs             # Linux kernel source
│       ├── rocky.rs             # Rocky ISO downloads
│       └── tools.rs             # recstrap/recfstab/recchroot
└── leviso-elf/                   # ELF utilities
    └── src/
        ├── lib.rs               # Public exports
        ├── analyze.rs           # readelf parsing
        ├── copy.rs              # File/library copying
        └── paths.rs             # Binary/library paths
```

### Leviso Internal Restructure
```
leviso/src/
├── main.rs              # CLI definitions only (~225 LOC)
├── lib.rs               # Public exports for testing
├── rebuild.rs           # Rebuild detection logic
│
├── artifact/            # Build artifacts
│   ├── mod.rs
│   ├── initramfs.rs     # Tiny initramfs builder
│   ├── squashfs.rs      # Squashfs packing
│   └── iso.rs           # ISO creation
│
├── commands/            # CLI command handlers
│   ├── mod.rs
│   ├── build.rs         # Build command
│   ├── run.rs           # QEMU run command
│   ├── clean.rs         # Clean command
│   ├── show.rs          # Show command
│   ├── download.rs      # Download command
│   ├── extract.rs       # Extract command
│   └── preflight.rs     # Preflight command
│
├── component/           # Declarative build system
│   ├── mod.rs
│   ├── builder.rs
│   ├── definitions.rs
│   ├── executor.rs
│   └── custom/          # Split from 1,283-line god module
│       ├── mod.rs
│       ├── filesystem.rs
│       ├── firmware.rs
│       ├── modules.rs
│       ├── etc.rs
│       ├── pam.rs
│       ├── packages.rs
│       └── live.rs
│
└── build/               # Build utilities
    ├── mod.rs
    ├── context.rs
    ├── kernel.rs
    ├── libdeps.rs
    └── users.rs
```

### Files Removed
- leviso/src/deps/ (entire directory - moved to leviso-deps)
- leviso/src/common/ (entire directory - moved to leviso-elf)
- leviso/src/component/custom.rs (split into custom/ directory)
- leviso/src/initramfs/ (moved to artifact/initramfs.rs)
- leviso/src/squashfs/ (merged into artifact/squashfs.rs)
- leviso/src/iso.rs (moved to artifact/iso.rs)

## Verification
```bash
cargo build --workspace     # ✓ Passes
cargo test -p leviso --lib  # ✓ 14 tests pass
cargo test -p leviso-deps   # ✓ 20 tests pass
cargo clippy -p leviso      # ✓ No errors
```

## Progress Log
- 2026-01-23: Started restructure
- 2026-01-23: Phase 1 complete - leviso-deps crate created
- 2026-01-23: Phase 2 complete - leviso-elf crate created
- 2026-01-23: Phase 3 complete - custom.rs split into 7 focused modules
- 2026-01-23: Phase 4 complete - artifact/ directory created
- 2026-01-23: Phase 5 complete - commands/ directory created, main.rs slimmed
- 2026-01-23: Phase 6 complete - all tests pass, structure verified
