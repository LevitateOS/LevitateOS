fn main() {
    println!("cargo:rustc-link-arg=-nostartfiles");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if target_arch == "aarch64" {
        let lib_path = format!("{}/libgcc_eh.a", out_dir);
        let _ = std::process::Command::new("ar").args(["rcs", &lib_path]).status();
        println!("cargo:rustc-link-search=native={}", out_dir);
    }
}
