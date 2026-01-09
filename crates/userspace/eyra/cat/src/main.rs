// TEAM_363: Eyra-based cat utility for LevitateOS
//
// Concatenates and prints files to standard output.
// Migrated from no_std levbox version to std.
//
// ## Usage
// ```
// cat [-u] [file...]
// ```
//
// ## Behavior IDs
// - [CAT1] Read file and output to stdout
// - [CAT2] Read from stdin when no files or "-" operand
// - [CAT3] Continue on error, report to stderr

// Required for -Zbuild-std compatibility
extern crate eyra;

use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

const BUF_SIZE: usize = 4096;

/// [CAT2] Cat stdin to stdout until EOF
fn cat_stdin() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buf = [0u8; BUF_SIZE];

    loop {
        let n = stdin.lock().read(&mut buf)?;
        if n == 0 {
            break;
        }
        stdout.write_all(&buf[..n])?;
    }
    Ok(())
}

/// [CAT1] Cat a file to stdout
fn cat_file(path: &str) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut stdout = io::stdout();
    let mut buf = [0u8; BUF_SIZE];

    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        stdout.write_all(&buf[..n])?;
    }
    Ok(())
}

fn print_help() {
    println!("Usage: cat [OPTION]... [FILE]...");
    println!("Concatenate FILE(s) to standard output.");
    println!();
    println!("With no FILE, or when FILE is -, read standard input.");
    println!();
    println!("  -u                  (ignored)");
    println!("      --help          display this help and exit");
    println!("      --version       output version information and exit");
}

fn print_version() {
    println!("cat (LevitateOS) 0.2.0");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for --help or --version first
    for arg in args.iter().skip(1) {
        if arg == "--help" {
            print_help();
            return;
        } else if arg == "--version" {
            print_version();
            return;
        }
    }

    let mut exit_code = 0i32;

    if args.len() <= 1 {
        // [CAT2] No arguments: read from stdin
        if let Err(e) = cat_stdin() {
            eprintln!("cat: stdin: {}", e);
            exit_code = 1;
        }
    } else {
        // Process each file argument
        for arg in args.iter().skip(1) {
            if arg == "-" {
                // [CAT2] "-" means stdin
                if let Err(e) = cat_stdin() {
                    eprintln!("cat: stdin: {}", e);
                    exit_code = 1;
                }
            } else if arg == "-u" {
                // Unbuffered mode - no-op
            } else if arg.starts_with("--") {
                eprintln!("cat: unrecognized option: {}", arg);
                exit_code = 1;
            } else if arg.starts_with('-') && arg.len() > 1 {
                eprintln!("cat: invalid option: {}", arg);
                exit_code = 1;
            } else {
                // [CAT1] Regular file
                if let Err(e) = cat_file(arg) {
                    eprintln!("cat: {}: {}", arg, e);
                    exit_code = 1;
                }
            }
        }
    }

    std::process::exit(exit_code);
}
