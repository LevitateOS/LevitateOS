# Phase 3: Implementation - Migrate from c-gull to musl

## Implementation Overview

### Order of Operations

1. **Verify musl works** (proof of concept)
2. **Update apps.rs** (Rust programs)
3. **Add c_apps.rs** (C programs)
4. **Update sysroot.rs** (simplify/remove)
5. **Update commands.rs** (new commands)
6. **Clean up** (remove c-gull artifacts)
7. **Update documentation**

## Step 1: Proof of Concept (Do This First!)

Before changing anything, verify musl works for our use case.

```bash
# Install musl target
rustup target add x86_64-unknown-linux-musl

# Try building coreutils with musl (outside our build system)
cd toolchain/coreutils
cargo build --release --target x86_64-unknown-linux-musl \
    --no-default-features --features "cat,echo" -p coreutils

# Check the binary
file target/x86_64-unknown-linux-musl/release/coreutils
# Should say: "statically linked"

# Try running it (on Linux host)
./target/x86_64-unknown-linux-musl/release/coreutils echo hello
```

If this works, proceed. If not, investigate before changing build system.

**Estimated work**: 1 unit (just verification)

## Step 2: Update apps.rs

**File**: `xtask/src/build/apps.rs`

### Changes

```rust
// BEFORE (complex)
fn get_sysroot_rustflags() -> String {
    let sysroot_path = ...;
    format!(
        "-C panic=abort \
         -C relocation-model=pic \
         -C link-arg=-nostartfiles \
         -C link-arg=-static-pie \
         -C link-arg=-Wl,--allow-multiple-definition \
         -C link-arg=-L{}/lib",
        sysroot_path.display()
    )
}

impl ExternalApp {
    pub fn build(&self, arch: &str) -> Result<()> {
        let target = linux_target(arch);  // x86_64-unknown-linux-gnu
        let rustflags = get_sysroot_rustflags();

        let status = Command::new("cargo")
            .current_dir(self.clone_dir())
            .arg("+nightly-2025-04-28")
            .env_remove("RUSTUP_TOOLCHAIN")
            .env("RUSTFLAGS", &rustflags)
            .args([
                "build", "--release", "--target", target,
                "-Z", "build-std=std,panic_abort",
                "-Z", "build-std-features=panic_immediate_abort",
                "-p", self.package,
            ])
            // ...
    }
}

// AFTER (simple)
impl ExternalApp {
    pub fn build(&self, arch: &str) -> Result<()> {
        let target = musl_target(arch);  // x86_64-unknown-linux-musl

        // Ensure target is installed
        ensure_musl_target(arch)?;

        let status = Command::new("cargo")
            .current_dir(self.clone_dir())
            .args([
                "build",
                "--release",
                "--target", target,
                "-p", self.package,
            ])
            // ...
    }
}

fn musl_target(arch: &str) -> &'static str {
    match arch {
        "x86_64" => "x86_64-unknown-linux-musl",
        "aarch64" => "aarch64-unknown-linux-musl",
        _ => "x86_64-unknown-linux-musl",
    }
}

fn ensure_musl_target(arch: &str) -> Result<()> {
    let target = musl_target(arch);
    let output = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()?;

    if !String::from_utf8_lossy(&output.stdout).contains(target) {
        println!("📥 Installing {} target...", target);
        let status = Command::new("rustup")
            .args(["target", "add", target])
            .status()?;
        if !status.success() {
            bail!("Failed to install musl target");
        }
    }
    Ok(())
}
```

### Delete
- `get_sysroot_rustflags()` function
- `linux_target()` function (replace with `musl_target()`)

**Estimated work**: 3 units

## Step 3: Add c_apps.rs

**File**: `xtask/src/build/c_apps.rs` (NEW)

