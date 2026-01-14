//! Behavior testing framework for LevitateOS
//!
//! This module provides a framework for testing that LevitateOS components
//! work correctly when booted inside a VM.

use anyhow::{bail, Context, Result};
use std::time::{Duration, Instant};

pub mod registry;
pub mod tests;

use super::helpers::TestVm;

/// A behavior test for LevitateOS
pub trait BehaviorTest: Send + Sync {
    /// Unique test identifier (e.g., "ls_basic")
    fn name(&self) -> &'static str;

    /// Test category (e.g., "coreutils")
    fn category(&self) -> &'static str;

    /// Brief description of what this tests
    fn description(&self) -> &'static str;

    /// Test phase: 0=boot, 1=shell, 2=general
    /// Tests in earlier phases must pass before later phases run
    fn phase(&self) -> u8 {
        2 // Default: general phase
    }

    /// Commands to send to the VM (empty for boot tests that just wait)
    fn commands(&self) -> Vec<&'static str>;

    /// Expected patterns in output (regex)
    fn expected_patterns(&self) -> Vec<&'static str>;

    /// Patterns that should NOT appear (errors)
    fn forbidden_patterns(&self) -> Vec<&'static str> {
        vec![]
    }

    /// Timeout for this specific test (seconds)
    fn timeout(&self) -> u64 {
        10
    }
}

/// Result of running a single test
#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_name: String,
    pub category: String,
    pub passed: bool,
    pub duration_ms: u64,
    pub error: Option<String>,
}

impl TestResult {
    pub fn passed(name: &str, category: &str, duration_ms: u64) -> Self {
        TestResult {
            test_name: name.to_string(),
            category: category.to_string(),
            passed: true,
            duration_ms,
            error: None,
        }
    }

    pub fn failed(name: &str, category: &str, error: String, duration_ms: u64) -> Self {
        TestResult {
            test_name: name.to_string(),
            category: category.to_string(),
            passed: false,
            duration_ms,
            error: Some(error),
        }
    }
}

/// Runs behavior tests against a LevitateOS VM
pub struct TestRunner {
    vm: TestVm,
    results: Vec<TestResult>,
    verbose: bool,
    logged_in: bool,
}

impl TestRunner {
    /// Start LevitateOS VM and prepare for testing
    pub fn new(verbose: bool) -> Result<Self> {
        println!("Starting LevitateOS VM...");
        let vm = TestVm::start_levitate()?;

        Ok(TestRunner {
            vm,
            results: Vec::new(),
            verbose,
            logged_in: false,
        })
    }

    /// Wait for boot and login as specified user
    pub fn wait_for_boot_and_login(&mut self, user: &str) -> Result<()> {
        println!("Waiting for login prompt...");

        // Wait for login prompt (60 second timeout for boot)
        if !self.vm.wait_for_pattern(r"login:", 60)? {
            let output = self.vm.read_output()?;
            bail!(
                "Timeout waiting for login prompt. Output:\n{}",
                output.chars().take(2000).collect::<String>()
            );
        }

        println!("Got login prompt, logging in as {}...", user);

        // Send username
        self.vm.send_line(user)?;
        std::thread::sleep(Duration::from_millis(500));

        // Wait for password prompt
        if !self.vm.wait_for_pattern(r"(?i)password:", 10)? {
            // No password prompt - might be passwordless login
            // Check if we already have a shell prompt
            if !self.vm.wait_for_pattern(r"[$#]\s*$", 5)? {
                let output = self.vm.read_output()?;
                bail!(
                    "Timeout waiting for password or shell prompt. Output:\n{}",
                    output.chars().take(2000).collect::<String>()
                );
            }
        } else {
            // Send password (default: username is the password)
            // root:root, live:live
            self.vm.send_line(user)?;
            std::thread::sleep(Duration::from_millis(500));

            // Wait for shell prompt
            if !self.vm.wait_for_pattern(r"[$#]\s*$", 10)? {
                let output = self.vm.read_output()?;
                bail!(
                    "Login failed - incorrect password or shell didn't start. Output:\n{}",
                    output.chars().take(2000).collect::<String>()
                );
            }
        }

        self.logged_in = true;
        println!("Logged in successfully.");
        Ok(())
    }

    /// Run a single test
    pub fn run_test(&mut self, test: &dyn BehaviorTest) -> TestResult {
        let start = Instant::now();
        let name = test.name();
        let category = test.category();

        if self.verbose {
            println!("  Running: {} - {}", name, test.description());
        }

        // For boot tests (phase 0), we just check patterns in existing output
        // For other tests, we send commands and check output
        let result = if test.phase() == 0 {
            self.run_boot_test(test)
        } else {
            self.run_command_test(test)
        };

        let duration_ms = start.elapsed().as_millis() as u64;

        match result {
            Ok(()) => TestResult::passed(name, category, duration_ms),
            Err(e) => TestResult::failed(name, category, e.to_string(), duration_ms),
        }
    }

