# Phase 2 — Root Cause Analysis

**Team**: TEAM_224
**Bug**: Unsafe User Memory Access in Syscalls
**Status**: Complete

---

## 1. Hypotheses

| # | Hypothesis | Evidence | Confidence |
|---|------------|----------|------------|
| H1 | Developer copied pattern without understanding MMU implications | Code uses `validate_user_buffer` correctly, then ignores its purpose | **High** |
| H2 | Originally worked because TTBR0/TTBR1 split wasn't fully implemented | Early code may have had identity mapping | Medium |
| H3 | Rushed implementation to get syscalls working | Comments show iterative development | Medium |

**Selected Hypothesis**: H1 — The pattern suggests the developer understood validation was needed but didn't understand that user VAs aren't directly accessible from kernel context.

---

## 2. Root Cause

### 2.1 Technical Explanation

On AArch64, the kernel runs with:
- **TTBR1_EL1**: Kernel page tables (addresses ≥ 0xFFFF_0000_0000_0000)
- **TTBR0_EL1**: User page tables (addresses < 0x0000_8000_0000_0000)

When the kernel accesses a user VA directly:
1. The MMU uses TTBR0 for translation
2. TTBR0 contains the *current process's* page tables
3. If context switches or page tables change, the access becomes invalid

The correct approach:
1. Walk the user's page tables (using `ttbr0` from TCB)
2. Find the physical address
3. Convert to kernel VA via `phys_to_virt()`
4. Access through the kernel VA

### 2.2 Why Validation Isn't Enough

`validate_user_buffer()` checks:
- Buffer is within user address space bounds
- Pages are currently mapped with correct permissions

But it does NOT:
- Pin the pages
- Prevent concurrent unmapping
- Make the VA accessible from kernel context

### 2.3 Faulty Code Pattern

```rust
// WRONG: Creates slice from user VA
let task = crate::task::current_task();
if mm_user::validate_user_buffer(task.ttbr0, path_ptr, path_len, false).is_err() {
    return errno::EFAULT;
}
let path_bytes = unsafe { core::slice::from_raw_parts(path_ptr as *const u8, path_len) };
```

### 2.4 Correct Code Pattern

```rust
// RIGHT: Copies through kernel-accessible pointers
let task = crate::task::current_task();
if mm_user::validate_user_buffer(task.ttbr0, path, path_len, false).is_err() {
    return errno::EFAULT;
}
let mut path_buf = [0u8; 256];
for i in 0..path_len {
    if let Some(ptr) = mm_user::user_va_to_kernel_ptr(task.ttbr0, path + i) {
        path_buf[i] = unsafe { *ptr };
    } else {
        return errno::EFAULT;
    }
}
```

---

## 3. Affected Locations (Detailed)

### 3.1 `sys_spawn` — Line 50

```rust
let path_bytes = unsafe { core::slice::from_raw_parts(path_ptr as *const u8, path_len) };
```

**Context**: Reading executable path from userspace.

### 3.2 `sys_exec` — Line 104

```rust
let path_bytes = unsafe { core::slice::from_raw_parts(path_ptr as *const u8, path_len) };
```

**Context**: Reading executable path for exec.

### 3.3 `sys_spawn_args` — Line 174

```rust
let path_bytes = unsafe { core::slice::from_raw_parts(path_ptr as *const u8, path_len) };
```

**Context**: Reading executable path with arguments.

### 3.4 `sys_spawn_args` — Line 220

```rust
let arg_bytes = unsafe { core::slice::from_raw_parts(entry.ptr as *const u8, arg_len) };
```

**Context**: Reading individual argument strings.

### 3.5 `sys_write` (stdout/stderr path) — Line 85

```rust
let slice = unsafe { core::slice::from_raw_parts(buf as *const u8, len) };
```

**Context**: Writing to console. Note: VfsFile path already copies correctly.

---

## 4. Investigation Summary

| Step | Finding |
|------|---------|
| Traced sys_spawn | Uses user VA directly after validation |
| Traced sys_exec | Same pattern as sys_spawn |
| Traced sys_spawn_args | Two instances of the pattern |
| Traced sys_write | Console path uses user VA; VFS path is correct |
| Checked sys_read | Uses `write_to_user_buf` helper — correct |
| Checked sys_openat | Uses byte-by-byte copy — correct |

---

## 5. Root Cause Statement

**The bug occurs because syscalls create Rust slices from user virtual addresses, which are not directly accessible from kernel context. The `validate_user_buffer()` call only verifies the mapping exists but doesn't make the memory kernel-accessible. The fix requires copying data through `user_va_to_kernel_ptr()` which translates user VAs to kernel-accessible pointers.**

---

## Phase 2 Complete

**Outcome**: Root cause fully identified. The fix is straightforward — replace direct slice creation with byte-by-byte copying through kernel pointers.