```rust
//! C application build support using musl
//!
//! Similar to apps.rs but for C programs like dash.

use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use std::process::Command;

pub struct ExternalCApp {
    pub name: &'static str,
    pub repo: &'static str,
    pub binary: &'static str,
    pub configure_args: &'static [&'static str],
    pub needs_autoreconf: bool,
    pub required: bool,
}

pub static C_APPS: &[ExternalCApp] = &[
    ExternalCApp {
        name: "dash",
        repo: "https://git.kernel.org/pub/scm/utils/dash/dash.git",
        binary: "src/dash",
        configure_args: &[
            "--enable-static",
            "--disable-fnmatch",
            "--disable-glob",
        ],
        needs_autoreconf: true,
        required: false,
    },
];

impl ExternalCApp {
    pub fn clone_dir(&self) -> PathBuf {
        PathBuf::from(format!("toolchain/{}", self.name))
    }

    pub fn output_dir(&self, arch: &str) -> PathBuf {
        PathBuf::from(format!("toolchain/{}-out/{}", self.name, arch))
    }

    pub fn output_path(&self, arch: &str) -> PathBuf {
        self.output_dir(arch).join(
            PathBuf::from(self.binary)
                .file_name()
                .unwrap()
        )
    }

    pub fn exists(&self, arch: &str) -> bool {
        self.output_path(arch).exists()
    }

    pub fn clone_repo(&self) -> Result<()> {
        let dir = self.clone_dir();
        if dir.exists() {
            return Ok(());
        }

        println!("📥 Cloning {}...", self.name);
        let status = Command::new("git")
            .args(["clone", "--depth=1", self.repo, &dir.to_string_lossy()])
            .status()?;

        if !status.success() {
            bail!("Failed to clone {}", self.name);
        }
        Ok(())
    }

    pub fn build(&self, arch: &str) -> Result<()> {
        self.clone_repo()?;
        ensure_musl_gcc()?;

        let clone_dir = self.clone_dir();

        // Run autoreconf if needed
        if self.needs_autoreconf {
            println!("🔧 Running autoreconf for {}...", self.name);
            let status = Command::new("autoreconf")
                .current_dir(&clone_dir)
                .arg("-fi")
                .status()
                .context("autoreconf failed - install autoconf/automake")?;

            if !status.success() {
                bail!("autoreconf failed for {}", self.name);
            }
        }

        // Configure
        println!("🔧 Configuring {}...", self.name);
        let mut configure = Command::new("./configure");
        configure
            .current_dir(&clone_dir)
            .env("CC", "musl-gcc")
            .env("CFLAGS", "-static -Os");

        for arg in self.configure_args {
            configure.arg(arg);
        }

        let status = configure.status()?;
        if !status.success() {
            bail!("configure failed for {}", self.name);
        }

        // Build
        println!("🔧 Building {}...", self.name);
        let status = Command::new("make")
            .current_dir(&clone_dir)
            .arg("-j4")
            .status()?;

        if !status.success() {
            bail!("make failed for {}", self.name);
        }

        // Copy to output
        let src = clone_dir.join(self.binary);
        let out_dir = self.output_dir(arch);
        std::fs::create_dir_all(&out_dir)?;
        let dst = self.output_path(arch);
        std::fs::copy(&src, &dst)?;

        println!("✅ {} built: {}", self.name, dst.display());
        Ok(())
    }
}

fn ensure_musl_gcc() -> Result<()> {
    if Command::new("musl-gcc").arg("--version").output().is_err() {
        bail!(
            "musl-gcc not found. Install with:\n\
             Fedora: sudo dnf install musl-gcc musl-devel\n\
             Ubuntu: sudo apt install musl-tools musl-dev\n\
             Arch:   sudo pacman -S musl"
        );
    }
    Ok(())
}

pub fn get_c_app(name: &str) -> Option<&'static ExternalCApp> {
    C_APPS.iter().find(|app| app.name == name)
}
```

**Estimated work**: 4 units

## Step 4: Update sysroot.rs

**File**: `xtask/src/build/sysroot.rs`

