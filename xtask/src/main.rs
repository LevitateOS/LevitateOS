use clap::Parser;

mod app;
mod cli;
mod tasks;
mod util;

fn main() {
    let cli = crate::cli::Cli::parse();
    if let Err(error) = crate::app::run(cli) {
        eprintln!("error: {error:#}");
        std::process::exit(crate::util::exit::code_for(&error));
    }
}
