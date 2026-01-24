//! Artifact builders for distribution images.
//!
//! This module provides wrappers for building:
//! - Squashfs images (mksquashfs)
//! - Initramfs archives (cpio + gzip)
//! - ISO images (xorriso)
//!
//! # Status: SKELETON
//!
//! These are placeholder modules. The actual implementations
//! remain in leviso until they can be properly abstracted
//! and tested with both LevitateOS and AcornOS.

pub mod initramfs;
pub mod iso;
pub mod squashfs;
