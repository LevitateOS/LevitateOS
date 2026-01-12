use anyhow::{bail, Context, Result};
use clap::Subcommand;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use crate::disk;

// TEAM_435: Removed Eyra command, added Sysroot and Coreutils
// TEAM_444: Migrated to musl, added Dash (C shell)
// TEAM_451: Added Busybox (replaces coreutils + dash + custom init)
#[derive(Subcommand)]
pub enum BuildCommands {
    /// Build everything (Kernel + Userspace + Disk + Apps)
    All,
    /// Build kernel only
    Kernel,
    /// Build userspace only
    Userspace,
    /// Build initramfs only
    Initramfs,
    /// Build bootable Limine ISO (includes apps)
    Iso,
    /// Ensure musl target is installed (legacy, now a no-op)
    Sysroot,
    /// Build coreutils (Rust, uses musl target)
    Coreutils,
    /// Build brush shell (Rust, uses musl target)
    Brush,
    /// Build dash shell (C, requires musl-gcc)
    Dash,
    /// Build BusyBox (C, requires musl-gcc) - provides init, shell, and 300+ utilities
    Busybox,
}

// TEAM_435: Replaced Eyra with c-gull sysroot approach
// TEAM_438: Uses apps registry for external app builds
// TEAM_444: Migrated to musl - much simpler now!
pub fn build_all(arch: &str) -> Result<()> {
    // Ensure musl target is installed (replaces sysroot build)
    super::sysroot::ensure_rust_musl_target(arch)?;

    // Build all external Rust apps (coreutils, brush, etc.) if not present
    super::apps::ensure_all_built(arch)?;

    // Build C apps if musl-gcc is available (optional)
    if super::c_apps::musl_gcc_available() {
        for app in super::c_apps::C_APPS {
            if !app.exists(arch) {
                // Don't fail build_all if C app build fails - it's optional
                if let Err(e) = app.build(arch) {
                    println!("⚠️  Optional C app {} failed to build: {}", app.name, e);
                }
            }
        }
    } else {
        println!("ℹ️  musl-gcc not found, skipping C apps (dash). Install musl-tools to enable.");
    }

    // TEAM_073: Build userspace first
    build_userspace(arch)?;
    // TEAM_451: Use BusyBox initramfs (replaces old init + dash + coreutils)
    create_busybox_initramfs(arch)?;
    // TEAM_121: Ensure disk image is populated
    disk::install_userspace_to_disk(arch)?;

    build_kernel_with_features(&[], arch)
}

pub fn build_kernel_only(arch: &str) -> Result<()> {
    build_kernel_with_features(&[], arch)
}

/// Build kernel with verbose feature for behavior testing (Rule 4: Silence is Golden)
pub fn build_kernel_verbose(arch: &str) -> Result<()> {
    build_kernel_with_features(&["verbose"], arch)
}

pub fn build_userspace(arch: &str) -> Result<()> {
    println!("Building userspace workspace for {}...", arch);
    
    let target = match arch {
        "aarch64" => "aarch64-unknown-none",
        "x86_64" => "x86_64-unknown-none",
        _ => bail!("Unsupported architecture: {}", arch),
    };

    // TEAM_120: Build the entire userspace workspace
    // We build in-place now as the workspace isolation issues should be resolved
    // by individual build.rs scripts and correct linker arguments.
    let status = Command::new("cargo")
        .current_dir("crates/userspace")
        .args([
            "build",
            "--release",
            "--workspace",
            "--target", target,
        ])
        .status()
        .context("Failed to build userspace workspace")?;

    if !status.success() {
        bail!("Userspace workspace build failed");
    }

    Ok(())
}

