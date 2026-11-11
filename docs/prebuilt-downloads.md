# Prebuilt downloads for our own programs

Plan of 2026-11-11, not built. The goal: an ISO build downloads the programs we
write instead of compiling them, the way it already downloads the kernel, and
compiles only when nothing matching has been published.

## What an ISO build compiles today

Every Fedora and Alpine package is downloaded. What is compiled is our own
software, all of it inside Ginux, where each system call is a round trip
through a single-threaded ptrace supervisor:

| Family | Built by | Inputs | Per ISO build |
|---|---|---|---|
| **Tools**: `recpart`; `recstrap`, `recchroot`, `recfstab`, `recsetup`; `recab`; `recipe`; `install-split` | `os-recipes/install-tools.rhai`, called per binary by `build_tool_tree` (`distro/builder/src/recipe/tools.rs`) | the crate's offline source closure (`prepare_tool_source`), static musl target | once per **binary and init system**: eight compiles per distro (the recstrap crate four times), and both distros compile separately |
| **TUI binary**: the install screen and guide | `os-recipes/tui-binary.rhai` with `bun-toolchain.rhai` | the TUI's JavaScript source closure, the libc target | per libc |
| **Desktop apps**: `recipe-gui`, `recab-gui`, `disk-gui`, `levitate-agent` | `fedora-rpm-apps-build-root.rhai` / `alpine-apps-build-root.rhai` (unpack 130–280 packages into a build root), then `desktop-apps.rhai` (CMake, `g++`) | `gui/`, the distro's apps-build lock (its Qt) | per distro |

Recipe's cache identity includes the Ginux launcher's digest, so every Ginux or
Recipe change, which happens here daily, recompiles all three families. No
compile time has been measured yet. The desktop builds of 2026-11-11 are the
first to run all three, and their logs give the baseline.

## The pattern to copy

`linux-prebuilt.rhai` fetches the kernel bundle we published, pinned by digest.
`linux.rhai` compiles only when asked. `cargo xtask kernel publish` builds and
uploads. Two things do not carry over:

- **The kernel's digests are written into the recipe by hand at each publish.**
  These programs change several times a day, so a digest per change in a
  committed file would be constant churn. The key has to be computable from
  the checkout instead.
- **The kernel is published from this machine.** These should be published by
  CI on every push, so nobody has to remember.

## The design

### 1. A key computed from exactly what would be compiled

Each prebuilt is identified by a **key**: the SHA-256 of everything its
compile depends on, computed the same way by the builder and by CI.

- **Tools:** the prepared offline source closure that `install-tools.rhai`
  receives today (`prepare_tool_source`: the crate, its local dependencies, the
  pruned `Cargo.lock`, vendored crates), plus the binary name, the target
  triple, and the compiler image digest the recipe compiles in.
- **TUI binary:** its source closure input, plus the libc target and the
  `bun` version pinned in `bun-toolchain.rhai`.
- **Desktop apps:** the content of `gui/` (the tree id `git rev-parse
  HEAD:gui`, or a hash of the worktree files when `gui/` has local changes),
  plus the digest of the distro's apps-build lock (its Qt, compiler and
  libraries), plus the distro id.

The key does **not** include the Ginux launcher or Recipe. Those change how the
compile is supervised, not what comes out of it. That is what lets a Ginux
change stop recompiling everything. It is the one place this plan departs from
"any Ginux change rebuilds everything", and it holds only for these
prebuilts, not for extracted roots or images.

Local, uncommitted edits change the key, so they miss and compile locally,
which is correct.

### 2. Where prebuilts live

A dedicated public repository, **`LevitateOS/prebuilt`**, one GitHub release
per family and key (tag `<family>-<key prefix>`). Each release has the program
archive plus `prebuilt.json` (key, inputs summary, source commits, size,
SHA-256 of the archive, CI run URL).

Not the superproject's own releases: those are immutable and reserved for
system update payloads (P14), and thousands of development builds would bury
them.

Retention: keep the newest N per family and anything a superproject pin
references; a scheduled job deletes the rest.

### 3. CI publishes on every push

One workflow in the superproject, `prebuilt.yml`:

- **Runs** on push to `master`, on a 15-minute schedule, and on
  `repository_dispatch` from the tool repositories (a one-line step each in
  `recab`, `recipe`, `recstrap`, `recpart`, `tui`).
