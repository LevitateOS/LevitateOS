//! Test fixtures - sample recipes for testing.

#![allow(dead_code)]

/// Simple binary recipe that downloads and extracts a tarball.
pub const SIMPLE_BINARY_RECIPE: &str = r#"
(package "testpkg" "1.0"
  (description "Test package")
  (license "MIT")
  (acquire
    (binary
      (x86_64 "https://example.com/test-x86_64.tar.gz")
      (aarch64 "https://example.com/test-aarch64.tar.gz")))
  (build (extract tar-gz))
  (install (to-bin "test-1.0/testbin")))
"#;

/// Recipe with SHA256 verification.
pub const VERIFIED_RECIPE: &str = r#"
(package "verified" "1.0"
  (description "Package with checksum verification")
  (license "MIT")
  (acquire
    (source "https://example.com/verified.tar.gz"
      (verify (sha256 "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")))))
"#;

/// Recipe that creates a user and directories.
pub const CONFIGURE_RECIPE: &str = r#"
(package "configured" "1.0"
  (description "Package with configuration")
  (license "MIT")
  (configure
    (create-user "testuser" system no-login)
    (create-dir "/var/lib/testpkg" "testuser")
    (create-dir "/var/log/testpkg" "testuser")))
"#;

/// Recipe with various install targets.
pub const INSTALL_RECIPE: &str = r#"
(package "installtest" "1.0"
  (description "Package testing install targets")
  (license "MIT")
  (install
    (to-bin "mybin" (mode 755))
    (to-lib "mylib.so")
    (to-config "myconfig.conf" "/etc/myapp/config.conf")
    (to-man "mybin.1")
    (to-share "data.txt" "myapp/data.txt")
    (link "$PREFIX/bin/mybin" "$PREFIX/bin/mybin-alias")))
"#;

/// Recipe with build steps.
pub const BUILD_RECIPE: &str = r#"
(package "buildtest" "1.0"
  (description "Package testing build steps")
  (license "MIT")
  (build
    (run "echo 'Building...'")
    (run "mkdir -p output")
    (run "echo '#!/bin/sh' > output/mybin")
    (run "echo 'echo hello' >> output/mybin")
    (run "chmod +x output/mybin")))
"#;

/// Git clone recipe.
pub const GIT_RECIPE: &str = r#"
(package "gitpkg" "1.0"
  (description "Package from git")
  (license "MIT")
  (acquire
    (git "https://github.com/example/repo.git"
      (tag "v1.0.0"))))
"#;
