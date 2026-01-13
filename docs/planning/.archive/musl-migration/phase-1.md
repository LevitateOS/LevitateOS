# Phase 1: Discovery - Migrate from c-gull to musl

## Summary

Replace c-gull (Rust libc) with musl (C libc) as the libc for all userspace programs. This unifies the toolchain for both Rust and C programs.

### Problem
- c-gull requires complex build flags (-Z build-std, custom RUSTFLAGS)
- c-gull cannot build C programs (no headers)
- c-gull has incomplete implementation (wait3, wait4 are TODO stubs)
- Two separate toolchains needed for Rust vs C programs

### Who Benefits
- Anyone building userspace programs
- Anyone debugging libc issues
- Anyone adding new external programs (Rust or C)

### Why musl
- Standard Rust target: `x86_64-unknown-linux-musl`
- Complete libc implementation
- Static linking by default
- Battle-tested (Alpine Linux, many embedded systems)
- Works for both Rust AND C programs

## Success Criteria

1. `cargo xtask build coreutils` works with musl target
2. `cargo xtask build brush` works with musl target
3. `cargo xtask build dash` works with musl (C program)
4. All existing behavior tests pass
5. Build is simpler (fewer flags, no -Z build-std)

## Current State

### What We Have

```
toolchain/
├── c-ward/              # Cloned c-ward repo
├── libc-levitateos/     # Wrapper crate for c-gull
│   ├── Cargo.toml       # Complex feature flags
│   └── src/lib.rs       # Just re-exports c-gull
└── sysroot/
    └── lib/libc.a       # Built from libc-levitateos
```

### Current Build Process (Rust programs)

```bash
# In apps.rs get_sysroot_rustflags()
RUSTFLAGS="-C panic=abort \
           -C relocation-model=pic \
           -C link-arg=-nostartfiles \
           -C link-arg=-static-pie \
           -C link-arg=-Wl,--allow-multiple-definition \
           -C link-arg=-L{sysroot}/lib"

cargo +nightly-2025-04-28 build \
    --target x86_64-unknown-linux-gnu \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort
```

### Problems with Current Approach

1. **Fragile**: Many interacting flags
2. **Nightly-only**: Requires specific nightly for -Z build-std
3. **Incomplete**: c-gull has TODO stubs
4. **Split brain**: Can't use same approach for C

## Target State

### What We Want

```
toolchain/
├── musl-sysroot/        # musl installation
│   ├── lib/
│   │   ├── libc.a       # Static libc
│   │   └── crt*.o       # C runtime objects
│   └── include/         # C headers
└── (c-ward removed)
```

### Target Build Process (Rust programs)

```bash
# Standard musl target - no special flags!
cargo build --release --target x86_64-unknown-linux-musl
```

### Target Build Process (C programs)

```bash
# Same sysroot works for C
CC="clang --target=x86_64-linux-musl --sysroot=toolchain/musl-sysroot"
./configure && make
```

## Files Affected

### Delete
- `toolchain/libc-levitateos/` (entire directory)
- `toolchain/c-ward/` (cloned repo, gitignored)
- `toolchain/sysroot/` (built output, gitignored)

### Modify
- `xtask/src/build/sysroot.rs` → rewrite for musl
- `xtask/src/build/apps.rs` → simplify significantly
- `xtask/src/build/commands.rs` → update build flow
- `.gitignore` → update paths
- `CLAUDE.md` → update build documentation

### Add
- `xtask/src/build/c_apps.rs` → C program support (trivial with musl)

## Constraints

1. **Must support x86_64 and aarch64**
2. **Static linking only** (no dynamic linker yet)
3. **Must not break existing tests**
4. **CI must continue to work**
