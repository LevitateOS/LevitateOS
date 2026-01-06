# Linux AArch64 ABI Compatibility Guide

**TEAM_217** | **Documentation for Future Teams**

To support the standard Rust library (`std`) and the broader Linux ecosystem, LevitateOS adheres to the **Linux AArch64 System Call ABI**. This document captures critical knowledge, gotchas, and patterns discovered during the alignment process.

---

## 1. Data Structure Alignment (The 128-byte Stat)

Standard binaries (compiled against `musl` or `glibc`) expect a very specific byte layout for kernel structures. 

### **Gotcha: The Stat Struct**
The `Stat` struct used by `fstat` must be exactly **128 bytes** on AArch64.
- **Padding**: Fields like `st_blksize` (i32) require matching padding (`__pad2`) to maintain 8-byte alignment for subsequent `i64` fields.
- **Types**: Use `i64` for sizes and timestamps, even if the kernel treats them as unsigned internally. Standard Rust `libc` expects signed values for many of these.

**Pattern**: Always verify structure sizes using `core::mem::size_of` in tests when modifying `syscall/mod.rs`.

---

## 2. Runtime Environment (Stack Layout)

Standard process entry points (`_start`) do not just receive `argc` and `argv`.

### **Requirement: Auxiliary Vector (auxv)**
The Rust runtime (`std::rt`) will fail to initialize if the **Auxiliary Vector** is missing from the stack.
- **Location**: It must follow the `envp` NULL terminator.
- **Mandatory Entries**:
    - `AT_PAGESZ` (6): Required for the allocator.
    - `AT_RANDOM` (25): Required for stack canaries and hash seeding.
    - `AT_PHDR`, `AT_PHENT`, `AT_PHNUM`: Required for dynamic linking and introspection.

**Action**: Use `crate::memory::user::setup_stack_args` to manage this complex layout.

---

## 3. Threading & TLS (TPIDR_EL0)

Rust uses **Thread Local Storage (TLS)** for its global state.
- **Register**: On AArch64, the `TPIDR_EL0` register holds the base address of the current thread's TLS area.
- **Gotcha**: The kernel **must** context-switch this register. If you forget to save/restore it in `cpu_switch_to`, all threads in a process will share the same `errno`, `panic` state, and other TLS variables.

---

## 4. I/O Performance (Vectored I/O)

Standard Rust `println!` does not call `write()` for every piece of data. It uses `writev()` (Vectored Write) to send multiple buffers (e.g., prefix + message + newline) in a single syscall.
- **Implementation**: `sys_writev` should iterate through the `iovec` array and dispatch to the underlying `sys_write` or VFS logic.

---

## 5. Error Code Alignment (Errno)

Standard binaries rely on specific numeric values for error reporting.

### **The ENOSYS Pitfall**
- **Linux AArch64**: `ENOSYS` is **38**.
- **Common Error**: Many kernels default to `-1`. 
- **Impact**: If `ENOSYS` is not `38`, a binary will fail to gracefully degrade when a syscall is missing, often resulting in an immediate crash or incorrect logic path.

**Pattern**: Always use the constants defined in `kernel/src/syscall/mod.rs::errno` which are aligned with `asm-generic/errno.h`.

---

## 6. Implementation Strategy: "The Redox Way"

When stuck on complex memory management (like `mmap`), refer to the **`rmm` (Redox Memory Manager)** logic.
- **LevitateOS Status**: We use the hardware "wires" (page table walking) but integrate the VMM (Virtual Memory Area) management patterns from `rmm` to handle address space allocation.

---

## Verification Techniques

### **QEMU Syscall Tracing**
To verify if a binary is hitting an unimplemented or misaligned syscall:
```bash
# Traces syscalls and exceptions
qemu-system-aarch64 -d unimp,guest_errors,int ...
```

### **Errno Consistency**
Always use the values defined in `asm-generic/errno.h`.
- **Warning**: Never use `-1` for `ENOSYS`. It must be `-38`. If a binary receives `-1`, it may think the syscall succeeded but returned an error value, leading to impossible-to-debug crashes.
