use anyhow::{bail, Result};
use clap::Subcommand;

use crate::{common, GlobalArgs};

/// Parse hexadecimal number (with or without 0x prefix).
fn parse_hex(s: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(s.trim_start_matches("0x"), 16)
}

#[derive(Subcommand)]
pub enum VmCommand {
    /// Start VM in background
    Start {
        /// Start with debug shell
        #[clap(long)]
        debug: bool,
    },

    /// Stop running VM
    Stop,

    /// Send text to VM serial console
    Send {
        /// Text to send
        text: String,
    },

    /// Show VM status
    Status,

    /// View VM output log
    Log {
        /// Don't follow log (one-shot)
        #[clap(long)]
        no_follow: bool,
    },

    /// Execute command in VM (captures output)
    Exec {
        /// Command to execute
        command: String,

        /// Timeout in seconds
        #[clap(long, default_value = "10")]
        timeout: u64,
    },

    /// Execute QEMU monitor command
    Qmp {
        /// HMP command to execute
        command: String,
    },

    /// Dump physical memory region
    MemDump {
        /// Physical address (hex, with or without 0x prefix)
        #[clap(value_parser = parse_hex)]
        addr: u64,
        /// Size in bytes
        size: u64,
        /// Output file path
        #[clap(short, long, default_value = "memory.bin")]
        output: String,
    },

    /// Take a screenshot (requires GUI mode)
    Screenshot {
        /// Output file path
        #[clap(short, long, default_value = "screenshot.ppm")]
        output: String,
    },

    /// Reset the VM
    Reset,
}

pub fn run(cmd: &VmCommand, global: &GlobalArgs) -> Result<()> {
    match cmd {
        VmCommand::Start { debug } => start(*debug, global),
        VmCommand::Stop => stop(global),
        VmCommand::Send { text } => send(text, global),
        VmCommand::Status => status(global),
        VmCommand::Log { no_follow } => log(*no_follow, global),
        VmCommand::Exec { command, timeout } => exec(command, *timeout, global),
        VmCommand::Qmp { command } => qmp_command(command, global),
        VmCommand::MemDump { addr, size, output } => memory_dump(*addr, *size, output, global),
        VmCommand::Screenshot { output } => screenshot(output, global),
        VmCommand::Reset => reset(global),
    }
}

fn start(_debug: bool, global: &GlobalArgs) -> Result<()> {
    common::verbose_println(global.verbose, "Starting VM via builder...");

    // Call builder's VM start functionality
    builder::builder::vm::commands::start()?;

    common::info_println(global.quiet, "VM started successfully");
    Ok(())
}

fn stop(global: &GlobalArgs) -> Result<()> {
    common::verbose_println(global.verbose, "Stopping VM...");
    builder::builder::vm::commands::stop()?;
    common::info_println(global.quiet, "VM stopped");
    Ok(())
}

fn send(text: &str, global: &GlobalArgs) -> Result<()> {
    common::verbose_println(global.verbose, &format!("Sending: {}", text));
    builder::builder::vm::commands::send(text)?;
    Ok(())
}

fn status(global: &GlobalArgs) -> Result<()> {
    common::verbose_println(global.verbose, "Getting VM status...");
    builder::builder::vm::commands::status()?;
    Ok(())
}

fn log(no_follow: bool, global: &GlobalArgs) -> Result<()> {
    common::verbose_println(global.verbose, "Viewing VM log...");
    builder::builder::vm::commands::log(!no_follow)?;
    Ok(())
}

fn exec(command: &str, _timeout: u64, global: &GlobalArgs) -> Result<()> {
    common::verbose_println(global.verbose, &format!("Executing in VM: {}", command));
    // This is one of the unimplemented commands in builder
    // For now, show that it's not implemented
    bail!("VM exec command not yet implemented in builder");
}

fn qmp_command(cmd: &str, global: &GlobalArgs) -> Result<()> {
    common::verbose_println(global.verbose, &format!("Executing QMP: {}", cmd));
    builder::builder::vm::commands::qmp_command(cmd)?;
    Ok(())
}

fn memory_dump(addr: u64, size: u64, output: &str, global: &GlobalArgs) -> Result<()> {
    common::verbose_println(global.verbose, &format!("Dumping memory 0x{:x}", addr));
    builder::builder::vm::commands::memory_dump(addr, size, output)?;
    Ok(())
}

fn screenshot(output: &str, global: &GlobalArgs) -> Result<()> {
    common::verbose_println(global.verbose, "Taking screenshot...");
    builder::builder::vm::commands::screenshot(output)?;
    common::info_println(global.quiet, &format!("Screenshot saved: {}", output));
    Ok(())
}

fn reset(global: &GlobalArgs) -> Result<()> {
    common::verbose_println(global.verbose, "Resetting VM...");
    builder::builder::vm::commands::reset()?;
    common::info_println(global.quiet, "VM reset");
    Ok(())
}
