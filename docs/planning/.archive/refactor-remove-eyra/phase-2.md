# Phase 2: Structural Extraction

## Core Principle: External Dependencies

**Coreutils and brush are NOT part of this repo.** They are external projects that get:
1. Cloned at build time (like npm install)
2. Built against our sysroot
3. Gitignored (not committed)

This is the same model as:
- Go modules (downloaded to GOPATH)
- npm packages (downloaded to node_modules)
- Cargo dependencies (downloaded to ~/.cargo)

**Why?**
- We don't maintain forks
- Upstream updates are automatic (re-clone to update)
- Repo stays small and focused on kernel/toolchain
- Clear separation: "our code" vs "external code"

## Target Design

### New Directory Layout

```
LevitateOS/
├── crates/
│   └── userspace/
│       ├── init/                   # Keep: no-std init process
│       ├── shell/                  # Keep: no-std built-in shell
│       ├── libsyscall/             # MOVE from eyra/ - raw syscall wrapper
│       └── [DELETED: eyra/]        # Remove entirely
│
├── toolchain/
│   │
│   │ # === COMMITTED (our code) ===
│   ├── libc-levitateos/            # Wrapper crate → libc.a
│   │   ├── Cargo.toml
│   │   ├── src/lib.rs
│   │   └── rust-toolchain.toml
│   ├── build-sysroot.sh            # Build libc.a
│   ├── build-coreutils.sh          # Build unmodified coreutils
│   ├── build-brush.sh              # Build unmodified brush
│   │
│   │ # === GITIGNORED (external, cloned at build time) ===
│   ├── c-ward/                     # git clone https://github.com/sunfishcode/c-ward
│   ├── coreutils/                  # git clone https://github.com/uutils/coreutils
│   ├── brush/                      # git clone https://github.com/reubeno/brush
│   │
│   │ # === GITIGNORED (build outputs) ===
│   ├── sysroot/lib/libc.a          # Built from libc-levitateos
│   ├── coreutils-out/              # Built coreutils binaries
│   └── brush-out/                  # Built brush binaries
│
├── tests/
│   ├── syscall_conformance/        # MOVE from eyra/syscall-conformance
│   └── userspace_integration.rs    # RENAME from eyra_integration_test.rs
│
└── xtask/
    └── src/
        └── build/
            └── commands.rs         # Update build commands
```

### Module Responsibilities

| Module | Responsibility | Dependencies |
|--------|---------------|--------------|
| `toolchain/libc-levitateos/` | Build c-gull as static library | c-ward (cloned) |
| `toolchain/sysroot/` | Pre-built libc for linking | libc-levitateos output |
| `crates/userspace/libsyscall/` | Raw syscall wrappers (no libc) | None |
| `xtask build sysroot` | Build sysroot | toolchain/libc-levitateos |
| `xtask build coreutils` | Build external coreutils | sysroot |

## Extraction Strategy

### Order of Operations

1. **First**: Move `libsyscall` out of eyra (no Eyra dependency)
2. **Second**: Update xtask to use toolchain/ for userspace builds
3. **Third**: Delete `crates/userspace/eyra/` entirely
4. **Fourth**: Clean up test files

### Coexistence Period

During migration, both paths should work temporarily:
- Old: `cargo xtask build eyra` (deprecated, prints warning)
- New: `cargo xtask build sysroot` + `cargo xtask build coreutils`

This allows incremental testing.

## New xtask Commands

### `cargo xtask build sysroot`
```rust
/// Build c-gull sysroot (libc.a and symlinks)
pub fn build_sysroot() -> Result<()> {
    // 1. Clone c-ward if not present
    // 2. Build libc-levitateos
    // 3. Copy to sysroot/lib/libc.a
    // 4. Create symlinks
}
```

### `cargo xtask build coreutils`
```rust
/// Build unmodified uutils/coreutils against sysroot
pub fn build_coreutils(arch: &str) -> Result<()> {
    // 1. Clone uutils/coreutils if not present
    // 2. Build with sysroot RUSTFLAGS
    // 3. Output to toolchain/coreutils-out/
}
```

### `cargo xtask build brush`
```rust
/// Build unmodified brush shell against sysroot
pub fn build_brush(arch: &str) -> Result<()> {
    // 1. Clone brush-shell if not present
    // 2. Build with sysroot RUSTFLAGS
    // 3. Output to toolchain/brush-out/
}
```

## File Size Targets (Rule 7)

| File | Current Lines | Target Lines | Action |
|------|--------------|--------------|--------|
| `xtask/src/build/commands.rs` | ~600 | <500 | Split sysroot logic to separate module |

### New Files

```
xtask/src/build/
├── commands.rs      # Main build commands
├── sysroot.rs       # NEW: Sysroot build logic
└── external.rs      # NEW: External project builds (coreutils, brush)
```

## RUSTFLAGS for External Projects

All external projects use the same flags:
```bash
export RUSTFLAGS="-C panic=abort \
    -C link-arg=-nostartfiles \
    -C link-arg=-static \
    -C link-arg=-Wl,--allow-multiple-definition \
    -C link-arg=-L${SYSROOT}/lib"

cargo +nightly-2025-04-28 build --release \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target ${TARGET}
```

Where:
- `${SYSROOT}` = `toolchain/sysroot`
- `${TARGET}` = `x86_64-unknown-linux-gnu` or `aarch64-unknown-linux-gnu`
