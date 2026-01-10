# TEAM_388: Graceful OOM Handling

## Mission
Make the kernel return ENOMEM to userspace instead of panicking when memory allocation fails.

## Status: Design Complete, Ready for Implementation

## Context
- Currently, when userspace (Eyra coreutils) triggers a large allocation that fails, the kernel panics
- This is too aggressive - Linux returns ENOMEM and lets userspace handle it gracefully
- The panic came from `cat hello.txt` trying to allocate ~9MB

## Design Decisions (from kernel-development.md Unix philosophy)

| Decision | Choice | Rule |
|----------|--------|------|
| Solution approach | Fix Eyra allocator, return ENOMEM | Rule 6, 11, 20 |
| Userspace panic | Exit with code 134 | Rule 14 |
| OOM killer | Defer (not MVP) | Rule 20 |
| Max heap size | 256MB | Reasonable default |
| OOM logging | Verbose only | Rule 4 |

## Planning Docs
- `docs/planning/graceful-oom/phase-1.md` - Discovery
- `docs/planning/graceful-oom/phase-2.md` - Design (complete)
- `docs/planning/graceful-oom/phase-3.md` - Implementation (ready)

## Log
- 2026-01-10: Team created, starting feature planning
- 2026-01-10: Phase 1-2 complete, design decisions made via Unix philosophy
- 2026-01-10: Phase 3 implementation plan ready
