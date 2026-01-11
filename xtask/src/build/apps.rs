//! External application registry and build abstraction
//!
//! All external apps (coreutils, brush, etc.) follow the same pattern:
//! 1. Clone from git if not present
//! 2. Build against our c-gull sysroot
//! 3. Copy output to toolchain/{name}-out/{target}/release/
//!
//! This module provides a uniform interface for all external apps,
//! ensuring consistent behavior and fail-fast error handling.

use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use std::process::Command;

/// An external application that can be built against our sysroot
#[derive(Debug, Clone)]
pub struct ExternalApp {
    /// Short name (e.g., "coreutils", "brush")
    pub name: &'static str,
    /// Git repository URL
    pub repo: &'static str,
    /// Cargo package name to build (may differ from name)
    pub package: &'static str,
    /// Output binary name
    pub binary: &'static str,
    /// Cargo features to enable (comma-separated, or empty)
    pub features: &'static str,
    /// Whether this app is required for a complete initramfs
    pub required: bool,
    /// Symlinks to create in initramfs (for multi-call binaries like coreutils)
    pub symlinks: &'static [&'static str],
}

/// Registry of all external applications
pub static APPS: &[ExternalApp] = &[
    ExternalApp {
        name: "coreutils",
        repo: "https://github.com/uutils/coreutils",
        package: "coreutils",
        binary: "coreutils",
        // Limited feature set - only utilities that work with current c-gull
        // Missing libc functions: getpwuid, getgrgid (ls), nl_langinfo (date)
        features: "cat,echo,head,mkdir,pwd,rm,tail,touch",
        required: true,
        symlinks: &["cat", "echo", "head", "mkdir", "pwd", "rm", "tail", "touch"],
    },
    ExternalApp {
        name: "brush",
        repo: "https://github.com/reubeno/brush",
        package: "brush",
        binary: "brush",
        features: "",
        required: false, // Shell works without brush
        symlinks: &[],
    },
];

impl ExternalApp {
    /// Get the clone directory for this app
    pub fn clone_dir(&self) -> PathBuf {
        PathBuf::from(format!("toolchain/{}", self.name))
    }

    /// Get the output directory for built binaries
    pub fn output_dir(&self, arch: &str) -> PathBuf {
        let target = linux_target(arch);
        PathBuf::from(format!("toolchain/{}-out/{}/release", self.name, target))
    }

    /// Get the path to the built binary
    pub fn output_path(&self, arch: &str) -> PathBuf {
        self.output_dir(arch).join(self.binary)
    }

    /// Check if the app has been built for the given architecture
    pub fn exists(&self, arch: &str) -> bool {
        self.output_path(arch).exists()
    }

    /// Clone the repository if not present (idempotent)
    pub fn clone_repo(&self) -> Result<()> {
        let dir = self.clone_dir();
        if dir.exists() {
            // Validate it's a git repo
            if !dir.join(".git").exists() {
                bail!(
                    "Directory {} exists but is not a git repository. \
                     Remove it and try again.",
                    dir.display()
                );
            }
            return Ok(());
        }

        println!("📥 Cloning {}...", self.name);
        let status = Command::new("git")
            .args(["clone", "--depth=1", self.repo, &dir.to_string_lossy()])
            .status()
            .with_context(|| format!("Failed to clone {}", self.name))?;

        if !status.success() {
            bail!("Failed to clone {} from {}", self.name, self.repo);
        }

        Ok(())
    }

