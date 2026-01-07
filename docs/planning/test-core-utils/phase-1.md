# Phase 1: Discovery - Automated Core Utils Testing

## Feature Summary
This feature implements an automated, in-VM test suite for the LevitateOS core utilities (`cat`, `ls`, `mkdir`, `cp`, `mv`, `rm`, `rmdir`, `pwd`, `ln`, `touch`). The goal is to provide a reliable way for AI agents and developers to verify that these utilities behave correctly according to their POSIX specifications without requiring manual shell interaction.

## Success Criteria
1.  **Automated Execution**: A single command (e.g., `cargo xtask run test`) executes tests for all core utilities.
2.  **Coverage**: At least one functional test case for each of the 10 core utilities.
3.  **Parseable Output**: Results reported in the standard `[TEST] name: PASS/FAIL` format.
4.  **No Regressions**: All existing tests in `test_runner` must continue to pass.
5.  **Shutdown Verification**: The test suite must end with a clean system shutdown.

## Current State Analysis
### Existing Infrastructure
- **`test_runner`**: An automated orchestrator that runs binaries from `TESTS` array and reports results. It currently handles syscall-level tests (`mmap_test`, `pipe_test`, etc.).
- **Core Utilities**: Exist as independent binaries in `userspace/levbox/src/bin/core`.
- **System tests**: `xtask test behavior` compares serial output to `golden_boot.txt`, but doesn't deep-dive into utility logic.

### Gaps
- Core utilities are currently only verified manually through the shell.
- No infrastructure exists to feed input to and verify output from these utilities automatically in a "black-box" fashion.

## Codebase Reconnaissance
### Files Involved
- [test_runner.rs](file:///home/vince/Projects/LevitateOS/userspace/levbox/src/bin/test/test_runner.rs): Needs to be updated to include core utility tests.
- [userspace/levbox/src/bin/core/](file:///home/vince/Projects/LevitateOS/userspace/levbox/src/bin/core/): Source code for the utilities to be tested.
- [docs/specs/levbox/](file:///home/vince/Projects/LevitateOS/docs/specs/levbox/): Behavioral specifications to derive test cases from.

### Constraints
- **Filesystem**: Many core utilities require a writable filesystem. `tmpfs` is available at `/tmp`.
- **Memory**: The test environment may have limited memory (e.g., Default 512MB).
- **No `std`**: All userspace code must be `no_std`, using `ulib` and `libsyscall`.

## Next Steps
- Move to Phase 2: Design to determine how to wrap these utilities into the `test_runner` flow.
