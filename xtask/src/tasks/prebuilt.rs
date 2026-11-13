//! `prebuilt`: keys, listing, building and publishing of our own prebuilt programs,
//! through distro-builder.

use anyhow::Result;

use crate::cli::{PrebuiltArgs, PrebuiltCmd};
use distro_builder::cli::Command;

/// `prebuilt key`, `list` and `publish` need no root, and `publish` must use the user's
/// own `gh` login, so the builder runs unprivileged for them. `build` compiles, which
/// needs root like every compile, so it takes the elevated path.
pub fn run(cmd: PrebuiltCmd) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let command = |args: PrebuiltArgs, make: fn(String, String, Option<String>) -> Command| {
        let PrebuiltArgs { family, name, distro } = args;
        make(family, name, distro.map(|distro| distro.id().to_string()))
    };
    match cmd {
        PrebuiltCmd::List => crate::util::distro_builder::run_unprivileged(&root, &Command::PrebuiltList),
        PrebuiltCmd::Key(args) => crate::util::distro_builder::run_unprivileged(
            &root,
            &command(args, |family, name, distro| Command::PrebuiltKey { family, name, distro }),
        ),
        PrebuiltCmd::Publish(args) => crate::util::distro_builder::run_unprivileged(
            &root,
            &command(args, |family, name, distro| Command::PrebuiltPublish { family, name, distro }),
        ),
        PrebuiltCmd::PublishRuntime => {
            crate::util::distro_builder::run_unprivileged(&root, &Command::PrebuiltPublishRuntime)
        }
        PrebuiltCmd::Build(args) => crate::util::distro_builder::run(
            &root,
            &command(args, |family, name, distro| Command::PrebuiltBuild { family, name, distro }),
        ),
    }
}
