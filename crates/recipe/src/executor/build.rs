//! Build phase - extracts archives and runs build steps.

use std::path::PathBuf;

use crate::{BuildSpec, BuildStep};

use super::context::Context;
use super::error::ExecuteError;
use super::util::{expand_vars, run_cmd, shell_quote};

/// Execute the build phase.
pub fn build(ctx: &Context, spec: &BuildSpec) -> Result<(), ExecuteError> {
    match spec {
        BuildSpec::Skip => {}
        BuildSpec::Extract(format) => {
            build_extract(ctx, format)?;
        }
        BuildSpec::Steps(steps) => {
            for step in steps {
                build_step(ctx, step)?;
            }
        }
    }
    Ok(())
}

fn build_extract(ctx: &Context, format: &str) -> Result<(), ExecuteError> {
    // Find the archive in build_dir
    let archive = find_archive(ctx)?;

    let cmd = match format {
        "tar-gz" | "tar.gz" | "tgz" => {
            format!(
                "tar xzf {} -C {}",
                shell_quote(archive.display()),
                shell_quote(ctx.build_dir.display())
            )
        }
        "tar-xz" | "tar.xz" | "txz" => {
            format!(
                "tar xJf {} -C {}",
                shell_quote(archive.display()),
                shell_quote(ctx.build_dir.display())
            )
        }
        "tar-bz2" | "tar.bz2" | "tbz2" => {
            format!(
                "tar xjf {} -C {}",
                shell_quote(archive.display()),
                shell_quote(ctx.build_dir.display())
            )
        }
        "tar" => {
            format!(
                "tar xf {} -C {}",
                shell_quote(archive.display()),
                shell_quote(ctx.build_dir.display())
            )
        }
        "zip" => {
            format!(
                "unzip -o {} -d {}",
                shell_quote(archive.display()),
                shell_quote(ctx.build_dir.display())
            )
        }
        _ => return Err(ExecuteError::UnsupportedFormat(format.to_string())),
    };

    run_cmd(ctx, &cmd)?;
    Ok(())
}

fn find_archive(ctx: &Context) -> Result<PathBuf, ExecuteError> {
    // Look for common archive extensions in build_dir
    let extensions = [
        ".tar.gz",
        ".tgz",
        ".tar.xz",
        ".txz",
        ".tar.bz2",
        ".tbz2",
        ".tar",
        ".zip",
    ];

    if let Ok(entries) = std::fs::read_dir(&ctx.build_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                for ext in &extensions {
                    if name.ends_with(ext) {
                        return Ok(path);
                    }
                }
            }
        }
    }

    Err(ExecuteError::MissingField("archive file".to_string()))
}

fn build_step(ctx: &Context, step: &BuildStep) -> Result<(), ExecuteError> {
    let cmd = match step {
        BuildStep::Configure(args) => {
            if args.is_empty() {
                format!(
                    "./configure --prefix={}",
                    shell_quote(ctx.prefix.display())
                )
            } else {
                expand_vars(ctx, args)
            }
        }
        BuildStep::Compile(args) => {
            if args.is_empty() {
                format!("make -j{}", ctx.nproc)
            } else {
                expand_vars(ctx, args)
            }
        }
        BuildStep::Test(args) => {
            if args.is_empty() {
                "make test".to_string()
            } else {
                expand_vars(ctx, args)
            }
        }
        BuildStep::Cargo(args) => {
            if args.is_empty() {
                "cargo build --release".to_string()
            } else {
                format!("cargo {}", expand_vars(ctx, args))
            }
        }
        BuildStep::Meson(args) => {
            if args.is_empty() {
                format!(
                    "meson setup build --prefix={}",
                    shell_quote(ctx.prefix.display())
                )
            } else {
                format!("meson {}", expand_vars(ctx, args))
            }
        }
        BuildStep::Ninja(args) => {
            if args.is_empty() {
                format!("ninja -C build -j{}", ctx.nproc)
            } else {
                format!("ninja {}", expand_vars(ctx, args))
            }
        }
        BuildStep::Run(cmd) => expand_vars(ctx, cmd),
    };

    run_cmd(ctx, &cmd)?;
    Ok(())
}
