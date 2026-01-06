fn main() {
    println!("cargo:rerun-if-changed=link.ld");
    println!("cargo:rustc-link-arg=-Trepro_crash/link.ld");
}
