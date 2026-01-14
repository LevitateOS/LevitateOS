//! Component registry - single source of truth for all buildable components.
//!
//! All components implement the [`Buildable`] trait and are registered here.

use super::{
    brush::Brush, diffutils::Diffutils, findutils::Findutils, helix::Helix,
    iproute2::Iproute2, iputils::Iputils, linux::Linux, procps::Procps,
    sudo_rs::SudoRs, systemd::Systemd, util_linux::UtilLinux, uutils::Uutils,
    Buildable,
};

/// All registered components.
///
/// Order matters for `build_all` - dependencies should come first.
pub static COMPONENTS: &[&dyn Buildable] = &[
    &Linux,
    &Systemd,
    &UtilLinux,
    &Uutils,
    &Findutils,
    &Diffutils,
    &SudoRs,
    &Brush,
    &Helix,
    &Procps,
    &Iproute2,
    &Iputils,
];

/// Get component by name.
#[must_use]
pub fn get(name: &str) -> Option<&'static dyn Buildable> {
    COMPONENTS.iter().find(|c| c.name() == name).copied()
}

/// List all component names.
pub fn names() -> impl Iterator<Item = &'static str> {
    COMPONENTS.iter().map(|c| c.name())
}
