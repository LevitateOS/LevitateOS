# Phase 5: Hardening and Handoff

**Parent:** [README.md](./README.md)  
**Depends:** Phase 4 complete  
**Status:** Planned

---

## Final Verification

### Build Verification

```bash
cargo build --release                    # Full release build
cargo build -p levitate-kernel --release # Kernel only
cargo clippy                             # Lint check
```

### Test Suite

```bash
cargo xtask test                         # Full test suite
cargo test --workspace                   # Unit tests
```

### Graphical Verification

```bash
./run.sh                                 # Visual check in QEMU
```

---

## Documentation Updates

### Files to Update

| File | Updates Needed |
|------|----------------|
| `README.md` | Update crate list, architecture diagram |
| `docs/ARCHITECTURE.md` | Update with new crate structure |
| `GOTCHAS.md` | Add any new gotchas discovered |
| Crate READMEs | Ensure each crate has accurate README |

### Architecture Diagram

Update to reflect new structure:
```
┌─────────────────────────────────────────────────────────┐
│                  levitate-kernel                        │
└────────────┬────────────┬────────────┬─────────────────┘
             │            │            │
    ┌────────┴───┐  ┌─────┴────┐  ┌────┴─────┐
    │ terminal   │  │    fs    │  │ drivers  │
    └────────────┘  └──────────┘  └────┬─────┘
                                       │
         ┌───────────┬─────────┬───────┴───────┐
         │           │         │               │
    ┌────┴───┐ ┌─────┴──┐ ┌────┴───┐ ┌────────┴───┐
    │  gpu   │ │  blk   │ │  net   │ │   input    │
    └────┬───┘ └────┬───┘ └────┬───┘ └──────┬─────┘
         │          │          │            │
         └──────────┴──────────┴────────────┘
                        │
              ┌─────────┴─────────┐
              │   levitate-virtio │
              └─────────┬─────────┘
                        │
              ┌─────────┴─────────┐
              │   levitate-hal    │
              └─────────┬─────────┘
                        │
              ┌─────────┴─────────┐
              │  levitate-utils   │
              └───────────────────┘
```

---

## Handoff Checklist

### Code Quality

- [ ] All builds pass (debug + release)
- [ ] All tests pass (unit + behavior + regression)
- [ ] No compiler warnings
- [ ] Clippy clean

### Documentation

- [ ] README.md updated
- [ ] ARCHITECTURE.md updated
- [ ] Each crate has README
- [ ] GOTCHAS.md updated

### Behavioral Verification

- [ ] Golden boot test passes
- [ ] QEMU graphical output works
- [ ] Keyboard input works
- [ ] Block device access works

### Cleanup

- [ ] No dead code
- [ ] No temporary files
- [ ] No stale breadcrumbs
- [ ] Team file updated with completion status

---

## Steps

### Step 1: Run Full Verification Suite
**File:** `phase-5-step-1.md`

Tasks:
1. cargo build --release
2. cargo xtask test
3. ./run.sh (visual check)
4. Document any issues

---

### Step 2: Update Documentation
**File:** `phase-5-step-2.md`

Tasks:
1. Update README.md
2. Update ARCHITECTURE.md
3. Create/update crate READMEs
4. Update GOTCHAS.md

---

### Step 3: Final Cleanup Pass
**File:** `phase-5-step-3.md`

Tasks:
1. Run cargo clippy
2. Fix any warnings
3. Remove any remaining TODOs
4. Final code review

---

### Step 4: Complete Handoff
**File:** `phase-5-step-4.md`

Tasks:
1. Update TEAM_101 file with completion status
2. Mark phase complete in README.md
3. Note any remaining work for future teams
4. Archive or close related questions

---

## Success Metrics

| Metric | Target |
|--------|--------|
| Build time | No significant regression |
| Binary size | No significant regression |
| Test coverage | Maintained or improved |
| Crate count | Clear, justified number |
| External deps | Minimal, all through internal crates |

---

## Post-Refactor

Once complete, future work unlocked:
- Easier driver testing (isolated crates)
- Platform portability (HAL is clean)
- New driver development (clear patterns)
- Filesystem expansion (levitate-fs)
- Network stack (built on levitate-drivers-net)
