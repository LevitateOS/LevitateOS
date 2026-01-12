//! Build commands module
//!
//! TEAM_322: Organized into submodule
//! TEAM_435: Added sysroot module (replaces Eyra)
//! TEAM_438: Added apps registry for uniform external app handling (replaces external module)
//! TEAM_444: Added c_apps for C program support, migrated to musl
//! TEAM_451: Added busybox module (replaces coreutils + dash + custom init)

mod commands;
pub mod apps;
pub mod busybox;
pub mod c_apps;
pub mod sysroot;

pub use commands::*;
