# Phase 5: Polish and Cleanup

**Parent**: `docs/planning/feature-clone-thread/`  
**Team**: TEAM_228  
**Status**: Not Started

## Cleanup Tasks

- [ ] Remove unused clone flag constants warnings (add `#[allow(dead_code)]`)
- [ ] Add documentation to `create_thread()` function
- [ ] Add documentation to `sys_clone` with Linux man page reference
- [ ] Update `docs/planning/std-support/PLAN.md` to mark Phase 4 complete

## Documentation Updates

### Code Comments
- Ensure all new functions have rustdoc comments
- Reference Linux clone(2) man page

### README Updates
- Update kernel README with threading support status
- Update std-support documentation

## Final Verification

Before marking complete:
- [ ] `cargo xtask build all` succeeds
- [ ] No new warnings introduced
- [ ] clone_test passes in QEMU
- [ ] Team file updated with completion status

## Handoff Notes

### Known Limitations
1. Fork-style clone not supported (returns ENOSYS)
2. CLONE_FILES, CLONE_FS, CLONE_SIGHAND are nops
3. Thread group management not implemented
4. Address space not reference-counted (orphan thread issue)

### Future Work
- Full fork() implementation
- Proper thread group tracking
- Reference-counted address spaces
- Complete CLONE_FILES for fd sharing
