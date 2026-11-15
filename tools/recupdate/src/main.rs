//! recupdate: the one program that talks to the release channel.
//! Design: docs/release-engineering.md (sections 5 and 10).

mod apply;
mod check;
mod fetch;
mod net;
mod notify;
mod release;
mod util;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "recupdate",
    version,
    about = "Checks the signed release channel and fetches releases"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Fetch the channel file, verify it, and say whether a newer release exists (no privileges).
    Check {
        #[arg(long)]
        json: bool,
    },
    /// Download the release's files into the cache folder and verify them (root).
    Fetch {
        #[arg(long)]
        json: bool,
    },
    /// Run the check and write available.json (root; run daily by a timer).
    Notice,
    /// Send one desktop notification per new serial (run at session start for each user).
    NotifyUser,
    /// Apply the release to / (mutable installs only; not written yet).
    Apply {
        #[arg(long)]
        json: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.cmd {
        Cmd::Check { json } => check::check(json),
        Cmd::Fetch { json } => fetch::run(json),
        Cmd::Notice => check::notice(),
        Cmd::NotifyUser => notify::run(),
        Cmd::Apply { json } => apply::run(json),
    };
    if let Err(error) = result {
        eprintln!("recupdate: {error:#}");
        std::process::exit(1);
    }
}
