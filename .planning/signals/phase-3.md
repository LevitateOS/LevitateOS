# Phase 3 – Implementation: Signal Handling

## Overview
This phase involves implementing the kernel-side signal state, the syscalls for signal management, and the architectural hooks for signal delivery on AArch64.

## Steps

### Step 1: Kernel Signal State
- [ ] **UoW 1:** Add `pending_signals`, `blocked_signals`, and `signal_handlers` to `TaskControlBlock` in `kernel/src/task/mod.rs`.
- [ ] **UoW 2:** Update `TaskControlBlock::from(UserTask)` to initialize signal state.

### Step 2: Signal Syscalls (Registration)
- [ ] **UoW 1:** Add `Syscall::Kill`, `Syscall::Pause`, `Syscall::SigAction`, `Syscall::SigReturn` to `kernel/src/syscall/mod.rs`.
- [ ] **UoW 2:** Add syscall handler stubs in `kernel/src/syscall/process.rs` (or a new `kernel/src/syscall/signal.rs`).

### Step 3: Architecture-Specific Delivery
- [ ] **UoW 1:** Implement signal check in `handle_sync_lower_el` and `handle_irq_lower_el`.
- [ ] **UoW 2:** Implement signal frame setup on user stack in `kernel/src/arch/aarch64/mod.rs`.
- [ ] **UoW 3:** Update assembly entry/exit if needed to detect signal return path.

### Step 4: Userspace Support
- [ ] **UoW 1:** Implement `libsyscall` wrappers for new syscalls.
- [ ] **UoW 2:** Implement `ulib` high-level `signal()` and default trampoline.

## Reference Design
See [Phase 2 Design](file:////home/vince/.gemini/antigravity/brain/40e9ecb1-13c7-4038-b598-ce740ac98c21/phase-2.md).
