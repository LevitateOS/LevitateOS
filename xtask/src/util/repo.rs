use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};

pub fn repo_root() -> Result<PathBuf> {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .context("xtask is expected at <repo>/xtask")
}

pub fn canonical_tools_install_command() -> &'static str {
    "just tools-install"
}

pub fn tools_prefix(root: &Path) -> Result<PathBuf> {
    let selection = root.join(".artifacts/tools/current");
    let published = match fs::canonicalize(&selection) {
        Ok(published) => published,
        Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
            return Err(error).with_context(|| format!(
                "cannot access QEMU tools selected at {}; published products retain coordinator ownership, so consume with matching privileges or explicitly install the selected product",
                selection.display()
            ));
        }
        Err(error) => {
            return Err(error).with_context(|| {
                format!(
                    "QEMU tools selection is unavailable at {}; prepare it with {}",
                    selection.display(),
                    canonical_tools_install_command()
                )
            });
        }
    };
    if !published.is_dir() {
        bail!(
            "QEMU tools selection at {} must identify a published directory; prepare it with {}",
            selection.display(),
            canonical_tools_install_command()
        );
    }
    Ok(published)
}

pub fn ovmf_path(root: &Path) -> Result<PathBuf> {
    let tools_prefix = tools_prefix(root)?;
    Ok(tools_prefix.join("usr/share/edk2/ovmf/OVMF_CODE.fd"))
}
