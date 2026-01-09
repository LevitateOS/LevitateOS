# TEAM_362 — Refactor Userspace to Eyra/std

**Created:** 2026-01-09  
**Plan:** `docs/planning/eyra-migration/`  
**Status:** 🔶 Planning

## Refactor Intent

**What:** Migrate all userspace apps from no_std/libsyscall to Eyra/std  
**Why:** ulib was a handrolled std replacement; now redundant since Eyra works  
**Pain Points:**
- Two runtimes to maintain (ulib + kernel)
- Limited functionality in ulib
- Non-standard Rust idioms

## Success Criteria

1. All userspace apps build with Eyra/std
2. All apps function identically to before
3. ulib is completely removed
4. Standard Rust idioms throughout userspace

## Affected Components

| Component | Action |
|-----------|--------|
| ulib | ✅ REMOVED |
| init | Rewrite with std |
| shell | Rewrite with std |
| levbox (10 utils) | Rewrite with std |
| libsyscall | Keep for custom syscalls only |

## Progress Log

### 2026-01-09
- Removed ulib from workspace
- Creating refactor plan
