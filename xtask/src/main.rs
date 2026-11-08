use clap::Parser;

mod app;
mod cli;
mod tasks;
mod util;

fn main() {
    let cli = crate::cli::Cli::parse();
    if let Err(error) = crate::app::run(cli) {
        eprintln!("error: {error:#}");
        if let Some(fix) = crate::util::exit::producer_fix(&error) {
            eprintln!("  fix: {fix}");
        }
        std::process::exit(crate::util::exit::code_for(&error));
    }
}
