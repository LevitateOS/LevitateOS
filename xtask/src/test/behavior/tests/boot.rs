//! Boot tests - verify system boots correctly

use crate::test::behavior::BehaviorTest;

/// Test that the system boots to a login prompt
pub struct BootToLogin;

impl BehaviorTest for BootToLogin {
    fn name(&self) -> &'static str {
        "boot_to_login"
    }

    fn category(&self) -> &'static str {
        "boot"
    }

    fn description(&self) -> &'static str {
        "System boots to login prompt"
    }

    fn phase(&self) -> u8 {
        0 // Boot phase - runs before login
    }

    fn commands(&self) -> Vec<&'static str> {
        vec![] // No commands - just check boot output
    }

    fn expected_patterns(&self) -> Vec<&'static str> {
        vec![
            r"login:", // Login prompt appears
        ]
    }

    fn timeout(&self) -> u64 {
        60 // Boot can take a while
    }
}
