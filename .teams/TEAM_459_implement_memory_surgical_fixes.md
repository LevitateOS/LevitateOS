# TEAM_459: Implement Memory Surgical Fixes

## Objective

Implement the three surgical fixes recommended by TEAM_458 review to prevent future memory management bugs like TEAM_455 and TEAM_456.

## Fixes Implemented

1. **Warning Comment**: Added prominent warning to `map_user_page()` about VMA tracking requirement
2. **Debug Assertion**: Added `verify_ttbr0_consistency()` at syscall entry (debug builds only)
3. **Documentation**: Added GOTCHA #37 (ttbr0/CR3 sync) and GOTCHA #38 (VMA tracking)

## Progress Log

### Session 1 (2026-01-12)
- Started implementation based on TEAM_458 review
- Decided against making `map_user_page()` private due to circular dependency issues (mm crate can't depend on sched crate where TCB lives)
- Added warning comment to `map_user_page()` in `mm/src/user/mapping.rs`
- Added `verify_ttbr0_consistency()` function in `syscall/src/lib.rs`
- Added GOTCHA #37 and #38 to `docs/GOTCHAS.md`
- Fixed aarch64 build issues:
  - Added cfg attributes to fork.rs debug logging (x86_64-specific register names)
  - Fixed ttbr0 AtomicUsize loading in main.rs signal handler
- Verified both x86_64 and aarch64 builds pass

## Files Modified

| File | Change |
|------|--------|
| `mm/src/user/mapping.rs` | Added warning comment to `map_user_page()` |
| `syscall/src/lib.rs` | Added `verify_ttbr0_consistency()` debug assertion |
| `docs/GOTCHAS.md` | Added GOTCHA #37 and #38 |
| `sched/src/fork.rs` | Fixed aarch64 build (cfg attributes on debug logs) |
| `levitate/src/main.rs` | Fixed ttbr0 AtomicUsize loading |

## Key Decisions

1. **Warning comment vs API restriction**: Making `map_user_page()` private would require the mm crate to depend on sched crate (for TCB access), creating circular dependencies. A warning comment is simpler and achieves the same goal of alerting developers.

2. **Debug-only assertion**: The ttbr0 consistency check only runs in debug builds to avoid runtime overhead in release.

3. **Two GOTCHA entries**: Created separate entries for the two different bug patterns:
   - GOTCHA #37: Forgetting to update task.ttbr0 after CR3 switch
   - GOTCHA #38: Forgetting to track VMAs when mapping pages

### Session 2 (2026-01-12)
- Investigated BusyBox ash shell not printing prompt
- Root cause: Shell stuck in job control loop
  - Ash calls `setsid()` to become session leader → pgid changes to its own PID
  - Then checks `TIOCGPGRP` (returns `FOREGROUND_PID=1`) vs `getpgid(0)` (returns 5)
  - Mismatch causes shell to think it's not in foreground → sends SIGTTOU, loops
- Fix 1: Set `FOREGROUND_PID` when init spawns (in `init.rs`)
- Fix 2: Set `FOREGROUND_PID` when process acquires controlling terminal via `TIOCSCTTY`
  - This is the key fix: when shell calls `setsid()` then `TIOCSCTTY`, we now update
    `FOREGROUND_PID` to the shell's pgid
- Also fixed keyboard input routing: characters now fed to `CONSOLE_TTY` from interrupt handler
- Reduced scheduler switch logging to trace level (was causing spam)
- **SUCCESS**: Shell prompt now displays, keyboard input works, commands execute!

### Verification Test Results
```
LevitateOS# test
LevitateOS# cat
-/bin/ash: cat: Function not implemented
LevitateOS#
```

Shell is fully interactive. Remaining issues discovered:
- `[SYSCALL] Unknown syscall number: 4` - stat syscall not implemented
- `cat: Function not implemented` - likely due to missing stat or other syscall

## Files Modified (Session 2)

| File | Change |
|------|--------|
| `levitate/src/init.rs` | Set `FOREGROUND_PID` when init spawns |
| `syscall/src/fs/fd.rs` | Implement `TIOCSCTTY` to set foreground pgid, add trace logging |
| `levitate/src/input.rs` | Feed keyboard input to `CONSOLE_TTY` from interrupt handler |
| `sched/src/lib.rs` | Reduce switch_to logging from info to trace (reduce spam) |
| `syscall/src/lib.rs` | Change syscall logging to trace level |
| `syscall/src/fs/read.rs` | Add trace logging for poll_to_tty |
| `syscall/src/process/groups.rs` | Change getpgid logging to trace level |

## Key Decisions (Session 2)

1. **TIOCSCTTY sets foreground pgid**: When a process acquires the controlling terminal via
   `TIOCSCTTY`, it becomes the foreground process group. This allows shells that call `setsid()`
   before checking job control to work correctly.

2. **Keyboard input fed directly to TTY**: Characters are fed to `CONSOLE_TTY` from the VirtIO
   input interrupt handler, not just buffered in `KEYBOARD_BUFFER`. This ensures TTY receives
   input even when no process is actively reading.

3. **Trace-level logging for hot paths**: Scheduler switch and syscall logging changed to trace
   level to avoid flooding output when shell is idle (constant yield/reschedule).

## Gotchas Discovered

- **Job control and session leaders**: Shells typically call `setsid()` to become session leaders,
  which changes their pgid. The TTY's foreground pgid must be updated when the shell acquires
  the controlling terminal, otherwise the shell thinks it's in the background.

- **TIOCSCTTY is critical for interactive shells**: The stub implementation that just returned 0
  was insufficient. Shells expect TIOCSCTTY to establish them as the foreground process group.

### Session 3 (2026-01-12)
- Fixed syscall 4 (stat) on x86_64
  - Added `Stat = 4` to SyscallNumber enum in `arch/x86_64/src/lib.rs`
  - Added `4 => Some(Self::Stat)` to `from_u64()` match arm (was missing!)
  - Wired up dispatcher to call `sys_fstatat(-100, pathname, statbuf, 0)` (AT_FDCWD)
- Root cause of "cat: Function not implemented":
  - BusyBox ash uses stat() to check if commands exist before executing them
  - stat syscall (nr=4) was not implemented → returned ENOSYS
  - ash interpreted ENOSYS as "command can't run" → "Function not implemented"
- Both issues resolved by implementing stat syscall

## Files Modified (Session 3)

| File | Change |
|------|--------|
| `arch/x86_64/src/lib.rs` | Added `Stat = 4` to enum and `from_u64()` match |
| `syscall/src/lib.rs` | Wired Stat to sys_fstatat dispatcher |

### Session 4 (2026-01-12)
- First fix attempt: Made `vfs_stat()` follow symlinks
  - Added `resolve_symlinks()` helper
  - Added `VfsError::TooManySymlinks` (ELOOP) for loop detection
  - Added `vfs_lstat()` for cases where symlinks shouldn't be followed
  - **This was necessary but not sufficient!**

- Actual root cause found: Initramfs hardcoded file permissions!
  - `make_inode()` used `0o444` for files, `0o555` for dirs
  - This stripped execute bits from all files including busybox
  - Shell checked mode for executable bits → no X → EACCES

- Final fix: Preserve actual CPIO permissions
  - Added `mode` field to `CpioEntry` struct
  - Store actual mode from CPIO header (including permission bits)
  - Use `entry.mode` directly instead of hardcoding in `make_inode()`

## Files Modified (Session 4)

| File | Change |
|------|--------|
| `vfs/src/dispatch.rs` | Added `resolve_symlinks()`, modified `vfs_stat()`, added `vfs_lstat()` |
| `vfs/src/error.rs` | Added `TooManySymlinks` variant |
| `vfs/src/lib.rs` | Exported `vfs_lstat` |
| `lib/utils/src/cpio.rs` | Added `mode` field to `CpioEntry` |
| `fs/initramfs/src/lib.rs` | Use `entry.mode` instead of hardcoded permissions |
| `syscall/src/fs/open.rs` | Added debug logging to faccessat |

### Session 5 (2026-01-12)
- Implemented missing syscalls for BusyBox commands:
  - `getdents64` (217): Wired to existing `sys_getdents` (already uses Dirent64 format)
  - `sendfile` (40): Copies data between file descriptors in kernel space
  - `lstat` (6): Gets file status without following symlinks

## Files Modified (Session 5)

| File | Change |
|------|--------|
| `arch/x86_64/src/lib.rs` | Added Getdents64, Sendfile, Lstat to enum and from_u64() |
| `syscall/src/lib.rs` | Wired new syscalls to dispatcher |
| `syscall/src/fs/read.rs` | Implemented sys_sendfile |
| `syscall/src/fs/stat.rs` | Implemented sys_lstat |
| `syscall/src/fs/mod.rs` | Exported sys_sendfile, sys_lstat |

## Verification Results (Session 5)
```
LevitateOS# ls /
bin   dev   etc   init  proc  root  sbin  sys   tmp

LevitateOS# cat /root/hello.txt
Hello from BusyBox initramfs!
```

## Remaining Work

- [x] Implement syscall 4 (`stat` on x86_64) - DONE
- [x] Investigate why `cat` returns "Function not implemented" - RESOLVED (was stat issue)
- [x] Fix "Permission denied" for commands - RESOLVED (initramfs permissions)
- [x] Implement syscall 217 (`getdents64`) - DONE
- [x] Implement syscall 40 (`sendfile`) - DONE
- [x] Implement syscall 6 (`lstat`) - DONE

**Known limitations** (not blocking basic shell usage):
- `mount: mounting proc on /proc failed: Invalid argument` - procfs not implemented
- `mount: mounting sysfs on /sys failed: Invalid argument` - sysfs not implemented

### Session 6 (2026-01-12)
- Brought aarch64 to parity with x86_64

**aarch64 Syscalls Added:**
- `Sendfile = 71` - Copy data between file descriptors (was x86_64 only)
- Note: aarch64's `Getdents = 61` is already getdents64 (no legacy 32-bit version)

**Build System Fixes:**
1. **Userspace linker fix**: Removed `linker = "aarch64-linux-gnu-gcc"` from root `.cargo/config.toml`
   - Root cause: Root config applied to all builds including userspace
   - Kernel has its own `.cargo/config.toml` with the gcc linker setting
   - Userspace needs default `rust-lld` for bare-metal targets

2. **Coreutils no longer required**: Changed `required: false` in `apps.rs`
   - BusyBox provides all utilities now
   - Can still build manually with `cargo xtask build coreutils`

3. **aarch64 BusyBox cross-compilation**: Added `setup_aarch64_cross_compiler()` function
   - Downloads musl cross-compiler from musl.cc
   - Stored in `toolchain/aarch64-linux-musl-cross/`
   - BusyBox now builds for both x86_64 and aarch64

## Files Modified (Session 6)

| File | Change |
|------|--------|
| `.cargo/config.toml` | Removed aarch64 linker override (moved to kernel-only config) |
| `arch/aarch64/src/lib.rs` | Added Sendfile = 71 to syscall enum |
| `syscall/src/lib.rs` | Removed cfg(x86_64) guard from Sendfile dispatcher |
| `xtask/src/build/apps.rs` | Changed coreutils to not required |
| `xtask/src/build/busybox.rs` | Added aarch64 cross-compilation support |

## Verification Results (Session 6)
```
# Both architectures build successfully:
cargo xtask --arch x86_64 build all  ✅
cargo xtask --arch aarch64 build all ✅

# BusyBox built for both architectures:
toolchain/busybox-out/x86_64/busybox    (x86_64 ELF)
toolchain/busybox-out/aarch64/busybox   (aarch64 ELF)
```

## Handoff Notes

All original objectives complete. **BusyBox ash shell is now fully interactive** with:
- Prompt displays correctly
- Keyboard input works
- External commands like `cat`, `echo`, `ls` work correctly
- **Both x86_64 and aarch64 architectures supported**

### Session 7 (2026-01-12)
- Fixed command substitution (`RESULT=$(echo "test")`) in ash shell

**Root Cause Analysis:**
1. Initial issue: fork child not getting CPU time
   - Traced to `ppoll` returning immediately even with nothing ready
   - Fixed in previous session by adding yield loop to ppoll

2. Pipe communication verified working:
   - Added debug logging to trace pipe addresses
   - Confirmed parent and child share same `Arc<Pipe>` after fork
   - Child successfully writes 5 bytes, parent successfully reads them

3. **Actual Bug**: `wait4`/`waitpid` ignoring `WNOHANG` flag!
   - Shell calls: `wait4(-1, status_ptr, 0, NULL)` - blocking wait (reaps child)
   - Then: `wait4(-1, status_ptr, WNOHANG, NULL)` - should return immediately
   - But `sys_waitpid` only took 2 args, ignored options (3rd arg)
   - Second wait4 blocked forever waiting for non-existent child
   - Result: shell hung after command substitution

**Fix:**
- Modified `sys_waitpid(pid, status_ptr)` → `sys_waitpid(pid, status_ptr, options)`
- Added WNOHANG (0x1) support: if no zombie and WNOHANG set, return 0 immediately
- Updated syscall dispatch to pass `frame.arg2()` as options

## Files Modified (Session 7)

| File | Change |
|------|--------|
| `syscall/src/process/lifecycle.rs` | Added `options` parameter to `sys_waitpid`, handle WNOHANG |
| `syscall/src/lib.rs` | Pass arg2 (options) to sys_waitpid |

## Verification Results (Session 7)
```
=== ASH SHELL TEST ===
[TEST 1] echo: PASS
[TEST 2] variables: PASS
[TEST 3] command substitution: PASS
[TEST 4] conditionals: PASS
[TEST 5] for loop: PASS
[TEST 6] arithmetic: PASS
[TEST 7] cat file: PASS
[TEST 8] ls directory: PASS
=== ALL TESTS COMPLETE ===
```

All 8 shell tests pass! Command substitution now works correctly.

## Handoff Notes

All original objectives complete. **BusyBox ash shell is now fully interactive** with:
- Prompt displays correctly
- Keyboard input works
- External commands like `cat`, `echo`, `ls` work correctly
- **Command substitution (`$()`) works** - requires fork, pipe, and wait4 with WNOHANG
- **Both x86_64 and aarch64 architectures supported**

Future teams should:
- Read GOTCHA #37 and #38 before working on memory management
- Use debug builds to catch ttbr0 desync issues early
- Check the warning comment on `map_user_page()` before using it directly
- Understand the TTY/session/pgid relationship when debugging shell issues
- When adding syscalls, remember to update BOTH the enum AND the `from_u64()` match!
- Remember that syscall numbers differ between x86_64 and aarch64
- **Always implement all wait4 flags (WNOHANG, WUNTRACED, etc.)** - shells depend on them!

### Session 8 (2026-01-12)
- Fixed `mkdir -p` failing with "mkdir: can't create directory '/': Invalid argument"

**Root Cause Analysis:**
1. BusyBox's `mkdir -p /some/path` iterates through all path components including "/"
2. For each component, it calls `mkdir()` to ensure the directory exists
3. When `mkdir("/")` was called:
   - `vfs_mkdir("/")` → `lookup_parent("/")` → `Err(InvalidArgument)`
   - `sys_mkdirat` mapped any unknown VFS error to EINVAL
4. `mkdir -p` expects EEXIST for existing directories to continue

**Fix:**
- Added special case in `sys_mkdirat` for root directory
- When path is "/" (or "//", etc.), immediately return EEXIST
- This allows `mkdir -p` to continue processing without error

**Boot-time Verification:**
- Added `mkdir -p` test to `/etc/inittab` sysinit phase
- Test creates `/tmp/testdir/nested` and verifies it exists
- Shows `[MKDIR-P TEST] PASS` on every boot

## Files Modified (Session 8)

| File | Change |
|------|--------|
| `syscall/src/fs/dir.rs` | Handle root directory in `sys_mkdirat` - return EEXIST |
| `xtask/src/build/commands.rs` | Added boot-time mkdir -p verification test |

## Key Decisions (Session 8)

1. **Return EEXIST not ENOENT for root**: Since "/" always exists, EEXIST is the correct
   error code. This matches Linux behavior where `mkdir("/")` returns EEXIST.

2. **Normalize path before checking**: Strip trailing slashes so "//" also returns EEXIST.

## Verification Results (Session 8)
```
LevitateOS (BusyBox) starting...
mount: mounting proc on /proc failed: Invalid argument
mount: mounting sysfs on /sys failed: Invalid argument
[MKDIR-P TEST] PASS
LevitateOS#
```

`mkdir -p` now works correctly for all paths.

### Session 9 (2026-01-12)
- Created comprehensive coreutils test suite at `/root/test-core.sh`
- Discovered kernel bug during testing

**Test Suite Created:**
- 13 phases testing 90+ operations
- Tests in deliberate dependency order:
  1. Output (echo, printf)
  2. Directory creation (mkdir)
  3. File creation (touch, echo >)
  4. File reading (cat, head, tail, wc)
  5. File manipulation (cp, mv, rm, ln)
  6. Directory listing (ls, pwd, basename, dirname)
  7. Text processing (grep, sed, tr, cut, sort, uniq)
  8. Conditionals (test, true, false, expr)
  9. Iteration (seq, xargs)
  10. System info (uname, id, hostname)
  11. Pipes and redirection
  12. Command substitution
  13. Find

**Kernel Bug Discovered:**
- **Symptom**: `ls /root` shows empty, but `cat /root/hello.txt` works
- **Root cause**: Initramfs subdirectory contents aren't populated in dentry cache
- Files CAN be read if you know the exact path
- But `readdir` (getdents64) doesn't enumerate them
- This affects all subdirectories in the initramfs (/root, /etc contents work, but not visible in ls)

**Impact on Test Suite:**
- Tests that depend on directory listings fail
- Tests that depend on file creation fail (can't verify via stat/ls)
- Basic shell operations work (echo, pipes, command substitution)

## Files Modified (Session 9)

| File | Change |
|------|--------|
| `xtask/src/build/commands.rs` | Created `/root/test-core.sh` with comprehensive test suite |
| `.teams/TEAM_459_implement_memory_surgical_fixes.md` | Documented kernel bug |

## Known Kernel Bug

**Initramfs readdir doesn't populate subdirectory entries**

To reproduce:
```
ls /root           # Shows empty (only . and ..)
cat /root/hello.txt  # Works! Shows "Hello from BusyBox initramfs!"
```

The VFS layer can resolve exact paths to inodes, but `readdir` doesn't enumerate
directory contents that come from the initramfs. This needs investigation in:
- `fs/initramfs/src/lib.rs` - How directory entries are created
- `vfs/src/dentry.rs` - How dentry cache is populated
- `syscall/src/fs/dir.rs` - How getdents works

Future team should fix this bug to enable full coreutils testing.

### Session 10 (2026-01-12)
- Pivoted from bug fixing to refactor audit (per user request)
- Created comprehensive refactor plan at `docs/planning/kernel-refactor/REFACTOR_PLAN.md`

**Audit Scope:**
- Searched for hardcoded values (magic numbers, paths, sizes, addresses)
- Identified scalability issues (O(n) algorithms, fixed arrays, global locks)

**Critical Findings (P0 - Blocks >50 processes):**

| Issue | Location | Impact |
|-------|----------|--------|
| VMA linear scan | `mm/src/vma.rs:99-105` | O(n) on every mmap/brk |
| FD alloc linear scan | `sched/src/fd_table.rs:142-159` | O(n) on every open() |
| MAX_FDS = 64 | `sched/src/fd_table.rs:18` | Hits limit quickly |
| Mount lookup linear | `vfs/src/mount.rs:198-200` | O(n) on every path op |
| Single scheduler lock | `sched/src/scheduler.rs:7-11` | Blocks all multi-core |

**Hardcoded Value Issues (P1):**

| Value | Location | Problem |
|-------|----------|---------|
| Screen 1280x800 | `init.rs:341-342` | Mismatched with input.rs |
| Screen 1024x768 | `input.rs:128` | Mismatched with init.rs |
| Tmpfs 16MB/64MB | `fs/tmpfs/src/node.rs:15-16` | Should be % of RAM |

**Recommended Implementation Order:**
1. FD table bitmap (quick win)
2. VMA interval tree (complex but critical)
3. Mount table radix trie
4. Create kernel config module
5. Per-CPU scheduler (SMP prep)

## Files Created (Session 10)

| File | Description |
|------|-------------|
| `docs/planning/kernel-refactor/REFACTOR_PLAN.md` | Comprehensive refactor plan |

## Handoff Notes (Session 10)

The kernel has accumulated significant technical debt:
- **Data structures don't scale**: VMA list, FD table, mount table all use O(n) algorithms
- **Limits too low**: MAX_FDS=64 will break any real application
- **Values inconsistent**: Screen resolution defined differently in two files
- **SMP blocked**: Single global scheduler lock prevents multi-core

The refactor plan is ready. Next team should:
1. Read `docs/planning/kernel-refactor/REFACTOR_PLAN.md`
2. Start with Phase 1 (data structure fixes) for immediate impact
3. Run stress tests after each change to verify scalability
