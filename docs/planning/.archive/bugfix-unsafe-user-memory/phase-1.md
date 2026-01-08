# Phase 1 — Understanding and Scoping

**Team**: TEAM_224
**Bug**: Unsafe User Memory Access in Syscalls
**Status**: Complete (from TEAM_223 investigation)

---

## 1. Bug Summary

**Description**: Multiple syscalls in the LevitateOS kernel create Rust slices directly from user-space virtual addresses. After calling `validate_user_buffer()`, the code uses `core::slice::from_raw_parts()` with the user VA as the pointer. This is unsafe because the validation only checks that pages are *currently* mapped—it doesn't prevent:

1. TTBR0 changes between validation and slice access
2. Page table modifications by other code paths
3. The slice pointing to kernel-inaccessible memory

**Severity**: **Critical** — Can cause undefined behavior, kernel panics, or security vulnerabilities.

**Impact**: 
- Potential kernel crashes on syscall use
- Security risk if attacker can manipulate page tables
- Undefined behavior violates Rust's safety guarantees

---

## 2. Reproduction Status

**Reproducible**: Yes, by code inspection. The bug is latent—it manifests as UB under specific timing conditions.

**Reproduction Steps**:
1. Call `sys_spawn("/path", 5)` from userspace
2. If another thread unmaps the path buffer between validation and slice creation, UB occurs

**Expected Behavior**: Syscalls should safely copy user data through kernel-accessible pointers.

**Actual Behavior**: Syscalls create slices from user VAs directly, bypassing the kernel's address space.

---

## 3. Context

### Affected Files

| File | Function | Line | Pattern |
|------|----------|------|---------|
| `kernel/src/syscall/process.rs` | `sys_spawn` | 50 | `from_raw_parts(path_ptr as *const u8, path_len)` |
| `kernel/src/syscall/process.rs` | `sys_exec` | 104 | `from_raw_parts(path_ptr as *const u8, path_len)` |
| `kernel/src/syscall/process.rs` | `sys_spawn_args` | 174 | `from_raw_parts(path_ptr as *const u8, path_len)` |
| `kernel/src/syscall/process.rs` | `sys_spawn_args` | 220 | `from_raw_parts(entry.ptr as *const u8, arg_len)` |
| `kernel/src/syscall/fs/write.rs` | `sys_write` | 85 | `from_raw_parts(buf as *const u8, len)` |

### Correct Pattern (for reference)

The correct pattern is already used in other syscalls like `sys_openat`:

```rust
// kernel/src/syscall/fs/open.rs:22-29
let mut path_buf = [0u8; 256];
for i in 0..path_len {
    if let Some(ptr) = mm_user::user_va_to_kernel_ptr(task.ttbr0, path + i) {
        path_buf[i] = unsafe { *ptr };
    } else {
        return errno::EFAULT;
    }
}
```

### Recent Changes
- No recent changes to these functions
- Bug has existed since original implementation

---

## 4. Constraints

- **Backwards Compatibility**: Syscall ABI must remain unchanged
- **Performance**: Copy overhead is acceptable for correctness
- **Time Sensitivity**: Should be fixed before any production use
- **Platforms**: AArch64 (primary), x86_64 (future)

---

## 5. Open Questions

| # | Question | Status |
|---|----------|--------|
| Q1 | Should we create a helper function for user string copying? | **Yes** — DRY principle |
| Q2 | Should VfsFile write also copy byte-by-byte? | **Yes** — already does it correctly |
| Q3 | Are there other syscalls with this pattern? | Checked — these are the only ones |

---

## 6. Steps

### Step 1 — Consolidate Bug Information ✅
Completed in TEAM_223 investigation.

### Step 2 — Confirm Reproduction Pattern ✅
Pattern confirmed by code inspection. All affected locations identified.

### Step 3 — Identify Fix Pattern ✅
Correct pattern exists in `sys_openat` and VFS write path. Will use as template.

---

## Phase 1 Complete

**Outcome**: Bug is fully understood and scoped. Ready for Phase 2.
