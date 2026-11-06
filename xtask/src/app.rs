use anyhow::Result;

use crate::cli::{Cmd, InspectCmd};

pub fn run(cli: crate::cli::Cli) -> Result<()> {
    match cli.cmd {
        Cmd::Setup => crate::tasks::setup::setup(),
        Cmd::Doctor { json } => crate::tasks::setup::doctor(json),
        Cmd::Env => crate::tasks::setup::env(),
        Cmd::Build { what } => crate::tasks::build::run(what),
        Cmd::Run(args) => crate::tasks::scenarios::run(args),
        Cmd::Test(args) => crate::tasks::scenarios::test(args),
        Cmd::Status { distro, json } => crate::tasks::scenarios::status(distro, json),
        Cmd::Reset { distro } => crate::tasks::scenarios::reset(distro),
        Cmd::Gen { check } => crate::tasks::generate::run(check),
        Cmd::Check => crate::tasks::check::run(),
        Cmd::Tui { app, args } => crate::tasks::tui::run(app, args),
        Cmd::Inspect { what } => match what {
            InspectCmd::Docs { slug, snapshot } => {
                crate::tasks::inspect_docs::run(crate::tasks::inspect_docs::Options {
                    slugs: slug,
                    columns: snapshot.columns,
                    rows: snapshot.rows,
                    seconds: snapshot.seconds,
                    out_dir: snapshot.out_dir,
                    stdout: snapshot.stdout,
                    ansi: snapshot.ansi,
                    keep_transcript: snapshot.keep_transcript,
                })
            }
            InspectCmd::Tui {
                app,
                cwd,
                command,
                input,
                input_delay_seconds,
                snapshot,
            } => crate::tasks::inspect_tui::run(crate::tasks::inspect_tui::Options {
                app,
                cwd,
                command,
                input,
                input_delay_seconds,
                columns: snapshot.columns,
                rows: snapshot.rows,
                seconds: snapshot.seconds,
                out_dir: snapshot.out_dir,
                stdout: snapshot.stdout,
                ansi: snapshot.ansi,
                keep_transcript: snapshot.keep_transcript,
            }),
        },
    }
}
