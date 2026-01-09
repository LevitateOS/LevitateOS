// TEAM_364: uutils cp wrapper for LevitateOS
extern crate eyra;

fn main() {
    std::process::exit(uu_cp::uumain(std::env::args_os()));
}
