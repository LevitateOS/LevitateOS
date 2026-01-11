# TEAM_433: Refactor Remove Eyra

## Objective

Remove the Eyra dependency from LevitateOS userspace and replace it with the c-gull sysroot approach, enabling truly **UNMODIFIED** upstream projects to run.

## Status: PLANNING

## Background

### Why Remove Eyra?

Eyra requires modifying every application:
- Add `std = { package = "eyra" }` to Cargo.toml
- Add `build.rs` with `-nostartfiles`
- Use forked repos with Eyra patches

This violates the core goal: **run any Unix program without modification**.

### The c-gull Alternative

c-gull provides a pre-built `libc.a` sysroot that any Rust program can link against:
- No source modifications
- Standard Linux target (`x86_64-unknown-linux-gnu`)
- Build with RUSTFLAGS pointing to sysroot
- Already proven working with unmodified uutils/coreutils

## Plan Location

Full plan at: `docs/planning/refactor-remove-eyra/`

| Phase | File | Description |
|-------|------|-------------|
| 1 | `phase-1.md` | Discovery & Safeguards |
| 2 | `phase-2.md` | Structural Extraction |
| 3 | `phase-3.md` | Migration |
| 4 | `phase-4.md` | Cleanup |
| 5 | `phase-5.md` | Hardening |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Delete vs deprecate | Delete entirely | No shims (Rule 5) |
| libsyscall | Move to crates/userspace/ | Still useful for no-std code |
| **External projects** | **Clone at build time** | **Not vendored in repo** |
| Custom target | None | Linux ABI compat |

### Externalization Model

**coreutils and brush are NOT part of this repo.** They are external dependencies:

```
toolchain/
├── coreutils/     # gitignored, `git clone` at build time
├── brush/         # gitignored, `git clone` at build time
└── c-ward/        # gitignored, `git clone` at build time
```

This is like:
- npm's `node_modules/` (downloaded, gitignored)
- Go's module cache (downloaded, external)
- Cargo's `~/.cargo/` (downloaded, external)

**Benefits**:
- No fork maintenance
- Upstream updates = re-clone
- Repo stays focused on kernel code
- Clear "our code" vs "external code" boundary

### Deployment: Static Linking

Binaries are **statically linked** with c-gull - no dynamic linker needed:

```
toolchain/coreutils/          # cloned source
    ↓ cargo build --release (sysroot RUSTFLAGS)
toolchain/coreutils-out/      # 2.4MB static ELF
    ↓ copy to initrd_root/
initramfs.cpio                # packed into kernel image
    ↓ at boot
/coreutils                    # runs directly, no ld.so
```

**Future**: Add `ld.so` for dynamic linking (separate epic)

## What Gets Deleted

```
crates/userspace/eyra/           # Entire directory
├── brush/                       # Rebuild against c-gull
├── coreutils/                   # Use unmodified upstream
├── eyra-hello/                  # Test binary, not needed
├── eyra-test-runner/            # Not needed
├── cgull-test/                  # Temporary test
├── libsyscall-tests/            # Merge into tests/
└── syscall-conformance/         # Move to tests/
```

## What Gets Added/Updated

```
toolchain/
├── build-sysroot.sh             # Build libc.a
├── build-coreutils.sh           # Exists, update paths
└── build-brush.sh               # NEW

xtask/src/build/
├── sysroot.rs                   # NEW
└── external.rs                  # NEW
```

## Progress Log

### Session 1 (2026-01-11)
- Created 5-phase refactor plan
- Documented in `docs/planning/refactor-remove-eyra/`
- No code changes yet (planning only)

## Remaining Work

- [ ] Phase 3: Execute migration steps
- [ ] Phase 4: Cleanup dead code
- [ ] Phase 5: Final verification

## Dependencies

- TEAM_430: c-gull toolchain (completed - sysroot builds)
- Requires: c-ward cloned at toolchain/c-ward/
- Requires: nightly-2025-04-28 toolchain

## Gotchas

1. **Git submodule**: Must `git submodule deinit` before deleting eyra/coreutils
2. **aarch64**: May need additional linker flags for cross-compilation
3. **Missing libc**: Some coreutils (ls, date) need functions c-gull doesn't have yet
