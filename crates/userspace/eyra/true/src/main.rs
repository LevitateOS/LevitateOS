// TEAM_367: uutils true wrapper for LevitateOS
extern crate eyra;

fn main() {
    std::process::exit(uu_true::uumain(std::env::args_os()));
}
