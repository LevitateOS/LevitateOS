// TEAM_364: uutils echo wrapper for LevitateOS
extern crate eyra;

fn main() {
    std::process::exit(uu_echo::uumain(std::env::args_os()));
}
