# Phase 1: Discovery & Safeguards

## Refactor Summary

**What**: Remove the Eyra dependency from LevitateOS userspace and replace it with c-gull sysroot approach.

**Why**:
- Eyra requires modifying every application (adding `std = { package = "eyra" }` dependency)
- This violates the goal of running **UNMODIFIED** programs
- c-gull provides a pre-built sysroot (`libc.a`) that any Rust program can link against
- This enables building unmodified upstream projects (uutils/coreutils, brush shell)

**Pain Points with Current Eyra Approach**:
1. Every userspace crate needs `std = { package = "eyra" }` in Cargo.toml
2. Every crate needs a `build.rs` with `-nostartfiles`
3. The coreutils submodule is a fork with Eyra modifications
4. Cannot use upstream repos directly
5. Complex `.cargo/config.toml` per workspace

## Success Criteria

### Before (Current State)
```
crates/userspace/eyra/
├── .cargo/config.toml          # Complex per-arch rustflags
├── brush/Cargo.toml            # Has: std = { package = "eyra" }
├── coreutils/                  # Forked submodule with Eyra mods
├── eyra-hello/                 # Test binary
├── eyra-test-runner/           # Test runner
├── libsyscall/                 # Raw syscall wrapper
└── ...
```

### After (Target State)
```
crates/userspace/
├── init/                       # Init process (no-std, raw syscalls)
├── shell/                      # Built-in kernel shell (no-std)
├── libsyscall/                 # Moved from eyra/ (raw syscall wrapper)
└── [removed: eyra/]            # Deleted entirely

toolchain/
│
│ # === COMMITTED (our code) ===
├── libc-levitateos/            # Wrapper crate builds libc.a
├── build-sysroot.sh            # Build c-gull sysroot
├── build-coreutils.sh          # Build unmodified coreutils
├── build-brush.sh              # Build unmodified brush
│
│ # === GITIGNORED (external dependencies, cloned at build time) ===
├── c-ward/                     # git clone https://github.com/sunfishcode/c-ward
├── coreutils/                  # git clone https://github.com/uutils/coreutils
├── brush/                      # git clone https://github.com/reubeno/brush
│
│ # === GITIGNORED (build outputs) ===
├── sysroot/lib/libc.a          # Built from libc-levitateos
├── coreutils-out/              # Built coreutils binaries
└── brush-out/                  # Built brush binaries
```

**Key insight**: coreutils/brush are **external dependencies** like node_modules or go modules.
They are downloaded at build time, gitignored, and never committed to the repo.

## Deployment Model

### Static Linking (Phase 1 - Now)

Binaries are **statically linked** with c-gull libc - no dynamic linker needed:

```
Build:
  git clone uutils/coreutils → toolchain/coreutils/
  cargo build --release (with sysroot RUSTFLAGS)
  → toolchain/coreutils-out/release/coreutils (2.4MB static ELF)

Deploy:
  cp coreutils → initrd_root/coreutils
  create symlinks: ls → coreutils, cat → coreutils, etc.
  cpio pack → initramfs.cpio

Runtime:
  Kernel unpacks initramfs to tmpfs
  /coreutils runs directly (no ld.so, no shared libs)
```

### Dynamic Linking (Future Enhancement)

For smaller binaries and true Linux compatibility:
- Implement `ld.so` (dynamic linker/loader)
- Provide `libc.so` and other shared libraries
- Binaries become ~100KB instead of ~2MB
- This is a separate epic, not part of this refactor

## Behavioral Contracts (Must NOT Change)

| Contract | Description | Verification |
|----------|-------------|--------------|
| Boot sequence | Kernel boots, runs init, spawns shell | `cargo xtask test behavior` |
| Shell commands | `cat`, `echo`, `pwd`, `ls` work | Manual test in QEMU |
| File operations | Read/write/create/delete files | Syscall conformance tests |
| Process lifecycle | exit, fork (when ready) | Syscall tests |

## Golden/Regression Tests to Lock In

1. **Behavior tests**: `tests/golden_boot_*.txt` - Must continue passing
2. **Syscall conformance**: `crates/userspace/eyra/syscall-conformance/` - Move to standalone
3. **Integration tests**: `tests/eyra_*.rs` - Rename to `tests/userspace_*.rs`

## Current Architecture

### Dependencies (what uses Eyra)
```
xtask/src/build/commands.rs:
  - build_eyra() - builds Eyra workspace
  - create_initramfs() - copies Eyra binaries
  - create_iso() - includes Eyra binaries

crates/userspace/eyra/:
  - brush/ - shell (depends on eyra + libsyscall)
  - coreutils/ - submodule with Eyra mods
  - eyra-hello/ - test binary
  - eyra-test-runner/ - test harness
  - libsyscall/ - raw syscall wrapper (keep)
  - libsyscall-tests/ - syscall tests
  - syscall-conformance/ - conformance tests

tests/:
  - eyra_integration_test.rs
  - eyra_regression_tests.rs
```

### Couplings to Break
1. `xtask` → `crates/userspace/eyra/` (build commands)
2. `initramfs` → `eyra/target/` (binary sources)
3. Test files → `eyra` naming

## Constraints

1. **c-gull sysroot must be built first** before any userspace
2. **External projects cloned at build time** - not vendored in repo
3. **No custom target** - use `x86_64-unknown-linux-gnu` / `aarch64-unknown-linux-gnu`
4. **nightly-2025-04-28** required for c-ward compatibility
5. **aarch64 parity** - both architectures must work

## What to Keep

| Component | Status | Rationale |
|-----------|--------|-----------|
| `libsyscall` | Move to `crates/userspace/` | Raw syscall wrapper, no Eyra dependency |
| `syscall-conformance` | Move to `tests/` | Valuable test suite |
| Build scripts | Keep in `toolchain/` | Already there |

## What to Delete

| Component | Rationale |
|-----------|-----------|
| `crates/userspace/eyra/` | Entire directory - replaced by c-gull approach |
| `crates/userspace/eyra/coreutils/` | Forked submodule - use unmodified instead |
| `crates/userspace/eyra/brush/` | Rebuild against c-gull sysroot |
| `tests/eyra_*.rs` | Rename or delete |
