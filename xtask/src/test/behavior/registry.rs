//! Test registry for behavior tests

use super::tests;
use super::BehaviorTest;

/// Collection of behavior tests
pub struct TestRegistry {
    tests: Vec<Box<dyn BehaviorTest>>,
}

impl TestRegistry {
    pub fn new() -> Self {
        TestRegistry { tests: Vec::new() }
    }

    /// Register a test
    pub fn register<T: BehaviorTest + 'static>(&mut self, test: T) {
        self.tests.push(Box::new(test));
    }

    /// Get all tests
    pub fn all(&self) -> &[Box<dyn BehaviorTest>] {
        &self.tests
    }

    /// Get tests by phase
    pub fn by_phase(&self, phase: u8) -> Vec<&Box<dyn BehaviorTest>> {
        self.tests.iter().filter(|t| t.phase() == phase).collect()
    }

    /// Get tests by category
    pub fn by_category(&self, category: &str) -> Vec<&Box<dyn BehaviorTest>> {
        self.tests
            .iter()
            .filter(|t| t.category() == category)
            .collect()
    }

    /// Get a specific test by name
    pub fn get(&self, name: &str) -> Option<&Box<dyn BehaviorTest>> {
        self.tests.iter().find(|t| t.name() == name)
    }

    /// List all unique categories
    pub fn categories(&self) -> Vec<&str> {
        let mut cats: Vec<_> = self.tests.iter().map(|t| t.category()).collect();
        cats.sort();
        cats.dedup();
        cats
    }
}

impl Default for TestRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Build the default registry with all tests
pub fn default_registry() -> TestRegistry {
    let mut reg = TestRegistry::new();

    // Phase 0: Boot tests
    reg.register(tests::boot::BootToLogin);

    // Phase 1: Shell tests
    reg.register(tests::shell::ShellResponds);

    // Phase 2: General tests
    // Coreutils
    reg.register(tests::coreutils::LsBasic);
    reg.register(tests::coreutils::CatFile);
    reg.register(tests::coreutils::EchoTest);

    // Auth
    reg.register(tests::auth::WhoamiTest);

    // Process
    reg.register(tests::process::PsBasic);
    reg.register(tests::process::UptimeBasic);

    // Network
    reg.register(tests::network::IpAddr);
    reg.register(tests::network::PingLocalhost);

    reg
}
