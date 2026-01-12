//! External C application build support using musl
//!
//! TEAM_444: C program support via musl-gcc.
//!
//! Similar to apps.rs but for C programs like dash.
//! Uses musl-gcc for static linking.

use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use std::process::Command;

/// An external C application that can be built with musl
#[derive(Debug, Clone)]
pub struct ExternalCApp {
    /// Short name (e.g., "dash")
    pub name: &'static str,
    /// Git repository URL
    pub repo: &'static str,
    /// Path to binary within the build directory
    pub binary: &'static str,
    /// Arguments for ./configure
    pub configure_args: &'static [&'static str],
    /// Whether autoreconf is needed before configure
    pub needs_autoreconf: bool,
    /// Whether this app is required for a complete initramfs
    pub required: bool,
}

/// Registry of all external C applications
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
        required: false, // Optional - simpler shell for debugging
    },
];

impl ExternalCApp {
    /// Get the clone directory for this app
    pub fn clone_dir(&self) -> PathBuf {
        PathBuf::from(format!("toolchain/{}", self.name))
    }

    /// Get the output directory for built binaries
    pub fn output_dir(&self, arch: &str) -> PathBuf {
        PathBuf::from(format!("toolchain/{}-out/{}", self.name, arch))
    }

    /// Get the path to the built binary
    pub fn output_path(&self, arch: &str) -> PathBuf {
        // Extract just the filename from the binary path
        let binary_name = PathBuf::from(self.binary)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| self.binary.to_string());
        self.output_dir(arch).join(binary_name)
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

    /// Build the app with musl-gcc
    pub fn build(&self, arch: &str) -> Result<()> {
        // Ensure cloned
        self.clone_repo()?;

        // Ensure musl-gcc is available
        ensure_musl_gcc()?;

        let clone_dir = self.clone_dir();

        // Run autoreconf if needed
        if self.needs_autoreconf {
            println!("🔧 Running autoreconf for {}...", self.name);
            let status = Command::new("autoreconf")
                .current_dir(&clone_dir)
                .arg("-fi")
                .status()
                .context("autoreconf failed - install autoconf and automake")?;

            if !status.success() {
                bail!(
                    "autoreconf failed for {}.\n\
                     Install autoconf and automake:\n\
                     Fedora: sudo dnf install autoconf automake\n\
                     Ubuntu: sudo apt install autoconf automake",
                    self.name
                );
            }
        }

        // Clean any previous build
        let _ = Command::new("make")
            .current_dir(&clone_dir)
            .arg("clean")
            .status();

        // Configure with musl-gcc
        println!("🔧 Configuring {} with musl...", self.name);
        let mut configure = Command::new("./configure");
        configure
            .current_dir(&clone_dir)
            .env("CC", "musl-gcc")
            .env("CFLAGS", "-static -Os");

        for arg in self.configure_args {
            configure.arg(arg);
        }

        let status = configure
            .status()
            .context("./configure failed")?;

        if !status.success() {
            bail!("configure failed for {}", self.name);
        }

        // Build
        println!("🔧 Building {}...", self.name);
        let status = Command::new("make")
            .current_dir(&clone_dir)
            .args(["-j4"])
            .status()
            .context("make failed")?;

        if !status.success() {
            bail!("make failed for {}", self.name);
        }

        // Verify it's statically linked
        let built_binary = clone_dir.join(self.binary);
        if !built_binary.exists() {
            bail!(
                "{} binary not found at {}",
                self.name,
                built_binary.display()
            );
        }

        let file_output = Command::new("file")
            .arg(&built_binary)
            .output()
            .context("Failed to run file command")?;

        let file_info = String::from_utf8_lossy(&file_output.stdout);
        if !file_info.contains("statically linked") && !file_info.contains("static-pie") {
            println!("⚠️  Warning: {} may not be statically linked", self.name);
            println!("    file output: {}", file_info.trim());
        }

        // Copy to output directory
        let out_dir = self.output_dir(arch);
        std::fs::create_dir_all(&out_dir)?;
        let dst = self.output_path(arch);
        std::fs::copy(&built_binary, &dst)?;

        println!("📦 Built {}: {}", self.name, dst.display());
        println!("✅ {} ready", self.name);
        Ok(())
    }

    /// Ensure the app is built, or fail with a clear error
    pub fn require(&self, arch: &str) -> Result<PathBuf> {
        let path = self.output_path(arch);
        if !path.exists() {
            bail!(
                "{} not found at {}.\n\
                 Run 'cargo xtask build {}' first.",
                self.name,
                path.display(),
                self.name
            );
        }
        Ok(path)
    }

    /// Build if not already built
    pub fn ensure_built(&self, arch: &str) -> Result<()> {
        if !self.exists(arch) {
            self.build(arch)?;
        }
        Ok(())
    }
}

/// Get a C app by name
pub fn get_c_app(name: &str) -> Option<&'static ExternalCApp> {
    C_APPS.iter().find(|app| app.name == name)
}

/// Get all required C apps
#[allow(dead_code)]
pub fn required_c_apps() -> impl Iterator<Item = &'static ExternalCApp> {
    C_APPS.iter().filter(|app| app.required)
}

/// Check if musl-gcc is available
fn ensure_musl_gcc() -> Result<()> {
    let output = Command::new("musl-gcc")
        .arg("--version")
        .output();

    if output.is_err() || !output.unwrap().status.success() {
        bail!(
            "musl-gcc not found.\n\n\
             Install musl development tools:\n\
             Fedora: sudo dnf install musl-gcc musl-devel\n\
             Ubuntu: sudo apt install musl-tools musl-dev\n\
             Arch:   sudo pacman -S musl"
        );
    }

    Ok(())
}

/// Check if musl-gcc is available (non-failing version for optional builds)
pub fn musl_gcc_available() -> bool {
    Command::new("musl-gcc")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_app_paths() {
        let app = get_c_app("dash").unwrap();
        assert_eq!(app.clone_dir(), PathBuf::from("toolchain/dash"));
        assert_eq!(
            app.output_path("x86_64"),
            PathBuf::from("toolchain/dash-out/x86_64/dash")
        );
    }
}
