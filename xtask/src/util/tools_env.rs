use anyhow::Result;
use std::env;
use std::path::Path;
use std::process::Command;

/// The PATH/LD_LIBRARY_PATH/OVMF_PATH wiring the repo `justfile` sets.
fn tool_env(repo_root: &Path) -> Result<Vec<(&'static str, std::ffi::OsString)>> {
    let tools = crate::util::repo::tools_prefix(repo_root)?;

    let usr_bin = tools.join("usr/bin");
    let usr_libexec = tools.join("usr/libexec");
    let ld_library_path = tools.join("usr/lib64");
    let ovmf = crate::util::repo::ovmf_path(repo_root)?;

    let existing_path = env::var_os("PATH").unwrap_or_default();
    let mut paths = Vec::new();
    paths.push(usr_bin);
    paths.push(usr_libexec);
    paths.extend(env::split_paths(&existing_path));

    Ok(vec![
        ("PATH", env::join_paths(paths)?),
        ("LD_LIBRARY_PATH", ld_library_path.into_os_string()),
        ("OVMF_PATH", ovmf.into_os_string()),
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
