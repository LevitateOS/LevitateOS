# TEAM_411: Syscall Abstractions Refactor

**Created**: 2026-01-10
**Status**: Planning

## Objective

Introduce common abstractions to reduce boilerplate and improve consistency across syscall implementations while maintaining full Linux ABI compatibility.

## Identified Abstractions

1. **VfsError → errno conversion** - Centralize error mapping
2. **UserBuffer wrapper** - Safe user-space memory handling
3. **with_fd helper** - Reduce fd lookup boilerplate
4. **resolve_at_path** - Proper dirfd support for `*at()` syscalls
5. **SyscallContext** - Ergonomic task/ttbr0 access

## Key Constraint

**All changes must preserve Linux ABI compatibility.** The syscall interface visible to userspace must remain identical.

## Planning Location

`docs/planning/syscall-abstractions/`

## Progress Log

- 2026-01-10: Team registered, starting refactor planning
- 2026-01-10: Completed 5-phase refactor plan in `docs/planning/syscall-abstractions/`

## Plan Files Created

| File | Content |
|------|---------|
| `README.md` | Overview and quick reference |
| `phase-1.md` | Discovery: Linux ABI contracts, current patterns, test baseline |
| `phase-2.md` | Extraction: New abstractions design (VfsError, UserSlice, get_fd, resolve_at_path) |
| `phase-3.md` | Migration: Call site inventory and migration order |
| `phase-4.md` | Cleanup: Dead code removal, visibility tightening |
| `phase-5.md` | Hardening: Verification, documentation, handoff |

## Status

**Planning complete.** Ready for implementation by future team.
