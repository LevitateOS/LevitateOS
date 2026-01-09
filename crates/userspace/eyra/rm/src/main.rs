// TEAM_364: uutils rm wrapper for LevitateOS
extern crate eyra;

fn main() {
    std::process::exit(uu_rm::uumain(std::env::args_os()));
}
