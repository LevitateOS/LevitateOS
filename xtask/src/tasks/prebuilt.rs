//! `prebuilt`: keys and publishing of our own prebuilt programs, through distro-builder.

use anyhow::Result;

use crate::cli::PrebuiltArgs;
use distro_builder::cli::Command;

/// `prebuilt key` (`publish` false) or `prebuilt publish`. Neither needs root, and
/// `publish` must use the user's own `gh` login, so the builder runs unprivileged.
pub fn run((publish, args): (bool, PrebuiltArgs)) -> Result<()> {
    let PrebuiltArgs { family, name, distro } = args;
    let distro = distro.map(|distro| distro.id().to_string());
    let command = if publish {
        Command::PrebuiltPublish { family, name, distro }
    } else {
        Command::PrebuiltKey { family, name, distro }
    };
    crate::util::distro_builder::run_unprivileged(&crate::util::repo::repo_root()?, &command)
}