// TEAM_435: Uses c-gull sysroot binaries instead of Eyra
// TEAM_444: Migrated to musl - Rust apps use musl target, C apps use musl-gcc
pub fn create_initramfs(arch: &str) -> Result<()> {
    println!("Creating initramfs for {}...", arch);
    let root = PathBuf::from("initrd_root");

    // TEAM_292: Always clean initrd_root to ensure correct arch binaries
    // Without this, stale binaries from other architectures persist
    if root.exists() {
        std::fs::remove_dir_all(&root)?;
    }
    std::fs::create_dir(&root)?;

    // 1. Create content
    std::fs::write(root.join("hello.txt"), "Hello from initramfs!\n")?;

    // 2. Copy userspace binaries (init, shell - bare-metal)
    let binaries = crate::get_binaries(arch)?;
    let target = match arch {
        "aarch64" => "aarch64-unknown-none",
        "x86_64" => "x86_64-unknown-none",
        _ => bail!("Unsupported architecture: {}", arch),
    };
    print!("📦 Creating initramfs ({} binaries)... ", binaries.len());
    let mut count = 0;
    for bin in &binaries {
        let src = PathBuf::from(format!("crates/userspace/target/{}/release/{}", target, bin));
        if src.exists() {
            std::fs::copy(&src, root.join(bin))?;
            count += 1;
        }
    }

    // TEAM_438: Use apps registry for external apps - fail fast on required, skip optional
    for app in super::apps::APPS {
        if app.required {
            // Required apps must exist - fail fast with helpful message
            let src = app.require(arch)?;
            std::fs::copy(&src, root.join(app.binary))?;
            count += 1;

            // Create symlinks for multi-call binaries
            for symlink_name in app.symlinks {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::symlink;
                    let link_path = root.join(symlink_name);
                    let _ = std::fs::remove_file(&link_path);
                    symlink(app.binary, &link_path)?;
                }
                #[cfg(not(unix))]
                {
                    std::fs::copy(&src, root.join(symlink_name))?;
                }
            }

            if app.symlinks.is_empty() {
                println!("  📦 Added {}", app.name);
            } else {
                println!("  📦 Added {} + {} symlinks", app.name, app.symlinks.len());
            }
        } else {
            // Optional apps - include if built, otherwise inform user
            if app.exists(arch) {
                let src = app.output_path(arch);
                std::fs::copy(&src, root.join(app.binary))?;
                count += 1;
                println!("  📦 Added {} (optional)", app.name);
            } else {
                println!("  ℹ️  {} not found (optional). Run 'cargo xtask build {}' to include it.", app.name, app.name);
            }
        }
    }

    // TEAM_444: Include C apps (dash, etc.) if built
    for app in super::c_apps::C_APPS {
        if app.exists(arch) {
            let src = app.output_path(arch);
            let binary_name = std::path::Path::new(app.binary)
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| app.name.to_string());
            std::fs::copy(&src, root.join(&binary_name))?;
            count += 1;
            println!("  📦 Added {} (C)", app.name);
        }
    }

    println!("[DONE] ({} added)", count);

    // 3. Create CPIO archive
    // TEAM_327: Use arch-specific filename to prevent cross-arch contamination
    // usage: find . | cpio -o -H newc > ../initramfs_{arch}.cpio
    let cpio_filename = format!("initramfs_{}.cpio", arch);
    let cpio_file = std::fs::File::create(&cpio_filename)?;
    
    let find = Command::new("find")
        .current_dir(&root)
        .arg(".")
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to run find")?;

    let mut cpio = Command::new("cpio")
        .current_dir(&root)
        .args(["-o", "-H", "newc"])
        .stdin(find.stdout.unwrap())
        .stdout(cpio_file)
        .spawn()
        .context("Failed to run cpio")?;

    let status = cpio.wait()?;
    if !status.success() {
        bail!("cpio failed");
    }

    Ok(())
}

/// TEAM_451: Create BusyBox-based initramfs
/// Single binary provides init, shell, and 300+ utilities
pub fn create_busybox_initramfs(arch: &str) -> Result<()> {
    println!("📦 Creating BusyBox initramfs for {}...", arch);
    
    // Require BusyBox to be built
    let busybox_path = super::busybox::require(arch)?;
    
    let root = PathBuf::from("initrd_root");

    // Clean and create directory structure
    if root.exists() {
        std::fs::remove_dir_all(&root)?;
    }
    std::fs::create_dir_all(&root)?;
    std::fs::create_dir_all(root.join("bin"))?;
    std::fs::create_dir_all(root.join("sbin"))?;
    std::fs::create_dir_all(root.join("etc"))?;
    std::fs::create_dir_all(root.join("proc"))?;
    std::fs::create_dir_all(root.join("sys"))?;
    std::fs::create_dir_all(root.join("tmp"))?;
    std::fs::create_dir_all(root.join("dev"))?;
    std::fs::create_dir_all(root.join("root"))?;

    // Copy BusyBox binary
    std::fs::copy(&busybox_path, root.join("bin/busybox"))?;
    
    // Make it executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(root.join("bin/busybox"))?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(root.join("bin/busybox"), perms)?;
    }

    // Create symlinks for all applets
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        
        for (applet, dir) in super::busybox::applets() {
            let link_path = root.join(dir).join(applet);
            let target = if *dir == "sbin" {
                "../bin/busybox"
            } else {
                "busybox"
            };
            let _ = std::fs::remove_file(&link_path);
            symlink(target, &link_path)?;
        }
    }
    
    // Create /init as a copy of busybox (kernel entry point)
    // TEAM_451: Can't use symlink - kernel ELF loader doesn't follow symlinks
    let _ = std::fs::remove_file(root.join("init"));
    std::fs::copy(&busybox_path, root.join("init"))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(root.join("init"))?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(root.join("init"), perms)?;
    }

    // Create /etc/inittab (per user answers: respawn shell, mount proc/sys)
    let inittab = r#"# LevitateOS BusyBox init configuration
