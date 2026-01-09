// TEAM_364: uutils mv wrapper for LevitateOS
extern crate eyra;

fn main() {
    std::process::exit(uu_mv::uumain(std::env::args_os()));
}
