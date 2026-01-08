# Phase 3 — Fix Design and Validation Plan

**Team**: TEAM_224
**Bug**: Unsafe User Memory Access in Syscalls
**Status**: Ready for Implementation

---

## 1. Root Cause Summary

Syscalls create Rust slices from user virtual addresses after validation. User VAs are not directly accessible from kernel context because they require TTBR0 translation. The kernel must copy data through `user_va_to_kernel_ptr()` which walks the user's page tables and returns a kernel-accessible pointer.

**Location**: `kernel/src/syscall/process.rs` and `kernel/src/syscall/fs/write.rs`

---

## 2. Fix Strategy

### 2.1 Approach: Copy Through Kernel Pointers

Replace all instances of:
```rust
let bytes = unsafe { core::slice::from_raw_parts(user_ptr as *const u8, len) };
```

With:
```rust
let mut buf = [0u8; MAX_LEN];
for i in 0..len {
    if let Some(ptr) = mm_user::user_va_to_kernel_ptr(ttbr0, user_ptr + i) {
        buf[i] = unsafe { *ptr };
    } else {
        return errno::EFAULT;
    }
}
```

### 2.2 Helper Function (DRY)

Create a reusable helper to reduce code duplication:

```rust
/// Copy a string from user space into a kernel buffer.
/// Returns the valid slice of the buffer on success.
pub fn copy_user_string<'a>(
    ttbr0: usize,
    user_ptr: usize,
    len: usize,
    buf: &'a mut [u8],
) -> Result<&'a str, i64> {
    let len = len.min(buf.len());
    if mm_user::validate_user_buffer(ttbr0, user_ptr, len, false).is_err() {
        return Err(errno::EFAULT);
    }
    for i in 0..len {
        if let Some(ptr) = mm_user::user_va_to_kernel_ptr(ttbr0, user_ptr + i) {
            buf[i] = unsafe { *ptr };
        } else {
            return Err(errno::EFAULT);
        }
    }
    core::str::from_utf8(&buf[..len]).map_err(|_| errno::EINVAL)
}
```

### 2.3 Tradeoffs

| Aspect | Before | After |
|--------|--------|-------|
| Safety | Unsafe — UB possible | Safe — proper memory access |
| Performance | Single pointer cast | O(n) byte copies |
| Code Size | Minimal | Slightly larger |
| Correctness | Incorrect | Correct |

**Decision**: Safety and correctness outweigh the minor performance cost.

---

## 3. Reversal Strategy

### 3.1 How to Revert

If the fix causes issues:
1. `git revert <commit>` the fix commit
2. Re-run test suite to confirm revert works
3. Document why revert was needed

### 3.2 Signals for Revert

- New test failures not present before fix
- Performance regression > 10% on syscall-heavy workloads
- New crashes in syscall paths

### 3.3 Revert Steps

```bash
git log --oneline -5  # Find fix commit hash
git revert <hash>
cargo xtask test      # Verify revert works
```

---

## 4. Test Strategy

### 4.1 Existing Tests

- `cargo xtask test` — Run full test suite before and after
- Golden boot test — Verify boot sequence unchanged

### 4.2 New Tests (if feasible)

Since this is a kernel, unit testing syscalls is complex. Verification will be:

1. **Build succeeds** — No compilation errors
2. **Boot test passes** — System boots and runs init
3. **Manual verification** — Run shell, execute commands
4. **Code review** — Ensure pattern matches `sys_openat`

### 4.3 Regression Protection

The fix itself is the regression protection — it prevents UB that could cause crashes.

---

## 5. Impact Analysis

### 5.1 API Changes

**None** — Syscall ABI is unchanged. Only internal implementation changes.

### 5.2 Behavior Changes

**None observable** — Correct programs will work identically. Programs exploiting the bug would fail safely with EFAULT.

### 5.3 Performance Impact

**Minimal** — String copying adds O(n) operations where n is typically < 256 bytes. This is negligible compared to the cost of process spawning or file I/O.

### 5.4 Downstream Impact

**None** — No modules depend on the buggy behavior.

---

## 6. Implementation Plan

### Step 1: Create Helper Function

Add `copy_user_string()` to `kernel/src/syscall/mod.rs`

### Step 2: Fix `sys_spawn`

Replace slice creation with helper call.

### Step 3: Fix `sys_exec`

Replace slice creation with helper call.

### Step 4: Fix `sys_spawn_args` (both locations)

Replace slice creation with helper call for path and arguments.

### Step 5: Fix `sys_write` (console path)

Replace slice creation with byte-by-byte copy.

### Step 6: Verify

1. `cargo build --release -p levitate-kernel`
2. `cargo xtask test`
3. `./run.sh` and manually test shell

---

## Phase 3 Complete

**Outcome**: Fix design is complete. Ready for Phase 4 implementation.
