# Phase 2: Design

## Proposed Solution
We will add 5 new test binaries to `userspace/levbox/src/bin/test/`. Each binary will focus on a specific domain of syscalls.

### 1. `stat_test.rs`
- **Goal**: Verify file metadata and timestamp updates.
- **Scenarios**:
  - `fstat` on a regular file: Check `st_size`, `st_mode` (type/perms), `st_nlink`.
  - `utimensat`: Update atime/mtime, verify change with `fstat`.
  - `fstat` on a directory: Check `st_mode` & S_IFDIR.

### 2. `link_test.rs`
- **Goal**: Verify hard and soft links.
- **Scenarios**:
  - `symlinkat`: Create symlink, `readlinkat` to verify target, `openat` symlink (should open target).
  - `linkat`: Create hard link, verify `st_nlink == 2`, modify via one link, verify other updates.
  - `unlinkat`: Remove links, verify file remains until last link gone.

### 3. `time_test.rs`
- **Goal**: Verify time syscalls.
- **Scenarios**:
  - `clock_gettime`: Verify monotonic increase.
  - `nanosleep`: Sleep for X ms, verify elapsed time >= X.

### 4. `sched_yield_test.rs`
- **Goal**: Verify voluntary preemption.
- **Scenarios**:
  - Fork/Clone two threads.
  - Each thread increments a shared counter and yields.
  - Verify interleaving (loosely) or just that both threads complete.

### 5. `error_test.rs`
- **Goal**: Negative testing.
- **Scenarios**:
  - `openat` non-existent file -> ENOENT.
  - `read` invalid fd -> EBADF.
  - `mmap` invalid flags -> EINVAL (if enforced).
  - `waitpid` non-child -> ECHILD/ESRCH.

## API Design
Tests will follow the standard pattern:
```rust
#[no_mangle]
pub fn main() -> i32 {
    if !test_case_1() { return 1; }
    if !test_case_2() { return 1; }
    println!("[PASS] test_name");
    0
}
```

## Open Questions
- **Q**: Do we have `readlinkat` wrapper?
  - **Check**: `SYS_READLINKAT` exists in `sysno.rs`. Need to confirm if wrapper exists in `fs.rs`.
  - **Resolution**: If missing, we must implement it first.
- **Q**: Do we have `clock_gettime` wrapper?
  - **Check**: `SYS_CLOCK_GETTIME` exists. Wrapper in `src/time.rs`.
- **Q**: How precise is `nanosleep` in current scheduler?
  - **Assumption**: It should be "at least" the requested duration.

## Design Decisions
- Keeping tests separate avoids "mega-test" brittleness.
- Using `libsyscall` directly (no `std`) ensures we are testing the raw ABI.