# TEAM_451: Generated by xtask

# System initialization
::sysinit:/bin/echo "LevitateOS (BusyBox) starting..."
::sysinit:/bin/mount -t proc proc /proc
::sysinit:/bin/mount -t sysfs sysfs /sys

# Start interactive shell (respawn if it exits)
::respawn:-/bin/ash

# Handle Ctrl+Alt+Del
::ctrlaltdel:/sbin/reboot

# Shutdown hooks
::shutdown:/bin/echo "System shutting down..."
"#;
    std::fs::write(root.join("etc/inittab"), inittab)?;

    // Create /etc/passwd
    let passwd = "root:x:0:0:root:/root:/bin/ash\n";
    std::fs::write(root.join("etc/passwd"), passwd)?;

    // Create /etc/group
    let group = "root:x:0:\n";
    std::fs::write(root.join("etc/group"), group)?;

    // Create /etc/profile
    let profile = r#"export PATH=/bin:/sbin
export HOME=/root
export PS1='LevitateOS# '
alias ll='ls -la'
"#;
    std::fs::write(root.join("etc/profile"), profile)?;

    // Create sample files
    std::fs::write(root.join("etc/motd"), "Welcome to LevitateOS!\n")?;
    std::fs::write(root.join("root/hello.txt"), "Hello from BusyBox initramfs!\n")?;

    // Show what we created
    let applet_count = super::busybox::applets().len();
    println!("  📦 BusyBox binary + {} applet symlinks", applet_count);
    println!("  📄 /etc/inittab, passwd, group, profile");

    // Create CPIO archive
    let cpio_filename = format!("initramfs_{}.cpio", arch);
    let cpio_file = std::fs::File::create(&cpio_filename)?;
    
    let find = Command::new("find")
        .current_dir(&root)
        .arg(".")
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to run find")?;

    let mut cpio = Command::new("cpio")
        .current_dir(&root)
        .args(["-o", "-H", "newc"])
        .stdin(find.stdout.unwrap())
        .stdout(cpio_file)
        .spawn()
        .context("Failed to run cpio")?;

    let status = cpio.wait()?;
    if !status.success() {
        bail!("cpio failed");
    }

    // Show final size
    let metadata = std::fs::metadata(&cpio_filename)?;
    let size_kb = metadata.len() / 1024;
    println!("✅ BusyBox initramfs created: {} ({} KB)", cpio_filename, size_kb);

    Ok(())
}

