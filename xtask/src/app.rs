use anyhow::Result;

pub fn run(cli: crate::cli::Cli) -> Result<()> {
    enforce_policy_guard_placement(&cli.cmd)?;
    match cli.cmd {
        crate::cli::Cmd::Env { shell } => crate::tasks::tooling::env::run(shell),
        crate::cli::Cmd::Doctor => crate::tasks::tooling::doctor::run(),
        crate::cli::Cmd::Kernels { cmd } => match cmd {
            crate::cli::KernelsCmd::Build {
                distro,
                rebuild,
                autofix,
                autofix_attempts,
                autofix_prompt_file,
                llm_profile,
            } => crate::tasks::kernels::build::run(
                distro,
                rebuild,
                crate::tasks::kernels::common::AutoFixOptions {
                    enabled: autofix,
                    attempts: autofix_attempts,
                    prompt_file: autofix_prompt_file,
                    llm_profile,
                },
            ),
            crate::cli::KernelsCmd::BuildAll {
                rebuild,
                autofix,
                autofix_attempts,
                autofix_prompt_file,
                llm_profile,
            } => crate::tasks::kernels::build_all::run(
                rebuild,
                crate::tasks::kernels::common::AutoFixOptions {
                    enabled: autofix,
                    attempts: autofix_attempts,
                    prompt_file: autofix_prompt_file,
                    llm_profile,
                },
            ),
            crate::cli::KernelsCmd::Prebuilt { distro, refresh } => {
                crate::tasks::kernels::prebuilt::run(distro, refresh)
            }
            crate::cli::KernelsCmd::Check { distro } => crate::tasks::kernels::check::run(distro),
        },
        crate::cli::Cmd::Hooks { cmd } => match cmd {
            crate::cli::HooksCmd::Install => crate::tasks::tooling::hooks::install(),
            crate::cli::HooksCmd::Remove => crate::tasks::tooling::hooks::remove(),
        },
        crate::cli::Cmd::Scenarios { cmd } => match cmd {
            crate::cli::ScenariosCmd::Boot {
                target,
                distro,
                inject,
                inject_file,
                ssh,
                ssh_port,
                ssh_timeout,
                no_shell,
                window,
                ssh_private_key,
            } => crate::tasks::testing::scenarios::boot(
                target,
                distro,
                inject,
                inject_file,
                ssh,
                ssh_port,
                ssh_timeout,
                no_shell,
                window,
                ssh_private_key,
            ),
            crate::cli::ScenariosCmd::Test {
                target,
                distro,
                inject,
                inject_file,
                force,
            } => crate::tasks::testing::scenarios::test(target, distro, inject, inject_file, force),
            crate::cli::ScenariosCmd::TestUpTo {
                target,
                distro,
                inject,
                inject_file,
            } => crate::tasks::testing::scenarios::test_up_to(target, distro, inject, inject_file),
            crate::cli::ScenariosCmd::Status { distro } => {
                crate::tasks::testing::scenarios::status(distro)
            }
            crate::cli::ScenariosCmd::Reset { distro } => {
                crate::tasks::testing::scenarios::reset(distro)
            }
        },
        crate::cli::Cmd::Policy { cmd } => match cmd {
            crate::cli::PolicyCmd::AuditLegacyBindings => {
                crate::tasks::tooling::policy::audit_legacy_bindings()
            }
        },
        crate::cli::Cmd::Docs { cmd } => match cmd {
            crate::cli::DocsCmd::Inspect {
                slug,
                columns,
                rows,
                seconds,
                out_dir,
                stdout,
                ansi,
                keep_transcript,
            } => crate::tasks::docs::inspect::run(crate::tasks::docs::inspect::Options {
                slugs: slug,
                columns,
                rows,
                seconds,
                out_dir,
                stdout,
                ansi,
                keep_transcript,
            }),
        },
        crate::cli::Cmd::Tui { cmd } => match cmd {
            crate::cli::TuiCmd::Inspect {
                app,
                cwd,
                command,
                input,
                input_delay_seconds,
                columns,
                rows,
                seconds,
                out_dir,
                stdout,
                ansi,
                keep_transcript,
            } => crate::tasks::tui::inspect::run(crate::tasks::tui::inspect::Options {
                app,
                cwd,
                command,
                input,
                input_delay_seconds,
                columns,
                rows,
                seconds,
                out_dir,
                stdout,
                ansi,
                keep_transcript,
            }),
        },
    }
}

fn enforce_policy_guard_placement(cmd: &crate::cli::Cmd) -> Result<()> {
    use crate::cli::{Cmd, KernelsCmd, ScenariosCmd};

    let requires_guard = matches!(
        cmd,
        Cmd::Kernels {
            cmd: KernelsCmd::Build { .. }
                | KernelsCmd::BuildAll { .. }
                | KernelsCmd::Prebuilt { .. }
        } | Cmd::Scenarios {
            cmd: ScenariosCmd::Boot { .. }
                | ScenariosCmd::Test { .. }
                | ScenariosCmd::TestUpTo { .. }
        }
    );
    if !requires_guard {
        return Ok(());
    }

    crate::tasks::tooling::policy::audit_legacy_bindings()
}
