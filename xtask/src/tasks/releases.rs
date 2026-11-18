//! `cargo xtask release recipe|keygen|sign|publish|site`: the standalone Recipe release and the signing and publishing of system releases. No system builds or VM runs.
use anyhow::{Result, bail};
pub fn run(args: Vec<String>) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("recipe") => super::recipe_release::run(args[1..].to_vec()),
        Some("keygen" | "sign" | "publish" | "site") => super::release::run(&args),
        None | Some("--help" | "-h") => {
            println!(
                "Usage: cargo xtask release recipe [ARGS]...\n\n`release recipe --help` lists its arguments.\n\n{}",
                super::release::USAGE
            );
            Ok(())
        }
        Some(other) => bail!(
            "unknown release command '{other}'; the release commands are recipe, keygen, sign, publish and site"
        ),
    }
}
