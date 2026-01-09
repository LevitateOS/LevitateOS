// TEAM_364: uutils ln wrapper for LevitateOS
extern crate eyra;

fn main() {
    std::process::exit(uu_ln::uumain(std::env::args_os()));
}
