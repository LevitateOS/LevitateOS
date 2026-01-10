# Phase 4: Cleanup

**TEAM_411** | Syscall Abstractions Refactor
**Parent**: `phase-3.md`
**Status**: Planning

---

## 1. Cleanup Objectives

After migration, remove:
1. Dead code paths that are no longer reachable
2. Unused helper functions superseded by new abstractions
3. Redundant imports
4. Any temporary compatibility shims

---

## 2. Dead Code Candidates

### 2.1 Potentially Unused After Migration

| Item | Location | Reason |
|------|----------|--------|
| Inline validate+copy patterns | Various | Replaced by UserSlice |
| Manual fd_table.get() + clone | Various | Replaced by get_fd() |
| Duplicate VfsError matches | Various | Replaced by From impl |

### 2.2 Review for Removal

These may still have uses elsewhere:
- `write_to_user_buf()` in `syscall/mod.rs` — Check if still needed
- `read_from_user()` in `syscall/mod.rs` — Check if still needed
- Individual errno match arms in old helpers

---

## 3. Encapsulation Tightening

### 3.1 Visibility Review

After migration, review whether these should be `pub(crate)` or private:
- `mm_user::validate_user_buffer` — Still needed externally?
- `mm_user::user_va_to_kernel_ptr` — Still needed externally?
- Individual VfsError variants — Are all used?

### 3.2 API Surface Reduction

If UserSlice becomes the standard pattern:
- Consider making raw `validate_user_buffer` private
- Force usage through UserSlice for safety

---

## 4. File Size Check

Per Rule 7, verify files stay readable:

| File | Current Lines | Target |
|------|---------------|--------|
| syscall/fs/fd.rs | ~760 | <1000 ✓ |
| syscall/fs/read.rs | ~250 | <500 ✓ |
| syscall/mod.rs | ~525 | <1000 ✓ |
| syscall/process.rs | TBD | <1000 |
| syscall/helpers.rs (new) | TBD | <500 |

If any file exceeds limits after adding helpers, consider splitting.

---

## 5. Phase 4 Steps

### Step 1: Remove Unused Inline Patterns
- Search for any remaining inline validate+copy that wasn't migrated
- Remove if truly unused

### Step 2: Clean Up Imports
- Remove unused `use` statements
- Consolidate imports where possible

### Step 3: Review Helper Functions
- Check if `write_to_user_buf`, `read_from_user` still used
- Remove or deprecate if superseded

### Step 4: Tighten Visibility
- Mark internal helpers as `pub(crate)` or private
- Document public API surface

### Step 5: Verify No Dead Code
```bash
# Run with warnings for unused code
RUSTFLAGS="-W dead_code" cargo build
```

---

## 6. Exit Criteria

- [ ] No dead code warnings
- [ ] No unused imports
- [ ] All files <1000 lines (preferably <500)
- [ ] Visibility appropriately restricted
- [ ] Build passes
- [ ] Tests pass

---

## Next Phase

→ `phase-5.md`: Hardening and Handoff
