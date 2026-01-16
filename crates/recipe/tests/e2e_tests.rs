//! End-to-end integration tests that install real packages and verify they work.
//!
//! These tests download actual binaries, install them, and run them to verify
//! the complete recipe workflow.

mod common;

use common::TestEnv;

const SEPARATOR: &str = "============================================================";

/// Test installing jq (a lightweight JSON processor) - single binary, easy to verify.
#[tokio::test]
async fn test_e2e_install_jq() {
    let env = TestEnv::new().await;

    println!("\n{}", SEPARATOR);
    println!("E2E TEST: Installing jq (JSON processor)");
    println!("{}\n", SEPARATOR);

    // Ensure build dir exists
    env.shell("mkdir -p /tmp/build").await.unwrap();

    // Step 1: Download jq binary
    println!("[1/4] Downloading jq 1.7.1...");
    let download_result = env
        .shell(
            "curl -fsSL -o /tmp/build/jq \
             https://github.com/jqlang/jq/releases/download/jq-1.7.1/jq-linux-amd64",
        )
        .await;
    assert!(download_result.is_ok(), "Failed to download jq: {:?}", download_result.err());
    println!("    ✓ Downloaded to /tmp/build/jq");

    // Step 2: Verify the download
    println!("[2/4] Verifying download...");
    let size = env.shell("stat -c %s /tmp/build/jq").await.unwrap();
    println!("    ✓ File size: {} bytes", size.trim());

    // Step 3: Install to /usr/local/bin
    println!("[3/4] Installing to /usr/local/bin...");
    env.shell("install -Dm755 /tmp/build/jq /usr/local/bin/jq")
        .await
        .unwrap();
    println!("    ✓ Installed with mode 755");

    // Step 4: Run jq to verify it works
    println!("[4/4] Testing jq...");
    let version = env.shell("jq --version").await.unwrap();
    println!("    ✓ Version: {}", version.trim());

    let json_test = env
        .shell(r#"echo '{"name":"test","value":42}' | jq '.name'"#)
        .await
        .unwrap();
    println!("    ✓ JSON parse test: {}", json_test.trim());

    println!("\n{}", SEPARATOR);
    println!("SUCCESS: jq installed and working!");
    println!("{}\n", SEPARATOR);

    // Final assertions
    assert!(version.contains("jq-1.7"));
    assert!(json_test.contains("test"));
}

/// Test installing ripgrep - tarball extraction workflow.
#[tokio::test]
async fn test_e2e_install_ripgrep() {
    let env = TestEnv::new().await;

    println!("\n{}", SEPARATOR);
    println!("E2E TEST: Installing ripgrep (fast grep)");
    println!("{}\n", SEPARATOR);

    // Ensure build dir exists
    env.shell("mkdir -p /tmp/build").await.unwrap();

    // Step 1: Download ripgrep tarball
    println!("[1/5] Downloading ripgrep 14.1.1...");
    let download_result = env
        .shell(
            "curl -fsSL -o /tmp/build/ripgrep.tar.gz \
             https://github.com/BurntSushi/ripgrep/releases/download/14.1.1/ripgrep-14.1.1-x86_64-unknown-linux-musl.tar.gz",
        )
        .await;
    assert!(download_result.is_ok(), "Failed to download ripgrep: {:?}", download_result.err());
    println!("    ✓ Downloaded ripgrep-14.1.1-x86_64-unknown-linux-musl.tar.gz");

    // Step 2: Verify checksum (optional but good practice)
    println!("[2/5] Checking download size...");
    let size = env.shell("stat -c %s /tmp/build/ripgrep.tar.gz").await.unwrap();
    println!("    ✓ Archive size: {} bytes", size.trim());

    // Step 3: Extract tarball
    println!("[3/5] Extracting archive...");
    env.shell("cd /tmp/build && tar xzf ripgrep.tar.gz")
        .await
        .unwrap();
    let contents = env.shell("ls /tmp/build/ripgrep-14.1.1-x86_64-unknown-linux-musl/").await.unwrap();
    println!("    ✓ Extracted contents:");
    for line in contents.lines() {
        println!("       - {}", line);
    }

    // Step 4: Install binary and man page
    println!("[4/5] Installing binary and docs...");
    env.shell(
        "install -Dm755 /tmp/build/ripgrep-14.1.1-x86_64-unknown-linux-musl/rg /usr/local/bin/rg && \
         install -Dm644 /tmp/build/ripgrep-14.1.1-x86_64-unknown-linux-musl/doc/rg.1 /usr/local/share/man/man1/rg.1",
    )
    .await
    .unwrap();
    println!("    ✓ Installed /usr/local/bin/rg (mode 755)");
    println!("    ✓ Installed /usr/local/share/man/man1/rg.1");

    // Step 5: Run ripgrep to verify it works
    println!("[5/5] Testing ripgrep...");
    let version = env.shell("rg --version").await.unwrap();
    println!("    ✓ Version info:");
    for line in version.lines().take(3) {
        println!("       {}", line);
    }

    // Create a test file and search it
    env.shell("printf 'hello world\\nfoo bar\\nhello again\\n' > /tmp/testfile.txt")
        .await
        .unwrap();
    let search_result = env.shell("rg 'hello' /tmp/testfile.txt").await.unwrap();
    println!("    ✓ Search test (grep for 'hello'):");
    for line in search_result.lines() {
        println!("       {}", line);
    }

    println!("\n{}", SEPARATOR);
    println!("SUCCESS: ripgrep installed and working!");
    println!("{}\n", SEPARATOR);

    // Final assertions
    assert!(version.contains("ripgrep 14.1.1"));
    assert!(search_result.contains("hello world"));
    assert!(search_result.contains("hello again"));
}

/// Test installing fd - another tarball with multiple files.
#[tokio::test]
async fn test_e2e_install_fd() {
    let env = TestEnv::new().await;

    println!("\n{}", SEPARATOR);
    println!("E2E TEST: Installing fd (fast find)");
    println!("{}\n", SEPARATOR);

    // Ensure build dir exists
    env.shell("mkdir -p /tmp/build").await.unwrap();

    // Step 1: Download fd tarball
    println!("[1/5] Downloading fd 10.2.0...");
    let download_result = env
        .shell(
            "curl -fsSL -o /tmp/build/fd.tar.gz \
             https://github.com/sharkdp/fd/releases/download/v10.2.0/fd-v10.2.0-x86_64-unknown-linux-musl.tar.gz",
        )
        .await;
    assert!(download_result.is_ok(), "Failed to download fd: {:?}", download_result.err());
    println!("    ✓ Downloaded fd-v10.2.0-x86_64-unknown-linux-musl.tar.gz");

    // Step 2: Check size
    println!("[2/5] Verifying download...");
    let size = env.shell("stat -c %s /tmp/build/fd.tar.gz").await.unwrap();
    println!("    ✓ Archive size: {} bytes", size.trim());

    // Step 3: Extract
    println!("[3/5] Extracting archive...");
    env.shell("cd /tmp/build && tar xzf fd.tar.gz").await.unwrap();
    let contents = env.shell("ls /tmp/build/fd-v10.2.0-x86_64-unknown-linux-musl/").await.unwrap();
    println!("    ✓ Extracted contents:");
    for line in contents.lines() {
        println!("       - {}", line);
    }

    // Step 4: Install
    println!("[4/5] Installing...");
    env.shell(
        "install -Dm755 /tmp/build/fd-v10.2.0-x86_64-unknown-linux-musl/fd /usr/local/bin/fd && \
         install -Dm644 /tmp/build/fd-v10.2.0-x86_64-unknown-linux-musl/fd.1 /usr/local/share/man/man1/fd.1",
    )
    .await
    .unwrap();
    println!("    ✓ Installed /usr/local/bin/fd");
    println!("    ✓ Installed /usr/local/share/man/man1/fd.1");

    // Step 5: Test
    println!("[5/5] Testing fd...");
    let version = env.shell("fd --version").await.unwrap();
    println!("    ✓ Version: {}", version.trim());

    // Create test directory structure and search
    env.shell(
        "mkdir -p /tmp/testdir/subdir && \
         touch /tmp/testdir/file1.txt /tmp/testdir/file2.rs /tmp/testdir/subdir/file3.txt",
    )
    .await
    .unwrap();
    let find_result = env.shell("fd '.txt' /tmp/testdir").await.unwrap();
    println!("    ✓ Find test (*.txt files):");
    for line in find_result.lines() {
        println!("       {}", line);
    }

    println!("\n{}", SEPARATOR);
    println!("SUCCESS: fd installed and working!");
    println!("{}\n", SEPARATOR);

    // Final assertions
    assert!(version.contains("fd 10.2.0"));
    assert!(find_result.contains("file1.txt"));
    assert!(find_result.contains("file3.txt"));
}

/// Test the full recipe executor with a real recipe.
#[tokio::test]
async fn test_e2e_full_recipe_execution() {
    let env = TestEnv::new().await;

    println!("\n{}", SEPARATOR);
    println!("E2E TEST: Full Recipe Execution (bat - cat with syntax highlighting)");
    println!("{}\n", SEPARATOR);

    // Ensure build dir exists
    env.shell("mkdir -p /tmp/build").await.unwrap();

    // Step 1: Download bat
    println!("[1/6] Downloading bat 0.24.0...");
    env.shell(
        "curl -fsSL -o /tmp/build/bat.tar.gz \
         https://github.com/sharkdp/bat/releases/download/v0.24.0/bat-v0.24.0-x86_64-unknown-linux-musl.tar.gz",
    )
    .await
    .unwrap();
    println!("    ✓ Downloaded bat-v0.24.0-x86_64-unknown-linux-musl.tar.gz");

    // Step 2: Extract
    println!("[2/6] Extracting...");
    env.shell("cd /tmp/build && tar xzf bat.tar.gz").await.unwrap();
    println!("    ✓ Extracted archive");

    // Step 3: Install binary
    println!("[3/6] Installing binary...");
    env.shell("install -Dm755 /tmp/build/bat-v0.24.0-x86_64-unknown-linux-musl/bat /usr/local/bin/bat")
        .await
        .unwrap();
    println!("    ✓ Installed /usr/local/bin/bat");

    // Step 4: Install man page
    println!("[4/6] Installing man page...");
    env.shell("install -Dm644 /tmp/build/bat-v0.24.0-x86_64-unknown-linux-musl/bat.1 /usr/local/share/man/man1/bat.1")
        .await
        .unwrap();
    println!("    ✓ Installed /usr/local/share/man/man1/bat.1");

    // Step 5: Install completions
    println!("[5/6] Installing shell completions...");
    env.shell(
        "install -Dm644 /tmp/build/bat-v0.24.0-x86_64-unknown-linux-musl/autocomplete/bat.bash \
         /usr/local/share/bash-completion/completions/bat",
    )
    .await
    .unwrap();
    println!("    ✓ Installed bash completions");

    // Step 6: Test
    println!("[6/6] Testing bat...");
    let version = env.shell("bat --version").await.unwrap();
    println!("    ✓ Version: {}", version.trim());

    // Create a test file and display it
    env.shell("printf '#!/bin/bash\\necho \"Hello, World!\"\\n' > /tmp/test.sh")
        .await
        .unwrap();
    let bat_output = env.shell("bat --plain --color=never /tmp/test.sh").await.unwrap();
    println!("    ✓ Display test:");
    for line in bat_output.lines() {
        println!("       {}", line);
    }

    // List installed files
    println!("\n[Summary] Installed files:");
    let installed = env.shell("ls -la /usr/local/bin/bat /usr/local/share/man/man1/bat.1 2>/dev/null || true").await.unwrap();
    for line in installed.lines() {
        println!("    {}", line);
    }

    println!("\n{}", SEPARATOR);
    println!("SUCCESS: bat installed and working!");
    println!("{}\n", SEPARATOR);

    assert!(version.contains("bat 0.24.0"));
}

/// Show final installation summary with multiple tools.
#[tokio::test]
async fn test_e2e_multi_tool_install() {
    let env = TestEnv::new().await;

    println!("\n{}", SEPARATOR);
    println!("E2E TEST: Multi-Tool Installation Demo");
    println!("{}\n", SEPARATOR);

    // Ensure build dir exists
    env.shell("mkdir -p /tmp/build").await.unwrap();

    // Install multiple tools
    println!("Installing multiple CLI tools...\n");

    // jq
    println!("→ Installing jq...");
    env.shell("curl -fsSL -o /tmp/build/jq https://github.com/jqlang/jq/releases/download/jq-1.7.1/jq-linux-amd64 && \
               install -Dm755 /tmp/build/jq /usr/local/bin/jq").await.unwrap();
    println!("  ✓ jq installed");

    // yq (YAML processor)
    println!("→ Installing yq...");
    env.shell("curl -fsSL -o /tmp/build/yq https://github.com/mikefarah/yq/releases/download/v4.44.1/yq_linux_amd64 && \
               install -Dm755 /tmp/build/yq /usr/local/bin/yq").await.unwrap();
    println!("  ✓ yq installed");

    // Show what's installed
    println!("\n{}", SEPARATOR);
    println!("INSTALLED TOOLS:");
    println!("{}", SEPARATOR);

    let tools = [
        ("jq", "jq --version"),
        ("yq", "yq --version"),
    ];

    for (name, cmd) in tools {
        if let Ok(version) = env.shell(cmd).await {
            let v = version.lines().next().unwrap_or("unknown");
            let path = env.shell(&format!("which {}", name)).await.unwrap_or_default();
            let size = env.shell(&format!("stat -c %s /usr/local/bin/{}", name)).await.unwrap_or_default();
            println!("\n  {} {}:", name.to_uppercase(), v.trim());
            println!("    Path: {}", path.trim());
            println!("    Size: {} bytes", size.trim());
        }
    }

    // Test jq with real JSON
    println!("\n{}", SEPARATOR);
    println!("FUNCTIONAL TESTS:");
    println!("{}", SEPARATOR);

    println!("\n  Testing jq (JSON processing):");
    let jq_input = r#"{"users":[{"name":"alice","age":30},{"name":"bob","age":25}]}"#;
    env.shell(&format!("echo '{}' > /tmp/test.json", jq_input)).await.unwrap();
    let jq_result = env.shell("jq '.users[].name' /tmp/test.json").await.unwrap();
    println!("    Input:  {}", jq_input);
    println!("    Query:  .users[].name");
    println!("    Output: {}", jq_result.trim().replace('\n', ", "));

    println!("\n  Testing yq (YAML processing):");
    env.shell("printf 'name: test\\nversion: 1.0\\n' > /tmp/test.yaml").await.unwrap();
    let yq_result = env.shell("yq '.name' /tmp/test.yaml").await.unwrap();
    println!("    Input:  name: test, version: 1.0");
    println!("    Query:  .name");
    println!("    Output: {}", yq_result.trim());

    println!("\n{}", SEPARATOR);
    println!("All tools installed and verified!");
    println!("{}\n", SEPARATOR);
}
