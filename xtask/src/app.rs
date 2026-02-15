use anyhow::Result;

pub fn run(cli: crate::cli::Cli) -> Result<()> {
    match cli.cmd {
        crate::cli::Cmd::Env { shell } => crate::tasks::tooling::env::run(shell),
        crate::cli::Cmd::Doctor => crate::tasks::tooling::doctor::run(),
        crate::cli::Cmd::Kernels { cmd } => match cmd {
            crate::cli::KernelsCmd::Build { distro, rebuild } => {
                crate::tasks::kernels::build::run(distro, rebuild)
            }
            crate::cli::KernelsCmd::BuildAll { rebuild } => {
                crate::tasks::kernels::build_all::run(rebuild)
            }
            crate::cli::KernelsCmd::Check { distro } => crate::tasks::kernels::check::run(distro),
        },
        crate::cli::Cmd::Hooks { cmd } => match cmd {
            crate::cli::HooksCmd::Install => crate::tasks::tooling::hooks::install(),
            crate::cli::HooksCmd::Remove => crate::tasks::tooling::hooks::remove(),
        },
        crate::cli::Cmd::Checkpoints { cmd } => match cmd {
            crate::cli::CheckpointsCmd::Boot { n, distro } => {
                crate::tasks::testing::checkpoints::boot(n, distro)
            }
            crate::cli::CheckpointsCmd::Test { n, distro } => {
                crate::tasks::testing::checkpoints::test(n, distro)
            }
            crate::cli::CheckpointsCmd::TestUpTo { n, distro } => {
                crate::tasks::testing::checkpoints::test_up_to(n, distro)
            }
            crate::cli::CheckpointsCmd::Status { distro } => {
                crate::tasks::testing::checkpoints::status(distro)
            }
            crate::cli::CheckpointsCmd::Reset { distro } => {
                crate::tasks::testing::checkpoints::reset(distro)
            }
        },
    }
}
