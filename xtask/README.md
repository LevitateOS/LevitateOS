# levitate-xtask

`cargo xtask` is the workspace's one front door: every workflow (set up, build, boot, test, regenerate, check) is a subcommand, and `cargo xtask --help` lists and explains them all, with exit codes and defaults. `just <command>` is a short alias for each.

xtask composes the producers and sets up the environment: distro-builder builds products, recipe builds kernels, install-tests runs scenarios and boots every VM as a libvirt domain on the user session.

## Where things live

`src/cli.rs` defines every command (over the distros it finds under `distro/variants` and the product and scenario registry in `distro_contract::registry`); `src/app.rs` dispatches; each command group has one file under `src/tasks/`; shared helpers are in `src/util/`. `tests/cli.rs` checks the CLI contract.

## Publishing a kernel

`cargo xtask build kernel` installs a kernel through Recipe (compile, or `--prebuilt` download). `cargo xtask kernel publish <distro> [--upload] [--from <step>] [--now] [--jobs N] [--busybox PATH]` makes the prebuilt bundle that `--prebuilt` downloads, in the foreground and without root. Steps: `fetch` (the pinned kernel.org source, sha256-checked, cached in `.artifacts/kernel/src`), `config` (defconfig plus the variant kconfig; fails listing every option that did not stick), `build` (make, output in `build.log`, a heartbeat every minute, killed after 10 minutes without output or 120 minutes in total), `package` (runtime and debug tarballs plus `SHA256SUMS-*` in `dist/`), `boot` (a libvirt session VM must print the release and load bridge, 8021q, tun, veth and nbd; 180 s limit) and, with `--upload`, `upload` (`gh release upload --clobber` to the repo in `linux-prebuilt.rhai`, then that variant's digest there is rewritten; nothing is deleted). Without `--upload` it stops after `boot` and prints what upload would do.

Everything is under `.artifacts/kernel/<distro>/<release>/`; `state.json` records each step, so a rerun resumes after the last finished step and `--from <step>` redoes a step and the ones after it. A failure ends with `[fail] <step>: <reason> (log: <path>)` and a nonzero exit. Kernel compiles are allowed 10:00-23:00 (`--now` overrides). After an upload, update the AcornOS digest hard-coded in `tools/recipe/tests/e2e/kernel_acquisition.rs` and commit the `linux-prebuilt.rhai` change.
