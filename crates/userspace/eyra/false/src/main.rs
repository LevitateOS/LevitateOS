// TEAM_367: uutils false wrapper for LevitateOS
extern crate eyra;

fn main() {
    std::process::exit(uu_false::uumain(std::env::args_os()));
}
