# systest

System regression and integration tests for LevitateOS.

## Overview

`systest` contains a suite of automated tests designed to verify kernel-userspace interface correctness and prevent regressions in core functionality.

## Test Categories

- **Filesystem**: `stat_test`, `link_test`
- **Time/Scheduling**: `time_test`, `sched_yield_test`
- **Error Handling**: `error_test`

## Execution

Tests are typically run as part of the CI/CD pipeline or manually via the shell within the VM:

```bash
/bin/stat_test
/bin/link_test
```

## Implementation

Each test is a standalone binary that returns 0 on success and non-zero on failure. They rely on `libsyscall` and `ulib`.
