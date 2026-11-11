//! `check`: the quick sweep before committing. Every step runs and is reported, so one
//! failure does not hide the others; a step whose tool is missing is skipped and named.

use anyhow::{Context, Result, anyhow, bail};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

/// The scripts a JavaScript workspace package may define for the sweep; it runs each
/// one the package defines, in this order.
pub const PACKAGE_SCRIPTS: &[&str] = &["typecheck", "lint", "format:check"];

/// The two shells a guest `#!/bin/sh` script runs under: bash on LevitateOS and
/// busybox ash on AcornOS.
const GUEST_SHELLS: &[(&str, &[&str])] = &[("bash", &["--posix", "-n"]), ("busybox", &["sh", "-n"])];

pub fn run() -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let mut sweep = Sweep::default();

    sweep.step(
        "cargo check --workspace --all-targets",
        command(&root, "cargo", &["check", "-q", "--workspace", "--all-targets"]),
    );
    // tools/recipe is linted by its own repo.
    sweep.step(
        "cargo clippy --workspace --all-targets",
        command(
            &root,
            "cargo",
            &["clippy", "-q", "--workspace", "--all-targets", "--exclude", "levitate-recipe", "--", "-D", "warnings"],
        ),
    );
    // Unit and integration tests that run unprivileged; tools/recipe's run in its own repo.
    sweep.step(
        "cargo test --workspace",
        command(
            &root,
            "cargo",
            &["test", "-q", "--workspace", "--no-fail-fast", "--exclude", "levitate-recipe"],
        ),
    );
    let scripts = guest_scripts(&root);
    for &(shell, args) in GUEST_SHELLS {
        let name = format!("guest scripts parse ({shell} {})", args.join(" "));
        if which::which(shell).is_ok() {
            sweep.step(&name, scripts.as_ref().map_err(clone_error).and_then(|s| parse_all(shell, args, s)));
        } else {
            // Without the target shell the host's sh still catches syntax errors.
            sweep.note(&format!("{name}: {shell} is not installed; parsed with the host's sh -n instead"));
            sweep.step(
                &format!("guest scripts parse (sh -n, for {shell})"),
                scripts.as_ref().map_err(clone_error).and_then(|s| parse_all("sh", &["-n"], s)),
            );
        }
    }

    let js_steps = [
        "bun install --frozen-lockfile",
        "generated files current",
        "docs content builds",
        "workspace package scripts",
    ];
    if which::which("bun").is_err() {
        for name in js_steps {
            sweep.skip(name, "bun is not installed");
        }
    } else {
        sweep.step(js_steps[0], install_js_workspace(&root));
        sweep.step(js_steps[1], crate::tasks::generate::run(true));
        // Its generated index is imported by the packages checked below.
        sweep.step(js_steps[2], command(&root.join("docs/content"), "bun", &["run", "build"]));
        match workspace_packages(&root) {
            Err(e) => sweep.step(js_steps[3], Err(e)),
            Ok(packages) => {
                for dir in packages {
                    let name = dir.strip_prefix(&root).unwrap_or(&dir).display().to_string();
                    let defined = match package_scripts(&dir) {
                        Ok(defined) => defined,
                        Err(e) => {
                            sweep.step(&format!("scripts of {name}"), Err(e));
                            continue;
                        }
                    };
                    for script in PACKAGE_SCRIPTS.iter().filter(|s| defined.iter().any(|d| d == *s)) {
                        sweep.step(&format!("{script} {name}"), command(&dir, "bun", &["run", script]));
                    }
                }
            }
        }
    }

    sweep.finish()
}

#[derive(Default)]
struct Sweep {
    failed: Vec<String>,
    skipped: Vec<String>,
    notes: Vec<String>,
}

impl Sweep {
    fn step(&mut self, name: &str, result: Result<()>) {
        match result {
            Ok(()) => eprintln!("[ok]   {name}"),
            Err(e) => {
                eprintln!("[FAIL] {name}: {e:#}");
                self.failed.push(name.to_string());
            }
        }
    }

    fn skip(&mut self, name: &str, why: &str) {
        eprintln!("[skip] {name}: {why}");
        self.skipped.push(name.to_string());
    }

    fn note(&mut self, text: &str) {
        self.notes.push(text.to_string());
    }

