# TEAM_467: Investigate uniq Command Hang

## Bug Report

Three issues reported:
1. **uniq command hangs** - Critical, needs investigation
2. **Test range syntax broken** - `--phase 1-3` doesn't work
3. **Missing BusyBox commands** - basename, dirname, expr, seq (not a bug, just missing from build)

## Symptom Analysis

### Issue 1: uniq Command Hang (PRIMARY FOCUS)

**Expected**: `uniq uniq.txt` should output deduplicated adjacent lines
**Actual**: Command hangs indefinitely, never returns

**Trigger**: Running phase 7 of coreutils tests:
```sh
echo "a" > uniq.txt
echo "a" >> uniq.txt
echo "b" >> uniq.txt
echo "b" >> uniq.txt
echo "a" >> uniq.txt
OUT=$(uniq uniq.txt)  # <-- HANGS HERE
```

**Observations**:
- `sort sort.txt` works (just before uniq in phase 7)
- File redirection with `>` and `>>` works
- Command substitution `$()` works for other commands
- The hang occurs specifically with `uniq` reading from a file

### Issue 2: Test Range Syntax

**Expected**: `--phase 1-3` runs phases 1, 2, and 3
**Actual**: "sh: 1-3: bad number" error

**Trigger**: Test script parsing:
```sh
start=$(echo "$PHASE_ARG" | cut -d- -f1)
end=$(echo "$PHASE_ARG" | cut -d- -f2)
```

### Issue 3: Missing Commands

Not a bug - these commands aren't compiled into the BusyBox build:
- basename, dirname, expr, seq

## Hypotheses (Issue 1: uniq hang)

### H1: BusyBox uniq reads stdin even with filename argument (Medium confidence)
Some BusyBox applets have quirks where they try to read stdin even when a filename is provided, or have different behavior than GNU coreutils.

**Evidence to check**:
- BusyBox uniq source code behavior
- Whether other file-reading commands like `cat` or `sort` with same file work
- Whether `uniq < file.txt` behaves differently than `uniq file.txt`

### H2: Pipe in command substitution causes deadlock (Medium confidence)
The shell's command substitution `$()` creates a pipe. If uniq's output handling interacts poorly with the pipe, it could block.

**Evidence to check**:
- Does `uniq uniq.txt` work without command substitution?
- Does redirecting to a file (`uniq uniq.txt > out.txt`) work?

### H3: File reading syscall issue for specific patterns (Low confidence)
There might be an edge case in how the kernel handles file reading that uniq triggers but sort doesn't.

**Evidence to check**:
- Compare syscalls used by sort vs uniq via strace on host
- Check if uniq uses different read patterns

## Investigation Log

### Step 1: Testing H2 - Command Substitution
Tested `cat uniq.txt | uniq` (stdin mode) - **WORKS**
Tested `uniq uniq.txt` (file argument mode) - **HANGS**

This ruled out pipe/command substitution as the cause. The issue is specific to BusyBox uniq opening a file directly.

### Step 2: Analyzing BusyBox uniq Source Code
Found the key behavior in `toolchain/busybox/coreutils/uniq.c` (lines 71-77):

```c
if (input_filename) {
    if (input_filename[0] != '-' || input_filename[1]) {
        close(STDIN_FILENO); /* == 0 */
        xopen(input_filename, O_RDONLY); /* fd will be 0 */
    }
    ...
}
```

**BusyBox uniq's strategy**:
1. Close fd 0 (stdin)
2. Open file - expects to get fd 0 (lowest available)
3. Read from `stdin` (fd 0) using `xmalloc_fgetline(stdin)`

### Step 3: Found Root Cause
Checked `crates/kernel/syscall/src/fs/open.rs` line 106-108:

```rust
if fd < 3 {
    return Err(EINVAL);
}
```

**The kernel was refusing to close fd 0, 1, or 2!**

When BusyBox called `close(0)`, the kernel returned EINVAL (which BusyBox ignores). Then when it opened the file, it got fd 3 instead of fd 0. The read from `stdin` (still fd 0) blocked on the original stdin which waits for keyboard input forever.

### Step 4: Fix Applied
Removed the `fd < 3` check in `sys_close`. This is a valid POSIX pattern - programs can close and reopen stdio fds.

**File modified**: `crates/kernel/syscall/src/fs/open.rs`

```rust
/// TEAM_168: sys_close - Close a file descriptor.
/// TEAM_421: Updated to return SyscallResult.
/// TEAM_467: Allow closing fd 0/1/2 - BusyBox uniq closes stdin to reopen file at fd 0.
pub fn sys_close(fd: usize) -> SyscallResult {
    let task = los_sched::current_task();
    let mut fd_table = task.fd_table.lock();

    // TEAM_467: Remove check for fd < 3. Programs like BusyBox uniq close stdin (fd 0)
    // and reopen a file to reuse the fd slot. This is a valid POSIX pattern.

    if fd_table.close(fd) {
        Ok(0)
    } else {
        Err(EBADF)
    }
}
```

### Step 5: Verification
- Re-enabled uniq tests in `xtask/initrd_resources/test-core.sh`
- Added both file argument and pipe tests
- All 83 coreutils tests pass

## Resolution

### Issue 1: uniq Command Hang - **FIXED**
**Root Cause**: `sys_close` rejected closing fd < 3, breaking BusyBox's fd reuse pattern
**Fix**: Removed the artificial restriction on closing stdio fds

### Issue 2: Test Range Syntax - Still pending investigation

### Issue 3: Missing BusyBox Commands - Not a kernel bug (just not in build config)

## Gotchas Discovered

**48. Allow Closing Stdio Fds**: Some programs (BusyBox uniq, dash, etc.) close stdin/stdout/stderr and reopen files at the same fd numbers. The kernel must allow closing fd 0/1/2. The fd allocator must return the lowest available fd (POSIX requirement).

## Files Modified
- `crates/kernel/syscall/src/fs/open.rs` - Removed fd < 3 check in sys_close
- `xtask/initrd_resources/test-core.sh` - Re-enabled uniq tests
