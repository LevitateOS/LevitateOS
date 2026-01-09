// TEAM_364: uutils ls wrapper for LevitateOS
extern crate eyra;

fn main() {
    std::process::exit(uu_ls::uumain(std::env::args_os()));
}
