# TEAM_395: Investigate Eyra Linker Error

## Symptom
Building the eyra workspace fails with undefined symbol errors (`Ok`, `Err`, `.repeat()` not found).
This indicates std prelude is not being properly provided.

## Root Cause (DEEP)
**Conflicting Eyra dependencies** between crates:

1. `libsyscall` had `eyra` as an optional dependency (not renamed to std)
2. `libsyscall-tests` used `std = { package = "eyra" }` (renamed)
3. When compiled together, TWO eyra instances exist → conflict

**Key insight:** `libsyscall` is `#![no_std]` and doesn't USE eyra at all - the dependency was unnecessary.

## Solution (PROPER)
**Architectural fix - not a workaround:**

1. **Remove eyra from libsyscall** - it's a pure `no_std` syscall library that makes raw syscalls
2. **Use official Eyra rename pattern** in binary crates:
   ```toml
   std = { package = "eyra", version = "0.22", features = ["experimental-relocate"] }
   ```
3. **No `extern crate eyra;` needed** - rename pattern provides native std support

## Files Changed
- `libsyscall/Cargo.toml` - **REMOVED** unnecessary eyra dependency
- `libsyscall-tests/Cargo.toml` - use `std = { package = "eyra" }`, no feature flags on libsyscall
- `eyra-hello/Cargo.toml` - use `std = { package = "eyra" }`
- `brush/Cargo.toml` - use `std = { package = "eyra" }`
- All `main.rs` files - removed `extern crate eyra;` (not needed with rename pattern)

## Architecture
```
Binary crates (eyra-hello, libsyscall-tests, brush)
    └── std = { package = "eyra" }  ← Native std support
    └── libsyscall (no_std library) ← Pure syscall wrappers, NO eyra
```

## Status
- [x] Root cause identified (conflicting dependencies)
- [x] Proper fix applied (removed unnecessary dependency)
- [x] Native std support working via rename pattern
- [x] Build verified for x86_64
