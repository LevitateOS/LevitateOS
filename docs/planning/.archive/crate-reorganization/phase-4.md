# Phase 4: Cleanup

**Parent:** [README.md](./README.md)  
**Depends:** Phase 3 complete  
**Status:** Planned

---

## Cleanup Scope

### Dead Code Removal (Rule 6)

| Item | Location | Action |
|------|----------|--------|
| `levitate-gpu/` | Workspace | **DELETE** entire crate |
| `virtio.rs` | levitate-hal/src/ | **DELETE** (moved to levitate-virtio) |
| Old block.rs | kernel/src/ | **DELETE** (if not done in Phase 3) |
| Old net.rs | kernel/src/ | **DELETE** (if not done in Phase 3) |
| Old input.rs | kernel/src/ | **DELETE** (if not done in Phase 3) |
| Unused imports | Various | Remove all |
| Debug breadcrumbs | Various | Remove TEAM_XXX comments that are resolved |

### Temporary Adapter Removal

- Any shim code added for coexistence
- Compatibility re-exports
- Deprecated type aliases

### Encapsulation Tightening

- Make internal fields private
- Remove unnecessary `pub` on modules
- Hide implementation details behind traits

---

## Steps

### Step 1: Delete levitate-gpu Crate
**File:** `phase-4-step-1.md`

Tasks:
1. Remove from workspace Cargo.toml
2. Delete levitate-gpu/ directory
3. Verify build
4. Run tests

---

### Step 2: Clean levitate-hal
**File:** `phase-4-step-2.md`

Tasks:
1. Delete virtio.rs (already moved)
2. Remove virtio-drivers dependency from Cargo.toml
3. Remove virtio module export from lib.rs
4. Verify build

---

### Step 3: Remove External Dependencies from Kernel
**File:** `phase-4-step-3.md`

Verify kernel/Cargo.toml no longer has:
- `virtio-drivers` (goes through driver crates)
- `embedded-sdmmc` (goes through levitate-fs)
- `ext4-view` (goes through levitate-fs)

Tasks:
1. Audit kernel/Cargo.toml
2. Remove any remaining direct external deps
3. Verify build

---

### Step 4: Tighten Visibility
**File:** `phase-4-step-4.md`

Tasks:
1. Audit each crate's public API
2. Make internal modules private
3. Make struct fields private where appropriate
4. Verify build

---

### Step 5: Remove Stale Breadcrumbs
**File:** `phase-4-step-5.md`

Tasks:
1. Search for BREADCRUMB comments
2. Remove those marked CONFIRMED or RULED_OUT
3. Keep any still-active investigations
4. Update team files

---

### Step 6: File Size Check (Rule 7)
**File:** `phase-4-step-6.md`

Verify all files are human-readable:
- < 1000 lines preferred
- < 500 lines ideal

Tasks:
1. Find files > 500 lines
2. Split if appropriate
3. Document any exceptions

---

## Perfection Criteria

- [ ] No dead code remains
- [ ] No temporary adapters remain
- [ ] All crate APIs are minimal and intentional
- [ ] All files < 1000 lines
- [ ] No stale breadcrumbs
- [ ] Build clean with no warnings
- [ ] All tests pass
