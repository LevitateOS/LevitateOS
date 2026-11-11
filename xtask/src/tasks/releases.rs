//! Release orchestration consumes explicit artifacts; no system builds or VM runs.
use anyhow::{Result, ensure};
pub fn run(args: Vec<String>) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let help = args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h");
    let mut command = std::process::Command::new("python3");
    command
        .arg(root.join(".github/scripts/recab_updates.py"))
        .arg("--repo-root")
        .arg(&root);
    if !help && !args.iter().any(|a| a == "--builder") {
        let builder = crate::util::repo::cargo_bin(&root, "distro-builder", "distro-builder")?;
        command.arg("--builder").arg(builder);
    }
    let args = if args.is_empty() {
        vec!["--help".to_string()]
    } else {
        args
    };
    let status = command.args(args).current_dir(&root).status()?;
    ensure!(status.success(), "release command failed ({status})");
    Ok(())
}
