//! TEAM_221: Kernel Logger implementation.
//!
//! Implements the `log::Log` trait to route log messages to the serial console.
//! Supports compile-time and runtime log level filtering.

use log::{Level, LevelFilter, Metadata, Record};
use los_hal::println;

/// Global logger instance
static LOGGER: SimpleLogger = SimpleLogger;

/// Simple Logger implementation
struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // TEAM_221: Simple format: [LEVEL] Message
            // We use println! which goes to serial console
            println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

/// Initialize the logger.
///
/// # Arguments
/// * `max_level` - The maximum log level to display.
pub fn init(max_level: LevelFilter) {
    log::set_logger(&LOGGER).expect("Failed to set logger");
    log::set_max_level(max_level);
    println!("[KERNEL] Logger initialized with level: {}", max_level);
}