/// TEAM_435: Create test-specific initramfs with coreutils.
/// TEAM_438: Uses apps registry for external apps.
/// Includes init, shell, and required apps for testing.
pub fn create_test_initramfs(arch: &str) -> Result<()> {
    println!("Creating test initramfs for {}...", arch);
    let root = PathBuf::from("initrd_test_root");

    // Clean and create directory
    if root.exists() {
        std::fs::remove_dir_all(&root)?;
    }
    std::fs::create_dir(&root)?;

    let bare_target = match arch {
        "aarch64" => "aarch64-unknown-none",
        "x86_64" => "x86_64-unknown-none",
        _ => bail!("Unsupported architecture: {}", arch),
    };

    // Copy init and shell for boot
    let init_src = PathBuf::from(format!("crates/userspace/target/{}/release/init", bare_target));
    let shell_src = PathBuf::from(format!("crates/userspace/target/{}/release/shell", bare_target));

    if init_src.exists() {
        std::fs::copy(&init_src, root.join("init"))?;
    }
    if shell_src.exists() {
        std::fs::copy(&shell_src, root.join("shell"))?;
    }

    // Create hello.txt for cat test
    std::fs::write(root.join("hello.txt"), "Hello from initramfs!\n")?;

    // TEAM_438: Use apps registry - only include required apps for test initramfs
    let mut app_count = 0;
    for app in super::apps::required_apps() {
        let src = app.require(arch)?;
        std::fs::copy(&src, root.join(app.binary))?;
        app_count += 1;

        // Create symlinks for multi-call binaries
        for symlink_name in app.symlinks {
            #[cfg(unix)]
            {
                use std::os::unix::fs::symlink;
                let link_path = root.join(symlink_name);
                let _ = std::fs::remove_file(&link_path);
                symlink(app.binary, &link_path)?;
            }
            #[cfg(not(unix))]
            {
                std::fs::copy(&src, root.join(symlink_name))?;
            }
        }
    }

    println!("📦 Test initramfs: {} apps + init/shell", app_count);

    // Create CPIO archive
    let cpio_file = std::fs::File::create("initramfs_test.cpio")?;
    
    let find = Command::new("find")
        .current_dir(&root)
        .arg(".")
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to run find")?;

    let mut cpio = Command::new("cpio")
        .current_dir(&root)
        .args(["-o", "-H", "newc"])
        .stdin(find.stdout.unwrap())
        .stdout(cpio_file)
        .spawn()
        .context("Failed to run cpio")?;

    let status = cpio.wait()?;
    if !status.success() {
        bail!("cpio failed");
    }

    println!("✅ Created initramfs_test.cpio");
    Ok(())
}

fn build_kernel_with_features(features: &[&str], arch: &str) -> Result<()> {
    println!("Building kernel for {}...", arch);
    let target = match arch {
        "aarch64" => "aarch64-unknown-none",
        "x86_64" => "x86_64-unknown-none",
        _ => bail!("Unsupported architecture: {}", arch),
    };

    let mut args = vec![
        "build".to_string(),
        "-Z".to_string(), "build-std=core,alloc".to_string(),
        "--release".to_string(),
        "--target".to_string(), target.to_string(),
        "-p".to_string(), "levitate-kernel".to_string(),  // TEAM_426: Only build kernel, not all workspace members
    ];

    if !features.is_empty() {
        args.push("--features".to_string());
        args.push(features.join(","));
    }

    // Kernel is its own workspace - build from kernel directory
    let status = Command::new("cargo")
        .current_dir("crates/kernel")
        .args(&args)
        .status()
        .context("Failed to run cargo build")?;

    if !status.success() {
        bail!("Kernel build failed");
    }

    // Convert to binary for boot protocol support (Rule 38)
    if arch == "aarch64" {
        println!("Converting to raw binary...");
        let objcopy_status = Command::new("aarch64-linux-gnu-objcopy")
            .args([
                "-O", "binary",
                "crates/kernel/target/aarch64-unknown-none/release/levitate-kernel",
                "kernel64_rust.bin",
            ])
            .status()
            .context("Failed to run objcopy - is aarch64-linux-gnu-objcopy installed?")?;

        if !objcopy_status.success() {
            bail!("objcopy failed");
        }
    } else {
        // x86_64 uses multiboot2 (ELF) directly or needs different conversion
        println!("x86_64 kernel build complete (ELF format for multiboot2)");
    }

    Ok(())
}

/// TEAM_283: Build a bootable Limine ISO
// TEAM_435: Replaced Eyra with c-gull sysroot
// TEAM_444: Migrated to musl
pub fn build_iso(arch: &str) -> Result<()> {
    build_iso_internal(&[], arch, false)
}

/// TEAM_286: Build ISO with verbose feature for behavior testing
pub fn build_iso_verbose(arch: &str) -> Result<()> {
    build_iso_internal(&["verbose"], arch, false)
}

/// TEAM_374: Build ISO for testing with test initramfs
pub fn build_iso_test(arch: &str) -> Result<()> {
    build_iso_internal(&["verbose"], arch, true)
}

