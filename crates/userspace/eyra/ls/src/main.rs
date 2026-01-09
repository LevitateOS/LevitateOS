// TEAM_363: Eyra-based ls utility for LevitateOS

extern crate eyra;

use std::env;
use std::fs;
use std::path::Path;

fn print_help() {
    println!("Usage: ls [OPTION]... [FILE]...");
    println!("List information about the FILEs (the current directory by default).");
    println!();
    println!("  -a, --all            do not ignore entries starting with .");
    println!("  -l                   use a long listing format");
    println!("  -1                   list one file per line");
    println!("      --help           display this help and exit");
    println!("      --version        output version information and exit");
}

fn print_version() {
    println!("ls (LevitateOS) 0.2.0");
}

struct Options {
    all: bool,
    long: bool,
    one_per_line: bool,
}

fn list_dir(path: &Path, opts: &Options) -> bool {
    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("ls: cannot access '{}': {}", path.display(), e);
            return false;
        }
    };

    let mut names: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name();
            let name_str = name.to_string_lossy();
            opts.all || !name_str.starts_with('.')
        })
        .collect();

    names.sort_by_key(|e| e.file_name());

    for entry in names {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if opts.long {
            let meta = entry.metadata().ok();
            let file_type = if let Some(ref m) = meta {
                if m.is_dir() { 'd' } else { '-' }
            } else {
                '?'
            };
            let size = meta.map(|m| m.len()).unwrap_or(0);
            println!("{}rw-r--r--  1 root root {:>8} {}", file_type, size, name_str);
        } else if opts.one_per_line {
            println!("{}", name_str);
        } else {
            print!("{}  ", name_str);
        }
    }

    if !opts.long && !opts.one_per_line {
        println!();
    }

    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options {
        all: false,
        long: false,
        one_per_line: true, // Default for simplicity
    };
    let mut paths: Vec<&str> = Vec::new();

    for arg in args.iter().skip(1) {
        if arg == "--help" {
            print_help();
            return;
        } else if arg == "--version" {
            print_version();
            return;
        } else if arg == "--all" {
            opts.all = true;
        } else if arg.starts_with("--") {
            eprintln!("ls: unrecognized option: {}", arg);
            std::process::exit(2);
        } else if arg.starts_with('-') && arg.len() > 1 {
            for c in arg.chars().skip(1) {
                match c {
                    'a' => opts.all = true,
                    'l' => opts.long = true,
                    '1' => opts.one_per_line = true,
                    _ => {
                        eprintln!("ls: invalid option -- '{}'", c);
                        std::process::exit(2);
                    }
                }
            }
        } else {
            paths.push(arg);
        }
    }

    let mut success = true;
    if paths.is_empty() {
        if !list_dir(Path::new("."), &opts) {
            success = false;
        }
    } else {
        let multi = paths.len() > 1;
        for (i, path) in paths.iter().enumerate() {
            if multi {
                if i > 0 {
                    println!();
                }
                println!("{}:", path);
            }
            if !list_dir(Path::new(path), &opts) {
                success = false;
            }
        }
    }

    if !success {
        std::process::exit(1);
    }
}