- **Checks out** the superproject with every submodule at the head of its own
  branch (`git submodule update --remote`). Developers' checkouts are normally
  those heads, since everything is pushed straight to `main`/`master`; building
  the pinned commits instead would almost never match.
- **For each family and target:** computes the key with the builder's own
  command (`distro-builder prebuilt key <family> …`). If `LevitateOS/prebuilt`
  has no release for it, it builds with **the same recipe the ISO build uses**,
  inside Ginux on the runner, then uploads it. No second build path, so CI and
  the ISO build cannot drift.

This needs Recipe and Ginux to run on GitHub's runners. The hosted CI work in
the Recipe acceptance queue (PRD13) already runs them there. If that is not
ready, CI can wait on it while steps 4–5 still work against prebuilts
published from this machine with the same command.

### 4. The ISO build downloads first

In `build_tool_tree`, the TUI step and the desktop-apps step, before compiling:

1. **Compute the key.**
2. **Local cache:** if `.artifacts/prebuilt/<family>/<key>/` holds a verified
   copy, use it.
3. **Download:** otherwise fetch `prebuilt.json` and the archive through a new
   `os-recipes/prebuilt-fetch.rhai` (Recipe, network grant `github.com` and the
   release-asset host, like `linux-prebuilt.rhai`). Check the archive's
   SHA-256 against `prebuilt.json` and that `prebuilt.json`'s key equals the
   computed key.
4. **Fall back:** on a miss or any failure, compile as today. Print one line
   saying which happened (`[prebuilt] recab: downloaded <key>` /
   `[prebuilt] recab: no prebuilt for <key>, compiling`).

The downloaded program goes into the same packaging step as before:
`install-tools.rhai` still writes recab's trial service for the init system,
and still packages Recipe's catalog, launcher and runtime image. Only the
compile is skipped. That needs a `prebuilt` input in `install-tools.rhai`,
`tui-binary.rhai` and `desktop-apps.rhai` that replaces their compile phase.

`--no-prebuilt` (builder and xtask) forces compiling everything.

### 5. Release builds

A downloaded binary is trusted because it came over HTTPS from our own
repository with a matching digest. That is fine for development, and not
release evidence ("Development convenience is not release evidence",
AGENTS.md). Release builds therefore keep compiling: the release path passes
`--no-prebuilt`. Later, if CI's builds are shown to be reproducible
(same key → same bytes), release builds could download and verify against a
rebuild or against signed provenance instead. That is not part of this plan.

## Cheap wins that need no CI

Worth doing first, whatever is decided about CI:

- **Compile each crate once.** `LIVE_TOOLS`
  (`distro/builder/src/pipeline/live_tools.rs`) asks for eight binaries, and
  `build_tool_tree` compiles each in its own work directory
  (`.artifacts/work/recipe-tools/<binary>/<init>`): the recstrap crate compiles
  four times. One compile per crate, then package per binary, makes it five
  compiles per distro instead of eight.
- **One compile shared by both distros.** The work directory is keyed by the
  init system, so LevitateOS (systemd) and AcornOS (OpenRC) compile all eight
  separately, 16 in total. The binaries are static musl and only recab's
  packaged service file depends on the init. Keyed by source instead, both
  distros share five compiles.
- **Steps 1, 2 and 4 alone**, with publishing from this machine (`distro-builder
  prebuilt publish <family>`), already give the download path for anyone else's
  checkout and for CI-less rebuilds.

## Order of work

1. Measure: per-family compile times from tonight's desktop build logs.
2. The cheap wins: once per crate, the local key cache.
3. `prebuilt key` and `prebuilt-fetch.rhai`, the `prebuilt` inputs, the
   download-first step in the builder, `--no-prebuilt`.
4. `LevitateOS/prebuilt` repository, `prebuilt publish` from this machine.
5. `prebuilt.yml` in CI, the dispatch steps in the tool repositories, retention.

## Open questions for the user

1. **A new public repository `LevitateOS/prebuilt`**, or another home (each
   tool's own repository's releases; GitHub Packages).
2. **Leave Ginux and Recipe out of the key** for these prebuilts (the point of
   the plan), while extracted roots and images keep including them.
3. **Release builds keep compiling** (`--no-prebuilt`), as above.
