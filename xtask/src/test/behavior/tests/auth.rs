//! Authentication tests - verify user/identity commands

use crate::test::behavior::BehaviorTest;

/// Test whoami command
pub struct WhoamiTest;

impl BehaviorTest for WhoamiTest {
    fn name(&self) -> &'static str {
        "whoami_test"
    }

    fn category(&self) -> &'static str {
        "auth"
    }

    fn description(&self) -> &'static str {
        "whoami shows current user"
    }

    fn commands(&self) -> Vec<&'static str> {
        vec!["whoami"]
    }

    fn expected_patterns(&self) -> Vec<&'static str> {
        // Should match root or live depending on login user
        vec![r"(root|live)"]
    }
}
