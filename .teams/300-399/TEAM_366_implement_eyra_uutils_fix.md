# TEAM_366 — Implement Eyra/uutils Linker Fix

**Created:** 2026-01-10  
**Plan:** `docs/planning/eyra-uutils-linker-fix/`  
**Status:** � Blocked — Hypothesis was wrong, needs proper investigation

## Objective

Implement Phase 4 of the bugfix plan: disable i18n features on blocked uutils utilities.

## Fix Strategy

For each blocked utility:
```toml
uu_XXX = { version = "0.2", default-features = false }
```

## Utilities to Fix

| Utility | Status |
|---------|--------|
| echo | Pending |
| env | Pending |
| touch | Pending |
| rm | Pending |
| rmdir | Pending |
| ln | Pending |
| cp | Pending |
| mv | Pending |
| true | Pending (minimal impl) | <- I FUCKING HATE THIS
| false | Pending (minimal impl) | <- I FUCKING HATE THIS

## CRITICAL FINDINGS

### Attempted Fixes (ALL FAILED)

| Attempt | Result |
|---------|--------|
| `default-features = false` | ICU still pulled in via uucore |
| Rename binary names | Symbol conflict persists |
| Rename package names | Symbol conflict persists |

### Root Cause NOT Identified

The `_start` symbol is being defined in the **binary's own object file**, not in a dependency. This suggests the issue is:
1. How Rust/LLVM generates code for certain binary names
2. How Eyra's origin crate interacts with `-Zbuild-std`
3. Something in the kernel's ELF loader expectations

### What Working Utilities Have in Common

- cat, pwd, mkdir, ls all work
- They have simpler dependency trees (no ICU)
- But disabling ICU on blocked utilities doesn't fix them

### Investigation Needed (NOT Shortcuts)

1. Compare object file symbols between working/blocked binaries
2. Read Eyra's origin crate source code
3. Check Eyra upstream for known issues
4. Investigate kernel ELF loader

## Progress Log

### 2026-01-10
- Created team file
- Attempted `default-features = false` — FAILED
- ICU dependencies persist regardless of feature flags
- **STOPPED** — Shortcuts don't work
- Created `FUTURE_TEAMS_README.md` documenting findings
- Real fix requires deeper investigation, not band-aids
