//! `cargo xtask release recipe`: the standalone Recipe release. No system builds or VM runs.
use anyhow::{Result, bail};
pub fn run(args: Vec<String>) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("recipe") => super::recipe_release::run(args[1..].to_vec()),
        None | Some("--help" | "-h") => {
            println!(
                "Usage: cargo xtask release recipe [ARGS]...\n\n`release recipe --help` lists its arguments."
            );
            Ok(())
        }
        Some(other) => bail!(
            "unknown release command '{other}'; the one release command is `cargo xtask release recipe`"
        ),
    }
}
