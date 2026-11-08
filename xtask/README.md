# levitate-xtask

`cargo xtask` is the workspace's one front door: every workflow (set up, build, boot, test, regenerate, check) is a subcommand, and `cargo xtask --help` lists and explains them all, with exit codes and defaults. `just <command>` is a short alias for each.

xtask composes the producers and sets up the environment: distro-builder builds products, recipe builds kernels, install-tests runs scenarios and boots every VM as a libvirt domain on the user session.

## Where things live

`src/cli.rs` defines every command (over the distros it finds under `distro/variants` and the product and scenario registry in `distro_contract::registry`); `src/app.rs` dispatches; each command group has one file under `src/tasks/`; shared helpers are in `src/util/`. `tests/cli.rs` checks the CLI contract.
