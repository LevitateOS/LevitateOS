# TEAM_367 — Investigate Eyra/uutils Linker Conflict

**Date:** 2026-01-10  
**Status:** ✅ COMPLETED - ROOT CAUSE FOUND AND FIXED

---

## Bug Report

**Symptom:** When building certain uutils crates with Eyra, linker reports:
```
duplicate symbol: _start
duplicate symbol: __dso_handle
```

**Previously Working:** cat, pwd, mkdir, ls  
**Previously Blocked:** echo, env, true, false, rm, cp, mv, ln, touch, rmdir

**Now:** ALL utilities build successfully

---

## Root Cause

The linker was pulling in system C runtime startup files (`Scrt1.o`, `crtbeginS.o`) which provide `_start` and `__dso_handle`. These conflicted with Eyra's Origin crate which provides its own implementations.

The `eyra-hello` example had the proper `build.rs` with `-nostartfiles`, but the other utilities were missing it.

---

## The Fix

Added to each utility:
1. **`build.rs`** with `println!("cargo:rustc-link-arg=-nostartfiles");`
2. **`.cargo/config.toml`** with `rustflags = ["-C", "target-feature=+crt-static"]`

---

## Investigation Timeline

### Entry 1 — Reproduced the Error
- Built rmdir with proper flags: `cargo build --release --target x86_64-unknown-linux-gnu -Zbuild-std=std,panic_abort`
- Got duplicate `_start` error from `Scrt1.o` conflicting with `origin` crate

### Entry 2 — Compared Working vs Blocked
- cat builds successfully, rmdir fails
- Dependency trees nearly identical
- Difference: uu_rmdir has direct libc dependency, uu_cat doesn't

### Entry 3 — Tested Hypotheses
- Adding libc to cat didn't break it
- LTO on/off didn't matter
- The real difference was in linker behavior

### Entry 4 — Found Root Cause
- Discovered `eyra-hello/build.rs` contains `-nostartfiles`
- This was missing from all other utilities
- Added `build.rs` and `.cargo/config.toml` to rmdir
- rmdir builds successfully!

### Entry 5 — Applied Fix to All Utilities
- Added `build.rs` and `.cargo/config.toml` to all 14 utilities
- Fixed Cargo.toml for coreutils-true/false to use uu_* dependencies
- All utilities now build successfully

---

## Files Modified

- `crates/userspace/eyra/*/build.rs` — Added to all utilities
- `crates/userspace/eyra/*/.cargo/config.toml` — Added to all utilities
- `crates/userspace/eyra/echo/Cargo.toml` — Added uu_echo dependency
- `crates/userspace/eyra/coreutils-true/Cargo.toml` — Added uu_true dependency
- `crates/userspace/eyra/coreutils-true/src/main.rs` — Updated to use uu_true
- `crates/userspace/eyra/coreutils-false/Cargo.toml` — Added uu_false dependency
- `crates/userspace/eyra/coreutils-false/src/main.rs` — Updated to use uu_false
- `docs/planning/eyra-uutils-linker-fix/FUTURE_TEAMS_README.md` — Updated with solution

---

## Handoff Checklist

- [x] Root cause identified
- [x] Fix applied to all blocked utilities
- [x] rmdir, echo, env, coreutils-true verified building
- [x] Documentation updated
- [x] Team file updated

