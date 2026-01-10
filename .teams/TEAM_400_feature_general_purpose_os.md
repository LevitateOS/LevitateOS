# TEAM_400: Feature - General Purpose OS (Milestone 1)

**Date**: 2026-01-10
**Status**: Design Complete - Ready for Implementation
**Type**: Feature

---

## Summary

Implement Milestone 1 of the General Purpose Unix-Compatible OS: **Static Binary Compatibility**.

After this feature, users can compile programs with `gcc -static` on any Linux system and run them on LevitateOS without modification.

---

## Goal

**Run any statically-linked Linux binary without modification.**

```bash
# On any Linux system:
gcc -static -o hello hello.c

# On LevitateOS:
./hello  # Just works
```

---

## Scope

### In Scope (Milestone 1)

| Component | Description |
|-----------|-------------|
| fork | Create child process (eager copy) |
| execve | Replace process image with new program |
| chdir/fchdir | Change working directory |
| chmod/chown | Permission syscalls (stubs) |
| poll/select | I/O multiplexing |
| uname | System identification |
| umask | File creation mask |

### Out of Scope (Future Milestones)

| Component | Milestone |
|-----------|-----------|
| libc.so.6 | Milestone 2 |
| ld-linux.so.2 | Milestone 2 |
| /proc filesystem | Milestone 2 |
| Networking | Milestone 3+ |
| Multi-user support | Future |

---

## Plan Documents

| Phase | Document | Status |
|-------|----------|--------|
| Discovery | `docs/planning/general-purpose-os/phase-1.md` | Complete |
| Design | `docs/planning/general-purpose-os/phase-2.md` | Complete |
| Implementation | `docs/planning/general-purpose-os/phase-3.md` | Complete |
| Integration | `docs/planning/general-purpose-os/phase-4.md` | Complete |
| Polish | `docs/planning/general-purpose-os/phase-5.md` | Complete |

---

## Resolved Design Questions

All questions have been answered. See `docs/questions/TEAM_400_general_purpose_os.md` for full rationale.

### Critical Questions (Resolved)

| ID | Question | Decision | Rationale |
|----|----------|----------|-----------|
| Q1 | Eager copy vs CoW for fork? | **Eager Copy** | CoW adds complexity; optimize when profiling shows need |
| Q2 | How should execve find executables? | **VFS resolution** | Standard behavior |
| Q3 | Orphan reparenting to PID 1? | **Yes** | Standard Unix semantics |
| Q4 | Implement both poll() and ppoll()? | **Both** | Maximum compatibility |

### Important Questions (Resolved)

| ID | Question | Decision | Rationale |
|----|----------|----------|-----------|
| Q5 | FD_CLOEXEC handling | **Track per-fd flag** | Prevents FD leaks |
| Q6 | Store chmod modes in VFS? | **No-op (just succeed)** | Single-user OS |
| Q7 | Signal reset on exec | **All except SIG_IGN** | Linux behavior |

### Deferred Questions

| ID | Question | Decision | Rationale |
|----|----------|----------|-----------|
| Q8 | pselect6 implementation | **Deferred** | YAGNI |
| Q9 | argv+envp size limit | **256KB** | Matches Linux ARG_MAX |
| Q10 | getppid for orphans | **Return 1** | Consistent with Q3 |

---

## Implementation Priority

| Priority | Component | Effort | Blocker |
|----------|-----------|--------|---------|
| P0 | fork | High | Core feature |
| P0 | execve | High | Core feature |
| P1 | chdir/fchdir | Low | Many programs need |
| P1 | uname | Low | Build systems |
| P2 | poll/select | Medium | Some programs |
| P2 | chmod/chown | Low | Compatibility |
| P3 | c-gull staticlib | Medium | Full compatibility |

---

## Success Criteria

- [ ] `gcc -static -o hello hello.c` binary runs
- [ ] `musl-gcc -static -o hello hello.c` binary runs
- [ ] Rust `+crt-static` binary runs
- [ ] Fork/exec lifecycle works
- [ ] No source modification required
- [ ] No Eyra dependency required
- [ ] All tests pass

---

## Dependencies

| Dependency | Status | Notes |
|------------|--------|-------|
| ELF loader | ✅ Complete | TEAM_354 |
| Linux ABI syscalls | ✅ Complete | TEAM_339-345 |
| clone (threads) | ✅ Complete | TEAM_230 |
| waitpid | ✅ Complete | TEAM_188 |
| epoll | ✅ Complete | TEAM_394 |
| ppoll | ✅ Complete | TEAM_360 |

---

## Related Teams

| Team | Relation |
|------|----------|
| TEAM_397 | Original feature plan (superseded) |
| TEAM_398 | Plan review |
| TEAM_399 | c-gull cdylib investigation |

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| fork complexity | Medium | High | Start with eager copy |
| execve edge cases | Medium | Medium | Thorough testing |
| c-gull integration issues | High | Medium | Defer to P3, have fallback |

---

## Timeline

No time estimates per project rules. Implementation order:

1. Simple syscalls (uname, umask, chdir)
2. poll/select
3. fork
4. execve
5. Integration testing
6. c-gull staticlib (if time permits)

---

## Notes

- This is a **reduced scope** from the original TEAM_397 plan
- **All Phase 2 questions have been answered** - implementation can proceed
- c-gull cdylib (libc.so) is deferred to Milestone 2 per TEAM_399 findings
- Key design decisions documented in `docs/questions/TEAM_400_general_purpose_os.md`
