//! Build command definitions
//!
//! `TEAM_466`: Refactored from monolithic 1,372-line file.
//! Contains only CLI enum - implementation moved to specialized modules.

use clap::Subcommand;

// TEAM_459: Simplified - BusyBox is the only external app now
#[derive(Subcommand)]
pub enum BuildCommands {
    /// Build everything (Kernel + Userspace + Disk + `BusyBox`)
    All,
    /// Build kernel only
    Kernel,
    /// Build userspace only
    Userspace,
    /// Build initramfs only
    Initramfs,
    /// Build bootable Limine ISO
    Iso,
    /// Build `BusyBox` - provides init, shell, and 300+ utilities
    Busybox,
}
