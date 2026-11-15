# levitate-xtask

`cargo xtask` is the workspace's front door. Every workflow (set up, build, boot, test, regenerate, check) is a subcommand. `cargo xtask --help` lists and explains them all, with exit codes and defaults. `just <command>` is a short alias for each (see the [`justfile`](../justfile)).

xtask composes the producers and sets up the environment: distro-builder builds products, Recipe builds kernels, install-tests runs scenarios and boots every VM as a libvirt domain on the user session.

## Where things live

- `src/cli.rs` defines every command, over the distros it finds under `distro/variants` and the product and scenario registry in `distro_contract::registry`.
- `src/app.rs` dispatches.
- `src/tasks/` has one file (or folder) per command group: `build.rs`, `kernels/`, `prebuilt.rs`, `progress.rs`, `recipe_release.rs`, `scenarios.rs`, `setup.rs`, `check.rs`, `generate.rs`, `tui/` and others.
- `src/util/` holds shared helpers.

## Building

`cargo xtask build <product> <distro>` is the one command that builds an ISO. The products are `test-base-rootfs`, `test-live-ssh`, `test-install-base` and `release`; `build erofs` and `build rootfs-source` make the parts. As a normal user it runs the builder through `sudo -n` (passwordless sudo is required: the Ginux coordinator needs root) with the user's toolchain and a separate offline cargo target directory. The builder picks the compiler image pinned in `tools/recipe/bootstrap/product-ci.json`, failing with a pointer to `tools/recipe/docs/setup-linux.md` if it is not produced, and acquires its package locks itself.

Builds take hours. Start one only when the owner asks for it.

`cargo xtask build kernel <distro>` installs a kernel through Recipe: compiled, or downloaded with `--prebuilt`.

## Watching a build: `cargo xtask progress`

`cargo xtask progress [<distro>] [--once]` shows where a running OS build is. It only reads files the builder and Recipe keep current, so it needs no privileges:

- `.artifacts/progress/<distro>.json`: the product, the run directory, the builder's pid, the step stack (`part > step > recipe`) and the active Recipe progress file.
- That Recipe progress file: `done`, `total` and `what`.

Per distro it prints the product, the step stack, `done/total what` with a percentage bar, the rate (items per minute over the last 5 minutes of samples; the average since the invocation started when there are too few, as with `--once`), the elapsed time of the step and of the build, and an ETA. The ETA comes from the rate when the step has a count, otherwise `~ from earlier runs`: the median duration of the same step in the `timeline.jsonl` of earlier runs under `.artifacts/out/<distro>/releases/*/*/`. The last finished steps follow, with their durations. Without `--once` it redraws every 2 s with plain ANSI (works over SSH); with it, one snapshot is printed and it exits.

With no state file it says no build is running; a state still marked `running` whose builder pid is gone is reported as stopped. `cargo xtask status <distro>` adds a one-line summary while a build runs. The builder's side (`timeline.jsonl`, `timeline.txt`, the state file) is in [`distro/builder/README.md`](../distro/builder/README.md).

## Publishing a kernel

`cargo xtask kernel publish <distro> [--upload] [--from <step>] [--now] [--jobs N] [--busybox PATH]` makes the prebuilt bundle that `build kernel --prebuilt` downloads. It runs in the foreground and needs no root. The steps:

1. `fetch`: the pinned kernel.org source, sha256-checked, cached in `.artifacts/kernel/src`.
2. `config`: defconfig plus the variant's kconfig; fails listing every option that did not stick.
3. `build`: make, output in `build.log`, a heartbeat every minute; killed after 10 minutes without output or 120 minutes in total.
4. `package`: runtime and debug tarballs plus `SHA256SUMS-*` in `dist/`.
5. `boot`: a libvirt session VM must print the release and load bridge, 8021q, tun, veth and nbd; 180 s limit.
6. `upload` (only with `--upload`): `gh release upload --clobber` to the repository named in `linux-prebuilt.rhai`, then that variant's digest there is rewritten; nothing is deleted.

Without `--upload` it stops after `boot` and prints what upload would do.

Everything is under `.artifacts/kernel/<distro>/<release>/`. `state.json` records each step, so a rerun resumes after the last finished step and `--from <step>` redoes a step and the ones after it. A failure ends with `[fail] <step>: <reason> (log: <path>)` and a nonzero exit. Kernel compiles are allowed 10:00-23:00 (`--now` overrides). After an upload, update the AcornOS digest hard-coded in `tools/recipe/tests/e2e/kernel_acquisition.rs` and commit the `linux-prebuilt.rhai` change.

## Other commands

`setup` and `doctor` prepare and check the environment; `run` boots a scenario for interactive use; `test` runs scenarios and caches results per input; `status` shows which pass, fail or are stale; `reset` removes a distro's scenario runs; `gen` regenerates derived files; `check` runs the quick checks (`check tui` the terminal apps); `tui` and `inspect` run and render the terminal apps; `prebuilt` prints keys and builds or publishes prebuilt programs; `base` lists the packages a variant takes from its distribution; `release recipe` builds the standalone Recipe release.
