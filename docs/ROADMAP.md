# LevitateOS Roadmap

**Last Updated:** 2026-01-06 (TEAM_164)

This document outlines the planned development phases for LevitateOS. Each completed item includes the responsible team for traceability.

---

## ✅ Phase 1: Foundation & Refactoring (Completed)

- **Objective**: Establish a modular, idiomatic Rust codebase.
- **Achievements**:
  - [x] Migrated to Cargo Workspace (`levitate-kernel`, `levitate-hal`, `levitate-utils`). (TEAM_009)
  - [x] Integrated `linked_list_allocator` for heap management.
  - [x] Basic UART (Console) and GIC (Interrupt) drivers.
  - [x] Basic VirtIO GPU and Input support.

---

## ✅ Phase 2: Idiomatic HAL & Basic Drivers (Completed)

- **Objective**: Harden the Hardware Abstraction Layer (HAL) and implement robust drivers.
- **Tasks**:
  - [x] **Timer**: AArch64 Generic Timer driver. (TEAM_010, TEAM_011)
  - [x] **PL011 UART**: Full PL011 driver with interrupt handling (RX/TX buffers). (TEAM_012, TEAM_014)
  - [x] **GICv2/v3**: Expanded GIC support with typed IRQ routing and FDT discovery. (TEAM_015, TEAM_048)
  - [x] **Safety**: All MMIO uses `volatile`, wrapper structs prevent unsafe state. (TEAM_016, TEAM_017, TEAM_048)

---

## ✅ Phase 3: Memory Management (MMU) (Completed)

- **Objective**: Enable virtual memory and implement higher-half kernel architecture.
- **Tasks**:
  - [x] **Page Tables**: AArch64 page table walking, modification, and optimized 2MB block mappings. (TEAM_018, TEAM_019, TEAM_020)
  - [x] **Identity Mapping**: Initial boot mapping for transition.
  - [x] **Higher-Half Kernel**: Kernel runs at `0xFFFF800000000000` using TTBR1. (TEAM_025, TEAM_026, TEAM_027)
  - [x] **HAL Integration**: `mmu.rs` with `virt_to_phys`/`phys_to_virt` helpers. (TEAM_028)
  - [x] **Kernel Audit**: Documented all behaviors for Phase 2-3 freeze. (TEAM_021, TEAM_022)

---

## ✅ Phase 4: Storage & Filesystem (Completed)

- **Objective**: Persistent storage and basic filesystem access.
- **Tasks**:
  - [x] **VirtIO Block**: Disk driver for QEMU `virtio-blk`. (TEAM_029, TEAM_030)
  - [x] **Filesystem**: FAT32 filesystem using `embedded-sdmmc`. (TEAM_032)
  - [x] **Initramfs**: Load an initial ramdisk for early userspace. (TEAM_035, TEAM_036, TEAM_038, TEAM_039)

---

## ✅ Phase 5: Memory Management II — Dynamic Allocator (Completed)

- **Objective**: Replace the static heap with scalable kernel allocators.
- **Achievements**:
  - [x] **Buddy Allocator**: Physical page allocator for large allocations. (TEAM_048: Dynamic Map)
  - [x] **Slab Allocator**: Fast allocation for fixed-size kernel objects (tasks, file handles). (TEAM_051: Complete)
  - [x] **Page Frame Allocator**: Integration with MMU for on-demand mapping. (TEAM_054: Complete)

---

## ✅ Phase 6: VirtIO Ecosystem Expansion & Hybrid Boot (Completed)

- **Objective**: Expand hardware support and formalize boot architecture.
- **Achievements**:
  - [x] **VirtIO Net**: Basic network packet transmission/reception (`virtio-net`). (TEAM_057)
  - [x] **GPU Refinement**: Text rendering on GPU framebuffer with ANSI support. (TEAM_058, TEAM_059, TEAM_060)
  - [x] **Hybrid Boot Specification**: Formalized boot stages (SEC/PEI/DXE/BDS) and interactive console. (TEAM_061, TEAM_063, TEAM_065)
  - [x] **Keyboard Support**: Direct input from QEMU window via `virtio-keyboard`. (TEAM_032, TEAM_060)
  - [x] **Warning Fixes**: Zero-warning build on bare-metal target. (TEAM_066)
  - [ ] **9P Filesystem**: Mount host directories via `virtio-9p`. (Deferred — see `docs/planning/virtio-ecosystem-phase6/task-6.3-9p-filesystem.md`)

---

## ✅ Phase 7: Multitasking & Scheduler (Completed)

- **Objective**: Run multiple tasks concurrently.
- **Achievements**:
  - [x] **Virtual Memory Reclamation**: `unmap_page()` with TLB invalidation and table reclamation. (TEAM_070)
  - [x] **Context Switching**: Assembly `cpu_switch_to` saves/restores callee-saved registers. (TEAM_070)
  - [x] **Scheduler**: Cooperative `yield_now()` and preemptive Round-Robin via timer interrupts. (TEAM_070)
  - [x] **Task Primitives**: `TaskControlBlock`, `Context`, `TaskId`, `TaskState` with atomic state management. (TEAM_070, TEAM_071)
  - [x] **Idle Task**: Power-efficient `idle_loop()` with `wfi` instruction (Rule 16). (TEAM_071)
  - [x] **Task Exit**: Proper `task_exit()` with state transition and cleanup. (TEAM_071)

