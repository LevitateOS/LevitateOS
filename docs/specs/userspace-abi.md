# LevitateOS Userspace ABI Specification

**Status:** DRAFT (Phase 10 Planning)
**Version:** 0.1.0

This document defines the Application Binary Interface (ABI) between the LevitateOS kernel and userspace applications. Adherence to this specification is required to support the Rust standard library (`std`) and UNIX-like utilities.

## 1. System Call ABI (AArch64)

LevitateOS uses the standard AArch64 system call convention (compatible with Linux/Redox).

| Register | Usage |
|----------|-------|
| `x8` | System Call Number (Syscall ID) |
| `x0` - `x5` | Arguments (up to 6) |
| `x0` | Return Value (Success: `>= 0`, Error: `-errno`) |
| `svc #0` | Instruction to trigger syscall |

- **Clobbered Registers:** `x0`-`x18` (Caller-saved)
- **Preserved Registers:** `x19`-`x29`, `sp` (Callee-saved)

## 2. Required System Calls for Rust `std`

To compile `std` for `target_os = "levitate"`, the following kernel interfaces must be provided.

### 2.1 Memory Management (Global Allocator)
Rust's `alloc` crate requires a way to map memory.
- `sys_brk(addr: usize) -> usize`: Set the program break (heap end).
- `sys_mmap(...) -> usize`: (Optional for now) Map file/device to memory.

### 2.2 Standard I/O (File Descriptors)
Rust's `std::io` (print, scan) relies on file descriptors 0, 1, and 2.
- `sys_write(fd: usize, buf: *const u8, len: usize) -> isize`: Write to stream.
- `sys_read(fd: usize, buf: *mut u8, len: usize) -> isize`: Read from stream.
- `sys_ioctl(fd: usize, request: usize, ...) -> isize`: Control TTY mode (Raw/Cooked).

### 2.3 Process Management
Rust's `std::process` and `std::thread`.
- `sys_exit(code: usize) -> !`: Terminate process.
- `sys_sched_yield() -> usize`: Yield CPU slice.
- `sys_getpid() -> usize`: Get current Process ID.
- `sys_spawn(path: *const str) -> isize`: Create new process (simplified `fork`+`exec`).
- `sys_waitpid(pid: usize, status: *mut i32) -> isize`: Wait for child exit.

### 2.4 Time
Rust's `std::time::Instant`.
- `sys_clock_gettime(clk_id: usize, tp: *mut Timespec) -> isize`: Monotonic/Realtime clock.

## 3. Comparison with External Kernels

| Feature | LevitateOS (Target) | Linux (AArch64) | Redox OS |
|---------|---------------------|-----------------|----------|
| **Trigger** | `svc #0` | `svc #0` | `svc #0` |
| **Syscall Reg** | `x8` | `x8` | `x8` |
| **Args** | `x0`-`x5` | `x0`-`x5` | `x0`-`x4` |
| **Path Format** | UTF-8 String + Len | Null-terminated C-String | UTF-8 Buffer |

## 4. Implementation Strategy (Phase 10)

1.  **Syscall Expansion:** Implement missing syscalls (`brk`, `waitpid`, `clock_gettime`) in `kernel/src/syscall.rs`.
2.  **ulib Shim:** Create `userspace/ulib` to wrap these raw syscalls into safe Rust traits (`Read`, `Write`, `GlobalAlloc`).
3.  **Coreutils:** Build `ls`, `cat` against `ulib`.

## References
- [Linux AArch64 Syscall Table](https://github.com/torvalds/linux/blob/master/include/uapi/asm-generic/unistd.h)
- [Redox Syscall Scheme](https://doc.redox-os.org/book/ch04-03-syscalls.html)
- `tock/doc/syscalls`
