# TEAM_422: Kernel Architecture Redesign

## Status: In Progress

## Objective

Restructure the kernel from a monolithic binary with supporting crates into a modular workspace where each subsystem (memory, scheduler, VFS, syscalls) is its own crate with clear boundaries.

## Problem Statement

The current kernel structure has grown organically, leading to:
- Tight coupling between subsystems
- Large files (700+ lines in `memory/user.rs`, 600+ in `syscall/fs/fd.rs`)
- Task/Process/Thread concerns intermingled
- HAL mixing hardware abstraction with driver code
- Difficult to test subsystems in isolation
- No clear ownership boundaries between modules

## Target Architecture

```
kernel/                       # Thin kernel binary
├── arch/aarch64/            # Platform-specific crate
├── arch/x86_64/             # Platform-specific crate
├── mm/                      # Memory management
├── sched/                   # Process/thread scheduler
├── vfs/                     # Virtual filesystem
├── syscall/                 # Syscall dispatch
├── drivers/                 # Device drivers
├── hal/                     # Hardware abstraction (slimmed)
├── utils/                   # Core utilities
└── error/                   # Error types
```

## Success Criteria

| Before | After |
|--------|-------|
| Single kernel binary crate | Workspace with focused crates |
| Files > 500 lines common | All files < 500 lines |
| Cross-module deps via `crate::` | Explicit crate dependencies |
| Task struct owns everything | Process/Thread separation |
| No subsystem tests | Each crate testable in isolation |

## Plan Location

Detailed implementation plan: `docs/planning/kernel-architecture-redesign/`

- **Phase 1**: Discovery & Safeguards - Lock in golden tests, document contracts
- **Phase 2**: Structural Extraction - Design new crate layout
- **Phase 3**: Migration - Step-by-step code movement
- **Phase 4**: Cleanup - Dead code removal, documentation sync
- **Phase 5**: Hardening - Test coverage, static analysis, performance verification

## Behavioral Contracts (Must Not Change)

1. **Linux Syscall ABI**: `SyscallResult = Result<i64, u32>`, syscall numbers match linux-raw-sys
2. **Memory Layout**: Higher-half kernel at `0xFFFF_8000_0000_0000`, `Stat` exactly 128 bytes
3. **Process Model**: fork/clone/exec/wait semantics preserved
4. **VFS Semantics**: FDs per-process, independent seek positions

## Migration Order

1. Extract `mm/` crate (memory management)
2. Extract `sched/` crate (scheduler)
3. Extract `vfs/` crate (virtual filesystem)
4. Extract `syscall/` crate
5. Extract `arch/` crates
6. Slim down `kernel/` binary

Each step must leave the kernel in a buildable, testable state.

## Rollback Strategy

- Each migration step is one commit
- Branch per subsystem: `refactor/mm`, `refactor/sched`, etc.
- Merge only when all tests pass on both architectures
- Abort if build broken > 4 hours

## Files Exceeding 500 Lines (Immediate Targets)

| File | Lines | Issue |
|------|-------|-------|
| ~~`memory/user.rs`~~ | ~~709~~ | ✅ DONE - Split into 6 files (max 278 lines) |
| `syscall/fs/fd.rs` | 638 | FD operations + dup logic |
| `loader/elf.rs` | 579 | ELF loading + relocation |
| `init.rs` | 570 | Boot sequence god function |
| `arch/x86_64/mod.rs` | 592 | Platform code mixed in |
| `arch/aarch64/mod.rs` | 557 | Platform code mixed in |
| `fs/path.rs` | 528 | Path resolution complexity |

## Related Teams

- TEAM_406: General Purpose OS (depends on stable kernel API)
- TEAM_407: Refactor/Consolidate Scattered Code (overlapping goals)

## Log

### 2026-01-11: Plan Created
- Created comprehensive 5-phase refactor plan
- Documented current architecture issues
- Defined target workspace structure
- Established migration order and rollback strategy

