// TEAM_364: uutils cat wrapper for LevitateOS
//
// GNU-compatible cat from uutils-coreutils.
// Replaces hand-written implementation with battle-tested uutils version.

extern crate eyra;

fn main() {
    std::process::exit(uu_cat::uumain(std::env::args_os()));
}
