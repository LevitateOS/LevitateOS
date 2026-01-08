# Phase 5: Polish, Docs, and Cleanup

## Cleanup Tasks
- Ensure no accidental debug prints left in `libsyscall`.
- Verify code style in new tests.

## Documentation Updates
- Update `libsyscall/README.md` (if exists) or root `README.md` to mention new test capabilities.
- Document any known kernel limitations discovered during testing (e.g. "nanosleep is only ms precise").

## Final Verification
- Run all tests one last time.
- Check `cargo clippy`.

## Handoff Checklist
- [ ] New wrappers added to `libsyscall`.
- [ ] All 5 test binaries implemented and passing.
- [ ] `levbox` builds cleanly.
- [ ] Team log updated.
