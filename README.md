# LevitateOS

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE)

**Customization from the package level.**

A Linux distribution where you maintain your own packages. Packages are Rhai
recipes that declare how to acquire, build and install each piece of software,
and the installed system updates atomically: A/B slots are the default, a
mutable install is an explicit opt-in.

## Two sibling distros

LevitateOS and AcornOS are built by the same pipeline and offer the same
features: the same install flow, the same A/B updates, the same recipes, and a
live ISO with and without a desktop. They differ only in the facts each one
declares in its own `distro/variants/<id>/distro.toml`: package backend (Fedora
RPMs or Alpine APKs), libc and init, the admin tool (sudo or doas), the desktop
(Plasma or LXQt) and the kernel line (latest or LTS). `distro_contract::load`
reads and validates that file; the `[profile]` section holds the differing
facts.

Each has its own purpose. LevitateOS is the daily-driver workstation: glibc and
systemd, so software built for mainstream Linux runs. AcornOS is the small,
strict system: musl, OpenRC and the fewest parts that do the job.

## Working in the repository

Everything runs through `cargo xtask`; `cargo xtask --help` is the command
reference and [`xtask/README.md`](xtask/README.md) says what xtask owns.
[`architecture.md`](architecture.md) says where each fact lives (products,
scenarios, outputs).

The current Recipe/Ginux release integration remains provisional. Its
[release scope and acceptance queue](docs/ginux-contribution-task-list.md)
requires both distros' base/live-tools products through guest installer,
installation and A/B verification on the delivered combination. Maintained
runtime delivery and final acceptance remain open; desktop products and
external AI-provider interoperability are outside that agreed verification scope.

## Installing

The live ISO boots into the install screen: a guide beside a shell, using the
`rec*` install tools. The same pages are on the website and in
[`docs/content`](docs/content).

## Recipes

See [`docs/package-recipe-format.md`](docs/package-recipe-format.md) and
[`tools/recipe/WRITING_RECIPES.md`](tools/recipe/WRITING_RECIPES.md).

## Repository layout

```
distro/      builder (products and ISOs), contract and variants (per-distro declarations)
tools/       recipe, the rec* install and boot tools, ginux (upstream build domains)
testing/     install-tests (boot/install scenarios)
tui/         terminal apps for the live ISO and installer
docs/        content (site and live docs), website, design notes, history
xtask/       the cargo xtask front door
```

## License

MIT OR Apache-2.0 for LevitateOS's own code; built images contain packages
under their own licenses. See [LICENSE](LICENSE).

## Links

- [Website](https://levitateos.org)
- [Documentation sources](docs/content)
