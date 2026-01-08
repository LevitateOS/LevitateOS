# Phase 1: Discovery and Safeguards

**Parent**: `docs/planning/std-support/`  
**Team**: TEAM_222  
**Status**: Not Started

## Purpose
Understand current state, establish baselines, and lock in regression protection before any std-related changes.

## Success Criteria
- All existing tests pass and are documented
- Behavioral contracts identified for affected syscalls
- Golden test baselines captured
- Dependency graph of affected code understood

---

## Step 1: Map Current Syscall Implementation

### UoW 1.1: Inventory Existing Syscalls
**File**: `phase-1-step-1-uow-1.md`

**Objective**: Create complete inventory of currently implemented syscalls.

**Tasks**:
1. Search `kernel/src/syscall/` for all `sys_*` functions
2. Document each syscall: number, signature, return type
3. Mark which are Linux-compatible vs custom (SYS_SPAWN, etc.)
4. Output: `docs/planning/std-support/syscall-inventory.md`

**Exit Criteria**: Inventory file exists with all syscalls listed.

---

### UoW 1.2: Map libsyscall Wrappers
**File**: `phase-1-step-1-uow-2.md`

**Objective**: Document userspace syscall wrappers in libsyscall.

**Tasks**:
1. Read `userspace/libsyscall/src/lib.rs`
2. List all public wrapper functions
3. Document which kernel syscalls each wrapper calls
4. Note any ABI differences from Linux

**Exit Criteria**: Wrapper-to-syscall mapping documented.

---

## Step 2: Establish Test Baselines

### UoW 2.1: Run and Document Existing Tests
**File**: `phase-1-step-2-uow-1.md`

**Objective**: Verify all tests pass and capture baseline.

**Tasks**:
1. Run `cargo xtask test` (or equivalent)
2. Capture output to `tests/baseline-pre-std.txt`
3. Document any flaky tests
4. Run golden boot test: `cargo xtask test --golden`

**Exit Criteria**: All tests pass, baseline captured.

---

### UoW 2.2: Identify Coverage Gaps
**File**: `phase-1-step-2-uow-2.md`

**Objective**: Find syscalls without test coverage.

**Tasks**:
1. Cross-reference syscall inventory with test files
2. List syscalls that have no direct tests
3. Prioritize gaps that will be affected by std work

**Exit Criteria**: Gap list created in planning docs.

---

## Step 3: Document Architectural Constraints

### UoW 3.1: Map Memory Layout
**File**: `phase-1-step-3-uow-1.md`

**Objective**: Understand current user process memory layout.

**Tasks**:
1. Read `kernel/src/memory/user.rs` (or equivalent)
2. Document stack setup: argc, argv, envp locations
3. Document heap management (brk/sbrk)
4. Identify where auxv would need to be added

**Exit Criteria**: Memory layout documented with auxv insertion point identified.

---

### UoW 3.2: Map Thread/Process Context
**File**: `phase-1-step-3-uow-2.md`

**Objective**: Understand current context switch and process state.

**Tasks**:
1. Read thread/process structures
2. Document what's saved/restored on context switch
3. Identify where TPIDR_EL0 (TLS) would be added
4. Document current spawn vs what clone needs

**Exit Criteria**: Context structure documented with TLS insertion point identified.

---

## Deliverables
- `syscall-inventory.md`
- `tests/baseline-pre-std.txt`
- `coverage-gaps.md`
- `memory-layout.md`
- `context-structure.md`
