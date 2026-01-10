# TEAM_365 — Bugfix: Eyra/uutils Linker Conflict

**Created:** 2026-01-10  
**Plan:** `docs/planning/eyra-uutils-linker-fix/`  
**Status:** 🔶 Planning

## Bug Summary

When building uutils-coreutils utilities with Eyra, some utilities fail to link with duplicate symbol errors:
- `duplicate symbol: _start`
- `duplicate symbol: __dso_handle`

**Working utilities:** cat, pwd, mkdir, ls  
**Blocked utilities:** true, false, echo, env, rmdir, touch, rm, ln, cp, mv

## Severity

- **Impact:** Blocks 10 of 14 target utilities from building
- **User-facing:** Yes — missing core utilities
- **Workaround:** Write minimal hand-written implementations

## Reproduction

```bash
cd crates/userspace/eyra/echo
cargo build --release --target x86_64-unknown-linux-gnu -Zbuild-std=std,panic_abort
```

**Error:**
```
rust-lld: error: duplicate symbol: _start
rust-lld: error: duplicate symbol: __dso_handle
```

## Root Cause

**Finding:** Blocked utilities enable uucore's `i18n-common` → `icu_locale` features which bring in ICU dependencies that define C runtime symbols (`_start`, `__dso_handle`) conflicting with Eyra.

**Working utilities:** Have simpler features (fs, pipes) without ICU chain.

## Recommended Fix — INVALIDATED

~~**Disable i18n features** via `default-features = false`~~ — **DID NOT WORK**

TEAM_366 attempted this fix and it failed:
- ICU dependencies persist regardless of feature flags
- The `_start` symbol comes from the binary's object file, not dependencies
- **Root cause hypothesis was WRONG**

See `docs/planning/eyra-uutils-linker-fix/FUTURE_TEAMS_README.md` for investigation guidance.

## Progress Log

### 2026-01-10
- Created team file
- Created Phase 1-3 planning documents
- Identified root cause: ICU dependencies from i18n features
- Recommended fix: Disable default features on blocked utilities
