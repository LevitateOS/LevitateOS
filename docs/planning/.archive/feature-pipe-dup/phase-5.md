# Phase 5: Polish

**Parent**: `docs/planning/feature-pipe-dup/`  
**Team**: TEAM_233  
**Status**: Not Started

## Cleanup Tasks

- [ ] Remove unused imports and dead code
- [ ] Silence or downgrade debug logging
- [ ] Review error messages for consistency

## Documentation Updates

- [ ] Add doc comments to all public APIs
- [ ] Update std-support/PLAN.md to mark Phase 6 complete
- [ ] Update ROADMAP.md if needed

## Final Verification

- [ ] All existing tests pass
- [ ] pipe_test passes
- [ ] clone_test still passes (regression check)
- [ ] Kernel builds with no warnings

## Handoff Notes

### Future Improvements
- Larger pipe buffer (64KB like Linux)
- O_NONBLOCK support
- SIGPIPE on write to closed pipe
- Pipe polling for select/poll

### Related Work
- Phase 7: Cleanup and Validation (std-support)
- Shell piping (`cmd1 | cmd2`)
