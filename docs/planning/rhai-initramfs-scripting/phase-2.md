# Phase 2: Design - Rhai Initramfs Scripting

> **This is the most important phase.** Every edge case, error condition, and behavioral decision must be documented here.

## Proposed Solution

### User-Facing Behavior

Users define initramfs contents in `initramfs.rhai` at the repository root:

```rhai
// initramfs.rhai - Declarative initramfs definition

// Static files
file("assets/hello.txt", "hello.txt");

// Bare-metal userspace binaries (from crates/userspace)
userspace("init", #{ required: true });
userspace("shell", #{ required: true });

// External apps (cloned + built against sysroot)
app("coreutils", #{
    repo: "https://github.com/uutils/coreutils",
    package: "coreutils",
    features: ["cat", "echo", "head", "mkdir", "pwd", "rm", "tail", "touch"],
    required: true,
    symlinks: ["cat", "echo", "head", "mkdir", "pwd", "rm", "tail", "touch"],
});

app("brush", #{
    repo: "https://github.com/reubeno/brush",
    package: "brush",
    required: false,
});

// Conditional logic
if ARCH == "x86_64" {
    file("assets/x86-specific.bin", "firmware.bin");
}

if BUILD_TYPE == "test" {
    file("test-fixtures/test-data.txt", "test-data.txt");
}
```

### System Behavior

1. `cargo xtask build initramfs` loads and executes `initramfs.rhai`
2. Script calls register entries with the `InitramfsBuilder`
3. After script completes, builder validates all entries
4. Required entries that don't exist → fail fast with clear error
5. Optional entries that don't exist → log info message, skip
6. Build process: clone repos, compile apps, copy files, create symlinks
7. Generate CPIO archive

## API Design

### Rhai Functions Exposed to Scripts

| Function | Signature | Description |
|----------|-----------|-------------|
| `file` | `file(src: String, dest: String)` | Include static file |
| `file` | `file(src: String, dest: String, opts: Map)` | Include with options |
| `userspace` | `userspace(name: String, opts: Map)` | Bare-metal binary from crates/userspace |
| `app` | `app(name: String, opts: Map)` | External app (clone + build) |
| `symlink` | `symlink(target: String, link_name: String)` | Create symlink |

### Built-in Variables

| Variable | Type | Description |
|----------|------|-------------|
| `ARCH` | String | Target architecture ("x86_64" or "aarch64") |
| `BUILD_TYPE` | String | Build type ("release", "test", "verbose") |
| `SYSROOT_PATH` | String | Path to c-gull sysroot |

### Options Maps

**File Options:**
```rhai
file("src", "dest", #{
    required: true,     // Default: true
    mode: 0o755,        // Default: preserve source mode
});
```

**Userspace Options:**
```rhai
userspace("init", #{
    required: true,     // Default: true
});
```

**App Options:**
```rhai
app("name", #{
    repo: "https://...",      // Required
    package: "crate-name",    // Default: same as name
    binary: "binary-name",    // Default: same as name
    features: ["a", "b"],     // Default: [] (default features)
    required: true,           // Default: true
    symlinks: ["a", "b"],     // Default: []
    no_default_features: false, // Default: false
});
```

### Error Handling

| Error Type | Behavior |
|------------|----------|
| Script syntax error | Fail immediately with line number and message |
| Unknown function call | Fail with "unknown function 'xyz'" |
| Missing required field | Fail with "app 'foo' missing required field 'repo'" |
| Required file not found | Fail with path and suggestion |
| Optional file not found | Log info, continue |
| Build failure | Fail with build output |
| Clone failure | Fail with git error |

### Rust Types

```rust
/// Entry types that can be added to initramfs
#[derive(Debug, Clone)]
pub enum InitramfsEntry {
    /// Static file to copy
    File {
        src: PathBuf,
        dest: String,
        required: bool,
    },
    /// Bare-metal userspace binary
    Userspace {
        name: String,
        required: bool,
    },
    /// External app to clone and build
    App {
        name: String,
        repo: String,
        package: String,
        binary: String,
        features: Vec<String>,
        required: bool,
        symlinks: Vec<String>,
        no_default_features: bool,
    },
    /// Symlink to create
    Symlink {
        target: String,
        link_name: String,
    },
}

/// Builder that collects entries from Rhai script
pub struct InitramfsBuilder {
    entries: Vec<InitramfsEntry>,
    arch: String,
    build_type: String,
}

impl InitramfsBuilder {
    pub fn new(arch: &str, build_type: &str) -> Self;
    pub fn add_entry(&mut self, entry: InitramfsEntry);
    pub fn entries(&self) -> &[InitramfsEntry];
    pub fn validate(&self) -> Result<()>;
    pub fn build(&self) -> Result<PathBuf>;
}
```

