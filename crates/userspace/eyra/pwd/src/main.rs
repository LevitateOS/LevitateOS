// TEAM_364: uutils pwd wrapper for LevitateOS
extern crate eyra;

fn main() {
    std::process::exit(uu_pwd::uumain(std::env::args_os()));
}
