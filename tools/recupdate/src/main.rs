//! recupdate: the one program that talks to the release channel.
//! Design: docs/release-engineering.md (sections 5 and 10).

mod apply;
mod boot;
mod check;
mod fetch;
mod net;
mod notify;
mod parts;
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
    /// Download the category parts of the release this medium was built from, verify them
    /// against the signed manifest and unpack them into Recipe's package cache (root).
    FetchParts {
        #[arg(long)]
        json: bool,
        /// Only these categories (comma separated); default: every part of the manifest.
        #[arg(long, value_delimiter = ',', value_name = "A,B")]
        categories: Vec<String>,
        /// Take release.json, release.json.sig and the part files from this folder instead of
        /// the network (the same verification).
        #[arg(long, value_name = "DIR")]
        from: Option<std::path::PathBuf>,
        /// Print the parts as JSON with their sizes and whether each is already unpacked, and
        /// download nothing.
        #[arg(long)]
        list: bool,
    },
    /// Run the check and write available.json (root; run daily by a timer).
    Notice,
    /// Send one desktop notification per new serial (run at session start for each user).
    NotifyUser,
    /// Install the release on this mutable installation and make its kernel the default boot
    /// entry (root; A/B installations use `recab update`).
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
        Cmd::FetchParts {
            json,
            categories,
            from,
            list,
        } => parts::run(parts::Args {
            json,
            categories,
            from,
            list,
        }),
        Cmd::Notice => check::notice(),
        Cmd::NotifyUser => notify::run(),
        Cmd::Apply { json } => apply::run(json),
    };
    if let Err(error) = result {
        eprintln!("recupdate: {error:#}");
        std::process::exit(1);
    }
}
