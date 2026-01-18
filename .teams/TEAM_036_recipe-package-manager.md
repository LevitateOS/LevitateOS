# TEAM_036: Recipe Package Manager - Local-First Design

## Status: COMPLETE

## Objective

Implement full package manager functionality for the recipe system: install, remove, update, upgrade, run commands with state tracking that lives inside the recipe files themselves.

## Core Design

- **Local-first, user-maintained**: Recipes live in `~/.local/share/recipe/recipes/`
- **State lives IN the recipe**: No separate database - recipe file IS the source of truth
- **Self-modifying recipes**: Engine updates state variables (installed, installed_version, installed_files) directly in recipe files

## Implementation Phases

### Phase 1: Recipe State Management ✅
- [x] Create `recipe_state.rs` - functions to read/modify recipe variables
- [x] Modify `install_*()` helpers to return installed file paths
- [x] Update `context.rs` to track installed files
- [x] Update `lifecycle.rs` to record state after install

### Phase 2: Remove Command ✅
- [x] Add `remove` CLI command
- [x] Read `installed_files` from recipe
- [x] Delete those files
- [x] Run recipe's `remove()` if defined
- [x] Update recipe state

### Phase 3: Update Command ✅
- [x] Add `http_get()` helper
- [x] Add `github_latest_release()` convenience helper
- [x] Add `update` CLI command
- [x] Run recipe's `check_update()` if defined
- [x] Update recipe's `version` variable if new version found

### Phase 4: Upgrade Command ✅
- [x] Add `upgrade` CLI command
- [x] Compare `version` vs `installed_version`
- [x] If different: remove → install

### Phase 5: Run Command - REMOVED
- Removed per user feedback: packages should install properly-wrapped binaries
- If a binary needs env setup, the recipe creates a wrapper script during install
- Users run installed binaries directly from PATH

## Files Modified

| File | Changes |
|------|---------|
| `recipe/src/bin/recipe.rs` | Complete rewrite: install, remove, update, upgrade, run, list, search, info commands |
| `recipe/src/engine/mod.rs` | Register new helpers, add remove/update/upgrade/run methods to RecipeEngine |
| `recipe/src/engine/lifecycle.rs` | Add: remove, update, upgrade, run_package functions |
| `recipe/src/engine/context.rs` | Track installed files during install phase |
| `recipe/src/engine/phases/install.rs` | Record installed file paths |
| `recipe/src/engine/recipe_state.rs` | **NEW**: Read/write recipe variables |
| `recipe/src/engine/util/http.rs` | **NEW**: http_get(), github_latest_release(), github_latest_tag() |
| `recipe/src/engine/util/exec.rs` | **NEW**: exec(), exec_output() |
| `recipe/src/engine/util/mod.rs` | Export new modules |
| `recipe/src/lib.rs` | Export recipe_state module |
| `recipe/Cargo.toml` | Add deps: dirs, ureq, serde_json |

## Decisions Made

1. State stored directly in recipe files (not external DB)
2. XDG-compliant default path: `~/.local/share/recipe/recipes/`
3. Engine auto-updates state variables after each operation
4. Added `shell()` alias for shell command to avoid conflict with recipe's `run()` function

## Key Features

### CLI Commands
```bash
recipe install <name>       # Install a package
recipe remove <name>        # Remove a package
recipe update [name]        # Check for updates (modifies recipes)
recipe upgrade [name]       # Apply updates (reinstall)
recipe list                 # List installed packages
recipe search <pattern>     # Search available recipes
recipe info <name>          # Show package info
```

### Recipe State Variables
```rhai
// STATE (engine-maintained)
let installed = false;           // Set to true after install
let installed_version = ();      // Version that was installed
let installed_at = ();           // Unix timestamp
let installed_files = [];        // List of installed file paths
```

### New Helpers for Recipes
```rhai
// HTTP helpers
http_get(url)                    // Fetch URL content
github_latest_release(repo)      // Get latest release tag
github_latest_tag(repo)          // Get latest tag

// Execution helpers
exec(cmd, args)                  // Execute command with args
exec_output(cmd, args)           // Execute and capture stdout
shell(cmd)                       // Alias for run() to avoid conflicts
```

## Testing

Tested with example recipe:
```bash
# Install
recipe install hello  # Creates /tmp/recipe-test/prefix/bin/hello
                      # Updates recipe: installed=true, installed_files=[...]

# List
recipe list           # Shows: hello [installed: 1.0.0]

# Info
recipe info hello     # Shows full package info including files

# Run
recipe run hello      # Executes the installed package

# Remove
recipe remove hello   # Deletes files, updates recipe: installed=false
```

All phases complete. Package manager is functional.
