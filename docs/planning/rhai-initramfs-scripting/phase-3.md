# Phase 3: Implementation - Rhai Initramfs Scripting

## Implementation Overview

### File Changes (In Order)

| Order | File | Action | Description |
|-------|------|--------|-------------|
| 1 | `xtask/Cargo.toml` | Modify | Add `rhai` dependency |
| 2 | `xtask/src/build/initramfs.rs` | Create | Rhai engine, builder, entry types |
| 3 | `xtask/src/build/mod.rs` | Modify | Add `initramfs` module |
| 4 | `initramfs.rhai` | Create | Default initramfs script |
| 5 | `xtask/src/build/commands.rs` | Modify | Use InitramfsBuilder instead of hardcoded logic |
| 6 | `xtask/src/main.rs` | Modify | Update build command handling |
| 7 | `xtask/src/build/apps.rs` | Remove | Replaced by initramfs.rs |

### Dependencies

```toml
# xtask/Cargo.toml
[dependencies]
rhai = { version = "1.23", features = ["sync"] }  # sync for thread-safe types
```

## Design Reference

See [Phase 2: Design](./phase-2.md) for:
- API signatures
- Error handling behavior
- Behavioral decisions
- Entry types

## Implementation Steps

### Step 1: Add Rhai Dependency

```toml
# xtask/Cargo.toml
rhai = { version = "1.23", features = ["sync"] }
```

### Step 2: Create InitramfsEntry Types

```rust
// xtask/src/build/initramfs.rs

use anyhow::{bail, Context, Result};
use rhai::{Engine, Scope, Dynamic, Map, Array};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Entry types that can be added to initramfs
#[derive(Debug, Clone)]
pub enum InitramfsEntry {
    File {
        src: PathBuf,
        dest: String,
        required: bool,
    },
    Userspace {
        name: String,
        required: bool,
    },
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
    Symlink {
        target: String,
        link_name: String,
    },
}
```

### Step 3: Create InitramfsBuilder

```rust
/// Builder that collects entries from Rhai script
pub struct InitramfsBuilder {
    entries: Arc<Mutex<Vec<InitramfsEntry>>>,
    arch: String,
    build_type: String,
}

impl InitramfsBuilder {
    pub fn new(arch: &str, build_type: &str) -> Self {
        Self {
            entries: Arc::new(Mutex::new(Vec::new())),
            arch: arch.to_string(),
            build_type: build_type.to_string(),
        }
    }

    pub fn add_entry(&self, entry: InitramfsEntry) {
        self.entries.lock().unwrap().push(entry);
    }

    pub fn entries(&self) -> Vec<InitramfsEntry> {
        self.entries.lock().unwrap().clone()
    }
}
```

### Step 4: Create Rhai Engine Setup

```rust
/// Create Rhai engine with initramfs functions registered
pub fn create_engine(builder: Arc<InitramfsBuilder>) -> Engine {
    let mut engine = Engine::new();

    // Register file() function
    let b = builder.clone();
    engine.register_fn("file", move |src: &str, dest: &str| {
        b.add_entry(InitramfsEntry::File {
            src: PathBuf::from(src),
            dest: dest.to_string(),
            required: true,
        });
    });

    // Register file() with options
    let b = builder.clone();
    engine.register_fn("file", move |src: &str, dest: &str, opts: Map| {
        let required = opts.get("required")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(true);
        b.add_entry(InitramfsEntry::File {
            src: PathBuf::from(src),
            dest: dest.to_string(),
            required,
        });
    });

    // Register userspace() function
    let b = builder.clone();
    engine.register_fn("userspace", move |name: &str, opts: Map| {
        let required = opts.get("required")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(true);
        b.add_entry(InitramfsEntry::Userspace {
            name: name.to_string(),
            required,
        });
    });

    // Register app() function
    let b = builder.clone();
    engine.register_fn("app", move |name: &str, opts: Map| {
        let repo = opts.get("repo")
            .and_then(|v| v.clone().into_string().ok())
            .unwrap_or_default();
        let package = opts.get("package")
            .and_then(|v| v.clone().into_string().ok())
            .unwrap_or_else(|| name.to_string());
        let binary = opts.get("binary")
            .and_then(|v| v.clone().into_string().ok())
            .unwrap_or_else(|| name.to_string());
        let features = opts.get("features")
            .and_then(|v| v.clone().into_typed_array::<String>().ok())
            .unwrap_or_default();
        let required = opts.get("required")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(true);
        let symlinks = opts.get("symlinks")
            .and_then(|v| v.clone().into_typed_array::<String>().ok())
            .unwrap_or_default();
        let no_default_features = opts.get("no_default_features")
            .and_then(|v| v.as_bool().ok())
            .unwrap_or(false);

        b.add_entry(InitramfsEntry::App {
            name: name.to_string(),
            repo,
            package,
            binary,
            features,
            required,
            symlinks,
            no_default_features,
        });
    });

    // Register symlink() function
    let b = builder.clone();
    engine.register_fn("symlink", move |target: &str, link_name: &str| {
        b.add_entry(InitramfsEntry::Symlink {
            target: target.to_string(),
            link_name: link_name.to_string(),
        });
    });

    engine
}
```

### Step 5: Script Execution

