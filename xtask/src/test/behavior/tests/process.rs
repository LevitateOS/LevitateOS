//! Process tests - verify process inspection utilities

use crate::test::behavior::BehaviorTest;

/// Test ps command
pub struct PsBasic;

impl BehaviorTest for PsBasic {
    fn name(&self) -> &'static str {
        "ps_basic"
    }

    fn category(&self) -> &'static str {
        "process"
    }

    fn description(&self) -> &'static str {
        "ps lists running processes"
    }

    fn commands(&self) -> Vec<&'static str> {
        vec!["ps aux"]
    }

    fn expected_patterns(&self) -> Vec<&'static str> {
        vec![
            r"PID",             // Header present
            r"(systemd|init)",  // Init process running
        ]
    }
}

/// Test uptime command
pub struct UptimeBasic;

impl BehaviorTest for UptimeBasic {
    fn name(&self) -> &'static str {
        "uptime_basic"
    }

    fn category(&self) -> &'static str {
        "process"
    }

    fn description(&self) -> &'static str {
        "uptime shows system uptime"
    }

    fn commands(&self) -> Vec<&'static str> {
        vec!["uptime"]
    }

    fn expected_patterns(&self) -> Vec<&'static str> {
        vec![r"up"] // "up X min" or similar
    }
}
