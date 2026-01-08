# Phase 2: Structural Extraction

## Target Design
The `libsyscall` crate will have the following structure:

```rust
// lib.rs
pub mod sysno;
pub mod errno;
pub mod io;
pub mod fs;
pub mod process;
pub mod mm;
pub mod net; // if needed
pub mod sched; // sched_yield

// Re-exports to maintain compatibility
pub use sysno::*;
pub use errno::*;
pub use io::*;
pub use fs::*;
pub use process::*;
pub use mm::*;
pub use net::*;
pub use sched::*;
```

## Extraction Strategy
We will extract in the following order:
1. **Constants**: `sysno` and `errno` are dependencies for everything else.
2. **Core IO**: `read`, `write`, `open`, `close` are fundamental.
3. **Subsystems**: `fs`, `mm`, `process`.

## Modular Refactoring Rules
- Internal syscall helpers (like `syscall0`, `syscall1`) should ideally be shared. We might put them in an internal module or `sysno`.
- Each syscall wrapper should be close to its related constants (e.g., `open` near `O_RDONLY`).

## Steps

### Step 1: Base Modules
- Extract `sysno.rs` (Syscall numbers).
- Extract `errno.rs` (Error codes).
- Extract necessary internal helpers (`syscall!` macros or functions).

### Step 2: IO and Filesystem
- Extract `io.rs` (`read`, `write`, `close`, `ioctl`).
- Extract `fs.rs` (`openat`, `mkdirat`, `linkat`, `unlinkat`, `stat`, etc.).

### Step 3: Memory and Process
- Extract `mm.rs` (`mmap`, `munmap`, `mprotect`, `brk`/`sbrk`).
- Extract `process.rs` (`getpid`, `exit`, `clone`, `exec`, `spawn`, `wait`).

### Step 4: Misc
- Extract `sched.rs` (`sched_yield`).
- Extract `net.rs` (if any networking syscalls exist/are stubbed).

### Step 5: Cleanup `lib.rs`
- Remove all moved code.
- Ensure all re-exports cover the previous public API.
