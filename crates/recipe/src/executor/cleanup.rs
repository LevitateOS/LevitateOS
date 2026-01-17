//! Cleanup phase - removes build artifacts to save space.

use crate::{CleanupSpec, CleanupTarget};

use super::context::Context;
use super::error::ExecuteError;

/// Execute the cleanup phase - remove build artifacts to save space.
pub fn cleanup(ctx: &Context, spec: &CleanupSpec) -> Result<(), ExecuteError> {
    if ctx.verbose || ctx.dry_run {
        eprintln!(
            "[{}] cleanup: {:?} (keep: {:?})",
            if ctx.dry_run { "dry-run" } else { "exec" },
            spec.target,
            spec.keep
        );
    }

    if ctx.dry_run {
        return Ok(());
    }

    match spec.target {
        CleanupTarget::All => {
            // Remove entire build directory, preserving 'keep' paths
            if spec.keep.is_empty() {
                std::fs::remove_dir_all(&ctx.build_dir)?;
            } else {
                cleanup_with_keep(ctx, &spec.keep)?;
            }
        }
        CleanupTarget::Downloads => {
            // Remove only archive files
            cleanup_by_extension(
                ctx,
                &[
                    ".tar.gz", ".tgz", ".tar.xz", ".txz", ".tar.bz2", ".tbz2", ".tar", ".zip",
                ],
            )?;
        }
        CleanupTarget::Sources => {
            // Remove extracted directories but keep archives
            cleanup_directories(ctx)?;
        }
        CleanupTarget::Artifacts => {
            // Remove build artifacts (target/, build/, *.o, etc.) but keep sources
            cleanup_build_artifacts(ctx)?;
        }
    }

    Ok(())
}

/// Remove all files except those in the keep list.
fn cleanup_with_keep(ctx: &Context, keep: &[String]) -> Result<(), ExecuteError> {
    let entries = std::fs::read_dir(&ctx.build_dir)?;

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if !keep.iter().any(|k| name == *k || name.starts_with(k)) {
            let path = entry.path();
            if path.is_dir() {
                std::fs::remove_dir_all(&path)?;
            } else {
                std::fs::remove_file(&path)?;
            }
        }
    }

    Ok(())
}

/// Remove files matching certain extensions.
fn cleanup_by_extension(ctx: &Context, extensions: &[&str]) -> Result<(), ExecuteError> {
    let entries = std::fs::read_dir(&ctx.build_dir)?;

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if extensions.iter().any(|ext| name.ends_with(ext)) {
            std::fs::remove_file(entry.path())?;
        }
    }

    Ok(())
}

/// Remove directories only (keep archive files).
fn cleanup_directories(ctx: &Context) -> Result<(), ExecuteError> {
    let entries = std::fs::read_dir(&ctx.build_dir)?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            std::fs::remove_dir_all(&path)?;
        }
    }

    Ok(())
}

/// Remove common build artifact directories.
fn cleanup_build_artifacts(ctx: &Context) -> Result<(), ExecuteError> {
    let artifact_dirs = ["target", "build", "_build", "out", "dist", ".cache"];
    let artifact_exts = [".o", ".a", ".so", ".dylib", ".rlib", ".rmeta"];

    let entries = std::fs::read_dir(&ctx.build_dir)?;

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if path.is_dir() && artifact_dirs.contains(&name.as_str()) {
            std::fs::remove_dir_all(&path)?;
        } else if path.is_file() && artifact_exts.iter().any(|ext| name.ends_with(ext)) {
            std::fs::remove_file(&path)?;
        }
    }

    Ok(())
}
