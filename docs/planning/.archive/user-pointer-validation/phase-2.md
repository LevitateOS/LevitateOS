# Phase 2 — Root Cause Analysis

## Root Cause

The kernel panic (Synchronous Exception: Data Abort) is caused by the kernel attempting to access a virtual address provided by userspace that is not mapped in the page tables.

Specifically:
1. `sys_write` receives a raw pointer `bad_ptr`.
2. It calls `core::slice::from_raw_parts(bad_ptr, len)` creating a slice.
3. It iterates over this slice (e.g. via `str::from_utf8` or `print!`).
4. The CPU attempts to load from `bad_ptr`.
5. The MMU raises a Data Abort because `bad_ptr` is unmapped.
6. The kernel's exception handler catches this. Since it happened in EL1 (kernel mode), it's treated as a fatal kernel error (panic).

## Key Code Areas

- `kernel/src/syscall.rs`: The call sites `unsafe { from_raw_parts(...) }`.
- `kernel/src/memory.rs`: Missing validation logic.

## Conclusion

We must implement a software check that mimics the MMU's check *before* the kernel attempts the access. If the check fails, we return `EFAULT` instead of crashing.
