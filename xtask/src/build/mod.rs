//! Build commands module
//!
//! TEAM_322: Organized into submodule
//! TEAM_435: Added sysroot module (replaces Eyra)
//! TEAM_438: Added apps registry for uniform external app handling (replaces external module)

mod commands;
pub mod apps;
pub mod sysroot;

pub use commands::*;
