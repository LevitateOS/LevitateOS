# Phase 4: Cleanup - Arch Abstraction

## Dead Code Removal (Rule 6)
- [ ] Remove `kernel/src/boot.rs` (logic moved to `arch/aarch64/boot.rs`).
- [ ] Remove `kernel/src/exceptions.rs` (logic moved to `arch/aarch64/exceptions.rs`).
- [ ] Remove redundant AArch64 imports in generic kernel files.

## Temporary Adapter Removal
- [ ] Remove any temporary re-exports in `kernel/src/mod.rs` or `main.rs`.

## Encapsulation Tightening
- [ ] Ensure `arch/aarch64` modules are private to the `arch` crate if possible, or only exposed via the `arch` trait/interface.

## Steps
1. **Step 1: Remove Old Files**
2. **Step 2: Clean up Imports**
3. **Step 3: Verify No AArch64 Leaks**
