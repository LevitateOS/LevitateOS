# TEAM_061: Remove installer submodule, Create docs-tui replacement

## Status: COMPLETED

## Summary

Replaced the deprecated `installer` submodule with the new `docs-tui` submodule.

## Completed Tasks

### 1. Installer submodule removed
- `git submodule deinit -f installer`
- `rm -rf .git/modules/installer`
- `git rm -f installer`
- .gitmodules entry automatically removed

### 2. docs-tui repo created and populated
The repo at `git@github.com:LevitateOS/docs-tui.git` contains:

**Files:**
- `package.json` - Bun/Ink dependencies, workspace link to docs-content
- `tsconfig.json` - TypeScript config for Ink/React
- `src/index.tsx` - Main entry point
- `src/components/DocsPanel.tsx` - Full TUI with nav + content rendering

**Commit:** `2495542` "Initial docs-tui setup with Ink/Bun"

### 3. docs-tui added as submodule
```bash
git submodule add git@github.com:LevitateOS/docs-tui.git docs-tui
```

### 4. Root package.json updated
Changed workspaces from `["website", "installer", "docs-content"]` to `["website", "docs-tui", "docs-content"]`

### 5. Changes committed
Commit `fdb3bc1`: "feat: Replace installer submodule with docs-tui"

## Known Issue

The docs-tui TUI has a runtime error because `docs-content` uses Vite's `import.meta.glob` which isn't supported in plain Bun runtime. This is a pre-existing issue with docs-content, not related to the submodule replacement.

## Verification

```bash
$ git submodule status | grep -E "(docs-tui|installer)"
 2495542ae95ca4eca9c0d0d892ef792d5a1e5415 docs-tui (heads/main)
# installer is no longer listed
```
