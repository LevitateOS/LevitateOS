use anyhow::{Context, Result, bail};
use std::fs;
use std::io::ErrorKind;
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
    let centralized = root.join(".artifacts/tools/.tools");
    let build_root = centralized
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| root.join(".artifacts/tools"));
    let meta = match fs::metadata(&centralized) {
        Ok(meta) => meta,
        Err(err) if err.kind() == ErrorKind::NotFound => {
            bail!(
                "component: xtask tools-env preflight\n\
                 surface: scenario boot/test runtime\n\
                 expectation: canonical tools root directory exists\n\
                 path: {}\n\
                 failure: directory not found\n\
                 remediation:\n  {}\n  cargo run -p levitate-recipe --bin recipe -- install --build-dir {} --recipes-path distro-builder/recipes --no-persist-ctx --define TOOLS_PREFIX={} qemu-deps",
                centralized.display(),
                canonical_tools_install_command(),
                build_root.display(),
                centralized.display()
            );
        }
        Err(err) => {
            return Err(err).with_context(|| {
                format!(
                    "checking canonical tools root metadata at {}",
                    centralized.display()
                )
            });
        }
    };

    if !meta.is_dir() {
        bail!(
            "component: xtask tools-env preflight\n\
             surface: scenario boot/test runtime\n\
             expectation: canonical tools root must be a directory\n\
             path: {}\n\
             failure: path exists but is not a directory\n\
             remediation:\n  rm -f {}\n  {}\n  cargo run -p levitate-recipe --bin recipe -- install --build-dir {} --recipes-path distro-builder/recipes --no-persist-ctx --define TOOLS_PREFIX={} qemu-deps",
            centralized.display(),
            centralized.display(),
            canonical_tools_install_command(),
            build_root.display(),
            centralized.display()
        );
    }
    Ok(centralized)
}

pub fn ovmf_path(root: &Path) -> Result<PathBuf> {
    let tools_prefix = tools_prefix(root)?;
    Ok(tools_prefix.join("usr/share/edk2/ovmf/OVMF_CODE.fd"))
}
