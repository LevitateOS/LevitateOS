// TEAM_363: Eyra-based pwd utility for LevitateOS
//
// Prints the current working directory.

extern crate eyra;

use std::env;

fn print_help() {
    println!("Usage: pwd [OPTION]...");
    println!("Print the full filename of the current working directory.");
    println!();
    println!("  -L, --logical     use PWD from environment (ignored)");
    println!("  -P, --physical    avoid all symlinks (ignored)");
    println!("      --help        display this help and exit");
    println!("      --version     output version information and exit");
}

fn print_version() {
    println!("pwd (LevitateOS) 0.2.0");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in args.iter().skip(1) {
        if arg == "--help" {
            print_help();
            return;
        } else if arg == "--version" {
            print_version();
            return;
        } else if arg == "-L" || arg == "--logical" || arg == "-P" || arg == "--physical" {
            // Ignore these - no symlink support yet
        } else if arg.starts_with('-') {
            eprintln!("pwd: invalid option -- '{}'", arg);
            std::process::exit(1);
        }
    }

    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => {
            eprintln!("pwd: error getting current directory: {}", e);
            std::process::exit(1);
        }
    }
}
