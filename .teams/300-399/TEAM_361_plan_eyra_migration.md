# TEAM_361 — Plan Eyra Migration

**Created:** 2026-01-09  
**Plan:** `docs/planning/eyra-migration/`  
**Status:** 🔶 Planning

## Objective

Create a migration plan to move userspace from ulib/no_std to Eyra/std.

## Current State Analysis

### Userspace Components

| Component | Size | Complexity | Migration Priority |
|-----------|------|------------|-------------------|
| **init** | 38 lines | Simple | P2 - Low priority |
| **shell** | 323 lines | Medium | P1 - High value |
| **levbox/core** | 10 utilities | Medium | P1 - High value |
| **ulib** | ~500 lines | N/A | Deprecated after migration |
| **libsyscall** | Low-level | N/A | Keep - always needed |

### levbox/core Utilities
- cat, cp, ln, ls, mkdir, mv, pwd, rm, rmdir, touch

## Progress Log

### 2026-01-09
- Analyzing current userspace structure
- Creating migration plan
