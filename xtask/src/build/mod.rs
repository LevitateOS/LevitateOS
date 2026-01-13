//! Build commands module
//!
//! `TEAM_322`: Organized into build submodule
//! `TEAM_435`: Added sysroot module (replaces Eyra)
//! `TEAM_438`: Added apps registry for uniform external app handling (replaces external module)
//! `TEAM_444`: Added `c_apps` for C program support, migrated to musl
//! `TEAM_451`: Added busybox module (replaces coreutils + dash + custom init)
//! `TEAM_466`: Refactored commands.rs (1,372 lines) into focused modules:
//!   - commands.rs: CLI enum only
//!   - orchestration.rs: `build_all`, `build_kernel_only`, `build_kernel_verbose`
//!   - kernel.rs: `build_kernel_with_features`
//!   - userspace.rs: `build_userspace`
//!   - initramfs.rs: all initramfs creation functions
//!   - iso.rs: ISO build + Limine

mod commands;
mod initramfs;
mod iso;
mod kernel;
mod orchestration;
mod userspace;

pub mod apps;
pub mod busybox;
pub mod c_apps;
pub mod sysroot;

// Re-export public API (maintains backward compatibility)
pub use commands::BuildCommands;
pub use initramfs::create_busybox_initramfs;
pub use iso::{build_iso, build_iso_test, build_iso_verbose};
pub use orchestration::{build_all, build_kernel_only, build_kernel_verbose};
pub use userspace::build_userspace;
