# Requirements for Rust `std` Support on LevitateOS

## **Overview**
To support the Rust standard library (`std`), LevitateOS must provide a baseline of Linux-compatible syscalls and environment initialization. This document details the gaps, requirements, and external crates that can help.

**Key Reference**: The **sunfishcode ecosystem** (`origin` + `rustix` + `eyra`) has achieved full `std` support on Linux/aarch64 without libc—study these for implementation guidance.

---

## **1. Runtime Environment (Stack Layout)** — P0

### **Status**
- [x] `argc`
- [x] `argv[]` (NULL terminated)
- [x] `envp[]` (NULL terminated)
- [ ] **Auxiliary Vector (`auxv`)** — **MISSING**

### **Requirement**
The `auxv` follows `envp` and provides critical system info. Without it, `std` initialization fails immediately.

| Entry | Value | Purpose |
|-------|-------|---------|
| `AT_PAGESZ` | 4096 | Page size |
| `AT_PHDR` | ELF phdr addr | Program headers location |
| `AT_PHENT` | 56 | Size of each phdr entry |
| `AT_PHNUM` | count | Number of program headers |
| `AT_RANDOM` | 16-byte ptr | Stack canaries, hash seeds |
| `AT_HWCAP` | flags | Hardware capabilities |
| `AT_NULL` | 0 | Terminator |

### **Action**
Update `setup_stack_args` in `kernel/src/memory/user.rs` to push `auxv` entries.

### **Reference Crate**
- **`origin`** ([crates.io](https://crates.io/crates/origin)) — `src/program.rs` shows auxv parsing
- **`linux-raw-sys`** — All `AT_*` constants

---

## **2. Memory Management** — P0

### **Status**
- [x] `brk` (`sys_sbrk`): Basic heap growth
- [ ] **`mmap` / `munmap`** — **MISSING**
- [ ] **`mprotect`** — **MISSING**

### **Requirement**
- **`mmap`**: Rust's allocator uses this for large allocations, thread stacks, and avoiding `brk` fragmentation
- **`mprotect`**: Stack guard pages, security features

### **Action**
Implement `sys_mmap`, `sys_munmap`, `sys_mprotect` syscalls.

### **Reference Crate**
- **`rustix::mm`** ([GitHub](https://github.com/bytecodealliance/rustix)) — `src/backend/linux_raw/mm/` for pure-Rust mmap implementation

---

## **3. Threading & Concurrency** — P1

### **Status**
- [x] `futex`: Basic WAIT/WAKE (TEAM_208)
- [ ] **`clone`** — **MISSING** (custom `spawn` exists)
- [ ] **`set_tid_address`** — **MISSING**
- [ ] **TLS (`TPIDR_EL0`)** — **MISSING**

### **Requirement**
- **`clone`**: Must support `CLONE_VM | CLONE_FS | CLONE_FILES | CLONE_SIGHAND | CLONE_THREAD`
- **TLS**: AArch64 uses `TPIDR_EL0` register. Kernel must:
  1. Context-switch `TPIDR_EL0` per-thread
  2. Support `set_tid_address` for clear-on-exit TID

### **Action**
1. Implement `sys_clone` with thread flags
2. Add `TPIDR_EL0` to thread context save/restore
3. Implement `sys_set_tid_address`

### **Reference Crate**
- **`origin`** ([crates.io](https://crates.io/crates/origin)) — `src/threads.rs` for TLS setup and thread creation
- **`rustix::thread`** — Clone flags handling

---

## **4. I/O & Filesystem** — P1/P2

### **Status**
- [x] `openat`, `close`, `read`, `write`, `fstat`, `getdents64`
- [ ] **`writev` / `readv`** — **MISSING** (P1: crucial for `println!`)
- [ ] **`pipe2`** — **MISSING** (P2: `std::process::Command`)
- [ ] **`dup` / `dup2` / `dup3`** — **MISSING** (P2: redirection)
- [ ] **`ioctl`** — **MISSING** (P2: TTY size)

### **Action**
1. **P1**: Implement `sys_writev`, `sys_readv`
2. **P2**: Implement `sys_pipe2`, `sys_dup`, `sys_dup3`, `sys_ioctl`

### **Reference Crate**
- **`rustix::io`** — Vectored I/O
- **`relibc`** ([GitHub](https://github.com/redox-os/relibc)) — `src/header/unistd/` for pipe/dup

---

## **5. Signals** — P2

### **Status**
- [x] `kill`, `sigaction`, `sigprocmask`, `sigreturn`, `pause`
- [ ] **Signal Trampoline** — Needs verification

### **Action**
Verify signal trampoline compatibility with `std` expectations.

### **Reference Crate**
- **`relibc`** — `src/header/signal/` for signal handling

---

## **6. What to Replace in `libsyscall`**

Currently hand-rolling ~200 lines of constants/structs that should come from maintained sources:

| Hand-rolled | Replace with | Benefit |
|-------------|--------------|---------|
| Syscall numbers (`SYS_READ`, etc.) | `linux-raw-sys` | Auto-generated, correct |
| Errno constants (`EPERM`, etc.) | `linux-raw-sys` | Complete set |
| Struct layouts (`Stat`, `Timespec`, `Dirent64`) | `linux-raw-sys` | ABI-correct |
| Signal constants (`SIGINT`, etc.) | `linux-raw-sys` | Complete set |
| Raw `svc #0` asm | `sc` or `syscalls` | Tested, multi-arch |

### **Recommended Dependency**
```toml
# userspace/libsyscall/Cargo.toml
[dependencies]
linux-raw-sys = { version = "0.9", default-features = false, features = ["errno", "general"] }
```

**Keep hand-rolled**: Wrapper functions, custom syscalls (`SYS_SPAWN`, etc.), print macros.

---

## **7. Prioritization Summary**

| Priority | Feature | Blocker For | Reference |
|----------|---------|-------------|-----------|
| **P0** | Auxv | `std` init | `origin` |
| **P0** | mmap/munmap | Allocator | `rustix::mm` |
| **P1** | clone + TLS | `std::thread` | `origin` |
| **P1** | writev | `println!` | `rustix::io` |
| **P2** | pipe2/dup | `Command` | `relibc` |
| **P2** | ioctl | TTY | `relibc` |

---

## **8. Key External Crates**

| Crate | Use For | Link |
|-------|---------|------|
| **`origin`** | Auxv, TLS, threads | https://github.com/sunfishcode/origin |
| **`rustix`** | Syscall wrappers | https://github.com/bytecodealliance/rustix |
| **`linux-raw-sys`** | Constants, structs | https://crates.io/crates/linux-raw-sys |
| **`relibc`** | Full libc reference | https://github.com/redox-os/relibc |
| **`eyra`** | Working std-without-libc | https://github.com/sunfishcode/eyra |

---

## **Conclusion**

**Immediate focus**: Auxv + mmap — these gate even the simplest `std` binary.

**Adopt `linux-raw-sys`** in `libsyscall` to stop hand-rolling ABI definitions.

**Study `origin`** for the kernel-side auxv and TLS implementation — it's exactly what we need.
