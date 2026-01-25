# TEAM_040: Recipe PM Feature Roadmap Implementation

## Mission
Implement the Recipe PM Feature Roadmap, starting with Phase 1 (colored output, progress reporting, hooks).

## Status: COMPLETED (Phase 1)

## Phase 1: Quick Wins - DONE
- [x] Colored Output (owo-colors) - All CLI output now uses colored text
- [x] Progress Reporting (indicatif) - Downloads show progress bars, commands show spinners
- [x] Pre/Post Install Hooks - Added pre_install, post_install, pre_remove, post_remove

## Phase 2: Core Enhancements (Future)
- [ ] Optional Dependencies
- [ ] Config File Tracking
- [ ] Disk Space Checking

## Phase 3: Dependency System (Future)
- [ ] Package Groups
- [ ] Provides/Virtual Packages
- [ ] Build-time Dependencies (makedeps)

## Phase 4: Safety & Polish (Future)
- [ ] Conflicts/Replaces Relations
- [ ] File Conflict Detection
- [ ] Parallel Downloads

## Files Modified

### New Files
- `recipe/src/engine/output.rs` - Centralized output styling module with colored text and progress bars

### Modified Files
- `recipe/Cargo.toml` - Added owo-colors v4 and indicatif v0.17
- `recipe/src/lib.rs` - Export output module
- `recipe/src/engine/mod.rs` - Register output module
- `recipe/src/bin/recipe.rs` - Updated CLI to use colored output
- `recipe/src/engine/lifecycle.rs` - Added hooks and colored output
- `recipe/src/engine/phases/acquire.rs` - Download progress bar using ureq
- `recipe/src/engine/phases/build.rs` - Spinner for commands and extraction
- `recipe/src/engine/phases/install.rs` - Colored detail messages

## Features Implemented

### 1. Colored Output
- Action headers in blue bold: `==> Installing ripgrep`
- Sub-actions in cyan: `-> acquire`, `-> build`, `-> install`
- Details in dimmed text
- Success messages in green
- Warnings in yellow
- Errors in red
- Package names highlighted in listings

### 2. Progress Reporting
- **Downloads**: Full progress bar with bytes/total and ETA
- **Extraction**: Spinner while extracting archives
- **Commands**: Spinner while running shell commands
- **Package counter**: Shows `(1/5) Installing package` for multi-package installs

### 3. Lifecycle Hooks
Recipes can now define optional hooks:
```rhai
fn pre_install() {
    // Runs before install phase
    // Use for: backup config, create users, etc.
}

fn post_install() {
    // Runs after install phase
    // Use for: ldconfig, systemd reload, etc.
}

fn pre_remove() {
    // Runs before file deletion
    // Use for: stop services, etc.
}

fn post_remove() {
    // Runs after file deletion
    // Use for: cleanup, remove users, etc.
}
```

## Verification
- All 240+ tests pass
- No compiler warnings
- Build succeeds

## Decisions Made
- Used ureq (already a dependency) for downloads instead of curl for better progress tracking
- Centralized all output styling in `output.rs` module for consistency
- Spinners clear after completion to keep output clean
- Long commands are truncated in spinner display for readability

## Problems Encountered
None - implementation went smoothly.

## Notes
- Download progress bar shows bytes/total and ETA when Content-Length header is available
- Falls back to spinner when content length is unknown
- Hooks silently ignore errors (use `let _ = call_action`) to not block removal on cleanup failures