    /// Build the app against our sysroot
    pub fn build(&self, arch: &str) -> Result<()> {
        // Ensure cloned
        self.clone_repo()?;

        // Ensure sysroot exists
        if !super::sysroot::sysroot_exists() {
            bail!(
                "Sysroot not found. Run 'cargo xtask build sysroot' first."
            );
        }

        let target = linux_target(arch);
        println!("🔧 Building {} for {}...", self.name, arch);

        let rustflags = get_sysroot_rustflags();

        let mut args = vec![
            "+nightly-2025-04-28".to_string(),
            "build".to_string(),
            "--release".to_string(),
            "--target".to_string(),
            target.to_string(),
            "-Z".to_string(),
            "build-std=std,panic_abort".to_string(),
            "-Z".to_string(),
            "build-std-features=panic_immediate_abort".to_string(),
            "-p".to_string(),
            self.package.to_string(),
        ];

        if !self.features.is_empty() {
            args.push("--no-default-features".to_string());
            args.push("--features".to_string());
            args.push(self.features.to_string());
        }

        let status = Command::new("cargo")
            .current_dir(self.clone_dir())
            .env_remove("RUSTUP_TOOLCHAIN")
            .env("RUSTFLAGS", &rustflags)
            .args(&args)
            .status()
            .with_context(|| format!("Failed to build {}", self.name))?;

        if !status.success() {
            bail!("Failed to build {}", self.name);
        }

        // Copy to output directory
        let src = self
            .clone_dir()
            .join("target")
            .join(target)
            .join("release")
            .join(self.binary);

        let out_dir = self.output_dir(arch);
        std::fs::create_dir_all(&out_dir)?;
        let dst = out_dir.join(self.binary);

        if src.exists() {
            std::fs::copy(&src, &dst)?;
            println!("📦 Built {}: {}", self.name, dst.display());
        } else {
            bail!("{} binary not found at {}", self.name, src.display());
        }

        println!("✅ {} ready", self.name);
        Ok(())
    }

    /// Ensure the app is built, or fail with a clear error
    pub fn require(&self, arch: &str) -> Result<PathBuf> {
        let path = self.output_path(arch);
        if !path.exists() {
            bail!(
                "{} not found at {}.\n\
                 Run 'cargo xtask build {}' first, or use 'cargo xtask build all' to build everything.",
                self.name,
                path.display(),
                self.name
            );
        }
        Ok(path)
    }

    /// Build if not already built (for build all/iso commands)
    pub fn ensure_built(&self, arch: &str) -> Result<()> {
        if !self.exists(arch) {
            self.build(arch)?;
        }
        Ok(())
    }
}

/// Get an app by name
pub fn get_app(name: &str) -> Option<&'static ExternalApp> {
    APPS.iter().find(|app| app.name == name)
}

/// Get all required apps
pub fn required_apps() -> impl Iterator<Item = &'static ExternalApp> {
    APPS.iter().filter(|app| app.required)
}

/// Get all optional apps
#[allow(dead_code)] // Available for future use
pub fn optional_apps() -> impl Iterator<Item = &'static ExternalApp> {
    APPS.iter().filter(|app| !app.required)
}

/// Build all apps that aren't already built (for build all/iso)
pub fn ensure_all_built(arch: &str) -> Result<()> {
    for app in APPS {
        app.ensure_built(arch)?;
    }
    Ok(())
}

/// Require all required apps to be built, fail fast if any missing
#[allow(dead_code)] // Available for future use
pub fn require_all(arch: &str) -> Result<()> {
    for app in required_apps() {
        app.require(arch)?;
    }
    Ok(())
}

/// Get RUSTFLAGS for building against our sysroot
fn get_sysroot_rustflags() -> String {
    let sysroot_path = std::env::current_dir()
        .map(|p| p.join("toolchain/sysroot"))
        .unwrap_or_else(|_| PathBuf::from("toolchain/sysroot"));

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

/// Convert architecture to Linux target triple
fn linux_target(arch: &str) -> &'static str {
    match arch {
        "x86_64" => "x86_64-unknown-linux-gnu",
        "aarch64" => "aarch64-unknown-linux-gnu",
        _ => "x86_64-unknown-linux-gnu", // fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_paths() {
        let app = get_app("coreutils").unwrap();
        assert_eq!(app.clone_dir(), PathBuf::from("toolchain/coreutils"));
        assert_eq!(
            app.output_path("x86_64"),
            PathBuf::from("toolchain/coreutils-out/x86_64-unknown-linux-gnu/release/coreutils")
        );
    }

    #[test]
    fn test_required_apps() {
        let required: Vec<_> = required_apps().collect();
        assert!(required.iter().any(|a| a.name == "coreutils"));
        assert!(!required.iter().any(|a| a.name == "brush"));
    }
}