    fn finish(self) -> Result<()> {
        for note in &self.notes {
            eprintln!("note: {note}");
        }
        if !self.failed.is_empty() {
            bail!("{} check(s) failed: {}", self.failed.len(), self.failed.join(", "));
        }
        if !self.skipped.is_empty() {
            return Err(crate::util::exit::missing(
                format!("{} check(s) skipped: {}", self.skipped.len(), self.skipped.join(", ")),
                "install bun (https://bun.sh)",
            ));
        }
        Ok(())
    }
}

fn clone_error(e: &anyhow::Error) -> anyhow::Error {
    anyhow!("{e:#}")
}

/// The guest scripts: files under `distro/builder/guest/` with a `#!/bin/sh` shebang.
fn guest_scripts(root: &Path) -> Result<Vec<PathBuf>> {
    let mut scripts = Vec::new();
    let mut stack = vec![root.join("distro/builder/guest")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).with_context(|| format!("reading {}", dir.display()))? {
            let path = entry?.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            let text = std::fs::read(&path).with_context(|| format!("reading {}", path.display()))?;
            let first = text.split(|&b| b == b'\n').next().unwrap_or_default();
            let shebang = String::from_utf8_lossy(first);
            if shebang.trim_end() == "#!/bin/sh" || shebang.starts_with("#!/bin/sh ") {
                scripts.push(path);
            }
        }
    }
    scripts.sort();
    Ok(scripts)
}

fn parse_all(shell: &str, args: &[&str], scripts: &[PathBuf]) -> Result<()> {
    let mut failures = Vec::new();
    for path in scripts {
        let output = Command::new(shell)
            .args(args)
            .arg(path)
            .output()
            .with_context(|| format!("running {shell}"))?;
        if !output.status.success() {
            failures.push(format!("{}: {}", path.display(), String::from_utf8_lossy(&output.stderr).trim()));
        }
    }
    if !failures.is_empty() {
        bail!("{}", failures.join("\n"));
    }
    Ok(())
}

/// The JavaScript workspace's packages: the root `package.json` `workspaces`.
pub fn workspace_packages(root: &Path) -> Result<Vec<PathBuf>> {
    let manifest = root.join("package.json");
    let json: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&manifest).with_context(|| format!("reading {}", manifest.display()))?,
    )
    .with_context(|| format!("parsing {}", manifest.display()))?;
    let members = json["workspaces"]
        .as_array()
        .ok_or_else(|| anyhow!("{} has no workspaces list", manifest.display()))?;
    members
        .iter()
        .map(|member| {
            let member = member
                .as_str()
                .ok_or_else(|| anyhow!("{}: a workspaces entry is not a string", manifest.display()))?;
            Ok(root.join(member))
        })
        .collect()
}

fn package_scripts(dir: &Path) -> Result<Vec<String>> {
    let manifest = dir.join("package.json");
    let json: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&manifest).with_context(|| format!("reading {}", manifest.display()))?,
    )
    .with_context(|| format!("parsing {}", manifest.display()))?;
    Ok(json["scripts"]
        .as_object()
        .map(|scripts| scripts.keys().cloned().collect())
        .unwrap_or_default())
}

static JS_WORKSPACE_INSTALLED: AtomicBool = AtomicBool::new(false);

/// Installs the one JavaScript workspace (root `package.json`, root `bun.lock`), once
/// per xtask run: every later caller (gen, check tui, inspect) reuses that install.
pub fn install_js_workspace(root: &Path) -> Result<()> {
    if JS_WORKSPACE_INSTALLED.load(Ordering::Relaxed) {
        return Ok(());
    }
    command(root, "bun", &["install", "--frozen-lockfile"])?;
    JS_WORKSPACE_INSTALLED.store(true, Ordering::Relaxed);
    Ok(())
}

/// Runs a command in `dir`; on failure, the error carries the tail of its output.
pub fn command(dir: &Path, program: &str, args: &[&str]) -> Result<()> {
    let output = Command::new(program)
        .current_dir(dir)
        .args(args)
        .output()
        .with_context(|| format!("running {program}"))?;
    if !output.status.success() {
        let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
        text.push_str(&String::from_utf8_lossy(&output.stderr));
        let tail: Vec<&str> = text.lines().rev().take(20).collect();
        let tail: Vec<&str> = tail.into_iter().rev().collect();
        bail!("{program} {} failed:\n{}", args.join(" "), tail.join("\n"));
    }
    Ok(())
}
