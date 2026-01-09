# TEAM_363 — Eyra Static-PIE Argument Crash

**Created:** 2026-01-09  
**Severity:** 🔴 Blocker for Eyra migration  
**Component:** Kernel ELF loader / argument passing

---

## Bug Description

Eyra (static-pie) binaries crash with **General Protection Fault** when invoked with more than 1 argument.

### Reproduction

```bash
# Works (1 arg)
cargo xtask vm exec "eyra-hello"
# Output: [OK] argc = 1

# Crashes (2 args)
cargo xtask vm exec "eyra-hello arg1"
# Output: KERNEL PANIC: EXCEPTION: GENERAL PROTECTION FAULT
```

### Stack Frame at Crash

```
instruction_pointer: 124658 (0x1E6F2)
code_segment: 35 (user code)
stack_pointer: 140737488289096
stack_segment: 27 (user data)
Error Code: 0
```

---

## Analysis

The crash happens **before** the binary's `main()` function is reached — during Eyra/Origin runtime initialization when parsing argc/argv.

### Root Cause Hypothesis

The kernel's argument passing to static-pie binaries likely has an issue with:
1. **auxv (auxiliary vector) setup** — may be malformed with multiple args
2. **Stack alignment** — may become misaligned with additional arguments
3. **argv pointer array** — may not be properly null-terminated or aligned
4. **Memory mapping** — argument memory may not be properly mapped

### Why eyra test passed before

The `cargo xtask test eyra` only tests with 1 argument (the program name), so this bug was not caught.

---

## Impact on Eyra Migration

This bug **blocks** the Eyra migration:
- `cat file.txt` crashes
- `ls /dir` crashes
- Any utility that takes arguments crashes

Only utilities with no args work (e.g., `pwd`, `cat` with stdin).

---

## Suggested Investigation

1. Check `crates/kernel/src/loader/elf.rs` — stack setup for static-pie
2. Check `crates/kernel/src/task/` — argument passing to new processes
3. Compare with how no_std binaries receive arguments (they work fine)
4. Add debug logging to show auxv/argv layout before jumping to userspace

---

## Workaround Options

1. **Fix the kernel** (preferred) — proper auxv/argv setup for static-pie
2. **Defer Eyra migration** — keep using no_std levbox until fixed
3. **Limit Eyra to no-arg utilities** — not practical for cat, ls, etc.

---

## Files to Investigate

- `crates/kernel/src/loader/elf.rs`
- `crates/kernel/src/task/mod.rs`
- `crates/kernel/src/syscall/process.rs`

---

**Status:** ✅ FIXED by TEAM_363

## Fix Applied

**File:** `crates/kernel/src/memory/user.rs`  
**Lines:** 323-344

Pre-calculate total array size and add padding before writing auxv/argv/argc to ensure 16-byte stack alignment per x86-64 ABI.
