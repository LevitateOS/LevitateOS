# Phase 1: Discovery

## Feature Summary
We need to harden `libsyscall` by adding comprehensive tests. Currently, we have reasonably high "happy path" coverage for core IO, process, and signal operations. We rely on `core_utils` tests for coverage, but dedicated syscall tests are better for isolation and edge cases. We are missing coverage for metadata (stat/links), time precision, scheduling, and error paths.

## Success Criteria
- [ ] New test binary `stat_test` validates `fstat`, `utimensat`.
- [ ] New test binary `link_test` validates `linkat`, `unlinkat`, `symlinkat`, `readlinkat`.
- [ ] New test binary `time_test` validates `nanosleep`, `clock_gettime`.
- [ ] New test binary `sched_yield_test` validates `sched_yield`.
- [ ] New test binary `error_test` validates negative returns (errno) for invalid inputs.
- [ ] All new tests pass in CI/QEMU.

## Current State Analysis
- **Existing Tests**:
  - `suite_test_core`: Covers `openat`, `read`, `write`, `mkdirat`, `unlinkat`, `renameat`, `dup2`.
  - `pipe_test`: Covers `pipe2`, `dup`, `dup3`.
  - `clone_test`, `signal_test`, `interrupt_test`: Process/Thread/Signal coverage.
  - `mmap_test`: Memory coverage.

- **Gaps**:
  - No explicit test for `symlinkat` or `linkat`.
  - No test for `fstat` correctness (checking fields).
  - No test for `nanosleep` duration accuracy.
  - No test specifically verifying `sched_yield`.
  - No negative testing (e.g., verifying `openat` returns -ENOENT).

## Codebase Reconnaissance
- **Target Location**: `userspace/levbox/src/bin/test/`
- **Dependencies**: `libsyscall`, `ulib` (for printing/assertions).
- **Execution**: The `init` process or `test_runner` executes these binaries.

## Constraints
- Tests must be `no_std`, `no_main` binaries.
- Must return 0 on success, non-zero on failure.
- Output should be minimal/parsable (PASS/FAIL).

## Plan
We will create separate binaries for each category to keep tests isolated and manageable.