> [!NOTE]
> **Demo Mode:** Build with `--features multitask-demo` to enable preemption verification tasks.
> **Plan Docs:** See `docs/planning/multitasking-phase7/` for design decisions and UoW breakdown.

---

## ✅ Phase 8a: Userspace Foundation (Completed)

- **Objective**: Run unprivileged user programs.
- **Achievements**:
  - [x] **EL0 Transition**: Switch CPU from EL1 (Kernel) to EL0 (User). (TEAM_073)
  - [x] **Syscall Interface**: `svc` handler with custom ABI (x8=nr, x0-x5=args). (TEAM_073)
  - [x] **ELF Loader**: Parse and load ELF64 binaries from initramfs. (TEAM_073, TEAM_079)
  - [x] **Device MMIO via TTBR1**: Devices accessible after TTBR0 switch. (TEAM_078)
  - [x] **Basic Syscalls**: `write`, `exit`, `getpid`. (TEAM_073)

> [!NOTE]
> **Milestone:** "Hello from userspace!" executes successfully.

---

## ✅ Phase 8b: Interactive Shell (COMPLETED)

- **Objective**: Boot to an interactive shell prompt with basic coreutils.
- **Tasks**:
  - [x] **GPU Terminal Fix**: Fixed userspace output not appearing on GPU. (TEAM_115)
  - [x] **Read Syscall**: Implemented `read(fd, buf, len)` for stdin/keyboard input. (TEAM_081)
  - [x] **Shell Binary**: Userspace `lsh` with prompt, line editing, command parsing. (TEAM_073)
  - [x] **Coreutils**: `echo`, `help`, `clear`, `exit`. (TEAM_073)
  - [ ] **Spawn Syscall**: Execute external programs from initramfs. (Future)

> [!NOTE]
> **Milestone:** Boot → see log on GPU → get `# ` prompt → run commands. ✅ ACHIEVED
> **Verification:** `cargo xtask run-vnc` → Browser → VNC → Shell interactive

---

## ✅ Phase 8c: Userspace Refactor (Completed)

- **Objective**: Eliminate code duplication and establish a modular userspace architecture.
- **Achievements**:
  - [x] **Workspace**: Converted `userspace/` to a Cargo workspace. (TEAM_118)
  - [x] **libsyscall**: Created shared library for syscall wrappers and panic handling. (TEAM_118)
  - [x] **Migration**: Refactored `shell` to use `libsyscall` and cleaned up legacy `hello`. (TEAM_118)
  - [x] **Linker Scripts**: Fixed conflict using per-crate build scripts. (TEAM_118)

---

## ✅ Phase 8d: Process Management (Completed)

- **Objective**: Implement multi-process management and process lifecycle.
- **Achievements**:
  - [x] **Init Process (PID 1)**: Established proper userspace boot sequence. (TEAM_120)
  - [x] **Spawn Syscall**: Kernel support for launching programs from initramfs. (TEAM_120)
  - [x] **Linter Sync**: Synchronize userspace lints with kernel's strict rules. (TEAM_120)
  - [x] **Build Integration**: Standardized userspace build in `xtask`. (TEAM_120)

> [!NOTE]
> **Milestone:** Boot → `init` starts → `init` spawns `shell` → shell is interactive.

---

## 📱 Phase 9: Hardware Targets

- **Current**: QEMU (`virt` machine, AArch64).
- **Next Step**: **Raspberry Pi 4/5** (Standard AArch64, widely documented, accessible UART).
- **Moonshot**: **Pixel 6 (Tensor GS101)**.
  - *Challenges*: Proprietary boot chain (pBL/sBL/ABL).
  - *Strategy*: Align LevitateOS stages (EarlyHAL, Memory, Console) with GS101 hardware (UART via SBU pins, SimpleFB) to ensure "Pixel-ready" architecture. (TEAM_061, TEAM_063)

---

## 🏗️ PART II: USERSPACE EXPANSION & APPS

The goal of Part II is to build a rich, POSIX-like userspace environment on top of the Phase 8 foundations.

### 📋 Phase 10: The Userspace Standard Library (`ulib`) — PLANNED

> **Planning:** See `docs/planning/ulib-phase10/`  
> **Questions:** See `.questions/TEAM_164_ulib_design.md` (7 questions awaiting answers)

