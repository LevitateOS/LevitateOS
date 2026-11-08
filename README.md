# LevitateOS

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE)

**Customization from the package level.**

A Linux distribution where you maintain your own packages. Packages are Rhai
recipes that declare how to acquire, build and install each piece of software,
and the installed system updates atomically: A/B slots are the default, a
mutable install is an explicit opt-in.

## Variants

| | LevitateOS | AcornOS |
|---|---|---|
| **Stack** | glibc / systemd / GNU | musl / OpenRC / busybox |
| **Base packages** | Fedora RPMs | Alpine APKs |
| **Best for** | Maximum compatibility | A smaller, simpler base |

Both are built by the same pipeline from their own declarations under
[`distro/variants/`](distro/variants).

## Building and testing

Everything runs through `cargo xtask` (`just <command>` is a short alias):

```bash
cargo xtask setup                              # repo SSH key, then a doctor report
cargo xtask build kernel levitate --prebuilt   # or compile it: build kernel levitate
cargo xtask build live-tools levitate          # the live ISO with the install tools
cargo xtask run live-tools levitate            # boot it on a serial console
cargo xtask test all levitate                  # boot, install, reboot and check the installed system
cargo xtask check                              # quick sweep before committing
```

`cargo xtask --help` lists every command; [`xtask/README.md`](xtask/README.md)
explains the exit codes and defaults, and [`architecture.md`](architecture.md)
describes products, scenarios and where outputs go.

## Installing

From the live ISO, the install tools work like Arch's:

```bash
recpart apply --disk /dev/nvme0n1 --confirm DESTROY   # A/B layout, mounted at /mnt (--mode mutable for one root)
recstrap /mnt                                          # extract the system
recfstab /mnt >> /mnt/etc/fstab
recsetup inspect --root /mnt --json                    # choose target configuration
recsetup apply --root /mnt --hostname myhostname        # apply machine settings
recchroot /mnt -- passwd                               # set the installed root password
```

The live ISO boots into the install screen, a shell beside the full guide
(`install-docs` opens the guide on its own), and the same pages are on the website.

## Recipes

A recipe is a Rhai script with package facts, readiness checks and
acquire/build/install hooks, run inside an isolated build domain. See
[`docs/package-recipe-format.md`](docs/package-recipe-format.md) and
[`tools/recipe/WRITING_RECIPES.md`](tools/recipe/WRITING_RECIPES.md).

## Repository layout

```
distro/      builder (products and ISOs), contract and variants (per-distro declarations)
tools/       recipe, the rec* install and boot tools, ginux (upstream build domains)
testing/     install-tests (boot/install scenarios), fsdbg (its artifact readers)
tui/         terminal apps for the live ISO and installer
docs/        content (site and live docs), website, design notes, history
xtask/       the cargo xtask front door
```

## Hardware

x86-64-v3 (Haswell or newer), UEFI, 16 GB RAM and an NVMe disk are the
baseline the distribution targets.

## License

MIT OR Apache-2.0 for LevitateOS's own code; built images contain packages
under their own licenses. See [LICENSE](LICENSE).

## Links

- [Website](https://levitateos.org)
- [Documentation sources](docs/content)
