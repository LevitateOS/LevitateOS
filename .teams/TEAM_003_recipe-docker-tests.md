# TEAM_003: Recipe Docker Integration Tests

## Goal
Research and plan Docker integration tests for the recipe crate.

## Status: IMPLEMENTED AND PASSING

### Results
```
14 unit tests + 34 integration tests = 48 total tests
All passing!
```

---

## Detailed Implementation Plan

### 1. Dependencies (Cargo.toml)

```toml
[dev-dependencies]
testcontainers = "0.23"
bollard = "0.18"              # Direct Docker API for exec
tokio = { version = "1", features = ["full", "process"] }
tempfile = "3"                # Temp directories for test artifacts
```

### 2. Directory Structure

```
crates/recipe/
├── src/
│   └── ...
├── tests/
│   ├── common/
│   │   ├── mod.rs            # Re-exports
│   │   ├── container.rs      # Container wrapper with exec support
│   │   └── fixtures.rs       # Test recipe files
│   ├── acquire_tests.rs      # Download/git clone tests
│   ├── build_tests.rs        # Extract/compile tests
│   ├── install_tests.rs      # File installation tests
│   └── configure_tests.rs    # User/directory creation tests
└── Cargo.toml
```

### 3. Test Container Setup

**tests/common/container.rs:**
```rust
use bollard::Docker;
use bollard::exec::{CreateExecOptions, StartExecResults};
use testcontainers::{runners::AsyncRunner, GenericImage, ContainerAsync};
use std::path::Path;

pub struct TestEnv {
    pub container: ContainerAsync<GenericImage>,
    pub docker: Docker,
    pub container_id: String,
}

impl TestEnv {
    pub async fn new() -> Self {
        let container = GenericImage::new("ubuntu", "22.04")
            .with_env_var("DEBIAN_FRONTEND", "noninteractive")
            .with_cmd(vec!["bash", "-c",
                "apt-get update && apt-get install -y curl git tar xz-utils bzip2 unzip && \
                 while true; do sleep 1; done"])
            .start()
            .await
            .expect("Failed to start container");

        let docker = Docker::connect_with_local_defaults()
            .expect("Failed to connect to Docker");
        let container_id = container.id().to_string();

        Self { container, docker, container_id }
    }

    /// Execute a command in the container and return stdout
    pub async fn exec(&self, cmd: &[&str]) -> Result<String, String> {
        let exec = self.docker.create_exec(&self.container_id, CreateExecOptions {
            cmd: Some(cmd.iter().map(|s| s.to_string()).collect()),
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            ..Default::default()
        }).await.map_err(|e| e.to_string())?;

        // Start and collect output...
        // (implementation details)
    }

    /// Copy file into container
    pub async fn copy_to(&self, local: &Path, container_path: &str) { ... }

    /// Read file from container
    pub async fn read_file(&self, path: &str) -> Result<String, String> { ... }

    /// Check if file exists in container
    pub async fn file_exists(&self, path: &str) -> bool { ... }

    /// Check file permissions
    pub async fn file_mode(&self, path: &str) -> Option<u32> { ... }
}
```

### 4. Test Scenarios

#### A. Acquire Phase Tests (`acquire_tests.rs`)

| Test | What It Verifies |
|------|------------------|
| `test_acquire_source_download` | Downloads URL, file exists |
| `test_acquire_source_sha256` | SHA256 verification passes |
| `test_acquire_source_sha256_fail` | Bad checksum returns error |
| `test_acquire_binary_x86_64` | Correct arch URL selected |
| `test_acquire_binary_aarch64` | Correct arch URL selected |
| `test_acquire_binary_unknown_arch` | Returns NoUrlForArch error |
| `test_acquire_git_clone` | Clones repo successfully |
| `test_acquire_git_tag` | Clones specific tag |
| `test_acquire_git_branch` | Clones specific branch |

```rust
#[tokio::test]
async fn test_acquire_source_sha256() {
    let env = TestEnv::new().await;

    // Create a test file with known content
    env.exec(&["bash", "-c", "echo -n 'test content' > /tmp/testfile"]).await.unwrap();
    let expected_sha = "6ae8a75555209fd6c44157c0aed8016e763ff435a19cf186f76863140143ff72";

    // Copy recipe that downloads this file
    let recipe = r#"
        (package "test" "1.0"
          (acquire (source "file:///tmp/testfile"
            (verify (sha256 "6ae8a75555209fd6c44157c0aed8016e763ff435a19cf186f76863140143ff72")))))
    "#;

    // Execute and verify
    // ...
}
```

#### B. Build Phase Tests (`build_tests.rs`)

| Test | What It Verifies |
|------|------------------|
| `test_build_extract_tar_gz` | Extracts .tar.gz correctly |
| `test_build_extract_tar_xz` | Extracts .tar.xz correctly |
| `test_build_extract_tar_bz2` | Extracts .tar.bz2 correctly |
| `test_build_extract_zip` | Extracts .zip correctly |
| `test_build_extract_unsupported` | Returns UnsupportedFormat error |
| `test_build_skip` | BuildSpec::Skip does nothing |
| `test_build_run_custom_cmd` | Runs arbitrary command |
| `test_build_variable_expansion` | $PREFIX, $NPROC, $ARCH expand |

