# TEAM_357: Investigate libgcc_eh Missing on aarch64

**Created:** 2026-01-09  
**Status:** 🔄 In Progress  
**Objective:** Understand why `libgcc_eh` is required for aarch64 static-pie but not x86_64

---

## Bug Report

**Symptom:** aarch64 static-pie binary build fails with:
```
/usr/bin/aarch64-linux-gnu-ld: cannot find -lgcc_eh: No such file or directory
```

**Expected:** Should build like x86_64 does

**Questions:**
1. Where is `libgcc_eh` referenced?
2. Is it a Rust library or GCC library?
3. Why does x86_64 succeed but aarch64 fails?

---

## Investigation Log

### Phase 1: Reproduce the Error

**aarch64 build fails with:**
```
/usr/bin/aarch64-linux-gnu-ld: cannot find -lgcc_eh: No such file or directory
```

**x86_64 build succeeds** - produces static-pie binary.

---

### Phase 2: Hypotheses

1. **libgcc_eh.a missing from cross-compiler** (HIGH confidence)
2. Some Rust crate requesting gcc_eh link (MEDIUM confidence)
3. Architecture-specific linking behavior (LOW confidence)

---

### Phase 3: Evidence Gathering

#### Finding 1: Library Availability

| Architecture | libgcc_eh.a Location | Status |
|--------------|---------------------|--------|
| x86_64 (native) | `/usr/lib/gcc/x86_64-redhat-linux/15/libgcc_eh.a` | ✅ Present (462KB) |
| aarch64 (cross) | `/usr/lib/gcc/aarch64-linux-gnu/15/libgcc_eh.a` | ❌ **MISSING** |

The aarch64 cross-compiler only has `libgcc.a` but NOT `libgcc_eh.a`.

#### Finding 2: Who Requests `-lgcc_eh`?

**Source: Rust's `libc` crate** (`~/.cargo/registry/.../libc-0.2.180/src/unix/mod.rs`):

```rust
#[link(
    name = "gcc_eh",
    kind = "static",
    modifiers = "-bundle",
    cfg(target_feature = "crt-static")
)]
```

When `crt-static` is enabled (which we use), the `libc` crate tells the linker to link `libgcc_eh.a` statically.

#### Finding 3: What is libgcc_eh?

**`libgcc_eh` is a GCC library, NOT a Rust library.**

- **Purpose**: Exception handling support for GCC-compiled code
- **Contains**: Unwinding primitives like `_Unwind_RaiseException`, `_Unwind_Resume`
- **Used by**: Rust's panic/unwind mechanism when using GCC-based linking

#### Finding 4: Why Does x86_64 Work?

| Component | x86_64 | aarch64 |
|-----------|--------|---------|
| GCC type | Native compiler | Cross-compiler |
| libgcc_eh.a | ✅ Included | ❌ Not included |
| Reason | Full GCC installation | Minimal cross-toolchain |

The Fedora `gcc-aarch64-linux-gnu` package is a **minimal cross-compiler** built with `--disable-shared` and does not include the `libgcc_eh.a` static library.

---

## Root Cause Analysis

### The Problem

```
┌─────────────────────────────────────────────────────────────────┐
│                        Rust Build Process                       │
├─────────────────────────────────────────────────────────────────┤
│  1. Cargo builds eyra-hello with -C target-feature=+crt-static │
│  2. libc crate (rustc-dep-of-std) sees crt-static flag         │
│  3. libc emits: #[link(name = "gcc_eh", kind = "static")]      │
│  4. Linker searches for libgcc_eh.a                            │
│  5. x86_64: Found at /usr/lib/gcc/.../libgcc_eh.a ✅           │
│  6. aarch64: NOT FOUND ❌                                       │
└─────────────────────────────────────────────────────────────────┘
```

### Why aarch64 Cross-Compiler Lacks libgcc_eh.a

The cross-compiler configure flags show:
```
--disable-shared --with-newlib --without-headers
```

This is a **barebones cross-compiler** for kernel/embedded development. It doesn't include:
- Shared libraries
- Full C runtime support
- The `libgcc_eh.a` static exception handling library

### Summary

| Question | Answer |
|----------|--------|
| **Where is libgcc_eh missing?** | `/usr/lib/gcc/aarch64-linux-gnu/15/` - not shipped with cross-compiler |
| **Is libgcc_eh a Rust library?** | **NO** - it's a GCC library for exception handling |
| **Why does x86_64 work?** | Native GCC includes full libraries; cross-GCC is minimal |

---

## Potential Fixes (NOT implementing now)

1. **Install full aarch64 cross-toolchain** with libgcc_eh.a
2. **Use LLVM's libunwind** instead of GCC's unwinder
3. **Build with `panic=abort`** to avoid needing unwinder entirely
4. **Create empty libgcc_eh.a stub** (c-scape approach - add to empty library list)

---

## Status: ✅ Fixed

Root cause identified and fixed.

### Fix Applied

Added empty `libgcc_eh.a` stub in `userspace/eyra-hello/build.rs`:
- Creates empty archive using `ar rcs` during build
- Only applies to aarch64 target
- Adds `OUT_DIR` to link search path

### Verification

```
$ ./scripts/build-eyra.sh aarch64
=== Build successful ===
target/aarch64-unknown-linux-gnu/release/eyra-hello: ELF 64-bit LSB executable, ARM aarch64, 
version 1 (SYSV), statically linked, stripped
```

Both x86_64 and aarch64 static binaries now build successfully.