## Data Model Changes

### Migration from `apps.rs`

Current `apps.rs`:
```rust
pub static APPS: &[ExternalApp] = &[...];
```

After migration:
- `apps.rs` can be removed entirely
- All app definitions live in `initramfs.rhai`
- `ExternalApp` struct replaced by `InitramfsEntry::App`

### Script Storage

| File | Purpose |
|------|---------|
| `initramfs.rhai` | Production initramfs definition |
| `initramfs-test.rhai` | Test-specific initramfs (optional) |

## Behavioral Decisions

### Q: What if `initramfs.rhai` doesn't exist?
**A:** Fail with clear error: "initramfs.rhai not found. Create it or run 'cargo xtask init' to generate a template."

### Q: What if script has syntax errors?
**A:** Fail immediately with Rhai's error message including line number.

### Q: What if an app's repo is unreachable?
**A:** Fail with git clone error. Suggest checking network or URL.

### Q: What if a required app fails to build?
**A:** Fail with full build output. Do not continue with partial initramfs.

### Q: What if an optional app fails to build?
**A:** Log warning, continue without it. User can re-run after fixing.

### Q: Can scripts import other scripts?
**A:** Phase 1: No. Keep it simple with single file.
**Future:** Could add `import "other.rhai"` support.

### Q: What order are entries processed?
**A:** Declaration order in script. Apps built in parallel where possible.

### Q: What if same file/binary added twice?
**A:** Last definition wins. Log warning about duplicate.

### Q: How to handle arch-specific binaries?
**A:** Script has access to `ARCH` variable for conditionals.

### Q: What if userspace binary doesn't exist?
**A:** If `required: true`, fail with: "Userspace binary 'init' not found. Run 'cargo xtask build userspace' first."

### Q: Can apps have custom build commands?
**A:** Phase 1: No. All apps use standard cargo build with sysroot.
**Future:** Could add `build_cmd` option.

### Q: What if features list is invalid?
**A:** Cargo build will fail with feature error. Pass through to user.

## Design Alternatives Considered

### Alternative 1: TOML Configuration
```toml
[[app]]
name = "coreutils"
repo = "..."
```
**Rejected:** No conditionals, no functions, less flexible.

### Alternative 2: Lua Scripting
**Rejected:** Rhai is pure Rust, better integration, no external dependencies.

### Alternative 3: Keep Rust Registry + Add TOML Override
**Rejected:** Two sources of truth, confusing which takes precedence.

### Alternative 4: Nix-style Expressions
**Rejected:** Too complex, unfamiliar to most developers.

## Open Questions

### Must Answer Before Phase 3

1. **Script location**: Should `initramfs.rhai` be at repo root or in `xtask/`?
   - Root: More visible, easier to find
   - xtask/: Grouped with build tooling
   - **Recommendation:** Repo root for visibility

2. **Test initramfs**: Separate script file or same script with `BUILD_TYPE` conditionals?
   - Separate: Cleaner, explicit
   - Conditionals: Single source of truth
   - **Recommendation:** Single script with conditionals

3. **Default script**: Should xtask ship a default/template script, or require user to create?
   - **Recommendation:** Ship default, fail if missing

4. **Parallel builds**: Build apps in parallel or sequential?
   - Parallel: Faster
   - Sequential: Simpler, deterministic output
   - **Recommendation:** Sequential for Phase 1, parallel later

5. **Caching**: Re-evaluate script on every build, or cache parsed result?
   - **Recommendation:** Re-evaluate (scripts are fast, <100ms)

### Nice to Have (Can Defer)

6. Should we support `include()` for splitting large scripts?
7. Should we support custom build commands for non-cargo apps?
8. Should we add a `cargo xtask initramfs validate` command?
9. Should we support downloading pre-built binaries instead of building?

## Security Considerations

1. **Sandboxed execution**: Rhai scripts cannot access filesystem directly
2. **No shell execution**: Scripts cannot run arbitrary commands
3. **Validated URLs**: Only https:// URLs allowed for repos
4. **No secrets**: Scripts should not contain credentials (use env vars if needed)
