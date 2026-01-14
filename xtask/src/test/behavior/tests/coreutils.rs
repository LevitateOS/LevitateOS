//! Coreutils tests - verify basic file/text utilities

use crate::test::behavior::BehaviorTest;

/// Test basic ls functionality
pub struct LsBasic;

impl BehaviorTest for LsBasic {
    fn name(&self) -> &'static str {
        "ls_basic"
    }

    fn category(&self) -> &'static str {
        "coreutils"
    }

    fn description(&self) -> &'static str {
        "ls command lists root directory"
    }

    fn commands(&self) -> Vec<&'static str> {
        vec!["ls /"]
    }

    fn expected_patterns(&self) -> Vec<&'static str> {
        vec![
            r"bin",  // /bin exists
            r"etc",  // /etc exists
            r"proc", // /proc exists
        ]
    }
}

/// Test cat command
pub struct CatFile;

impl BehaviorTest for CatFile {
    fn name(&self) -> &'static str {
        "cat_file"
    }

    fn category(&self) -> &'static str {
        "coreutils"
    }

    fn description(&self) -> &'static str {
        "cat displays /etc/passwd contents"
    }

    fn commands(&self) -> Vec<&'static str> {
        vec!["cat /etc/passwd"]
    }

    fn expected_patterns(&self) -> Vec<&'static str> {
        vec![
            r"root:.*:0:0:",     // root user entry
            r"live:.*:1000:1000:", // live user entry
        ]
    }
}

/// Test echo command
pub struct EchoTest;

impl BehaviorTest for EchoTest {
    fn name(&self) -> &'static str {
        "echo_test"
    }

    fn category(&self) -> &'static str {
        "coreutils"
    }

    fn description(&self) -> &'static str {
        "echo outputs text correctly"
    }

    fn commands(&self) -> Vec<&'static str> {
        vec!["echo 'ECHO_OUTPUT_12345'"]
    }

    fn expected_patterns(&self) -> Vec<&'static str> {
        vec![r"ECHO_OUTPUT_12345"]
    }
}
