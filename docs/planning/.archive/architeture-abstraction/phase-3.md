# Phase 3: Migration - Arch Abstraction

## Migration Strategy
- Switch `main.rs`, `syscall.rs`, and `task/mod.rs` to import from `crate::arch`.
- Use `Arch` trait or conditional module exports to hide AArch64 details.

## Call Site Inventory
- `kernel/src/main.rs`: `boot::init_heap`, `boot::init_mmu`.
- `kernel/src/syscall.rs`: `SyscallFrame` definitions.
- `kernel/src/task/mod.rs`: `Context`, `cpu_switch_to`.

## Steps
1. **Step 1: Migrate Task Context Switching**
2. **Step 2: Migrate Syscall Framing**
3. **Step 3: Migrate Boot/MMU Initialization**