```rust
//! Sysroot management (simplified for musl)
//!
//! With musl, we use system-installed musl. No need to build our own libc.

use anyhow::{bail, Result};
use std::process::Command;

/// Ensure musl target is installed for Rust
pub fn ensure_rust_musl_target(arch: &str) -> Result<()> {
    let target = match arch {
        "x86_64" => "x86_64-unknown-linux-musl",
        "aarch64" => "aarch64-unknown-linux-musl",
        _ => bail!("Unsupported architecture: {}", arch),
    };

    let output = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()?;

    let installed = String::from_utf8_lossy(&output.stdout);
    if !installed.contains(target) {
        println!("📥 Installing Rust musl target: {}", target);
        let status = Command::new("rustup")
            .args(["target", "add", target])
            .status()?;

        if !status.success() {
            bail!("Failed to install {} target", target);
        }
    }

    Ok(())
}

/// Check if musl-gcc is available for C programs
pub fn check_musl_gcc() -> Result<()> {
    if Command::new("musl-gcc").arg("--version").output().is_err() {
        bail!(
            "musl-gcc not found.\n\n\
             Install with:\n\
             Fedora: sudo dnf install musl-gcc musl-devel\n\
             Ubuntu: sudo apt install musl-tools musl-dev\n\
             Arch:   sudo pacman -S musl"
        );
    }
    Ok(())
}

/// Legacy function - now always returns true
/// Kept for backward compatibility with code that checks sysroot_exists()
pub fn sysroot_exists() -> bool {
    true
}

/// Legacy function - now just ensures musl target is installed
pub fn build_sysroot(arch: &str) -> Result<()> {
    println!("ℹ️  Using system musl (no sysroot build needed)");
    ensure_rust_musl_target(arch)?;
    println!("✅ musl target ready");
    Ok(())
}
```

**Estimated work**: 2 units

## Step 5: Update commands.rs

**File**: `xtask/src/build/commands.rs`

### Add Dash command

```rust
#[derive(Subcommand)]
pub enum BuildCommands {
    All,
    Kernel,
    Userspace,
    Initramfs,
    Iso,
    Sysroot,    // Now just installs musl target
    Coreutils,
    Brush,
    Dash,       // NEW
}
```

### Update build_all to include C apps

```rust
pub fn build_all(arch: &str) -> Result<()> {
    // Ensure musl target installed
    super::sysroot::ensure_rust_musl_target(arch)?;

    // Build Rust apps
    super::apps::ensure_all_built(arch)?;

    // Build C apps (optional - only if musl-gcc available)
    if super::sysroot::check_musl_gcc().is_ok() {
        for app in super::c_apps::C_APPS {
            if !app.required {
                // Optional C apps - build if possible
                let _ = app.build(arch);
            }
        }
    }

    build_userspace(arch)?;
    create_initramfs(arch)?;
    disk::install_userspace_to_disk(arch)?;
    build_kernel_with_features(&[], arch)
}
```

**Estimated work**: 2 units

## Step 6: Clean Up

### Remove files
```bash
rm -rf toolchain/libc-levitateos/
# toolchain/c-ward/ is gitignored, will be orphaned
# toolchain/sysroot/ is gitignored, will be orphaned
```

### Update .gitignore
```gitignore
# Old c-gull artifacts (can remove these lines)
# toolchain/c-ward/
# toolchain/sysroot/

# New musl artifacts
toolchain/dash/
toolchain/dash-out/
```

**Estimated work**: 1 unit

## Step 7: Update Documentation

### CLAUDE.md changes

```markdown
## Build Commands

### Building

# Build c-gull sysroot (REMOVED)
# cargo xtask build sysroot   # No longer needed

# Build dash shell (NEW)
cargo xtask build dash        # Build dash (requires musl-gcc)
```

### Add musl requirements section

```markdown
## Prerequisites

### For Rust userspace (coreutils, brush)
- Rust with musl target: `rustup target add x86_64-unknown-linux-musl`

### For C userspace (dash)
- musl-gcc: `sudo dnf install musl-gcc musl-devel` (Fedora)
- autoconf/automake: For building dash from source
```

**Estimated work**: 1 unit

## Total Estimated Work

| Step | Description | Units |
|------|-------------|-------|
| 1 | Proof of concept | 1 |
| 2 | Update apps.rs | 3 |
| 3 | Add c_apps.rs | 4 |
| 4 | Update sysroot.rs | 2 |
| 5 | Update commands.rs | 2 |
| 6 | Clean up | 1 |
| 7 | Documentation | 1 |
| **Total** | | **14** |

## Dependencies

- Step 1 must pass before proceeding
- Steps 2-5 can be done in any order
- Step 6-7 after verification
