# Phase 2: Structural Extraction - Syscall Refactor

## Target Design
The syscall handlers will be organized into a new module tree:
- `kernel/src/syscall/mod.rs`: Dispatcher, error codes, and shared traits/helpers.
- `kernel/src/syscall/process.rs`: Process management (exit, spawn, exec, yield, getpid).
- `kernel/src/syscall/fs.rs`: File system operations (read, write, open, close, fstat).
- `kernel/src/syscall/mm.rs`: Memory management (sbrk).
- `kernel/src/syscall/time.rs`: Time-related operations (nanosleep, clock_gettime).
- `kernel/src/syscall/sys.rs`: System-wide operations (shutdown).

## Extraction Strategy
1. **Parallel Extraction**: Create the new files and copy the code.
2. **Modularization**: Ensure each module has its own imports.
3. **Internal Helpers**: Move internal helpers (like `write_to_user_buf`) to the most appropriate place or keep them in `mod.rs` if shared.

## Steps
1. **Step 1 – Create `kernel/src/syscall/mod.rs`**
   - Define the module structure.
   - Move `errno`, `SyscallNumber`, and `syscall_dispatch`.
2. **Step 2 – Create sub-modules**
   - `fs.rs`, `process.rs`, `mm.rs`, `time.rs`, `sys.rs`.
3. **Step 3 – Move implementations**
   - Copy functions one by one from `syscall.rs`.
