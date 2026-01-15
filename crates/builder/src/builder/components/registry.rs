//! Component registry - single source of truth for all buildable components.
//!
//! All components implement the [`Buildable`] trait and are registered here.

use super::{linux::Linux, Buildable};

/// All registered components.
///
/// Order matters for `build_all` - dependencies should come first.
/// Note: Most userspace comes from Fedora ISO now, only kernel is built from source.
pub static COMPONENTS: &[&dyn Buildable] = &[&Linux];

/// Get component by name.
#[must_use]
pub fn get(name: &str) -> Option<&'static dyn Buildable> {
    COMPONENTS.iter().find(|c| c.name() == name).copied()
}

/// List all component names.
pub fn names() -> impl Iterator<Item = &'static str> {
    COMPONENTS.iter().map(|c| c.name())
}
