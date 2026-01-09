// TEAM_364: uutils env wrapper for LevitateOS
extern crate eyra;

fn main() {
    std::process::exit(uu_env::uumain(std::env::args_os()));
}
