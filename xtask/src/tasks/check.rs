//! `check`: the quick sweep before committing. Nothing runs it for you.

use anyhow::{Context, Result, bail};
use std::path::Path;
use std::process::Command;

/// TypeScript packages with a `typecheck` script.
const TS_PACKAGES: &[&str] = &[
    "docs/content",
    "tui/kit/core",
    "tui/apps/install/disk-plan",
    "tui/apps/live-tools/install-docs",
    "tui/apps/runtime/recab",
];

pub fn run() -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    if which::which("bun").is_err() {
        return Err(crate::util::exit::missing(
            "bun is not installed (TypeScript typecheck)",
            "install bun (https://bun.sh)",
        ));
    }

    let mut failed = Vec::new();
    let mut step = |name: &str, result: Result<()>| match result {
        Ok(()) => eprintln!("[ok]   {name}"),
        Err(e) => {
            eprintln!("[FAIL] {name}: {e:#}");
            failed.push(name.to_string());
        }
    };

    step(
        "cargo check --workspace --all-targets",
        command(&root, "cargo", &["check", "-q", "--workspace", "--all-targets"]),
    );
    step("generated files current", crate::tasks::generate::run(true));
    step(
        "docs content builds",
        command(&root.join("docs/content"), "bun", &["run", "build"]),
    );
    for package in TS_PACKAGES {
        let dir = root.join(package);
        let result = ensure_node_modules(&dir)
            .and_then(|()| command(&dir, "bun", &["run", "typecheck"]));
        step(&format!("typecheck {package}"), result);
    }

    if !failed.is_empty() {
        bail!("{} check(s) failed: {}", failed.len(), failed.join(", "));
    }
    Ok(())
}

pub fn ensure_node_modules(dir: &Path) -> Result<()> {
    if dir.join("node_modules").is_dir() {
        return Ok(());
    }
    command(dir, "bun", &["install"])
}

fn command(dir: &Path, program: &str, args: &[&str]) -> Result<()> {
    let output = Command::new(program)
        .current_dir(dir)
        .args(args)
        .output()
        .with_context(|| format!("running {program}"))?;
    if !output.status.success() {
        let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
        text.push_str(&String::from_utf8_lossy(&output.stderr));
        let tail: Vec<&str> = text.lines().rev().take(15).collect();
        let tail: Vec<&str> = tail.into_iter().rev().collect();
        bail!("{program} {} failed:\n{}", args.join(" "), tail.join("\n"));
    }
    Ok(())
}
