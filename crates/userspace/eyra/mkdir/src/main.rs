// TEAM_364: uutils mkdir wrapper for LevitateOS
extern crate eyra;

fn main() {
    std::process::exit(uu_mkdir::uumain(std::env::args_os()));
}
