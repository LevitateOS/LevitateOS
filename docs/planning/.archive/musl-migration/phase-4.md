# Phase 4: Integration - Migrate from c-gull to musl

## Integration Points

### 1. CI Workflow

**File**: `.github/workflows/ci.yml` (or equivalent)

```yaml
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-action@stable
        # NOTE: Can now use stable! No nightly required for musl

      - name: Install musl target
        run: rustup target add x86_64-unknown-linux-musl

      - name: Install musl toolchain (for C programs)
        run: sudo apt-get install -y musl-tools musl-dev autoconf automake

      - name: Build all
        run: cargo xtask build all
```

**Key changes:**
- Remove nightly requirement
- Add musl-tools package
- Add autoconf for dash

### 2. Initramfs Integration

No changes needed to initramfs creation. The `create_initramfs()` function already:
1. Copies Rust app binaries (now musl-linked instead of c-gull-linked)
2. Can copy C app binaries the same way

```rust
// In create_initramfs() - add after Rust apps
for app in c_apps::C_APPS {
    if app.exists(arch) {
        let src = app.output_path(arch);
        std::fs::copy(&src, root.join(app.name))?;
        println!("  📦 Added {} (C)", app.name);
    }
}
```

### 3. Kernel

**No changes required.** The kernel provides Linux syscall ABI. Whether userspace links against c-gull or musl is transparent to the kernel.

## Test Strategy

### Verification Tests

**V1: musl binary is static**
```bash
cargo xtask build coreutils
file toolchain/coreutils-out/x86_64-unknown-linux-musl/release/coreutils
# Expected: "statically linked"
```

**V2: musl binary runs on LevitateOS**
```bash
cargo xtask run
# In shell:
/coreutils echo hello
# Expected: "hello"
```

**V3: dash builds and is static**
```bash
cargo xtask build dash
file toolchain/dash-out/x86_64/dash
# Expected: "statically linked"
```

**V4: dash runs on LevitateOS**
```bash
cargo xtask run
# In shell:
/dash -c "echo hello"
# Expected: "hello"
```

### Regression Tests

All existing behavior tests should pass unchanged:
```bash
cargo xtask test behavior
cargo xtask test unit
```

### Performance Comparison

Compare binary sizes (musl should be similar or smaller):

| Binary | c-gull | musl | Delta |
|--------|--------|------|-------|
| coreutils | ? KB | ? KB | ? |
| brush | ? KB | ? KB | ? |

## CI Changes

### New Dependencies

```yaml
# Ubuntu/Debian
- musl-tools      # provides musl-gcc
- musl-dev        # provides headers
- autoconf        # for dash autoreconf
- automake        # for dash autoreconf

# For aarch64 cross-compilation
- gcc-aarch64-linux-gnu
- musl:arm64      # or build musl for aarch64
```

### Build Matrix

```yaml
strategy:
  matrix:
    arch: [x86_64]  # Start with x86_64
    # Add aarch64 after x86_64 is stable
```

## Impact Analysis

### What Gets Simpler

| Before | After |
|--------|-------|
| Nightly Rust required | Stable Rust works |
| 6+ RUSTFLAGS | Zero RUSTFLAGS |
| -Z build-std | Not needed |
| Custom sysroot build | Use system musl |
| Can't build C programs | C programs work |

### What Stays the Same

- Kernel (no changes)
- Userspace ABI (still Linux)
- Behavior tests (same expected output)
- QEMU invocation

### What's New

- musl-tools system dependency
- autoconf/automake for C apps
- C app support in build system

## Rollback Plan

If musl migration fails:

1. Keep c-gull code on a branch
2. Revert apps.rs, sysroot.rs changes
3. Re-enable c-gull build path

The migration is file-level reversible.

## Verification Checklist

Before declaring migration complete:

- [ ] `cargo xtask build coreutils` works with musl
- [ ] `cargo xtask build brush` works with musl
- [ ] `cargo xtask build dash` works
- [ ] coreutils binary is statically linked
- [ ] brush binary is statically linked
- [ ] dash binary is statically linked
- [ ] `cargo xtask test behavior` passes
- [ ] `cargo xtask test unit` passes
- [ ] Binaries run correctly in QEMU
- [ ] CI builds pass
- [ ] Documentation updated
