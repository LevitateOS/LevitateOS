# levitate-xtask

`cargo xtask` is the workspace's one front door: every workflow (set up, build, boot, test, regenerate, check) is a subcommand, and `cargo xtask --help` lists them all. The producers do the work: distro-builder builds products, recipe builds kernels, install-tests runs scenarios. xtask only composes them and sets up the environment.

## Commands

```bash
cargo xtask setup                              # repo SSH key, doctor
cargo xtask doctor [--json]                    # what is missing, with the command that fixes it

cargo xtask build kernel levitate --prebuilt   # or compile: build kernel levitate [--now] [--rebuild]
cargo xtask build rootfs-source levitate [--refresh]
cargo xtask build live-tools levitate          # also base-rootfs, live-boot; --all for every distro
cargo xtask build erofs live-tools levitate    # a product's EROFS images, no ISO
cargo xtask base lock levitate [--check]       # take Fedora's updates: re-lock the signed base RPMs

cargo xtask run live-tools levitate            # serial console with the install UX (Ctrl+] leaves; the VM keeps running)
cargo xtask run live-boot levitate --ssh       # wait for SSH, open a shell over it
cargo xtask run live-boot levitate --no-shell  # boot, check it came up, stop (no terminal needed)
cargo xtask run installed-boot levitate        # the disk the latest install scenario produced

Every VM, from `run` and from `test`, is a libvirt domain on your user
session (`qemu:///session`), named `<distro>-<scenario>` (`<distro>-run-<scenario>`
for `run`): watch and manage it in Cockpit (Virtual Machines) or with
`virsh -c qemu:///session list`. Its screen is a VNC display Cockpit shows; its
serial transcript is in `$XDG_RUNTIME_DIR/levitate-vm/<name>.serial.log`.

cargo xtask test all levitate [--force] [--json]   # every scenario, in order, each on its own
cargo xtask test install levitate
cargo xtask status levitate [--json]
cargo xtask reset levitate

cargo xtask gen [--check]                      # schemas, TS declarations, tool help from the Rust types
cargo xtask check                              # cargo check, generated files current, TS typecheck

cargo xtask tui install-docs                   # run a terminal app from tui/ (any name in its package.json bin)
cargo xtask inspect install-docs --stdout -- --slug installation-disk
cargo xtask inspect install-docs --all-pages   # every page the app lists
cargo xtask inspect disk-plan --input 'jj'     # type keys once it has drawn, then capture
```

`just <command>` is a short alias for each of these.

## Behaviour every command keeps

- **Exit codes:** 0 ok, 1 failed, 2 bad usage, 3 missing prerequisite. A prerequisite error ends with `fix: <command>`.
- **No hidden waiting:** `run` and `tui` need a terminal; without one they exit 2 and name the command that works without one. `test` never waits for input.
- **SSH keys:** live boots trust the repo key in `.artifacts/ssh` (created on first use), or `--ssh-key <private key>`. SSH runs with `-F /dev/null` and a known-hosts file of its own, so nothing reads `~/.ssh`.
- **Boot injection:** `--inject KEY=VALUE[,...]` and `--inject-file PATH` add to what every live boot gets (the SSH key, and the serial install screen for an interactive `run`). Installed systems get none.
- **VMs:** `test` and `run` boot through install-tests' one launcher, as libvirt domains named `<distro>-<scenario>` on the host's QEMU, OVMF and qemu-img. A second run of a scenario whose domain is still running fails instead of stopping it.
- **Nothing expensive by surprise:** kernel compiles run 23:00-10:00 unless `--now`; LLM autofix only with `--autofix`.
- **What next:** builds print what they made and the command that tests it.

## Layout

`src/cli.rs` defines every command (over the variants it finds and the product and scenario registry); `src/app.rs` dispatches; each command group has one file under `src/tasks/` (`setup.rs`, `build.rs`, `scenarios.rs`, `generate.rs`, `check.rs`, `tui.rs`, `inspect_*.rs`, `kernels/`). Shared helpers live in `src/util/`. `tests/cli.rs` checks the CLI contract (status, reset, exit codes).
