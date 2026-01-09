# levbox

A Busybox-inspired multicall binary for LevitateOS.

## Overview

`levbox` provides a collection of core system utilities (ls, cat, mkdir, etc.) in a single binary. This reduces the footprint of the initramfs and simplifies userspace deployment.

## Included Utilities

- **File Operations**: `cat`, `cp`, `ls`, `mkdir`, `mv`, `rm`, `rmdir`, `touch`
- **System Info**: `pwd`
- **Internal/Test**: `suite_test_core`, `test_runner`, `interrupt_test`, `clone_test`, `mmap_test`, `pipe_test`, `signal_test`, `tty_test`, `pty_test`, `pty_interact`

## Usage

Utilities can be invoked by symlinking their name to the `levbox` binary, or by passing the utility name as the first argument:

```bash
/bin/levbox ls /
# or if linked:
/bin/ls /
```

## Dependencies

- `libsyscall`: System call ABI.
- `ulib`: Userspace library for common helpers.
