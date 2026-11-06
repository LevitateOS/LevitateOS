//! The environment the repo's QEMU tools (`.artifacts/tools/current`) need.
//! Commands that boot or build apply it to their children; `cargo xtask env`
//! prints it for a shell. Without the repo tools, the host's are used as they
//! are.

use anyhow::Result;
use std::env;
use std::ffi::OsString;
use std::path::Path;
use std::process::Command;

fn tool_env(repo_root: &Path) -> Result<Vec<(&'static str, OsString)>> {
    let Some(tools) = crate::util::repo::repo_tools(repo_root)? else {
        return Ok(Vec::new());
    };
    let ovmf = crate::util::repo::ovmf_path(repo_root)?;
    let ovmf_vars = ovmf.with_file_name("OVMF_VARS.fd");

    let mut paths = vec![tools.join("usr/bin"), tools.join("usr/libexec")];
    paths.extend(env::split_paths(&env::var_os("PATH").unwrap_or_default()));

    Ok(vec![
        ("PATH", env::join_paths(paths)?),
        ("LD_LIBRARY_PATH", tools.join("usr/lib64").into_os_string()),
        ("OVMF_PATH", ovmf.into_os_string()),
        ("OVMF_VARS_PATH", ovmf_vars.into_os_string()),
    ])
}

/// Give `cmd` the tool environment.
pub fn apply_to_command(cmd: &mut Command, repo_root: &Path) -> Result<()> {
    for (key, value) in tool_env(repo_root)? {
        cmd.env(key, value);
    }
    Ok(())
}

/// Give this process (and what it spawns) the tool environment. Call before
/// starting threads.
pub fn apply_to_process(repo_root: &Path) -> Result<()> {
    for (key, value) in tool_env(repo_root)? {
        // SAFETY: called from the single-threaded command entry point.
        unsafe { env::set_var(key, value) };
    }
    Ok(())
}

/// `export` lines for a POSIX shell; PATH keeps the shell's own `$PATH`.
pub fn shell_exports(repo_root: &Path) -> Result<String> {
    let Some(tools) = crate::util::repo::repo_tools(repo_root)? else {
        return Err(crate::util::exit::missing(
            "the repo QEMU tools are not installed (the host's are used as they are)",
            "cargo xtask setup",
        ));
    };
    let ovmf = crate::util::repo::ovmf_path(repo_root)?;
    Ok(format!(
        "export PATH=\"{}:{}:$PATH\"\n\
         export LD_LIBRARY_PATH=\"{}\"\n\
         export OVMF_PATH=\"{}\"\n\
         export OVMF_VARS_PATH=\"{}\"\n",
        tools.join("usr/bin").display(),
        tools.join("usr/libexec").display(),
        tools.join("usr/lib64").display(),
        ovmf.display(),
        ovmf.with_file_name("OVMF_VARS.fd").display(),
    ))
}
