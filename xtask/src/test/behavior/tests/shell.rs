//! Shell tests - verify shell works correctly

use crate::test::behavior::BehaviorTest;

/// Test that the shell responds to commands
pub struct ShellResponds;

impl BehaviorTest for ShellResponds {
    fn name(&self) -> &'static str {
        "shell_responds"
    }

    fn category(&self) -> &'static str {
        "shell"
    }

    fn description(&self) -> &'static str {
        "Shell executes commands and returns output"
    }

    fn phase(&self) -> u8 {
        1 // Shell phase - runs after login
    }

    fn commands(&self) -> Vec<&'static str> {
        vec!["echo 'SHELL_TEST_OK'"]
    }

    fn expected_patterns(&self) -> Vec<&'static str> {
        vec![r"SHELL_TEST_OK"]
    }
}
