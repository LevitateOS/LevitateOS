//! Patch executor - applies patches to source code after acquisition.

use std::path::Path;
use std::process::Command;

use crate::recipe::{PatchSource, PatchSpec, Verify};

use super::context::Context;
use super::error::ExecuteError;
use super::util;

/// Apply patches to the source code.
pub fn apply_patches(ctx: &Context, spec: &PatchSpec, recipe_dir: &Path) -> Result<(), ExecuteError> {
    if ctx.verbose {
        println!("[patch] Applying {} patches with strip level {}", spec.patches.len(), spec.strip);
    }

    for (idx, patch) in spec.patches.iter().enumerate() {
        let patch_path = match patch {
            PatchSource::Local(path) => {
                // Local patches are relative to the recipe file
                let full_path = recipe_dir.join(path);
                if !full_path.exists() {
                    return Err(ExecuteError::Io(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!("patch not found: {}", full_path.display()),
                    )));
                }
                full_path
            }
            PatchSource::Remote { url, verify } => {
                // Download remote patch to build directory
                let filename = util::url_filename(url);
                let dest = ctx.build_dir.join(&filename);

                if ctx.verbose {
                    println!("[patch] Downloading patch from {}", url);
                }

                if !ctx.dry_run {
                    download_patch(url, &dest)?;

                    // Verify checksum if specified
                    if let Some(Verify::Sha256(expected)) = verify {
                        verify_sha256(&dest, expected)?;
                    }
                }

                dest
            }
        };

        if ctx.verbose {
            println!("[patch] Applying patch {}: {}", idx + 1, patch_path.display());
        }

        if !ctx.dry_run {
            apply_patch(ctx, &patch_path, spec.strip)?;
        }
    }

    Ok(())
}

/// Download a patch file from a URL.
fn download_patch(url: &str, dest: &Path) -> Result<(), ExecuteError> {
    // Try curl first, fall back to wget
    let status = Command::new("curl")
        .args(["-fsSL", "-o"])
        .arg(dest)
        .arg(url)
        .status();

    match status {
        Ok(s) if s.success() => return Ok(()),
        _ => {}
    }

    // Try wget as fallback
    let status = Command::new("wget")
        .args(["-q", "-O"])
        .arg(dest)
        .arg(url)
        .status()
        .map_err(|e| ExecuteError::Io(e))?;

    if !status.success() {
        return Err(ExecuteError::CommandFailed {
            cmd: format!("wget {}", url),
            code: status.code(),
        });
    }

    Ok(())
}

/// Verify SHA256 checksum of a file.
fn verify_sha256(path: &Path, expected: &str) -> Result<(), ExecuteError> {
    use std::io::Read;
    use sha2::{Sha256, Digest};

    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let result = hasher.finalize();
    let actual = format!("{:x}", result);

    if actual != expected.to_lowercase() {
        return Err(ExecuteError::ChecksumMismatch {
            expected: expected.to_string(),
            actual,
        });
    }

    Ok(())
}

/// Apply a single patch file using the `patch` command.
fn apply_patch(ctx: &Context, patch_path: &Path, strip: u32) -> Result<(), ExecuteError> {
    let status = Command::new("patch")
        .current_dir(&ctx.build_dir)
        .arg(format!("-p{}", strip))
        .arg("-i")
        .arg(patch_path)
        .status()
        .map_err(|e| ExecuteError::Io(e))?;

    if !status.success() {
        return Err(ExecuteError::CommandFailed {
            cmd: format!("patch -p{} -i {}", strip, patch_path.display()),
            code: status.code(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_dry_run_does_not_apply() {
        let temp = TempDir::new().unwrap();
        let ctx = Context {
            prefix: PathBuf::from("/opt/test"),
            build_dir: temp.path().to_path_buf(),
            arch: "x86_64".to_string(),
            nproc: 4,
            dry_run: true,
            verbose: true,
        };

        let spec = PatchSpec {
            patches: vec![PatchSource::Local(PathBuf::from("nonexistent.patch"))],
            strip: 1,
        };

        // In dry-run mode, local patches should still be checked for existence
        // But remote patches would not be downloaded
        let result = apply_patches(&ctx, &spec, temp.path());
        // Should fail because local patch doesn't exist (even in dry-run)
        assert!(result.is_err());
    }
}
