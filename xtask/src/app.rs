use anyhow::Result;

use crate::cli::{BaseCmd, Cmd};

pub fn run(cli: crate::cli::Cli) -> Result<()> {
    match cli.cmd {
        Cmd::Setup => crate::tasks::setup::setup(),
        Cmd::Doctor { json } => crate::tasks::setup::doctor(json),
        Cmd::Env => crate::tasks::setup::env(),
        Cmd::Build { what } => crate::tasks::build::run(what),
        Cmd::Base { what: BaseCmd::Lock { distro, check } } => crate::tasks::base::lock(distro, check),
        Cmd::Run(args) => crate::tasks::scenarios::run(args),
        Cmd::Test(args) => crate::tasks::scenarios::test(args),
        Cmd::Status { distro, json } => crate::tasks::scenarios::status(distro, json),
        Cmd::Reset { distro } => crate::tasks::scenarios::reset(distro),
        Cmd::Gen { check } => crate::tasks::generate::run(check),
        Cmd::Check => crate::tasks::check::run(),
        Cmd::Tui { app, args } => crate::tasks::tui::run(app, args),
        Cmd::Inspect(args) => crate::tasks::tui::inspect::run(crate::tasks::tui::inspect::Options {
            app: args.app,
            args: args.args,
            input: args.input,
            all_pages: args.all_pages,
            columns: args.columns,
            rows: args.rows,
            timeout_seconds: args.timeout_seconds,
            settle_ms: args.settle_ms,
            out_dir: args.out_dir,
            stdout: args.stdout,
            ansi: args.ansi,
            keep_transcript: args.keep_transcript,
        }),
    }
}
