# TEAM_397: General Purpose OS Feature Plan

**Date**: 2026-01-10  
**Status**: Planning Complete

## Summary

Created comprehensive feature plan for making LevitateOS a General Purpose Unix-Compatible OS.

## Definition

**General Purpose** = Can run any Unix program without modification.

**The Test**: Download a Linux binary → Run it → It works.

## Key Documents Created

- `docs/planning/general-purpose-os/FEATURE_PLAN.md` - Full feature breakdown

## Critical Path

1. **Phase A**: Complete syscall ABI (fork, execve, poll, permissions)
2. **Phase B**: Build c-gull as libc.a/libc.so.6 (THE CRITICAL MILESTONE)
3. **Phase C**: Implement dynamic linker (ld-linux.so.2)
4. **Phase D**: POSIX filesystem layout (/proc, /dev, /etc)
5. **Phase E**: Full TTY/terminal support
6. **Phase F**: Networking (optional for v1)

## Milestones

| Milestone | Goal | Metric |
|-----------|------|--------|
| 1 | Static Binary Compatibility | `gcc -static` programs run |
| 2 | Dynamic Binary Compatibility | Downloaded Linux binaries run |
| 3 | Development Environment | Can compile on LevitateOS |
| 4 | Package Management | `pkg install vim` works |

## Current Blockers for General Purpose

1. **No libc.so** - Programs can't find standard C library
2. **No dynamic linker** - Can't load shared libraries
3. **Missing syscalls** - fork, execve, poll, chmod, uid/gid
4. **No /proc** - Many programs expect /proc/self/*
5. **Incomplete TTY** - termios not fully implemented

## Recommended First Steps

1. Evaluate c-gull for building as libc.a
2. Implement fork with Copy-on-Write
3. Implement execve (replace spawn)
4. Implement poll/select
5. Test with statically-linked hello world

## Related Documentation Updates

Also updated these files with "General Purpose" definition:
- docs/VISION.md
- docs/ROADMAP.md  
- docs/ARCHITECTURE.md
- CLAUDE.md
- README.md
- CONTRIBUTING.md
- crates/kernel/README.md
