//! Utility functions for the executor.

use std::process::{Command, Output};

use super::context::Context;
use super::error::ExecuteError;

/// Extract filename from a URL.
pub fn url_filename(url: &str) -> String {
    url.rsplit('/')
        .next()
        .unwrap_or("download")
        .split('?')
        .next()
        .unwrap_or("download")
        .to_string()
}

/// Shell-quote a value for safe interpolation.
pub fn shell_quote(s: impl std::fmt::Display) -> String {
    let s = s.to_string();
    if s.chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.' || c == '/')
    {
        s
    } else {
        format!("'{}'", s.replace('\'', "'\"'\"'"))
    }
}

/// Expand variables in a string.
pub fn expand_vars(ctx: &Context, s: &str) -> String {
    s.replace("$PREFIX", &ctx.prefix.display().to_string())
        .replace("$NPROC", &ctx.nproc.to_string())
        .replace("$ARCH", &ctx.arch)
        .replace("$BUILD_DIR", &ctx.build_dir.display().to_string())
}

/// Run a shell command with variable expansion.
pub fn run_cmd(ctx: &Context, cmd: &str) -> Result<Output, ExecuteError> {
    let expanded = expand_vars(ctx, cmd);

    if ctx.verbose || ctx.dry_run {
        eprintln!(
            "[{}] {}",
            if ctx.dry_run { "dry-run" } else { "exec" },
            expanded
        );
    }

    if ctx.dry_run {
        return Ok(Output {
            status: std::process::ExitStatus::default(),
            stdout: Vec::new(),
            stderr: Vec::new(),
        });
    }

    let output = Command::new("sh")
        .arg("-c")
        .arg(&expanded)
        .current_dir(&ctx.build_dir)
        .output()?;

    if !output.status.success() {
        return Err(ExecuteError::CommandFailedWithStderr {
            cmd: expanded,
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        });
    }

    Ok(output)
}
