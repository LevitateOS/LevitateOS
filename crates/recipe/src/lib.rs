//! S-expression package recipe parser for LevitateOS.
//!
//! This crate parses package recipes in S-expression format:
//!
//! ```text
//! (package "ripgrep" "14.1.0"
//!   (description "Fast grep alternative written in Rust")
//!   (license "MIT")
//!   (deps)
//!   (build (extract tar-gz))
//!   (install (to-bin "rg")))
//! ```
//!
//! # Enhanced Features
//!
//! The recipe format supports:
//! - **Version constraints**: `(deps "openssl >= 1.1.0" "zlib")`
//! - **Features/variants**: `(features (default "x264") (x264 "H.264 support"))`
//! - **Patches**: `(patches "fix.patch" (url "https://..." (sha256 "...")))`
//! - **Provides/conflicts**: `(provides "editor") (conflicts "vim")`
//! - **Split packages**: `(subpackages (openssl-dev ...))`
//!
//! # Example
//!
//! ```
//! use levitate_recipe::{parse, Recipe};
//!
//! let input = r#"(package "hello" "1.0.0" (deps))"#;
//! let expr = parse(input).unwrap();
//! let recipe = Recipe::from_expr(&expr).unwrap();
//! assert_eq!(recipe.name, "hello");
//! ```
//!
//! # Example with features and version constraints
//!
//! ```
//! use levitate_recipe::{parse, Recipe};
//!
//! let input = r#"
//!     (package "myapp" "1.0"
//!       (features
//!         (default "ssl")
//!         (ssl "Enable SSL support"))
//!       (deps
//!         "zlib"
//!         (if ssl "openssl >= 1.1.0")))
//! "#;
//! let expr = parse(input).unwrap();
//! let recipe = Recipe::from_expr(&expr).unwrap();
//! assert!(recipe.features.is_some());
//! ```

mod ast;
mod executor;
pub mod features;
mod parser;
mod recipe;
pub mod version;

pub use ast::Expr;
pub use executor::{Context, ExecuteError, Executor};
pub use features::{expand_feature_conditionals, DepSpec, Feature, FeatureError, FeatureSet};
pub use parser::{parse, ParseError};
pub use recipe::{
    AcquireSpec, BuildSpec, BuildStep, CleanupSpec, CleanupTarget, ConfigureSpec, ConfigureStep,
    GitRef, InstallFile, InstallSpec, PatchSource, PatchSpec, Recipe, RecipeError, RemoveSpec,
    RemoveStep, SandboxConfig, Shell, StartSpec, StopSpec, Subpackage, Verify,
};
pub use version::{Dependency, Version, VersionConstraint, VersionError};
