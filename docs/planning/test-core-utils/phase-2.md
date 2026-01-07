# Phase 2: Design - Automated Core Utils Testing

## Proposed Solution
We will create a dedicated test suite binary `suite_test_core` that executes each core utility as a subprocess and verifies its behavior. To allow the test suite to capture and verify the output of these utilities, we will extend the kernel's `spawn` mechanism to support file descriptor inheritance.

## Infrastructure Changes (Kernel)
### [MODIFY] [process.rs](file:///home/vince/Projects/LevitateOS/kernel/src/syscall/process.rs)
- Modify `sys_spawn_args` to support FD inheritance.
- Instead of creating a fresh `FdTable` for every new task, we will clone the parent's `FdTable`.
- This follows standard POSIX behavior and is required for pipe-based output redirection.

## Test Suite Design (Userspace)
### [NEW] [suite_test_core](file:///home/vince/Projects/LevitateOS/userspace/levbox/src/bin/test/suite_test_core.rs)
A new binary that includes test cases for:
- `mkdir`: Basic creation, nested (`-p`), error on existing.
- `ls`: Listing directories, empty directories.
- `cat`: Printing file contents, stdin support.
- `rm`/`rmdir`: Deleting files and directories.
- `touch`: Updating timestamps, creating empty files.
- `cp`/`mv`: Managing file movement.

### Test Verification Pattern
For each utility test:
1.  **Setup**: Create necessary files/folders in `/tmp/core_tests`.
2.  **Redirect**: Create a pipe and `dup2` it to FD 1/2 in the parent.
3.  **Execute**: Use `spawn_args` to run the utility.
4.  **Wait**: `waitpid` for completion.
5.  **Verify**:
    - Exit code matches expectation.
    - Captures output from pipe and matches against golden string (if applicable).
    - Checks filesystem state (e.g., `stat` or `open` to verify file existence).

## API Design
### `spawn_args` (Modified Behavior)
- Now inherits the parent's file descriptor table (shallow clone).
- All open file descriptors (except those marked O_CLOEXEC, not yet implemented) are available in the child.

## Open Questions
1.  **Q1**: Should we support `stdin` piping for tests?
    - **A**: Yes, specifically for `cat` and other filter-like utilities.
2.  **Q2**: How should we handle failure in one utility's test?
    - **A**: The suite should continue if possible, reporting individual failures, but the overall exit code will be 1.
3.  **Q3**: Where should the test files live?
    - **A**: `/tmp/core_tests` on `tmpfs` is ideal as it's fast and cleaned on reboot.

## Design Alternatives Considered
- **Individual test binaries**: Rejected as it bloats the initramfs and `test_runner` list. A single suite binary is more manageable.
- **Shell-based testing**: Rejected as it's hard to automate and verify precisely in the current headless environment.

## Next Steps
- Move to Phase 3: Implementation.
