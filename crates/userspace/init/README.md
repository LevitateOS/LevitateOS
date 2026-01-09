# init

The PID 1 process for LevitateOS.

## Overview

`init` is the first userspace process started by the kernel. Its primary responsibility is to bring up the rest of the system, including mounting filesystems, starting services, and eventually launching the interactive shell.

## Responsibilities

1. **System Setup**: Perform early userspace initialization.
2. **Process Reaping**: Handle orphaned processes and clean up zombie states.
3. **Shell Spawning**: Launch the default interactive shell on the primary TTY.

## Implementation Details

`init` uses `libsyscall` directly to perform operations like `fork`, `exec`, and `wait`. It is a minimal, statically linked binary.

```rust
fn main() {
    // PID 1 logic here
}
```
