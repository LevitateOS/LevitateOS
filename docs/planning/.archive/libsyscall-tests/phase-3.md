# Bugfix Plan: Extract Tests from Levbox

## Goal
Move the newly created `libsyscall` tests from `levbox` into a dedicated crate `userspace/systest` to avoid logical circular dependencies and architectural mess.

## Reversal Strategy
- If everything fails, revert to `levbox` location (git revert).

## Proposed Changes
1.  Create new crate `userspace/systest` (binary crate).
2.  Move the `.rs` files from `levbox/src/bin/test/` to `systest/src/bin/`.
3.  Update `userspace/Cargo.toml` to include `systest` in workspace.
4.  Update `xtask/src/main.rs` (or `image.rs`) to ensure it picks up binaries from `systest`.
    - *Analysis*: `xtask` scans `userspace/target/{arch}/release/`. Since all workspace members output to the same target dir, `xtask` should *automatically* pick up the new binaries without code changes!
5.  Remove entries from `levbox/Cargo.toml`.

## Steps
1.  **Create Crate**: `cargo init --bin userspace/systest --name systest --edition 2021` (manually creating folder structure).
2.  **Configure`: Add correct `[dependencies]` to `systest/Cargo.toml` (`libsyscall`, `ulib`).
3.  **Migrate Files**: Move `stat_test.rs`, `link_test.rs`, `time_test.rs`, `sched_yield_test.rs`, `error_test.rs`.
4.  **Register Binaries**: In `systest/Cargo.toml` as `[[bin]]`.
5.  **Cleanup**: Remove from `levbox`.

## Risk
Low. Build system relies on target dir output, which is shared.
