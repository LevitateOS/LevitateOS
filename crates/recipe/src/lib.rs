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

mod ast;
mod executor;
mod parser;
mod recipe;

pub use ast::Expr;
pub use executor::{Context, ExecuteError, Executor};
pub use parser::{parse, ParseError};
pub use recipe::{
    AcquireSpec, BuildSpec, BuildStep, CleanupSpec, CleanupTarget, ConfigureSpec, ConfigureStep,
    GitRef, InstallFile, InstallSpec, Recipe, RecipeError, RemoveSpec, RemoveStep, SandboxConfig,
    StartSpec, StopSpec, Verify,
};