    fn run_boot_test(&self, test: &dyn BehaviorTest) -> Result<()> {
        // For boot tests, we wait for each expected pattern
        for pattern in test.expected_patterns() {
            if !self.vm.wait_for_pattern(pattern, test.timeout())? {
                let output = self.vm.read_output()?;
                bail!(
                    "Expected pattern not found: '{}'\nOutput:\n{}",
                    pattern,
                    output.chars().take(1000).collect::<String>()
                );
            }
        }

        // Check forbidden patterns in the accumulated output
        let output = self.vm.read_output()?;
        for pattern in test.forbidden_patterns() {
            let re = regex::Regex::new(pattern)?;
            if re.is_match(&output) {
                bail!("Forbidden pattern found: {}", pattern);
            }
        }

        Ok(())
    }

    fn run_command_test(&mut self, test: &dyn BehaviorTest) -> Result<()> {
        // Record output position before commands
        let before_len = self.vm.read_output()?.len();

        // Send all commands
        for cmd in test.commands() {
            self.vm.send_line(cmd)?;
            std::thread::sleep(Duration::from_millis(200));
        }

        // Wait for shell prompt after commands
        if !self.vm.wait_for_pattern(r"[$#]\s*$", test.timeout())? {
            bail!("Timeout waiting for command completion");
        }

        // Get new output since commands were sent
        let full_output = self.vm.read_output()?;
        let new_output = &full_output[before_len..];

        // Check expected patterns in new output
        for pattern in test.expected_patterns() {
            let re = regex::Regex::new(pattern)?;
            if !re.is_match(new_output) {
                bail!(
                    "Expected pattern not found: '{}'\nOutput:\n{}",
                    pattern,
                    new_output.chars().take(500).collect::<String>()
                );
            }
        }

        // Check forbidden patterns
        for pattern in test.forbidden_patterns() {
            let re = regex::Regex::new(pattern)?;
            if re.is_match(new_output) {
                bail!("Forbidden pattern found: {}", pattern);
            }
        }

        Ok(())
    }

    /// Run all tests from a registry
    pub fn run_all(&mut self, registry: &registry::TestRegistry, user: &str) -> Result<()> {
        // Phase 0: Boot tests (run before login)
        println!("\n=== Phase 0: Boot ===");
        let phase0_tests = registry.by_phase(0);
        for test in &phase0_tests {
            let result = self.run_test(test.as_ref());
            self.print_result(&result);
            let passed = result.passed;
            self.results.push(result);
            if !passed {
                bail!("Boot test failed - aborting remaining tests");
            }
        }

        // Login after boot tests pass
        self.wait_for_boot_and_login(user)?;

        // Phase 1: Shell tests
        println!("\n=== Phase 1: Shell ===");
        let phase1_tests = registry.by_phase(1);
        let mut phase1_failed = false;
        for test in &phase1_tests {
            let result = self.run_test(test.as_ref());
            self.print_result(&result);
            if !result.passed {
                phase1_failed = true;
            }
            self.results.push(result);
        }
        if phase1_failed {
            bail!("Shell tests failed - aborting remaining tests");
        }

        // Phase 2: General tests (grouped by category)
        println!("\n=== Phase 2: General ===");
        let phase2_tests = registry.by_phase(2);

        // Group by category for nicer output
        let mut categories: Vec<&str> = phase2_tests.iter().map(|t| t.category()).collect();
        categories.sort();
        categories.dedup();

        for category in categories {
            println!("\n  Category: {}", category);
            let cat_tests: Vec<_> = phase2_tests
                .iter()
                .filter(|t| t.category() == category)
                .collect();
            for test in cat_tests {
                let result = self.run_test(test.as_ref());
                self.print_result(&result);
                self.results.push(result);
            }
        }

        Ok(())
    }

    fn print_result(&self, result: &TestResult) {
        let status = if result.passed { "PASS" } else { "FAIL" };
        let duration = format!("{:.1}s", result.duration_ms as f64 / 1000.0);

        if result.passed {
            println!("    [{}] {} ({})", status, result.test_name, duration);
        } else {
            println!("    [{}] {} ({})", status, result.test_name, duration);
            if let Some(ref err) = result.error {
                // Indent error message
                for line in err.lines().take(5) {
                    println!("           {}", line);
                }
            }
        }
    }

    /// Print summary and save artifacts
    pub fn summarize(&mut self) -> Result<bool> {
        let total = self.results.len();
        let passed = self.results.iter().filter(|r| r.passed).count();
        let failed = total - passed;

        println!("\n=== Summary ===");
        println!("Passed: {}/{}", passed, total);
        if failed > 0 {
            println!("Failed: {}", failed);
            println!("\nFailed tests:");
            for result in &self.results {
                if !result.passed {
                    println!("  - {}: {}", result.test_name, result.error.as_deref().unwrap_or("unknown error"));
                }
            }
        }

        // Save artifact
        self.vm.set_test_name("behavior");
        let artifact = self.vm.save_artifact()?;
        println!("\nArtifact saved: {}", artifact);

        Ok(failed == 0)
    }

    /// Stop VM and cleanup
    pub fn stop(mut self) -> Result<()> {
        self.vm.stop()
    }
}
