//! Build command definitions
//!
//! `TEAM_466`: Refactored from monolithic 1,372-line file.
//! `TEAM_475`: Linux kernel is default; custom kernel is opt-in.
//! Contains only CLI enum - implementation moved to specialized modules.

use clap::Subcommand;

#[derive(Subcommand)]
pub enum BuildCommands {
    /// Build everything (custom kernel + userspace) - for --custom-kernel mode
    All,
    /// Build custom LevitateOS kernel only
    Kernel,
    /// Build userspace only
    Userspace,
    /// Build BusyBox initramfs (default init system)
    Initramfs,
    /// Build bootable Limine ISO (custom kernel)
    Iso,
    /// Build BusyBox - provides shell and 300+ utilities
    Busybox,
    /// Build/update Linux kernel from submodule
    Linux,
    /// Build OpenRC init system from source
    Openrc,
    /// Build OpenRC-based initramfs (BusyBox + OpenRC)
    OpenrcInitramfs,
}
