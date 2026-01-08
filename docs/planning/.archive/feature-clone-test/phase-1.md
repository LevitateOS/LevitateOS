# Phase 1: Discovery

## Feature Summary
Implement a `clone_test` binary in `userspace/levbox` to verify the kernel's `sys_clone` implementation. This test will ensure that threads can be created, share memory with the parent, and be joined upon exit.

## Success Criteria
- `clone_test` compiles successfully.
- Running `clone_test` in QEMU outputs success messages.
- The test verifies:
  1. Thread creation (valid TID returned).
  2. Memory sharing (child writes to parent's variable).
  3. Thread exit and cleanup (parent wakes up from futex wait).

## Current State Analysis
- **Kernel**: `sys_clone` is implemented with `CLONE_VM`, `CLONE_THREAD`, `CLONE_CHILD_CLEARTID` support.
- **Userspace**: `libsyscall` exposes `clone` and necessary constants. `levbox` contains other utilities but lacks a threading test.
- **Gap**: No automated way to verify thread creation works as expected.

## Codebase Reconnaissance
- **Target Location**: `userspace/levbox/src/bin/clone_test.rs`
- **Dependencies**: `libsyscall` (for `clone`, `futex`, `exit`, `print`).
- **Mechanics**:
  - Stack allocation: Need to manually allocate a stack buffer (Vec or array).
  - Thread entry: A function that takes no args (or args via stack/registers if we supported it, but our `clone` entry wrapper is simple).
  - Synchronization: `CLONE_CHILD_CLEARTID` requires the child to provide an address that the kernel clears and wakes on exit.

## Constraints
- **Stack**: Must be 16-byte aligned (AArch64 requirement).
- **Entry Point**: `clone` syscall takes a stack pointer and entry point (userspace wrappers usually handle the setup, but we'll use raw syscall).
- **TLS**: Not strictly needed for this basic test, can pass 0.
