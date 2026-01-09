# ✅ SOLVED: Eyra/uutils Linker Conflict

**TEAM_367** | 2026-01-10

---

## 🎉 ROOT CAUSE FOUND AND FIXED

The duplicate `_start` / `__dso_handle` symbols conflict has been **solved**.

### Root Cause

The linker was pulling in system C runtime startup files (`Scrt1.o`, `crtbeginS.o`) which provide `_start` and `__dso_handle`. These conflicted with Eyra's Origin crate which provides its own implementations.

### The Fix

Each Eyra-based utility needs two files:

**1. `build.rs`** — Tell linker not to use system startup code:
```rust
fn main() {
    println!("cargo:rustc-link-arg=-nostartfiles");
    
    // Create empty libgcc_eh.a stub for aarch64 cross-compilation
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    
    if target_arch == "aarch64" {
        let lib_path = format!("{}/libgcc_eh.a", out_dir);
        let status = std::process::Command::new("ar")
            .args(["rcs", &lib_path])
            .status();
        if status.is_ok() {
            println!("cargo:rustc-link-search=native={}", out_dir);
        }
    }
}
```

**2. `.cargo/config.toml`** — Enable static CRT:
```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
rustflags = ["-C", "target-feature=+crt-static"]

[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "target-feature=+crt-static"]
```

### Why It Worked Before (cat, pwd, mkdir, ls)

It was **luck**. Some utilities happened to not trigger the linker to include `Scrt1.o`, while others did. The `eyra-hello` example had the proper `build.rs` with `-nostartfiles`, but the other utilities were missing it.

---

## Current Status — ALL UTILITIES NOW WORK

| Utility | Status |
|---------|--------|
| cat | ✅ Works |
| pwd | ✅ Works |
| mkdir | ✅ Works |
| ls | ✅ Works |
| echo | ✅ Works |
| env | ✅ Works |
| touch | ✅ Works |
| rm | ✅ Works |
| rmdir | ✅ Works |
| ln | ✅ Works |
| cp | ✅ Works |
| mv | ✅ Works |
| coreutils-true | ✅ Works |
| coreutils-false | ✅ Works |

---

## Build Command

```bash
cd crates/userspace/eyra/<utility>
cargo build --release --target x86_64-unknown-linux-gnu -Zbuild-std=std,panic_abort
```

---

## For Future Utilities

When adding a new Eyra-based utility:

1. Copy `build.rs` from any existing utility
2. Copy `.cargo/config.toml` from any existing utility
3. Use the standard Cargo.toml format:
```toml
[package]
name = "<utility>"
version = "0.1.0"
edition = "2021"

[workspace]

[dependencies]
eyra = { version = "0.22", features = ["experimental-relocate"] }
uu_<utility> = "0.2"

[profile.release]
panic = "abort"
opt-level = "s"
lto = true
strip = true

[unstable]
build-std-features = ["panic_immediate_abort"]
```

4. Create `src/main.rs`:
```rust
extern crate eyra;

fn main() {
    std::process::exit(uu_<utility>::uumain(std::env::args_os()));
}
```

---

## Investigation Notes (TEAM_367)

### What Previous Teams Tried (TEAM_364-366)
- ❌ `default-features = false` — Didn't help
- ❌ Renaming binaries — Didn't help
- ❌ Minimal implementations — Against project goals

### What TEAM_367 Discovered
1. The `eyra-hello` example had a `build.rs` with `-nostartfiles`
2. The blocked utilities were missing this crucial linker flag
3. The `-nostartfiles` flag tells the linker to NOT include system startup code
4. Eyra's Origin crate provides its own `_start` and `__dso_handle`

### The Key Insight
The difference between "working" and "blocked" utilities was NOT about:
- Binary names
- ICU dependencies
- libc features

It was simply that the linker was including `Scrt1.o` for some builds but not others, based on subtle dependency graph differences. The fix is to explicitly tell the linker `-nostartfiles`.
