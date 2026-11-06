use anyhow::Result;

use crate::cli::{BaseCmd, CheckCmd, Cmd};

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
        Cmd::Check { what: None } => crate::tasks::check::run(),
        Cmd::Check {
            what: Some(CheckCmd::Tui { bless }),
        } => crate::tasks::tui::check::run(bless),
        Cmd::Tui { app, args } => crate::tasks::tui::run(app, args),
        Cmd::Inspect(args) => crate::tasks::tui::inspect::run(args),
        Cmd::Stories { port, no_open, export } => {
            crate::tasks::tui::stories::run(crate::tasks::tui::stories::Options {
                port,
                open: !no_open,
                export,
            })
        }
    }
}
