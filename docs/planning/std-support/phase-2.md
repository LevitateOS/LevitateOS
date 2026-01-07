# Phase 2: Auxv Implementation (P0)

**Parent**: `docs/planning/std-support/`  
**Team**: TEAM_222  
**Status**: Not Started  
**Priority**: P0 — Blocks all std binaries

## Purpose
Implement the auxiliary vector (auxv) that `std` expects on the stack at process startup.

## Success Criteria
- Auxv entries pushed to user stack after envp
- AT_PAGESZ, AT_RANDOM, AT_NULL at minimum
- Userspace can read auxv values
- Existing tests still pass

## Reference
- `origin` crate: `src/program.rs` for auxv parsing
- `linux-raw-sys`: AT_* constants

---

## Step 1: Add Auxv Constants

### UoW 2.1.1: Add AT_* Constants to Kernel
**File**: `phase-2-step-1-uow-1.md`

**Objective**: Define auxv type constants in kernel.

**Tasks**:
1. Create `kernel/src/process/auxv.rs` (or add to existing module)
2. Define constants:
   ```rust
   pub const AT_NULL: u64 = 0;
   pub const AT_PAGESZ: u64 = 6;
   pub const AT_PHDR: u64 = 3;
   pub const AT_PHENT: u64 = 4;
   pub const AT_PHNUM: u64 = 5;
   pub const AT_RANDOM: u64 = 25;
   pub const AT_HWCAP: u64 = 16;
   ```
3. Add `mod auxv;` to parent module

**Exit Criteria**: Constants compile, no test regressions.

---

## Step 2: Modify Stack Setup

### UoW 2.2.1: Locate Stack Setup Code
**File**: `phase-2-step-2-uow-1.md`

**Objective**: Find and understand the stack setup code.

**Tasks**:
1. Find `setup_stack_args` or equivalent function
2. Read and document current flow:
   - Where argc is pushed
   - Where argv pointers are pushed
   - Where envp pointers are pushed
   - Where NULL terminators are placed
3. Identify exact insertion point for auxv (after envp NULL)

**Exit Criteria**: Document with code locations and insertion point.

---

### UoW 2.2.2: Implement Auxv Push
**File**: `phase-2-step-2-uow-2.md`

**Objective**: Add auxv entries to stack setup.

**Tasks**:
1. After envp NULL terminator, push auxv entries:
   ```
   // Each entry is two u64s: type, value
   push AT_PAGESZ, 4096
   push AT_RANDOM, <pointer to 16 random bytes>
   push AT_NULL, 0  // terminator
   ```
2. For AT_RANDOM: allocate 16 bytes on stack, fill with pseudo-random or zeros initially
3. Update stack pointer calculations

**Exit Criteria**: Auxv entries on stack, boot still works.

---

### UoW 2.2.3: Add ELF Header Auxv Entries
**File**: `phase-2-step-2-uow-3.md`

**Objective**: Add AT_PHDR, AT_PHENT, AT_PHNUM from ELF loader.

**Tasks**:
1. In ELF loader, capture:
   - Program header address (AT_PHDR)
   - Program header entry size (AT_PHENT = 56 for 64-bit)
   - Program header count (AT_PHNUM)
2. Pass these values to stack setup
3. Push as auxv entries

**Exit Criteria**: ELF auxv entries present on stack.

---

## Step 3: Userspace Verification

### UoW 2.3.1: Add Auxv Reader to libsyscall
**File**: `phase-2-step-3-uow-1.md`

**Objective**: Add userspace function to read auxv.

**Tasks**:
1. Add to `libsyscall`:
   ```rust
   pub fn getauxval(type_: u64) -> Option<u64> {
       // Walk auxv from known location
   }
   ```
2. Document how userspace finds auxv (after envp NULL)

**Exit Criteria**: Function compiles.

---

### UoW 2.3.2: Add Auxv Test Program
**File**: `phase-2-step-3-uow-2.md`

**Objective**: Create test that verifies auxv is present.

**Tasks**:
1. Create test binary that:
   - Finds auxv on stack
   - Reads AT_PAGESZ, verifies = 4096
   - Reads AT_RANDOM, verifies non-null pointer
   - Prints success/failure
2. Add to test suite

**Exit Criteria**: Test passes, auxv verified working.

---

## Deliverables
- `kernel/src/process/auxv.rs` (or equivalent location)
- Modified stack setup code
- `libsyscall` auxv reader
- Auxv verification test
