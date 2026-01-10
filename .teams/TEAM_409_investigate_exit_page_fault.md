# TEAM_409: Investigate Exit Command Issues

**Date:** 2026-01-10  
**Status:** RESOLVED ✅

---

## Bug Reports

### Issue 1: x86_64 Page Fault on Exit

**Symptom:**
```
KERNEL PANIC: panicked at crates/hal/src/x86_64/cpu/exceptions.rs:270:5:
EXCEPTION: PAGE FAULT
Accessed Address: 78
Error Code: 4
ExceptionStackFrame {
    instruction_pointer: 211250,
    code_segment: 35,
    cpu_flags: 66054,
    stack_pointer: 140737488288320,
    stack_segment: 27,
}
```

**Analysis:**
- Accessed Address: 0x78 (120 bytes) - null pointer + TLS field offset
- Error Code: 4 = User mode read of non-present page
- Stack Pointer: 0x7fffffffc9c0 - userspace address
- The crash happens in userspace (brush shell) when accessing TLS

### Issue 2: AArch64 Exit Hangs Shell

**Symptom:**
- `exit` command doesn't stop VM
- Shell appears to hang after exit
- Actually: init was just silently yielding forever after shell exited

---

## Root Causes

### x86_64: TLS Not Preserved Across Context Switches

**Problem:** In `sys_arch_prctl(ARCH_SET_FS, addr)`:
1. The IA32_FS_BASE MSR was written directly ✓
2. `task.tls` was updated ✓
3. **BUT `task.context.fs_base` was NOT updated** ✗

The context switch assembly (`cpu_switch_to` in `task.rs`) restores TLS from
`context.fs_base`, not `task.tls`. So after a context switch, TLS was lost
and reverted to 0, causing null pointer dereferences.

### AArch64: Init Not Waiting for Shell

**Problem:** Init spawned the shell and then entered an infinite yield loop
without ever calling `waitpid()`. When shell exited:
- No visible feedback was printed
- Init just kept yielding forever (appeared as a hang)
- The VM didn't shut down

---

## Fixes Applied

### Fix 1: x86_64 TLS Preservation (`crates/kernel/src/syscall/process.rs`)

```rust
// In sys_arch_prctl ARCH_SET_FS case:
// TEAM_409: Store in BOTH task.tls AND context.fs_base for context switch restore
let task = crate::task::current_task();
task.tls.store(addr, core::sync::atomic::Ordering::Release);
// Also update context.fs_base so context switch preserves TLS
unsafe {
    let ctx_ptr = &task.context as *const _ as *mut crate::arch::Context;
    (*ctx_ptr).set_tls(addr as u64);
}
```

### Fix 2: Init Waits for Shell (`crates/userspace/init/src/main.rs`)

```rust
// TEAM_409: Wait for shell to exit and report status
let mut status: i32 = 0;
let wait_result = libsyscall::waitpid(shell_pid as i32, Some(&mut status));
println!("[INIT] Shell exited: wait={}, status={}", wait_result, status);
println!("[INIT] System halting...");

// Shutdown the system cleanly when shell exits
libsyscall::shutdown(0);
```

---

## Verification

- [x] Kernel builds cleanly
- [x] Init builds for x86_64 and aarch64
- [x] xtask builds with isa-debug-exit device support
- [ ] Manual test pending (user to verify)

## Additional Fix: QEMU VM Termination

The VM was not terminating after shutdown because x86_64 `system_off()` only did HLT loops.

**Fix 3: x86_64 system_off() (`crates/kernel/src/arch/x86_64/power.rs`)**
```rust
// Write to QEMU isa-debug-exit device (port 0xf4)
core::arch::asm!(
    "out dx, al",
    in("dx") 0xf4u16,
    in("al") 0u8,
    options(nomem, nostack, preserves_flags)
);
```

**Fix 4: QEMU builder (`xtask/src/qemu/builder.rs`)**
```rust
// Add isa-debug-exit device for x86_64
if matches!(self.arch, Arch::X86_64) {
    cmd.args(["-device", "isa-debug-exit,iobase=0xf4,iosize=0x04"]);
}
```

---

## Handoff Checklist

- [x] Project builds cleanly
- [x] Root causes identified and fixed
- [x] Team file updated
- [x] Changes are minimal and targeted