```rust
/// Load and execute initramfs script
pub fn load_script(arch: &str, build_type: &str) -> Result<Vec<InitramfsEntry>> {
    let script_path = PathBuf::from("initramfs.rhai");
    if !script_path.exists() {
        bail!(
            "initramfs.rhai not found in repository root.\n\
             Create it to define your initramfs contents."
        );
    }

    let script = std::fs::read_to_string(&script_path)
        .context("Failed to read initramfs.rhai")?;

    let builder = Arc::new(InitramfsBuilder::new(arch, build_type));
    let engine = create_engine(builder.clone());

    // Set up scope with built-in variables
    let mut scope = Scope::new();
    scope.push_constant("ARCH", arch.to_string());
    scope.push_constant("BUILD_TYPE", build_type.to_string());

    // Execute script
    engine.run_with_scope(&mut scope, &script)
        .map_err(|e| anyhow::anyhow!("Script error: {}", e))?;

    Ok(builder.entries())
}
```

### Step 6: Build Logic

```rust
/// Build initramfs from entries
pub fn build_initramfs(arch: &str, build_type: &str) -> Result<PathBuf> {
    println!("Loading initramfs.rhai...");
    let entries = load_script(arch, build_type)?;

    println!("Found {} entries", entries.len());

    let root = PathBuf::from("initrd_root");
    if root.exists() {
        std::fs::remove_dir_all(&root)?;
    }
    std::fs::create_dir(&root)?;

    let mut count = 0;
    for entry in &entries {
        match entry {
            InitramfsEntry::File { src, dest, required } => {
                if src.exists() {
                    std::fs::copy(src, root.join(dest))?;
                    count += 1;
                    println!("  📄 {}", dest);
                } else if *required {
                    bail!("Required file not found: {}", src.display());
                } else {
                    println!("  ℹ️  Skipping optional file: {}", src.display());
                }
            }
            InitramfsEntry::Userspace { name, required } => {
                // Copy from crates/userspace/target/...
                let target = bare_metal_target(arch);
                let src = PathBuf::from(format!(
                    "crates/userspace/target/{}/release/{}",
                    target, name
                ));
                if src.exists() {
                    std::fs::copy(&src, root.join(name))?;
                    count += 1;
                    println!("  📦 {}", name);
                } else if *required {
                    bail!(
                        "Required userspace binary '{}' not found.\n\
                         Run 'cargo xtask build userspace' first.",
                        name
                    );
                } else {
                    println!("  ℹ️  Skipping optional: {}", name);
                }
            }
            InitramfsEntry::App { name, required, symlinks, .. } => {
                // Use existing app build logic
                let app = build_app_from_entry(entry, arch)?;
                if app.exists() {
                    std::fs::copy(&app, root.join(name))?;
                    count += 1;
                    // Create symlinks
                    for link in symlinks {
                        create_symlink(name, link, &root)?;
                    }
                    println!("  📦 {} + {} symlinks", name, symlinks.len());
                } else if *required {
                    bail!("Required app '{}' build failed", name);
                }
            }
            InitramfsEntry::Symlink { target, link_name } => {
                create_symlink(target, link_name, &root)?;
                println!("  🔗 {} -> {}", link_name, target);
            }
        }
    }

    println!("[DONE] {} entries added", count);

    // Create CPIO archive
    let cpio_path = create_cpio(&root, arch)?;
    Ok(cpio_path)
}
```

### Step 7: Create Default Script

```rhai
// initramfs.rhai - LevitateOS initramfs definition
//
// This script defines what goes into the initramfs (initial RAM filesystem).
// Edit this file to add/remove components.
//
// Available functions:
//   file(src, dest)           - Include a static file
//   file(src, dest, opts)     - Include with options (required: bool)
//   userspace(name, opts)     - Bare-metal binary from crates/userspace
//   app(name, opts)           - External app (clone + build against sysroot)
//   symlink(target, link)     - Create a symlink
//
// Available variables:
//   ARCH       - Target architecture ("x86_64" or "aarch64")
//   BUILD_TYPE - Build type ("release", "test", "verbose")

// Static files
file("assets/hello.txt", "hello.txt", #{ required: false });

// Bare-metal userspace binaries
userspace("init", #{ required: true });
userspace("shell", #{ required: true });

// External apps (built against c-gull sysroot)
app("coreutils", #{
    repo: "https://github.com/uutils/coreutils",
    package: "coreutils",
    features: ["cat", "echo", "head", "mkdir", "pwd", "rm", "tail", "touch"],
    required: true,
    no_default_features: true,
    symlinks: ["cat", "echo", "head", "mkdir", "pwd", "rm", "tail", "touch"],
});

app("brush", #{
    repo: "https://github.com/reubeno/brush",
    package: "brush",
    required: false,
});

// Test-specific content
if BUILD_TYPE == "test" {
    // Add test fixtures here
}

// Architecture-specific content
if ARCH == "x86_64" {
    // x86_64-specific files here
}
```

### Step 8: Update commands.rs

Replace `create_initramfs()` with call to `initramfs::build_initramfs()`.

### Step 9: Remove apps.rs

Delete `xtask/src/build/apps.rs` and remove from `mod.rs`.

## Dependencies Between Steps

```
Step 1 (Cargo.toml)
    ↓
Step 2-4 (initramfs.rs + script) - can be done in parallel
    ↓
Step 5 (commands.rs) - depends on initramfs.rs
    ↓
Step 6 (main.rs)
    ↓
Step 7 (remove apps.rs) - last, after everything works
```

## Testing Strategy

1. Unit tests for Rhai function registration
2. Integration test: load sample script, verify entries
3. End-to-end: `cargo xtask build all` produces working initramfs
4. Error cases: missing script, syntax errors, missing required files
