# Phase 3 — Fix Design and Validation Plan

## Root Cause Summary
Kernel crashes (EL1 Data Abort) when accessing unmapped user pointers because it trusts the pointer without validation.

## Fix Strategy

We will implement **Option A: Page Table Walk**.
We will add a helper function `validate_user_buffer` that manually walks the current process's page tables (TTBR0) to verify that the entire range `[ptr, ptr+len)` is:
1. Mapped (Valid bit set).
2. Accessible to EL0 (User bit set).
3. Has correct permissions (Read-only vs Read-Write).

### Proposed API

```rust
// kernel/src/memory/validation.rs (New Module)

/// Error types for validation
pub enum ValidationError {
    NotMapped,
    PermissionDenied,
    InvalidRange, // Overflow or kernel space
}

/// Validate that a user pointer range is readable.
pub fn check_user_read(addr: usize, len: usize) -> Result<(), ValidationError> { ... }

/// Validate that a user pointer range is writable.
pub fn check_user_write(addr: usize, len: usize) -> Result<(), ValidationError> { ... }
```

### Integration

In `syscall.rs`:

```rust
fn sys_write(fd: usize, buf: usize, len: usize) -> i64 {
    // ... existing fd checks ...
    
    // NEW VALIDATION
    if let Err(_) = crate::memory::check_user_read(buf, len) {
        return errno::EFAULT;
    }
    
    // SAFETY: We just validated the memory is mapped and readable.
    let slice = unsafe { core::slice::from_raw_parts(buf as *const u8, len) };
    
    // ...
}
```

## Reversal Strategy
If the fix causes valid programs to fail (false positives):
1. Revert the calls in `syscall.rs`.
2. Investigating `check_user_read` logic.

## Test Strategy

1. **Reproduction Test**: Run `userspace/repro_crash`.
   - **Before**: Kernel Panic.
   - **After**: `sys_write` returns `-3` (EFAULT), app prints "Survived! Ret = -3" and exits cleanly.
   
2. **Regression Test**: Run `userspace/shell`.
   - Ensure standard output still works (valid pointers).