- **Objective**: Create a robust `std`-like library to support complex applications.
- **Specification**: See [`docs/specs/userspace-abi.md`](file:///home/vince/Projects/LevitateOS/docs/specs/userspace-abi.md)
- **Units of Work**:
  - [ ] **Global Allocator**: Implement a userspace heap allocator (`dlmalloc` or `linked_list_allocator`) backed by `sbrk`.
  - [ ] **File Abstractions**: `File`, `OpenOptions`, `Metadata` structs wrapping raw syscalls.
  - [ ] **Directory Iteration**: `ReadDir` iterator (requires `sys_getdents`).
  - [ ] **Buffered I/O**: `BufReader` and `BufWriter` for performance.
  - [ ] **Environment**: `std::env::args()` and `std::env::vars()` parsing from stack.
  - [ ] **Time**: `sys_time` and `sys_sleep` for `std::time::Instant` and `Duration`.
  - [ ] **Error Handling**: Standard `io::Error` and `Result` types.

### 🛠️ Phase 11: Core Utilities (The "Busybox" Phase)

- **Objective**: Implement essential file management and text tools.
- **Units of Work**:
  - [ ] **`ls`**: List directory contents (flags: `-l`, `-a`, color output).
  - [ ] **`cat`**: Concatenate and print files to stdout.
  - [ ] **`touch`**: Create files or update timestamps.
  - [ ] **`mkdir`** / **`rmdir`**: Directory creation/removal.
  - [ ] **`rm`**: File removal (flag: `-r` for recursive).
  - [ ] **`cp`** / **`mv`**: Copy and move files.
  - [ ] **`pwd`**: Print working directory.
  - [ ] **`ln`**: Hard and soft links (requires FS support).

### 🚦 Phase 12: Process & System Management

- **Objective**: Tools to monitor and control the operating system.
- **Units of Work**:
  - [ ] **Process Info**: `sys_info` or `/proc` virtual filesystem to expose kernel stats.
  - [ ] **`ps`**: List running processes (PID, State, Name, Memory).
  - [ ] **`kill`**: `sys_kill` syscall for signaling/terminating processes.
  - [ ] **`top`**: Dynamic real-time view of running tasks.
  - [ ] **`free`**: Memory usage statistics.
  - [ ] **`shutdown` / `reboot`**: ACPI/PSCI integration for system power control.
  - [ ] **`uptime`**: System uptime display.

### 📝 Phase 13: Text Editing & Interaction

- **Objective**: Productive text manipulation within the OS.
- **Units of Work**:
  - [ ] **Terminal Raw Mode**: `sys_ioctl` (or similar) to control TTY driver behavior.
  - [ ] **`grep`**: Basic pattern matching.
  - [ ] **`more`** / **`less`**: Paging through long text.
  - [ ] **`vi` (micro)**: A tiny screen-oriented text editor.
    - Buffer management
    - Cursor movement
    - Insert/Normal modes
    - File saving

### 📦 Phase 14: Package Management & Self-Hosting (Long Term)

- **Objective**: The path to self-compilation.
- **Units of Work**:
  - [ ] **Dynamic Linking**: Support for `.so`/`.dll` loading (long term).
  - [ ] **Interpreter**: Port `lua` or write a basic BASIC interpreter userspace.
  - [ ] **Assembler**: A simple AArch64 assembler.

---

## 🔐 PART III: MULTI-USER SECURITY & AUTHENTICATION (Future)

Once the userspace foundation is solid, we move to secure multi-user support.

### 🛡️ Phase 15: Identity & Authentication

- **Objective**: Identify users and protect resources.
- **Units of Work**:
  - [ ] **User Database**: Implement `/etc/passwd` and `/etc/group` logic.
  - [ ] **Secret Management**: Implement `/etc/shadow` with Argon2 hashing.
  - [ ] **`login`**: The gatekeeper program (replacing direct shell spawn).
  - [ ] **`su`**: Switch User functionality.

### 🔑 Phase 16: Privilege Escalation & Access Control

- **Objective**: Controlled administration access.
- **Units of Work**:
  - [ ] **`doas`**: A minimal, config-based privilege escalation tool (simpler than `sudo`).
  - [ ] **Permission Enforcement**: Kernel-level check of UID/GID against file modes (`rwx`).
  - [ ] **Capabilities**: Fine-grained permissions (e.g., `CAP_NET_ADMIN`) to avoid full root requirements.
  - [ ] **Session Management**: Session IDs and Process Groups (for shell job control).

---

## Team Registry Summary

| Phase | Teams | Description |
|-------|-------|-------------|
| 1 | 001-009 | Foundation, Workspace Refactor |
| 2 | 010-017 | Timer, UART, GIC, HAL Hardening |
| 3 | 018-028 | MMU, Higher-Half Kernel, Audit |
| 4 | 029-039 | VirtIO Block, FAT32, Initramfs |
| 5 | 041-055 | Buddy/Slab Allocators, GIC Hardening, FDT Discovery |
| 6 | 056-066 | VirtIO Ecosystem (Net, GPU, Input), Hybrid Boot Spec |
| 7 | 067-071 | Multitasking, Scheduler, Context Switching |
| 8a | 072-079 | Userspace Foundation (EL0, Syscalls, ELF) |
| 8b | 080+ | Interactive Shell & Coreutils |
| 8c | 118+ | Userspace Architecture Refactor |
| 8d | 120+ | Process Management (Init, Spawn) |
| Maintenance | 121-163 | Bug fixes, refactors, architecture improvements |
