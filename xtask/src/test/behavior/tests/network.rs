//! Network tests - verify network utilities

use crate::test::behavior::BehaviorTest;

/// Test ip addr command
pub struct IpAddr;

impl BehaviorTest for IpAddr {
    fn name(&self) -> &'static str {
        "ip_addr"
    }

    fn category(&self) -> &'static str {
        "network"
    }

    fn description(&self) -> &'static str {
        "ip addr shows network interfaces"
    }

    fn commands(&self) -> Vec<&'static str> {
        vec!["ip addr"]
    }

    fn expected_patterns(&self) -> Vec<&'static str> {
        vec![
            r"lo:",          // Loopback interface
            r"127\.0\.0\.1", // Loopback address
        ]
    }
}

/// Test ping localhost
pub struct PingLocalhost;

impl BehaviorTest for PingLocalhost {
    fn name(&self) -> &'static str {
        "ping_localhost"
    }

    fn category(&self) -> &'static str {
        "network"
    }

    fn description(&self) -> &'static str {
        "ping can reach localhost"
    }

    fn commands(&self) -> Vec<&'static str> {
        vec!["ping -c 1 127.0.0.1"]
    }

    fn expected_patterns(&self) -> Vec<&'static str> {
        vec![
            r"1 packets transmitted",
            r"(1 received|1 packets received)",
        ]
    }

    fn timeout(&self) -> u64 {
        15 // Ping might take a moment
    }
}
