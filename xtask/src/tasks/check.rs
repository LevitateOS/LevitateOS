//! `check`: the quick sweep before committing. Nothing runs it for you.

use anyhow::{Context, Result, bail};
use std::path::Path;
use std::process::Command;

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
    step("bun install --frozen-lockfile", install_js_workspace(&root));
    let mut packages = vec![root.join("docs/content")];
    packages.extend(distro_contract::tui_apps::tui_packages(&root)?);
    for dir in packages {
        let name = dir.strip_prefix(&root).unwrap_or(&dir).display().to_string();
        step(
            &format!("typecheck {name}"),
            command(&dir, "bun", &["run", "typecheck"]),
        );
    }

    if !failed.is_empty() {
        bail!("{} check(s) failed: {}", failed.len(), failed.join(", "));
    }
    Ok(())
}

/// Installs the one JavaScript workspace (root `package.json`, root `bun.lock`).
/// Every TUI package is a member, so this is the only install there is.
pub fn install_js_workspace(root: &Path) -> Result<()> {
    command(root, "bun", &["install", "--frozen-lockfile"])
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
