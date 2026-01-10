# TEAM_364 — uutils-coreutils Integration

**Created:** 2026-01-09  
**Plan:** `docs/planning/uutils-coreutils/`  
**Status:** 🔶 Discovery

## Feature Intent

**What:** Replace hand-written Eyra userspace utilities with uutils-coreutils  
**Why:** 
- uutils is battle-tested, GNU-compatible Rust implementation
- Individual utilities available as crates (`uu_cat`, `uu_ls`, etc.)
- Full feature parity with GNU coreutils
- Maintained by active community (655+ contributors)
- Eliminates need to write/maintain our own utilities

## Context

TEAM_363 migrated cat, pwd, mkdir to Eyra/std. However:
- ls is blocked on `getdents64` syscall (not implemented)
- Each utility requires manual implementation
- Missing many features compared to GNU coreutils

With std support via Eyra, we can potentially use uutils-coreutils directly.

## Key Questions

1. **Binary size:** uutils utilities are feature-rich — how large will binaries be?
2. **Syscall coverage:** Which syscalls does uutils require that we don't have?
3. **Integration approach:** Multicall binary vs individual binaries?
4. **Selective features:** Can we build with reduced feature sets?

## Progress Log

### 2026-01-09
- Created team file
- Created Phase 1-3 planning documents
- Created questions file - user answered: Q1-A, Q2-B, Q3-Minimum, Q5-A
- **Full pivot to uutils approved**

#### Working uutils (builds with Eyra):
| Utility | Crate | Size |
|---------|-------|------|
| cat | uu_cat 0.2.2 | 790KB |
| pwd | uu_pwd 0.2 | 747KB |
| mkdir | uu_mkdir 0.2 | 780KB |
| ls | uu_ls 0.2 | 1.5MB |

#### Blocked (linker conflicts with Eyra):
- true, false, echo, env, rmdir, touch, rm, ln, cp, mv
- Root cause: Some uutils crates define `_start` symbol conflicting with Eyra's entry point
- Binary names `true`/`false` also conflict with reserved symbols

#### Next Steps:
1. Investigate Eyra/uutils linker conflict (may need Eyra patch or different build approach)
2. For blocked utilities, consider minimal hand-written implementations
3. Test working utilities (cat, pwd, mkdir, ls) in LevitateOS

#### Files Created:
- `crates/userspace/eyra/cat/` - uu_cat wrapper ✅
- `crates/userspace/eyra/pwd/` - uu_pwd wrapper ✅
- `crates/userspace/eyra/mkdir/` - uu_mkdir wrapper ✅
- `crates/userspace/eyra/ls/` - uu_ls wrapper ✅
- `crates/userspace/eyra/echo/` - blocked
- `crates/userspace/eyra/env/` - blocked
- `crates/userspace/eyra/touch/` - blocked
- `crates/userspace/eyra/rm/` - blocked
- `crates/userspace/eyra/ln/` - blocked
- `crates/userspace/eyra/rmdir/` - blocked
- `crates/userspace/eyra/cp/` - blocked
- `crates/userspace/eyra/mv/` - blocked
