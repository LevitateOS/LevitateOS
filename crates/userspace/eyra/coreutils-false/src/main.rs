// TEAM_364: Minimal false for LevitateOS
// Note: uu_false has linker conflicts with Eyra, using minimal impl
extern crate eyra;

fn main() {
    // false always exits with failure (1)
    std::process::exit(1);
}
