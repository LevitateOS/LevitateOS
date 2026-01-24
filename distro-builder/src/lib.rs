//! Shared infrastructure for building Linux distribution ISOs.
//!
//! This crate provides common abstractions used by both leviso (LevitateOS)
//! and AcornOS builders. It extracts the distro-agnostic parts:
//!
//! - Component system (traits, generic operations)
//! - Artifact builders (squashfs, initramfs, ISO wrappers)
//! - Build utilities (filesystem operations, context)
//! - Preflight checks (host tool validation)
//!
//! # Architecture
//!
//! ```text
//! distro-builder (this crate)
//!     │
//!     ├── Defines: Installable trait, generic Op variants
//!     ├── Defines: BuildContext trait, DistroConfig trait
//!     └── Provides: Filesystem utilities, squashfs/ISO wrappers
//!
//! leviso ─────────────────────┐
//!     │                       │
//!     ├── Uses: distro-builder│
//!     ├── Implements: LevitateOS-specific components
//!     └── Uses: distro-spec::levitate
//!
//! AcornOS ────────────────────┤
//!     │                       │
//!     ├── Uses: distro-builder│
//!     ├── Implements: AcornOS-specific components
//!     └── Uses: distro-spec::acorn
//! ```
//!
//! # Status: SKELETON
//!
//! This crate is currently a structural skeleton. The abstractions are defined
//! but not all functionality is extracted from leviso yet. Full extraction
//! requires testing with both LevitateOS and AcornOS builds.

pub mod artifact;
pub mod build;
pub mod component;
pub mod preflight;

pub use build::context::{BuildContext, DistroConfig};
pub use component::{Installable, Op, Phase};