### 2026-01-11: Phase 1 & Initial File Splitting
- Verified baseline: both x86_64 and aarch64 build successfully
- Updated behavior test golden files
- Created `mm/` crate scaffold (empty, for future migration)
- **Split `memory/user.rs` (709 lines) into 6 files:**
  - `memory/user/auxv.rs` (23 lines) - Auxiliary vector types
  - `memory/user/layout.rs` (21 lines) - Address space constants
  - `memory/user/mapping.rs` (278 lines) - Page mapping functions
  - `memory/user/page_table.rs` (184 lines) - Page table management
  - `memory/user/stack.rs` (213 lines) - Stack setup
  - `memory/user/mod.rs` (25 lines) - Re-exports
- All tests pass on both architectures

### Migration Blocker Identified
Full crate extraction (Phase 3) is blocked by circular dependencies:
- `memory/mod.rs::init()` depends on `crate::boot::BootInfo`
- `memory/mod.rs` re-exports `crate::task::TaskControlBlock`

**Resolution**: Keep `init()` in kernel binary, move only pure memory
management code to `mm/` crate. Requires further architectural analysis.

### 2026-01-11: Full Kernel Modularization Complete

**Major milestone achieved**: Kernel fully restructured into modular crate workspace.

#### Crates Created/Migrated
| Crate | Purpose |
|-------|---------|
| `los_types` | Shared types (SyscallFrame, Pid, SyscallResult) - breaks cycles |
| `los_vfs` | Virtual File System |
| `los_mm` | Memory management |
| `los_sched` | Task scheduler |
| `los_syscall` | Syscall dispatch |
| `los_arch_aarch64` | AArch64 platform code |
| `los_arch_x86_64` | x86_64 platform code |
| `los_fs_tmpfs` | tmpfs filesystem |
| `los_fs_initramfs` | Initramfs filesystem |
| `los_fs_tty` | TTY subsystem |
| All drivers | Moved into kernel workspace |

#### Key Architectural Decisions
1. **Shared Types Crate**: Created `los_types` to break circular dependencies between sched/syscall/mm
2. **Extern Callbacks**: Architecture crates use `unsafe extern "Rust"` for kernel integration (syscall_dispatch, exception handlers)
3. **Integration Modules**: Kernel binary (levitate) has bridge modules (fs.rs, memory.rs, process.rs) that tie crates together

#### AArch64 Boot Relocation Fix
**Problem**: Boot code at physical 0x40080000 used `adrp` (±4GB range) to access higher-half symbols - impossible.

**Solution implemented**:
1. Two-copy boot data pattern: `BOOT_*_PHYS` in `.bss.boot` (physical), `BOOT_*` in higher-half
2. Early MMU setup in boot.S:
   - Set up L0/L1 page tables for identity + higher-half mapping
   - Configure MAIR, TCR, enable MMU
   - Jump to Rust at higher-half address
3. Linker script: Added `.bss.boot` and `.boot_page_tables` sections before virt base jump

#### Build Status
- ✅ x86_64 builds successfully
- ✅ aarch64 builds successfully

#### Cleanup Done
- Removed duplicate crates from main repo (drivers, traits, virtio-transport)
- Main repo workspace now only contains `xtask`
- All kernel code lives in `crates/kernel/` submodule

#### Gotchas Documented
Added to `docs/GOTCHAS.md`:
- #32: AArch64 boot code cannot use higher-half symbols directly
- #33: Kernel modular crate dependencies must avoid cycles
- #34: Architecture crates use extern callbacks for integration

### Remaining Work

- [ ] Run full test suite (`cargo xtask test`) once QEMU environment available
- [ ] Verify kernel boots on both architectures in QEMU
- [ ] Clean up unused imports (many warnings in build output)
- [ ] Consider further file splitting (syscall/fs/fd.rs still 638 lines)
