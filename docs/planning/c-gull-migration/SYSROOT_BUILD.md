# Pre-built Sysroot for LevitateOS

**Goal**: Build any Rust program for LevitateOS with ZERO source modifications.

---

## How Redox Does It

Redox OS has `x86_64-unknown-redox` as an official Rust target. They:
1. Build **relibc** (Rust libc) as `libc.a`
2. Install to sysroot: `DESTDIR=sysroot/usr make install`
3. Programs built for `x86_64-unknown-redox` automatically link against relibc

**No source modifications needed** - the toolchain handles everything.

---

## Our Approach: Custom Target + Sysroot

Since adding an upstream Rust target requires significant effort, we'll use:
1. **Custom target JSON** - Defines `x86_64-levitateos-linux-gnu`
2. **c-gull as libc.a** - Built from source
3. **Pre-built sysroot** - std compiled against c-gull
4. **Cargo wrapper** - Handles `--target` and `--sysroot` flags

### Step 1: Build c-gull as libc.a

```bash
# Clone c-ward
git clone https://github.com/sunfishcode/c-ward
cd c-ward/c-gull

# Build as static library with take-charge mode
cargo build --release \
    --features take-charge \
    -Z build-std=core,alloc,compiler_builtins \
    --target x86_64-unknown-linux-gnu

# The output is target/.../libc_gull.a
# We need to rename and repackage it as libc.a
```

### Step 2: Create Custom Target Spec

File: `targets/x86_64-levitateos.json`
```json
{
  "arch": "x86_64",
  "cpu": "x86-64",
  "data-layout": "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128",
  "dynamic-linking": false,
  "env": "gnu",
  "has-thread-local": true,
  "linker": "rust-lld",
  "linker-flavor": "gnu-lld",
  "llvm-target": "x86_64-unknown-linux-gnu",
  "max-atomic-width": 64,
  "os": "linux",
  "panic-strategy": "abort",
  "position-independent-executables": true,
  "pre-link-args": {
    "gnu-lld": [
      "-nostartfiles",
      "-static",
      "-L/path/to/levitateos-sysroot/lib"
    ]
  },
  "crt-objects-fallback": "false",
  "relro-level": "full",
  "static-position-independent-executables": true,
  "target-family": ["unix"],
  "target-pointer-width": "64",
  "vendor": "levitateos"
}
```

### Step 3: Build the Sysroot

```bash
# Set environment
export RUST_TARGET_PATH=/path/to/targets
export LEVITATEOS_SYSROOT=/path/to/sysroot

# Build std with our target
cargo +nightly build \
    -Z build-std=std,panic_abort \
    --target x86_64-levitateos \
    --release

# The sysroot is now at:
# target/x86_64-levitateos/release/deps/
```

### Step 4: Package Sysroot

```bash
# Create sysroot structure
mkdir -p $LEVITATEOS_SYSROOT/lib/rustlib/x86_64-levitateos/lib

# Copy compiled libraries
cp target/x86_64-levitateos/release/deps/*.rlib \
   $LEVITATEOS_SYSROOT/lib/rustlib/x86_64-levitateos/lib/

# Copy c-gull libc.a
cp /path/to/c-gull/libc.a \
   $LEVITATEOS_SYSROOT/lib/
```

### Step 5: Build Any Program (No Modifications!)

```bash
# Clone ORIGINAL uutils coreutils
git clone https://github.com/uutils/coreutils
cd coreutils

# Build with our sysroot - NO SOURCE CHANGES
cargo +nightly build --release \
    --target x86_64-levitateos \
    --sysroot $LEVITATEOS_SYSROOT

# Binary ready for LevitateOS!
```

---

## Key Differences from Eyra/Mustang

| Approach | Source Changes | Build Complexity |
|----------|---------------|------------------|
| **Eyra** | `std = { package = "eyra" }` in Cargo.toml | Low |
| **Mustang** | `mustang::can_run_this!();` macro | Medium |
| **Pre-built Sysroot** | NONE | High (one-time setup) |

---

## Implementation TODO

### Phase 1: Build c-gull as libc.a
- [ ] Fork/patch c-gull to build as staticlib
- [ ] Create build script that produces `libc.a`
- [ ] Include origin startup code in the library
- [ ] Test: link a simple C program against it

### Phase 2: Create Target Spec
- [ ] Write `x86_64-levitateos.json`
- [ ] Test with `-Z build-std`
- [ ] Verify linker finds our `libc.a`

### Phase 3: Build Sysroot
- [ ] Script to build std from source
- [ ] Package as distributable sysroot
- [ ] Test: build simple Rust program

### Phase 4: Test with Real Programs
- [ ] Build original uutils/coreutils
- [ ] Build ripgrep (complex dependencies)
- [ ] Run on LevitateOS

### Phase 5: Automation
- [ ] `cargo xtask build sysroot` command
- [ ] CI/CD for sysroot builds
- [ ] Documentation for users

---

## Challenges

### 1. Origin Startup Code
The `_start` symbol must be included. Options:
- Link origin.a into libc.a
- Use `--whole-archive` to force inclusion
- Provide crt0.o separately

### 2. TLS (Thread-Local Storage)
Rust std uses TLS. Need to verify c-gull provides:
- `__tls_get_addr`
- TLS initialization

### 3. Unwinding
For panic=unwind (not abort), need:
- libunwind or equivalent
- DWARF unwinding support

### 4. libc Crate Compatibility
Rust's `libc` crate provides FFI bindings. Need to verify:
- All symbols used by std are provided by c-gull
- Same ABI/calling conventions

---

## References

- [Redox relibc](https://github.com/redox-os/relibc) - Rust libc implementation
- [c-ward/c-gull](https://github.com/sunfishcode/c-ward) - Another Rust libc
- [Rust Platform Support](https://doc.rust-lang.org/nightly/rustc/platform-support/redox.html) - Redox target docs
- [rustc-build-sysroot](https://github.com/RalfJung/rustc-build-sysroot) - Sysroot building tool
- [cargo -Z build-std](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std) - Build std from source
