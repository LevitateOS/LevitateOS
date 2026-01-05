# Phase 4 — Implementation and Tests

**TEAM_131 → TEAM_132** | Reduce Unsafe Code via Safe Abstractions

## Implementation Steps

### Step 1: Add Dependencies (Low complexity)

**Files:** `levitate-hal/Cargo.toml`, `kernel/Cargo.toml`

**Tasks:**
1. Add `aarch64-cpu = "11.2"` to levitate-hal
2. Add `intrusive-collections = "0.10"` to levitate-hal
3. Verify `safe-mmio` is available
4. Run `cargo check --all-targets`

**UoW Size:** ~10 lines, 1 session

---

### Step 2: Migrate Barriers (Low complexity)

**Files:** `levitate-hal/src/gic.rs`, `levitate-virtio/src/queue.rs`, `kernel/src/task/mod.rs`

**Tasks:**
1. Replace `asm!("dsb sy")` with `aarch64_cpu::asm::barrier::dsb(SY)`
2. Replace `asm!("dmb sy")` with `aarch64_cpu::asm::barrier::dmb(SY)`
3. Replace `asm!("isb")` with `aarch64_cpu::asm::barrier::isb(SY)`
4. Replace `asm!("wfi")` with `aarch64_cpu::asm::wfi()`
5. Run `cargo check --all-targets`

**UoW Size:** ~20 lines changed, 1 session

---

### Step 3: Migrate System Registers (Medium complexity)

**Files:** Multiple in levitate-hal and kernel

**Tasks:**
1. Migrate `levitate-hal/src/interrupts.rs` (DAIF)
2. Migrate `levitate-hal/src/mmu.rs` (TTBR, SCTLR, TCR, MAIR, TLB)
3. Migrate `levitate-hal/src/timer.rs` (CNT* registers)
4. Migrate `levitate-hal/src/gic.rs` (ICC_* registers)
5. Migrate `kernel/src/exceptions.rs` (ESR, ELR, VBAR)
6. Run full test suite

**UoW Size:** ~100 lines changed, 2-3 sessions

---

### Step 4: Migrate Intrusive Lists (Medium complexity)

**Files:** `levitate-hal/src/allocator/buddy.rs`, `levitate-hal/src/allocator/slab/list.rs`

**Tasks:**
1. Add `LinkedListLink` to `Page` struct
2. Create adapter with `intrusive_adapter!` macro
3. Replace manual linked list ops with `LinkedList` API
4. Run allocator unit tests

**UoW Size:** ~150 lines changed, 2 sessions

---

## Test Plan

### Required Test Commands

After **each implementation step**, run:

```bash
# 1. Unit tests (must pass)
cargo xtask test unit

# 2. Regression tests (check for new regressions only)
cargo xtask test regress

# 3. Behavior tests (golden file verification)
cargo xtask test behavior
```

### Pass Criteria
- All existing tests must pass after each step
- No NEW regressions introduced (pre-existing failures are documented)
- Behavior tests in `tests/golden_boot.txt` must match

### New Tests

| Module | Test |
|--------|------|
| `barrier` | Compile-only (barriers have no observable effect in tests) |
| `volatile` | Unit tests for read/write semantics |
| `sysreg` | Compile-only for aarch64, stub for host |
| `intrusive_list` | Unit tests for push/pop/remove operations |

### Unsafe Audit

After each step, run:
```bash
grep -rn "unsafe {" kernel/ levitate-hal/ levitate-virtio/ --include="*.rs" | wc -l
```

Track reduction from baseline of 148.

---

## Step Dependencies

```
Step 1 (deps) ──> Step 2 (barriers) ──> Step 3 (sysregs) ──> Step 4 (intrusive lists)
```

Steps are sequential — each depends on previous completing successfully.
