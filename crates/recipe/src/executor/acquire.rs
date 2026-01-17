//! Acquire phase - downloads sources, binaries, or clones git repositories.

use std::path::Path;

use crate::{AcquireSpec, GitRef, Verify};

use super::context::Context;
use super::error::ExecuteError;
use super::util::{run_cmd, shell_quote, url_filename};

/// Execute the acquire phase.
pub fn acquire(ctx: &Context, spec: &AcquireSpec) -> Result<(), ExecuteError> {
    match spec {
        AcquireSpec::Source { url, verify } => {
            acquire_source(ctx, url, verify.as_ref())?;
        }
        AcquireSpec::Binary { urls } => {
            acquire_binary(ctx, urls)?;
        }
        AcquireSpec::Git { url, reference } => {
            acquire_git(ctx, url, reference.as_ref())?;
        }
        AcquireSpec::OsPackage { packages: _ } => {
            // Skip OS package installation - out of scope
        }
    }
    Ok(())
}

fn acquire_source(ctx: &Context, url: &str, verify: Option<&Verify>) -> Result<(), ExecuteError> {
    let filename = url_filename(url);
    let dest = ctx.build_dir.join(&filename);

    // Download the file
    let cmd = format!(
        "curl -fsSL -o {} {}",
        shell_quote(dest.display()),
        shell_quote(url)
    );
    run_cmd(ctx, &cmd)?;

    // Verify checksum if provided
    if let Some(verify) = verify {
        verify_checksum(ctx, &dest, verify)?;
    }

    Ok(())
}

fn acquire_binary(ctx: &Context, urls: &[(String, String)]) -> Result<(), ExecuteError> {
    // Find URL for current architecture
    let url = urls
        .iter()
        .find(|(arch, _)| arch == &ctx.arch)
        .map(|(_, url)| url)
        .ok_or_else(|| ExecuteError::NoUrlForArch(ctx.arch.clone()))?;

    let filename = url_filename(url);
    let dest = ctx.build_dir.join(&filename);

    let cmd = format!(
        "curl -fsSL -o {} {}",
        shell_quote(dest.display()),
        shell_quote(url)
    );
    run_cmd(ctx, &cmd)?;

    Ok(())
}

fn acquire_git(ctx: &Context, url: &str, reference: Option<&GitRef>) -> Result<(), ExecuteError> {
    let repo_name = url
        .rsplit('/')
        .next()
        .unwrap_or("repo")
        .trim_end_matches(".git");
    let dest = ctx.build_dir.join(repo_name);

    // Clone with depth=1 for faster checkout
    let mut cmd = format!(
        "git clone --depth 1 {} {}",
        shell_quote(url),
        shell_quote(dest.display())
    );

    // Add branch/tag if specified
    if let Some(git_ref) = reference {
        let ref_arg = match git_ref {
            GitRef::Tag(t) | GitRef::Branch(t) => format!(" --branch {}", shell_quote(t)),
            GitRef::Commit(_) => String::new(), // Will checkout after clone
        };
        cmd = format!(
            "git clone --depth 1{} {} {}",
            ref_arg,
            shell_quote(url),
            shell_quote(dest.display())
        );
    }

    run_cmd(ctx, &cmd)?;

    // Checkout specific commit if needed
    if let Some(GitRef::Commit(commit)) = reference {
        let checkout_cmd = format!(
            "cd {} && git fetch --depth 1 origin {} && git checkout {}",
            shell_quote(dest.display()),
            shell_quote(commit),
            shell_quote(commit)
        );
        run_cmd(ctx, &checkout_cmd)?;
    }

    Ok(())
}

fn verify_checksum(ctx: &Context, path: &Path, verify: &Verify) -> Result<(), ExecuteError> {
    match verify {
        Verify::Sha256(expected) => {
            let cmd = format!("sha256sum {}", shell_quote(path.display()));
            let output = run_cmd(ctx, &cmd)?;
            let stdout = String::from_utf8_lossy(&output.stdout);
            let actual = stdout.split_whitespace().next().unwrap_or("");

            if actual != expected {
                return Err(ExecuteError::Sha256Mismatch {
                    expected: expected.clone(),
                    actual: actual.to_string(),
                });
            }
        }
        Verify::Sha256Url(checksum_url) => {
            // Download checksum file and verify
            let checksum_file = ctx.build_dir.join("checksum.sha256");
            let cmd = format!(
                "curl -fsSL -o {} {}",
                shell_quote(checksum_file.display()),
                shell_quote(checksum_url)
            );
            run_cmd(ctx, &cmd)?;

            // Verify using the checksum file
            let filename = path.file_name().unwrap_or_default().to_string_lossy();
            let cmd = format!(
                "cd {} && grep {} checksum.sha256 | sha256sum -c -",
                shell_quote(ctx.build_dir.display()),
                shell_quote(&filename)
            );
            run_cmd(ctx, &cmd)?;
        }
    }
    Ok(())
}
