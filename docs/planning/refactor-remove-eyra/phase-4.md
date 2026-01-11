# Phase 4: Cleanup

## Dead Code Removal (Rule 6)

### Files to Delete

| Path | Reason |
|------|--------|
| `crates/userspace/eyra/` | Entire directory - replaced by c-gull |
| `crates/userspace/eyra/brush/` | Will rebuild against c-gull |
| `crates/userspace/eyra/coreutils/` | Git submodule - use unmodified upstream |
| `crates/userspace/eyra/eyra-hello/` | Test binary - not needed |
| `crates/userspace/eyra/eyra-test-runner/` | Test runner - not needed |
| `crates/userspace/eyra/cgull-test/` | Temporary test - not needed |
| `crates/userspace/eyra/.cargo/` | Eyra-specific config |
| `crates/userspace/eyra/*.md` | Documentation for deleted code |
| `xtask/src/tests/eyra.rs` | Eyra-specific tests |

### Files to Modify (Remove Dead Code)

| File | Dead Code to Remove |
|------|---------------------|
| `xtask/src/build/commands.rs` | `build_eyra()` function, eyra paths |
| `xtask/src/main.rs` | `Eyra` subcommand variant |
| `xtask/src/tests/mod.rs` | `mod eyra;` |
| `.gitmodules` | eyra/coreutils submodule entry |

## Temporary Adapters to Remove

**None** - We're doing a clean break per Rule 5 (Breaking Changes > Fragile Compatibility).

No compatibility shims:
- No `build_eyra()` → `build_coreutils()` redirect
- No path aliasing
- No deprecation wrappers

Just delete and fix call sites.

## Encapsulation Tightening

### xtask Build Module

Split commands.rs into focused modules:

```
xtask/src/build/
├── mod.rs           # Re-exports
├── commands.rs      # Top-level build commands (<200 lines)
├── kernel.rs        # Kernel build logic
├── userspace.rs     # Init, shell build logic
├── initramfs.rs     # Initramfs creation
├── sysroot.rs       # NEW: c-gull sysroot
└── external.rs      # NEW: External projects
```

### Module Privacy

```rust
// sysroot.rs
pub fn build_sysroot() -> Result<()>;     // Public entry point
fn clone_c_ward() -> Result<()>;          // Private impl
fn build_libc() -> Result<()>;            // Private impl
fn create_symlinks() -> Result<()>;       // Private impl
```

## File Size Check

| File | Before | After | Status |
|------|--------|-------|--------|
| `xtask/src/build/commands.rs` | ~600 | <300 | Split |
| `xtask/src/build/sysroot.rs` | N/A | ~150 | New |
| `xtask/src/build/external.rs` | N/A | ~200 | New |
| `xtask/src/build/initramfs.rs` | N/A | ~200 | Extracted |

Target: All files < 500 lines (ideal < 300).

## Documentation Updates

### Files to Update

| File | Update |
|------|--------|
| `CLAUDE.md` | Replace Eyra section with c-gull instructions |
| `README.md` | Update build instructions |
| `docs/planning/c-gull-migration/` | Mark as "implemented" |

### CLAUDE.md Changes

Replace:
```markdown
### Eyra Integration (CRITICAL - READ THIS)
[...current Eyra docs...]
```

With:
```markdown
### c-gull Sysroot (CRITICAL - READ THIS)

**What is c-gull?** c-gull provides a pure-Rust libc implementation.
We build it as a static library (libc.a) that any Rust program can
link against, enabling UNMODIFIED upstream projects to run.

#### Build Commands
```bash
cargo xtask build sysroot      # Build libc.a
cargo xtask build coreutils    # Build unmodified uutils
cargo xtask build all          # Everything
```

#### How It Works
1. c-gull is built as `toolchain/sysroot/lib/libc.a`
2. External projects are cloned to `toolchain/`
3. Built with RUSTFLAGS pointing to sysroot
4. No source modifications required
```

## Verification Checklist

- [ ] `grep -r "eyra" --include="*.rs"` returns 0 results
- [ ] `grep -r "eyra" --include="*.toml"` returns 0 results (except toolchain)
- [ ] `ls crates/userspace/eyra/` returns "No such file or directory"
- [ ] `cargo xtask build all` succeeds
- [ ] `cargo xtask test` passes
- [ ] No files > 1000 lines
- [ ] Ideally no files > 500 lines
