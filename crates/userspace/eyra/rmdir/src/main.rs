// TEAM_364: uutils rmdir wrapper for LevitateOS
extern crate eyra;

fn main() {
    std::process::exit(uu_rmdir::uumain(std::env::args_os()));
}
