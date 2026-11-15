# LevitateOS

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE)

**Customization from the package level.**

A Linux distribution where you maintain your own packages. Packages are Rhai
recipes that declare how to acquire, build and install each piece of software.
A/B slots are the default install: the running root is locked, all changes are
made in the other root, and a changed root that fails to boot falls back to the
last working one. A mutable (single-root) install is an explicit opt-in.

## Two sibling distros

LevitateOS and AcornOS are two independent operating systems built by the same
pipeline: the same install flow, the same A/B behavior, the same recipes, and
one release ISO each — a split-screen installer (system terminal and guide)
that reboots into the desktop after installation. They are not based on Fedora
or Alpine; their userlands are built from Fedora 45 RPM packages and Alpine
3.24 APK packages respectively, which are a parts supplier only (own kernel,
own installer, own rec* tools, no `dnf`/`apk` anywhere). They differ in the
facts each one declares in its own `distro/variants/<id>/distro.toml`: package
backend (Fedora RPMs or Alpine APKs), libc and init, the admin tool (sudo or
doas), the desktop (Plasma or LXQt) and the kernel line (latest or LTS).
`distro_contract::load` reads and validates that file; the `[profile]` section
holds the differing facts.

Each has its own purpose. LevitateOS is the daily-driver workstation: glibc and
systemd, so software built for mainstream Linux runs. AcornOS is the small,
strict system: musl, OpenRC and the fewest parts that do the job.

## Working in the repository

Everything runs through `cargo xtask`; `cargo xtask --help` is the command
reference and [`xtask/README.md`](xtask/README.md) says what xtask owns.
Before changing the installer, recab, Recipe, the desktop apps, or public
wording about the OSes, read [`docs/product-handoff.md`](docs/product-handoff.md):
it records the settled product design, the work order (upstream Ginux first),
and the known defects.

The current Recipe/Ginux release integration remains provisional. Its
[release scope and acceptance queue](docs/ginux-contribution-task-list.md)
requires both distros' base/test-install-base products through guest installer,
installation and A/B verification on the delivered combination. Maintained
runtime delivery and final acceptance remain open; desktop products and
external AI-provider interoperability are outside that agreed verification scope.

## Installing

The live ISO boots into the install screen: a guide beside a shell, using the
`rec*` install tools. The same pages are on the website and in
[`docs/content`](docs/content).

## Recipes

One Rhai format and one engine, [`tools/recipe`](tools/recipe), serve two consumers:

| Collection | Owner and purpose |
| --- | --- |
| [`distro/builder/os-recipes/`](distro/builder/os-recipes/) | The OS builder: acquire base packages, build the kernel and tools, and assemble system images and live overlays. |
| [`recipes/`](recipes/) | User-managed software: the application catalog, starting with ripgrep. Recipe activates packages and manages updates/removal using the installed receipt. |

The split is about who consumes the output and manages its lifetime. Both
collections use Recipe API 2 and Ginux; there is no separate recipe language or
execution engine. Software shipped with the OS above the minimal base —
including the desktop environment itself — is Recipe-installed with a receipt,
so it can be uninstalled or swapped in Recipe like anything else
([`docs/product-handoff.md`](docs/product-handoff.md) §2.4).

`recipe install ripgrep` produces a verified package tree in Ginux and atomically
activates it. `recipe update ripgrep` uses the current catalog's pinned version;
`recipe remove ripgrep` deactivates it using the installed receipt. The OS builder
uses `--no-activate` to consume outputs without changing its host. Package profiles
and override recipes currently persist on shared `/var`. System-level changes go
through Recipe into the **unlocked** other root (the booted root is locked) and
take effect on the next boot of that root; the settled A/B contract, its current
implementation gap, and the work order are recorded in
[`docs/product-handoff.md`](docs/product-handoff.md) §2.2/§2.3 and §5.1.

See [`docs/package-recipe-format.md`](docs/package-recipe-format.md) for ownership
and output semantics, and [`tools/recipe/WRITING_RECIPES.md`](tools/recipe/WRITING_RECIPES.md)
for the shared format.

## Repository layout

```
distro/      builder (products and ISOs), contract and variants (per-distro declarations)
recipes/     user-managed software catalog (ripgrep)
tools/       recipe, the rec* install and boot tools, ginux (upstream build domains),
             gui (Qt desktop apps) and tui (terminal apps for the live ISO and installer)
testing/     install-tests (boot/install scenarios)
docs/        content (site and live docs), website, design notes, history
xtask/       the cargo xtask front door
```

## License

MIT OR Apache-2.0 for LevitateOS's own code; built images contain packages
under their own licenses. See [LICENSE](LICENSE).

## Links

- [Website](https://levitateos.org)
- [Documentation sources](docs/content)
