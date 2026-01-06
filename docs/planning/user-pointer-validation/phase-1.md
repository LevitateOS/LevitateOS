# Phase 1 — Understanding and Scoping

**TEAM_135** | User Pointer Validation — Security Critical

## Bug Summary

**Issue:** Syscall handlers accept user-provided pointers without proper validation, creating a security vulnerability.

**Severity:** **CRITICAL (Security)**

**Impact:**
- Malicious userspace can crash kernel via page fault on unmapped addresses
- Potential information disclosure if kernel reads from wrong memory
- Potential privilege escalation if kernel writes to unexpected locations

## Current State

### Affected Syscalls

| Syscall | File | Line | Pattern |
|---------|------|------|---------|
| `sys_read` | `kernel/src/syscall.rs` | 217 | `from_raw_parts_mut(buf, max_read)` |
| `sys_write` | `kernel/src/syscall.rs` | 290 | `from_raw_parts(buf, len)` |
| `sys_open` | `kernel/src/syscall.rs` | 347 | `from_raw_parts(path_ptr, path_len)` |
| `sys_stat` | `kernel/src/syscall.rs` | 402 | `from_raw_parts(path_ptr, path_len)` |

### Current "Validation"

```rust
// Only checks address range, NOT:
// - Whether memory is mapped
// - Whether user has read/write permission
// - Whether entire range is valid
if buf >= 0x0000_8000_0000_0000 {
    return errno::EFAULT;
}

// SAFETY: We've validated the buffer is in user address space.
// ^^^ THIS IS INSUFFICIENT ^^^
let user_buf = unsafe { core::slice::from_raw_parts_mut(buf as *mut u8, max_read) };
```

## Security Vulnerabilities

### V1: Unmapped Memory Access

**Attack:** Pass pointer to unmapped user address
**Result:** Page fault in kernel context
**Severity:** DoS (crash) at minimum

### V2: Permission Bypass

**Attack:** Pass read-only page address to write syscall
**Result:** Kernel writes to read-only memory
**Severity:** Memory corruption

### V3: Cross-Address-Space Access

**Attack:** Pass address in another process's address space (future concern)
**Result:** Information disclosure or memory corruption
**Severity:** Full compromise

### V4: Range Overflow

**Attack:** Pass valid start address with length that extends into kernel space
**Result:** Kernel memory access
**Severity:** Full compromise

**Note:** Current code caps length at 4096, mitigating this somewhat.

## Requirements

A proper user pointer validation must:

1. **Verify mapping exists** — Walk user page tables to confirm pages are mapped
2. **Verify permissions** — Check read/write bits match operation
3. **Verify entire range** — All pages from `ptr` to `ptr+len` must be valid
4. **Atomic check-and-use** — Prevent TOCTOU (time-of-check-time-of-use) races
5. **Handle edge cases** — Zero length, page-crossing buffers, etc.

## Design Options

### Option A: Walk Page Tables (Recommended)

Walk the user's TTBR0 page tables to verify each page is mapped with correct permissions.

```rust
/// Validate a user pointer for reading.
/// Returns Ok(slice) if valid, Err(EFAULT) if not.
pub fn validate_user_read<'a>(
    ttbr0: usize,
    ptr: usize,
    len: usize,
) -> Result<&'a [u8], i64> {
    // Walk page tables for each page in range
    let start_page = ptr & !0xFFF;
    let end_page = (ptr + len - 1) & !0xFFF;
    
    for page in (start_page..=end_page).step_by(4096) {
        if !is_user_page_readable(ttbr0, page) {
            return Err(errno::EFAULT);
        }
    }
    
    // SAFETY: We've verified all pages are mapped and readable
    Ok(unsafe { core::slice::from_raw_parts(ptr as *const u8, len) })
}
```

**Pros:**
- Precise validation
- No architectural changes
- Works with current page table design

**Cons:**
- Page table walk on every syscall (performance cost)
- Must handle concurrent page table modifications

### Option B: Copy-In/Copy-Out

Always copy user data to kernel buffer, never access user memory directly.

```rust
pub fn copy_from_user(dst: &mut [u8], src_ptr: usize) -> Result<(), i64> {
    // Copy byte-by-byte with page fault handling
    for (i, dst_byte) in dst.iter_mut().enumerate() {
        *dst_byte = read_user_byte(src_ptr + i)?;
    }
    Ok(())
}
```

**Pros:**
- Simpler to reason about
- Natural TOCTOU protection

**Cons:**
- Memory overhead (kernel buffer)
- Copy overhead
- Still needs page table walk per byte (unless using fault handler)

### Option C: Use Page Fault Handler (Linux-like)

Access user memory directly, let page fault handler validate.

**Pros:**
- Fast path has zero overhead
- Deferred validation

**Cons:**
- Complex exception handling
- Harder to return proper error codes
- Requires careful kernel state management

## Recommended Approach: Option A

For LevitateOS's current stage, **Option A (page table walk)** is the best balance of:
- Security (proper validation)
- Simplicity (no exception handler changes)
- Performance (acceptable for current workload)

## Open Questions

1. **How to get current TTBR0?** — Need to access current task's page table pointer
2. **Locking?** — How to prevent page table modifications during validation?
3. **Should we cache validation?** — Performance optimization for repeated accesses

---

## Steps

- [ ] Step 1 — Design `validate_user_ptr()` API
- [ ] Step 2 — Implement page table walk for user pages
- [ ] Step 3 — Integrate with syscall handlers
- [ ] Step 4 — Add tests

---

## References

- Linux `copy_from_user()` / `copy_to_user()`
- FreeBSD `copyin()` / `copyout()`
- Redox OS syscall validation
