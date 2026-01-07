# Phase 4: Integration and Testing - Automated Core Utils Testing

## Goal
Verify that the new test suite provides accurate results and that FD inheritance is working as expected.

## Steps
1.  **Verify FD Inheritance**:
    - Run `pipe_test` (existing) to ensure it still passes.
    - Run a small test in `suite_test_core` that redirects `mkdir` output to a pipe and reads it back.
2.  **Verify All Utilities**:
    - Run `suite_test_core` for all 10 utilities.
    - Address any failures (usually due to minor shim/kernel behavior differences).
3.  **CI/Golden File Update**:
    - Run `xtask test behavior` to ensure the new `test_runner` output doesn't cause unexpected regression failures (though it will change the serial output).
    - If needed, update the golden files or ignore the test-specific lines.

## Regression Protection
- Confirm that `shell` still works correctly (inheriting FDs should not break it).
- Confirm that `init` still spawns userspace correctly.
