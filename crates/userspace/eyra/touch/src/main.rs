// TEAM_364: uutils touch wrapper for LevitateOS
extern crate eyra;

fn main() {
    std::process::exit(uu_touch::uumain(std::env::args_os()));
}