fn build_iso_internal(features: &[&str], arch: &str, use_test_initramfs: bool) -> Result<()> {
    if arch != "x86_64" {
        bail!("ISO build currently only supported for x86_64");
    }

    println!("💿 Building Limine ISO for {}...", arch);

    // TEAM_438: Build sysroot and all external apps if not present
    // TEAM_444: Now just ensures musl target is installed
    super::sysroot::ensure_rust_musl_target(arch)?;
    super::apps::ensure_all_built(arch)?;

    build_userspace(arch)?;
    // TEAM_451: Always use BusyBox initramfs now
    create_busybox_initramfs(arch)?;
    crate::disk::install_userspace_to_disk(arch)?;
    build_kernel_with_features(features, arch)?;

    let iso_root = PathBuf::from("iso_root");
    let boot_dir = iso_root.join("boot");
    
    // Clean and create staging directory
    if iso_root.exists() {
        std::fs::remove_dir_all(&iso_root)?;
    }
    std::fs::create_dir_all(&boot_dir)?;

    // 2. Copy components to ISO root
    let kernel_path = "crates/kernel/target/x86_64-unknown-none/release/levitate-kernel";
    // TEAM_374: Use test initramfs when in test mode
    let initramfs_path = if use_test_initramfs {
        "initramfs_test.cpio".to_string()
    } else {
        format!("initramfs_{}.cpio", arch)
    };
    let limine_cfg_path = "limine.cfg";

    std::fs::copy(kernel_path, boot_dir.join("levitate-kernel"))
        .context("Failed to copy levitate-kernel to ISO boot dir")?;
    if std::path::Path::new(&initramfs_path).exists() {
        std::fs::copy(&initramfs_path, boot_dir.join("initramfs.cpio"))
            .context("Failed to copy initramfs to ISO boot dir")?;
    }
    std::fs::copy(limine_cfg_path, iso_root.join("limine.cfg"))
        .context("Failed to copy limine.cfg - ensure it exists in repo root")?;

    // 3. Download/Prepare Limine binaries if needed
    prepare_limine_binaries(&iso_root)?;

    // 4. Create ISO using xorriso
    let iso_file = "levitate.iso";
    let status = Command::new("xorriso")
        .args([
            "-as", "mkisofs",
            "-b", "limine-bios-cd.bin",
            "-no-emul-boot", "-boot-load-size", "4", "-boot-info-table",
            "--efi-boot", "limine-uefi-cd.bin",
            "-efi-boot-part", "--efi-boot-image", "--protective-msdos-label",
            &iso_root.to_string_lossy(),
            "-o", iso_file,
        ])
        .status()
        .context("Failed to run xorriso")?;

    if !status.success() {
        bail!("xorriso failed to create ISO");
    }

    println!("✅ ISO created: {}", iso_file);
    Ok(())
}

// TEAM_435: build_eyra() removed - replaced by build::external::build_coreutils()

fn prepare_limine_binaries(iso_root: &PathBuf) -> Result<()> {
    let limine_dir = PathBuf::from("limine-bin");
    let files = [
        "limine-bios-cd.bin",
        "limine-uefi-cd.bin",
        "limine-bios.sys",
    ];
    
    // TEAM_304: Check if all required files exist, not just directory
    let all_files_exist = files.iter().all(|f| limine_dir.join(f).exists());
    
    if !all_files_exist {
        println!("📥 Downloading Limine binaries (v7.x)...");
        std::fs::create_dir_all(&limine_dir)?;
        
        let base_url = "https://github.com/limine-bootloader/limine/raw/v7.x-binary/";

        for file in &files {
            let url = format!("{}{}", base_url, file);
            let output = limine_dir.join(file);
            println!("  Fetching {}...", file);
            
            let status = Command::new("curl")
                .args(["-L", "-f", "-o", output.to_str().unwrap(), &url])
                .status()
                .context(format!("Failed to run curl for {}", file))?;
            
            if !status.success() {
                bail!("Failed to download {} from {}", file, url);
            }
        }
    }

    // Copy to ISO root for xorriso
    for file in &files {
        let src = limine_dir.join(file);
        let dst = iso_root.join(file);
        std::fs::copy(&src, &dst)
            .with_context(|| format!("Failed to copy {} to {}", src.display(), dst.display()))?;
    }

    Ok(())
}