```rust
#[tokio::test]
async fn test_build_extract_tar_gz() {
    let env = TestEnv::new().await;

    // Create test archive in container
    env.exec(&["bash", "-c",
        "mkdir -p /tmp/myapp-1.0 && \
         echo 'hello' > /tmp/myapp-1.0/file.txt && \
         tar czf /tmp/build/myapp-1.0.tar.gz -C /tmp myapp-1.0"
    ]).await.unwrap();

    // Execute build phase with extract tar-gz
    // Verify /tmp/build/myapp-1.0/file.txt exists
}
```

#### C. Install Phase Tests (`install_tests.rs`)

| Test | What It Verifies |
|------|------------------|
| `test_install_to_bin` | Binary installed to $PREFIX/bin |
| `test_install_to_bin_mode` | Correct permissions (755) |
| `test_install_to_bin_rename` | dest parameter renames file |
| `test_install_to_lib` | Library installed to $PREFIX/lib |
| `test_install_to_config` | Config file to absolute path |
| `test_install_to_man` | Man page to correct section dir |
| `test_install_to_share` | Data file to $PREFIX/share |
| `test_install_link` | Symlink created correctly |

```rust
#[tokio::test]
async fn test_install_to_bin_mode() {
    let env = TestEnv::new().await;

    // Create test binary in build dir
    env.exec(&["bash", "-c", "echo '#!/bin/sh' > /tmp/build/mybin"]).await.unwrap();

    // Execute install with mode 755
    // Verify: stat -c %a /usr/local/bin/mybin == 755
    let mode = env.file_mode("/usr/local/bin/mybin").await;
    assert_eq!(mode, Some(0o755));
}
```

#### D. Configure Phase Tests (`configure_tests.rs`)

| Test | What It Verifies |
|------|------------------|
| `test_configure_create_user` | User created in /etc/passwd |
| `test_configure_create_user_system` | System user (low UID) |
| `test_configure_create_user_nologin` | Shell is /sbin/nologin |
| `test_configure_create_dir` | Directory created |
| `test_configure_create_dir_owner` | Correct ownership |
| `test_configure_template` | {{VAR}} substitution works |
| `test_configure_run` | Arbitrary command runs |

```rust
#[tokio::test]
async fn test_configure_create_user_system() {
    let env = TestEnv::new().await;

    // Execute configure with create-user system
    // Verify: grep redis /etc/passwd shows UID < 1000
    let passwd = env.read_file("/etc/passwd").await.unwrap();
    let redis_line = passwd.lines().find(|l| l.starts_with("redis:")).unwrap();
    let uid: u32 = redis_line.split(':').nth(2).unwrap().parse().unwrap();
    assert!(uid < 1000, "System user should have UID < 1000");
}
```

#### E. Full Recipe Tests (`integration.rs`)

| Test | What It Verifies |
|------|------------------|
| `test_full_binary_recipe` | ripgrep-style recipe works end-to-end |
| `test_full_source_recipe` | redis-style build-from-source works |
| `test_dry_run_mode` | dry_run=true logs but doesn't execute |
| `test_verbose_mode` | Commands printed to stderr |

### 5. Test Fixtures (`fixtures.rs`)

Pre-defined recipes for testing:

```rust
pub const SIMPLE_BINARY_RECIPE: &str = r#"
(package "testpkg" "1.0"
  (acquire (binary (x86_64 "https://example.com/test.tar.gz")))
  (build (extract tar-gz))
  (install (to-bin "test-1.0/bin")))
"#;

pub const SOURCE_BUILD_RECIPE: &str = r#"
(package "buildtest" "1.0"
  (acquire (source "https://example.com/src.tar.gz"))
  (build
    (configure "./configure --prefix=$PREFIX")
    (compile "make -j$NPROC"))
  (install (to-bin "buildtest")))
"#;
```

### 6. Running Tests

```bash
# Run all tests (including Docker integration)
cargo test -p levitate-recipe

# Run only integration tests
cargo test -p levitate-recipe --test '*'

# Run specific test file
cargo test -p levitate-recipe --test acquire_tests

# Run with output
cargo test -p levitate-recipe -- --nocapture
```

### 7. CI Considerations

```yaml
# .github/workflows/test.yml
- name: Run integration tests
  run: |
    # Docker is available in GitHub Actions by default
    cargo test -p levitate-recipe --test '*'
```

---

## Summary

**Total planned tests: ~30**

| Category | Tests | Coverage |
|----------|-------|----------|
| Acquire | 9 | Downloads, checksums, git |
| Build | 8 | Archive extraction, commands |
| Install | 8 | File placement, permissions |
| Configure | 7 | Users, directories, templates |
| Full E2E | 4 | Complete recipes |

**Key architectural decisions:**
1. Use `testcontainers` for lifecycle + `bollard` for exec
2. Ubuntu 22.04 base image with build tools pre-installed
3. Each test gets isolated container (parallel-safe)
4. Test both success and error paths
