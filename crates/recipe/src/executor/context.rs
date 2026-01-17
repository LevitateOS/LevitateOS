//! Execution context providing configuration for recipe execution.

use std::path::PathBuf;

/// Execution context providing configuration for recipe execution.
#[derive(Debug, Clone)]
pub struct Context {
    /// Installation prefix (default: /usr/local)
    pub prefix: PathBuf,
    /// Temporary build directory
    pub build_dir: PathBuf,
    /// Target architecture (e.g., "x86_64", "aarch64")
    pub arch: String,
    /// Number of parallel jobs for builds
    pub nproc: usize,
    /// If true, log commands without executing them
    pub dry_run: bool,
    /// If true, print commands as they execute
    pub verbose: bool,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            prefix: PathBuf::from("/usr/local"),
            build_dir: std::env::temp_dir().join("levitate-build"),
            arch: std::env::consts::ARCH.to_string(),
            nproc: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1),
            dry_run: false,
            verbose: false,
        }
    }
}

impl Context {
    /// Create a new context with the given prefix.
    pub fn with_prefix(prefix: impl Into<PathBuf>) -> Self {
        Self {
            prefix: prefix.into(),
            ..Default::default()
        }
    }

    /// Set the build directory.
    pub fn build_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.build_dir = dir.into();
        self
    }

    /// Set the target architecture.
    pub fn arch(mut self, arch: impl Into<String>) -> Self {
        self.arch = arch.into();
        self
    }

    /// Set dry run mode.
    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// Set verbose mode.
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
}
