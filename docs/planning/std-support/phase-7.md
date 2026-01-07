# Phase 7: Cleanup and Validation

**Parent**: `docs/planning/std-support/`  
**Team**: TEAM_222  
**Status**: Not Started  
**Priority**: Final

## Purpose
Final validation, cleanup, and documentation of std support.

## Success Criteria
- All tests pass
- Golden tests updated
- No dead code from implementation
- Documentation complete
- Simple std program compiles and runs

---

## Step 1: Adopt linux-raw-sys in libsyscall

### UoW 7.1.1: Replace Hand-Rolled Constants
**File**: `phase-7-step-1-uow-1.md`

**Objective**: Use linux-raw-sys for ABI definitions.

**Tasks**:
1. Add dependency to `userspace/libsyscall/Cargo.toml`:
   ```toml
   [dependencies]
   linux-raw-sys = { version = "0.9", default-features = false, features = ["errno", "general"] }
   ```
2. Replace hand-rolled syscall numbers with `linux_raw_sys::general::*`
3. Replace hand-rolled errno with `linux_raw_sys::errno::*`
4. Keep custom syscalls (SYS_SPAWN, etc.)

**Exit Criteria**: libsyscall uses linux-raw-sys, compiles.

---

### UoW 7.1.2: Replace Hand-Rolled Structs
**File**: `phase-7-step-1-uow-2.md`

**Objective**: Use linux-raw-sys for struct definitions.

**Tasks**:
1. Replace `Stat` struct with `linux_raw_sys::general::stat`
2. Replace `Timespec` with `linux_raw_sys::general::timespec`
3. Replace `Dirent64` with `linux_raw_sys::general::linux_dirent64`
4. Replace `IoVec` with `linux_raw_sys::general::iovec`
5. Update all code using these structs

**Exit Criteria**: Structs from linux-raw-sys, all tests pass.

---

## Step 2: Dead Code Removal

### UoW 7.2.1: Remove Unused Code
**File**: `phase-7-step-2-uow-1.md`

**Objective**: Clean up any dead code.

**Tasks**:
1. Run `cargo clippy` with dead_code warnings
2. Remove any unused functions, constants, modules
3. Remove any commented-out code
4. Remove any temporary compatibility shims

**Exit Criteria**: No dead code warnings.

---

## Step 3: Test Suite Validation

### UoW 7.3.1: Run Full Test Suite
**File**: `phase-7-step-3-uow-1.md`

**Objective**: Verify all tests pass.

**Tasks**:
1. Run `cargo xtask test`
2. Run golden boot test
3. Run all new std-support tests:
   - Auxv test
   - mmap test
   - Threading test
   - Vectored I/O test
   - Pipe test
   - Dup/redirect test
4. Fix any failures

**Exit Criteria**: All tests pass.

---

### UoW 7.3.2: Update Golden Files
**File**: `phase-7-step-3-uow-2.md`

**Objective**: Update baseline if boot output changed.

**Tasks**:
1. If boot output changed (new messages, etc.):
   - Review changes are expected
   - Update `tests/golden_boot.txt`
2. Document any behavioral changes

**Exit Criteria**: Golden files current.

---

## Step 4: std Program Verification

### UoW 7.4.1: Create std Test Program
**File**: `phase-7-step-4-uow-1.md`

**Objective**: Verify std actually works.

**Tasks**:
1. Create simple program using std:
   ```rust
   // userspace/std-test/src/main.rs
   use std::thread;
   use std::io::Write;
   
   fn main() {
       println!("Hello from std!");
       
       let handle = thread::spawn(|| {
           println!("Hello from thread!");
       });
       
       handle.join().unwrap();
       println!("Done!");
   }
   ```
2. This requires building with actual std (not no_std)
3. May need custom target JSON or build configuration

**Exit Criteria**: std program compiles.

---

### UoW 7.4.2: Run std Test Program
**File**: `phase-7-step-4-uow-2.md`

**Objective**: Run std program on LevitateOS.

**Tasks**:
1. Build std test program
2. Add to initrd
3. Boot and run program
4. Verify output:
   ```
   Hello from std!
   Hello from thread!
   Done!
   ```

**Exit Criteria**: std program runs correctly.

---

## Step 5: Documentation

### UoW 7.5.1: Update Architecture Docs
**File**: `phase-7-step-5-uow-1.md`

**Objective**: Document std support.

**Tasks**:
1. Update `docs/ARCHITECTURE.md` with:
   - Std support status
   - What works, what doesn't
2. Update `README.md` if needed
3. Mark requirements.md items as complete

**Exit Criteria**: Docs updated.

---

### UoW 7.5.2: Create std-support Summary
**File**: `phase-7-step-5-uow-2.md`

**Objective**: Final summary document.

**Tasks**:
1. Create `docs/planning/std-support/COMPLETE.md`:
   - What was implemented
   - What's still missing (P3+ items)
   - Known limitations
   - How to build std programs
2. Close out TEAM_222 log

**Exit Criteria**: Summary complete, team logged off.

---

## Deliverables
- libsyscall using linux-raw-sys
- Clean codebase (no dead code)
- All tests passing
- Working std test program
- Updated documentation
- Completion summary
