# Phase 3: Implementation - Automated Core Utils Testing

## Overview
We will implement the infrastructure for FD inheritance in the kernel and then build the `suite_test_core` binary to verify the core utilities.

## Steps

### Step 1: Kernel - FD Inheritance
1.  Add `Clone` derive to `FdTable` in `kernel/src/task/fd_table.rs`.
2.  Update `From<UserTask> for TaskControlBlock` in `kernel/src/task/mod.rs`.
    - Instead of creating a fresh `FdTable`, it should receive a cloned table from the parent.
    - **Wait**: `From<UserTask>` is called in `sys_spawn`. We need a way to pass the parent's table to the `UserTask` or to the conversion.

**Design Refinement**:
Modify `UserTask::new` to accept a `SharedFdTable`.
In `sys_spawn` and `sys_spawn_args`, we pass `current_task().fd_table.clone()`.

### Step 2: Userspace - `suite_test_core`
1.  Create `userspace/levbox/src/bin/test/suite_test_core.rs`.
2.  Implement a skeleton to run tests.
3.  Add helper functions `assert_exit_code`, `assert_file_exists`, `assert_stdout_matches`.

### Step 3: Registration
1.  Add `suite_test_core` to `userspace/levbox/Cargo.toml`.
2.  Add `"suite_test_core"` to the `TESTS` array in `userspace/levbox/src/bin/test/test_runner.rs`.

## Unit of Work (UoW)
- **UoW 1**: Kernel changes for FD inheritance.
- **UoW 2**: Initial `suite_test_core` implementation (mkdir/ls).
- **UoW 3**: Full coverage implementation (remaining utilities).
- **UoW 4**: Integration into `test_runner`.

## Exit Criteria
- `test_runner` starts and runs `suite_test_core`.
- `suite_test_core` reports passes for at least 5 utilities initially.
- No system crashes during inheritance.
