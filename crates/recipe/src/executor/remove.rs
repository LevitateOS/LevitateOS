//! Remove phase - uninstallation.

use crate::{Recipe, RemoveSpec, RemoveStep};

use super::context::Context;
use super::error::ExecuteError;
use super::service::stop;
use super::util::{expand_vars, run_cmd, shell_quote};

/// Execute the remove action.
pub fn remove(ctx: &Context, spec: &RemoveSpec, recipe: &Recipe) -> Result<(), ExecuteError> {
    // Stop first if requested
    if spec.stop_first {
        if let Some(ref stop_spec) = recipe.stop {
            let _ = stop(ctx, stop_spec); // Ignore errors, process might not be running
        }
    }

    for step in &spec.steps {
        remove_step(ctx, step)?;
    }

    Ok(())
}

fn remove_step(ctx: &Context, step: &RemoveStep) -> Result<(), ExecuteError> {
    let cmd = match step {
        RemoveStep::RmPrefix => {
            format!("rm -rf {}", shell_quote(ctx.prefix.display()))
        }
        RemoveStep::RmBin(name) => {
            let path = ctx.prefix.join("bin").join(name);
            format!("rm -f {}", shell_quote(path.display()))
        }
        RemoveStep::RmConfig { path, prompt: _ } => {
            // TODO: implement prompting
            format!("rm -f {}", shell_quote(&expand_vars(ctx, path)))
        }
        RemoveStep::RmData { path, keep } => {
            if *keep {
                return Ok(()); // Keep data, don't remove
            }
            format!("rm -rf {}", shell_quote(&expand_vars(ctx, path)))
        }
        RemoveStep::RmUser(name) => {
            format!("userdel {}", shell_quote(name))
        }
    };

    run_cmd(ctx, &cmd)?;
    Ok(())
}
