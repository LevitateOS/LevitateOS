use anyhow::Result;

use crate::cli::{BaseCmd, CheckCmd, Cmd, KernelCmd, TuiCmd};

pub fn run(cli: crate::cli::Cli) -> Result<()> {
    match cli.cmd {
        Cmd::Setup => crate::tasks::setup::setup(),
        Cmd::Doctor { json, packages: None } => crate::tasks::setup::doctor(json),
        Cmd::Doctor { packages: Some(family), .. } => crate::tasks::setup::packages(&family),
        Cmd::Build { what } => crate::tasks::build::run(what),
        Cmd::Kernel { what: KernelCmd::Publish(args) } => crate::tasks::kernels::publish(args),
        Cmd::Base { what: BaseCmd::Lock { distro, check } } => crate::tasks::base::lock(&distro, check),
        Cmd::Run(args) => crate::tasks::scenarios::run(args),
        Cmd::Test(args) => crate::tasks::scenarios::test(args),
        Cmd::Status { distro, json } => crate::tasks::scenarios::status(&distro, json),
        Cmd::Reset { distro } => crate::tasks::scenarios::reset(&distro),
        Cmd::Gen { check } => crate::tasks::generate::run(check),
        Cmd::Check { what: None } => crate::tasks::check::run(),
        Cmd::Check {
            what: Some(CheckCmd::Tui { bless }),
        } => crate::tasks::tui::check::run(bless),
        Cmd::Tui { cmd: TuiCmd::List { app: None } } => crate::tasks::tui::list(),
        Cmd::Tui { cmd: TuiCmd::List { app: Some(app) } } => crate::tasks::tui::stories(&app),
        Cmd::Tui { cmd: TuiCmd::Run(mut line) } => {
            let app = line.remove(0);
            crate::tasks::tui::run(app, line)
        }
        Cmd::Inspect(args) => crate::tasks::tui::inspect::run(args),
    }
}
