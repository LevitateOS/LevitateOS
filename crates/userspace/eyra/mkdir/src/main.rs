// TEAM_363: Eyra-based mkdir utility for LevitateOS

extern crate eyra;

use std::env;
use std::fs;

fn print_help() {
    println!("Usage: mkdir [OPTION]... DIRECTORY...");
    println!("Create the DIRECTORY(ies), if they do not already exist.");
    println!();
    println!("  -p, --parents     make parent directories as needed");
    println!("  -v, --verbose     print a message for each created directory");
    println!("      --help        display this help and exit");
    println!("      --version     output version information and exit");
}

fn print_version() {
    println!("mkdir (LevitateOS) 0.2.0");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut parents = false;
    let mut verbose = false;
    let mut dirs: Vec<&str> = Vec::new();

    for arg in args.iter().skip(1) {
        if arg == "--help" {
            print_help();
            return;
        } else if arg == "--version" {
            print_version();
            return;
        } else if arg == "-p" || arg == "--parents" {
            parents = true;
        } else if arg == "-v" || arg == "--verbose" {
            verbose = true;
        } else if arg.starts_with('-') {
            eprintln!("mkdir: invalid option -- '{}'", arg);
            std::process::exit(1);
        } else {
            dirs.push(arg);
        }
    }

    if dirs.is_empty() {
        eprintln!("mkdir: missing operand");
        std::process::exit(1);
    }

    let mut exit_code = 0;
    for dir in dirs {
        let result = if parents {
            fs::create_dir_all(dir)
        } else {
            fs::create_dir(dir)
        };

        match result {
            Ok(()) => {
                if verbose {
                    println!("mkdir: created directory '{}'", dir);
                }
            }
            Err(e) => {
                eprintln!("mkdir: cannot create directory '{}': {}", dir, e);
                exit_code = 1;
            }
        }
    }

    std::process::exit(exit_code);
}
