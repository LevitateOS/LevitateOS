//! Service management - start and stop actions.

use crate::{StartSpec, StopSpec};

use super::context::Context;
use super::error::ExecuteError;
use super::util::{run_cmd, shell_quote};

/// Execute the start action.
pub fn start(ctx: &Context, spec: &StartSpec) -> Result<(), ExecuteError> {
    let cmd = match spec {
        StartSpec::Exec(args) => args.join(" "),
        StartSpec::Service { kind, name } => match kind.as_str() {
            "systemd" => format!("systemctl start {}", shell_quote(name)),
            "openrc" => format!("rc-service {} start", shell_quote(name)),
            _ => format!("systemctl start {}", shell_quote(name)),
        },
        StartSpec::Sandbox { config: _, exec } => {
            // TODO: implement sandboxing with landlock/seccomp
            exec.join(" ")
        }
    };

    run_cmd(ctx, &cmd)?;
    Ok(())
}

/// Execute the stop action.
pub fn stop(ctx: &Context, spec: &StopSpec) -> Result<(), ExecuteError> {
    let cmd = match spec {
        StopSpec::ServiceStop(name) => format!("systemctl stop {}", shell_quote(name)),
        StopSpec::Pkill(name) => format!("pkill {}", shell_quote(name)),
        StopSpec::Signal { name, signal } => {
            format!("pkill -{} {}", shell_quote(signal), shell_quote(name))
        }
    };

    run_cmd(ctx, &cmd)?;
    Ok(())
}
