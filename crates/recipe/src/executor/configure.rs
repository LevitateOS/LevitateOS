//! Configure phase - user/dir/template setup.

use crate::{ConfigureSpec, ConfigureStep};

use super::context::Context;
use super::error::ExecuteError;
use super::util::{expand_vars, run_cmd, shell_quote};

/// Execute the configure phase.
pub fn configure(ctx: &Context, spec: &ConfigureSpec) -> Result<(), ExecuteError> {
    for step in &spec.steps {
        configure_step(ctx, step)?;
    }
    Ok(())
}

fn configure_step(ctx: &Context, step: &ConfigureStep) -> Result<(), ExecuteError> {
    let cmd = match step {
        ConfigureStep::CreateUser {
            name,
            system,
            no_login,
        } => {
            let mut args = vec!["useradd"];
            if *system {
                args.push("-r");
            }
            if *no_login {
                args.push("-s");
                args.push("/sbin/nologin");
            }
            args.push(name);
            args.join(" ")
        }
        ConfigureStep::CreateDir { path, owner } => {
            let expanded = expand_vars(ctx, path);
            let mut cmd = format!("mkdir -p {}", shell_quote(&expanded));
            if let Some(owner) = owner {
                cmd.push_str(&format!(
                    " && chown {} {}",
                    shell_quote(owner),
                    shell_quote(&expanded)
                ));
            }
            cmd
        }
        ConfigureStep::Template { path, vars } => {
            // Simple template substitution using sed
            let expanded_path = expand_vars(ctx, path);
            let mut cmd = format!(
                "cp {} {}.bak",
                shell_quote(&expanded_path),
                shell_quote(&expanded_path)
            );
            for (key, value) in vars {
                // Template uses {{KEY}} syntax, we need to escape braces for format!
                cmd.push_str(&format!(
                    " && sed -i 's/{{{{{}}}}}/{}/g' {}",
                    key,
                    value.replace('/', "\\/"),
                    shell_quote(&expanded_path)
                ));
            }
            cmd
        }
        ConfigureStep::Run(cmd) => expand_vars(ctx, cmd),
    };

    run_cmd(ctx, &cmd)?;
    Ok(())
}
