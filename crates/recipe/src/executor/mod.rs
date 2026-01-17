//! Recipe executor - runs parsed recipes to acquire, build, and install packages.

mod acquire;
mod build;
mod cleanup;
mod configure;
mod context;
mod error;
mod install;
mod patch;
mod remove;
mod service;
mod util;

pub use context::Context;
pub use error::ExecuteError;

use std::collections::HashSet;
use std::path::Path;

use crate::features::FeatureSet;
use crate::Recipe;

/// Recipe executor that runs acquire, build, install, and other actions.
pub struct Executor {
    ctx: Context,
    /// Enabled features for this execution
    enabled_features: HashSet<String>,
    /// Directory containing the recipe file (for relative patch paths)
    recipe_dir: Option<std::path::PathBuf>,
}

impl Executor {
    /// Create a new executor with the given context.
    pub fn new(ctx: Context) -> Self {
        Self {
            ctx,
            enabled_features: HashSet::new(),
            recipe_dir: None,
        }
    }

    /// Set the directory containing the recipe file.
    pub fn with_recipe_dir(mut self, dir: impl AsRef<Path>) -> Self {
        self.recipe_dir = Some(dir.as_ref().to_path_buf());
        self
    }

    /// Set enabled features for this execution.
    pub fn with_features(mut self, features: HashSet<String>) -> Self {
        self.enabled_features = features;
        self
    }

    /// Resolve and set features based on defaults and user overrides.
    pub fn resolve_features(
        mut self,
        feature_set: &FeatureSet,
        additions: &[String],
        removals: &[String],
    ) -> Result<Self, ExecuteError> {
        self.enabled_features = feature_set
            .resolve_with_defaults(additions, removals)
            .map_err(|e| ExecuteError::FeatureError(e.to_string()))?;
        Ok(self)
    }

    /// Get the currently enabled features.
    pub fn enabled_features(&self) -> &HashSet<String> {
        &self.enabled_features
    }

    /// Execute a complete recipe.
    pub fn execute(&self, recipe: &Recipe) -> Result<(), ExecuteError> {
        // Ensure build directory exists
        if !self.ctx.dry_run {
            std::fs::create_dir_all(&self.ctx.build_dir)?;
        }

        // Execute each phase in order
        if let Some(ref spec) = recipe.acquire {
            self.acquire(spec)?;
        }

        // Apply patches after acquire, before build
        if let Some(ref spec) = recipe.patches {
            self.apply_patches(spec)?;
        }

        if let Some(ref spec) = recipe.build {
            self.build(spec)?;
        }

        if let Some(ref spec) = recipe.install {
            self.install(spec)?;
        }

        if let Some(ref spec) = recipe.configure {
            self.configure(spec)?;
        }

        // Cleanup build artifacts if specified
        if let Some(ref spec) = recipe.cleanup {
            self.cleanup(spec)?;
        }

        Ok(())
    }

    /// Apply patches to source code.
    pub fn apply_patches(&self, spec: &crate::PatchSpec) -> Result<(), ExecuteError> {
        let recipe_dir = self.recipe_dir.as_deref().unwrap_or(Path::new("."));
        patch::apply_patches(&self.ctx, spec, recipe_dir)
    }

    /// Execute the acquire phase.
    pub fn acquire(&self, spec: &crate::AcquireSpec) -> Result<(), ExecuteError> {
        acquire::acquire(&self.ctx, spec)
    }

    /// Execute the build phase.
    pub fn build(&self, spec: &crate::BuildSpec) -> Result<(), ExecuteError> {
        build::build(&self.ctx, spec)
    }

    /// Execute the install phase.
    pub fn install(&self, spec: &crate::InstallSpec) -> Result<(), ExecuteError> {
        install::install(&self.ctx, spec)
    }

    /// Execute the configure phase.
    pub fn configure(&self, spec: &crate::ConfigureSpec) -> Result<(), ExecuteError> {
        configure::configure(&self.ctx, spec)
    }

    /// Execute the start action.
    pub fn start(&self, spec: &crate::StartSpec) -> Result<(), ExecuteError> {
        service::start(&self.ctx, spec)
    }

    /// Execute the stop action.
    pub fn stop(&self, spec: &crate::StopSpec) -> Result<(), ExecuteError> {
        service::stop(&self.ctx, spec)
    }

    /// Execute the remove action.
    pub fn remove(&self, spec: &crate::RemoveSpec, recipe: &Recipe) -> Result<(), ExecuteError> {
        remove::remove(&self.ctx, spec, recipe)
    }

    /// Execute the cleanup phase - remove build artifacts to save space.
    pub fn cleanup(&self, spec: &crate::CleanupSpec) -> Result<(), ExecuteError> {
        cleanup::cleanup(&self.ctx, spec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_expand_vars() {
        let ctx = Context {
            prefix: PathBuf::from("/opt/myapp"),
            build_dir: PathBuf::from("/tmp/build"),
            arch: "x86_64".to_string(),
            nproc: 4,
            dry_run: false,
            verbose: false,
        };

        assert_eq!(
            util::expand_vars(&ctx, "--prefix=$PREFIX"),
            "--prefix=/opt/myapp"
        );
        assert_eq!(util::expand_vars(&ctx, "make -j$NPROC"), "make -j4");
        assert_eq!(util::expand_vars(&ctx, "arch is $ARCH"), "arch is x86_64");
    }

    #[test]
    fn test_url_filename() {
        assert_eq!(
            util::url_filename("https://example.com/ripgrep-14.1.0.tar.gz"),
            "ripgrep-14.1.0.tar.gz"
        );
        assert_eq!(
            util::url_filename("https://example.com/file.zip?token=abc"),
            "file.zip"
        );
    }

    #[test]
    fn test_shell_quote() {
        assert_eq!(util::shell_quote("simple"), "simple");
        assert_eq!(util::shell_quote("/path/to/file"), "/path/to/file");
        assert_eq!(util::shell_quote("has space"), "'has space'");
        assert_eq!(util::shell_quote("has'quote"), "'has'\"'\"'quote'");
    }

    #[test]
    fn test_context_default() {
        let ctx = Context::default();
        assert_eq!(ctx.prefix, PathBuf::from("/usr/local"));
        assert!(!ctx.dry_run);
        assert!(!ctx.verbose);
    }

    #[test]
    fn test_context_builder() {
        let ctx = Context::with_prefix("/opt/app")
            .arch("aarch64")
            .dry_run(true)
            .verbose(true);

        assert_eq!(ctx.prefix, PathBuf::from("/opt/app"));
        assert_eq!(ctx.arch, "aarch64");
        assert!(ctx.dry_run);
        assert!(ctx.verbose);
    }
}
