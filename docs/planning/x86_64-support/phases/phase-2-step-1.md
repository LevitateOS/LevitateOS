# Phase 2 — Step 1: Toolchain and Build Infrastructure Refactor

## Parent
Phase 2: Design — x86_64 Support

## Goal
Generalize `xtask` to support multiple architectures and update the project toolchain.

## Tasks
1. **Update `rust-toolchain.toml`**: Add `x86_64-unknown-none` to the target list.
2. **Refactor `xtask` CLI**:
    - Add a global `--arch` flag to the `Cli` struct in `xtask/src/main.rs`.
    - Default to `aarch64` if not specified.
3. **Refactor `xtask/src/build.rs`**:
    - Pass `arch` to `build_userspace` and `build_kernel_with_features`.
    - Use the correct target triple based on `arch`.
    - Handle `objcopy` command differences (e.g., `aarch64-linux-gnu-objcopy` vs `llvm-objcopy` or `x86_64-unknown-none-objcopy`).
4. **Refactor `xtask/src/run.rs`**:
    - Select `qemu-system-aarch64` or `qemu-system-x86_64` based on `arch`.
    - Select correct QEMU machine, CPU, and flags for `x86_64` (e.g., `q35`).
5. **Update `xtask/src/image.rs`**:
    - Ensure disk image creation and installation paths account for target architecture if needed.

## Expected Outputs
- `cargo xtask build --arch aarch64` works exactly as before.
- `cargo xtask build --arch x86_64` attempts to build for x86_64 (may fail due to missing kernel code, but toolchain/xtask parts should be sound).
