# Phase 5: Cleanup, Regression Protection, and Handoff

**Team:** TEAM_077  
**Parent:** `plan.md`  
**Prerequisite:** `phase-4.md` (implementation complete)  
**Status:** READY (execute after Phase 4)

---

## 1. Cleanup Tasks

### Code Cleanup
- [ ] Remove TEAM_076 breadcrumb from `kernel/src/task/process.rs:75`
- [ ] Remove any temporary debug prints added during investigation
- [ ] Ensure no dead code remains from old identity mapping logic

### Documentation Updates
- [ ] Update `docs/ARCHITECTURE.md` if memory layout changed
- [ ] Update `levitate-hal/README.md` if API changed

---

## 2. Regression Protection

### Behavioral Baseline
After fix is verified, update the golden boot test to include userspace output.

**File:** `tests/golden_boot.txt`

Expected new lines:
```
[SPAWN] Looking for 'hello' in initramfs...
[SPAWN] Found 'hello' (NNN bytes)
[SPAWN] Created user process PID=1 entry=0x10000 sp=0x7fffffff0000
[SPAWN] Starting user process...
Hello from userspace!
```

### Test Commands
```bash
# Full test suite
cargo xtask test

# Manual verification
cargo xtask run
```

---

## 3. Handoff Checklist

| Item | Status |
|------|--------|
| Project builds cleanly | [ ] |
| `cargo xtask test` passes | [ ] |
| Userspace process runs | [ ] |
| Team file updated with completion status | [ ] |
| Breadcrumbs removed | [ ] |
| Golden test updated | [ ] |

---

## 4. Post-Fix Verification

### Immediate
1. Kernel boots and prints to console
2. `cargo xtask test` passes
3. Userspace process runs and prints "Hello from userspace!"

### Extended (if time permits)
4. VirtIO block read works
5. VirtIO GPU renders (if configured)
6. Timer interrupts work (preemption)
7. Keyboard input works

---

## 5. Known Issues / Future Work

### Deferred
- **Step 7 (Remove Identity Mapping):** May be deferred if it causes boot issues. Document if skipped.

### Future Improvements
- Consider device tree-driven device address discovery
- Consider more granular device page mappings (4KB instead of 2MB blocks)

---

## 6. Team File Update Template

After completion, update `.teams/TEAM_077_bugfix_device_mmio_ttbr1.md`:

```markdown
## Status
**COMPLETE** - Device MMIO now mapped via TTBR1

## Summary
- Mapped device regions (UART, GIC, VirtIO) via TTBR1 at high VA
- Updated all device drivers to use high VA addresses
- Userspace execution now works
- Golden test updated

## Files Modified
- levitate-hal/src/mmu.rs
- levitate-hal/src/console.rs
- levitate-hal/src/gic.rs
- levitate-hal/src/virtio/*.rs
- tests/golden_boot.txt

## Verification
- [x] cargo xtask test passes
- [x] Userspace process runs
- [x] Breadcrumb removed
```

---

**Phase 5 Ready.** Execute after Phase 4 implementation is complete.
