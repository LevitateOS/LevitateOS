# Research: Requirements for Rust `std` Support on LevitateOS

## **Overview**
To support the Rust standard library (`std`), LevitateOS must provide a baseline of Linux-compatible syscalls and environment initialization. This document details the specific gaps and technical requirements identified during research.

## **1. Runtime Environment (Stack Layout)**
Rust `std` (and most `libc` implementations) expect a specific stack layout at the entry point (`_start`).

### **Current Status**
- [x] `argc`
- [x] `argv[]` (NULL terminated)
- [x] `envp[]` (NULL terminated)
- [ ] **Auxiliary Vector (`auxv`)** - **MISSING**

### **Requirement: Auxiliary Vector**
The `auxv` follows `envp` and provides critical system information:
- `AT_PAGESZ`: Page size (4096).
- `AT_PHDR`, `AT_PHENT`, `AT_PHNUM`: ELF program header info (required for some dynamic linking/introspection).
- `AT_RANDOM`: Pointer to 16 bytes of random data (required for stack canaries/hashing).
- `AT_HWCAP`: Hardware capabilities.

**Action**: Update `setup_stack_args` in `kernel/src/memory/user.rs` to push `auxv` entries.

---

## **2. Memory Management**
### **Current Status**
- [x] `brk` (`sys_sbrk`): Basic heap growth.
- [ ] `mmap` / `munmap`: **MISSING**
- [ ] `mprotect`: **MISSING**

### **Requirement**
- **`mmap`**: Rust's default allocator (and `jemalloc` or `mimalloc`) frequently uses `mmap` for large allocations and to avoid fragmentation issues with `brk`. It is also required for loading shared libraries and thread stacks.
- **`mprotect`**: Required for stack guard pages and potentially for JIT/security features.

---

## **3. Threading & Concurrency**
### **Current Status**
- [x] `futex`: Basic WAIT/WAKE implemented (TEAM_208).
- [ ] `clone`: **MISSING** (We use a custom `spawn`).
- [ ] `set_tid_address`: **MISSING**
- [ ] **TLS (Thread Local Storage)**: **MISSING**

### **Requirement**
- **`clone`**: Must support `CLONE_VM`, `CLONE_FS`, `CLONE_FILES`, `CLONE_SIGHAND`, `CLONE_THREAD` to implement `std::thread`.
- **TLS**: AArch64 uses the `TPIDR_EL0` register to store the thread pointer. The kernel must:
    1.  Allow `set_tid_address` to register the clear-on-exit TID address.
    2.  Properly context-switch `TPIDR_EL0`.
- **`futex`**: Needs verification of full Linux-compatible operations (e.g., `FUTEX_REQUEUE` might be needed eventually).

---

## **4. I/O & Filesystem**
### **Current Status**
- [x] `openat`, `close`, `read`, `write`, `fstat`, `getdents64`.
- [ ] `writev` / `readv`: **MISSING** (Crucial for efficient `println!`).
- [ ] `pipe2`: **MISSING** (Required for `std::process::Command` pipes).
- [ ] `dup` / `dup2` / `dup3`: **MISSING** (Required for redirection).
- [ ] `ioctl`: **MISSING** (Required for TTY/terminal size).

---

## **5. Signals**
### **Current Status**
- [x] `kill`, `sigaction`, `sigprocmask`, `sigreturn`, `pause`.
- [ ] **Signal Trampoline**: Need to verify if `std` expects a specific trampoline or if it provides its own.

---

## **6. Technical ROI & Prioritization**

| Priority | Feature | Reason |
|----------|---------|--------|
| **P0** | **Auxv** | `std` initialization fails without basic system info. |
| **P0** | **mmap/munmap** | Required for standard allocators. |
| **P1** | **clone (Threading)** | Unlocks `std::thread`. |
| **P1** | **writev** | Standard Rust `io::stdout()` uses vectored writes. |
| **P2** | **pipe2/dup** | Required for process orchestration. |

## **Conclusion**
The immediate focus should be on **Auxv support** and **mmap/munmap**, as these are the "gatekeepers" for even the simplest `std` binary to start and allocate memory.
