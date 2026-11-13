# Where Ginux belongs in Recipe

Design of 2026-11-13. Nothing in it is built. It answers this question:
should Ginux wrap the whole recipe run, as it does today, or only the
third-party code a recipe runs?

**Recommendation.** Move the line down. Recipe's own code and helpers run on
the host, confined by Recipe itself. Ginux, with its root image, runs only
when a recipe executes programs it did not write: build tools, shell
scripts, downloaded binaries. The first step needs no Ginux change: run a
recipe on the host when its source calls no helper that starts a program,
and keep today's full Ginux run for the rest. That alone takes 18 of the 25
catalog recipes off Ginux, and two more (antigravity, ninja) after small
edits.

## 1. Today

Every recipe operation is one Ginux launch.

- The coordinator stages inputs, unpacks or clones the verified 1.3 GB Rocky
  root, and copies the `recipe` binary into it as the worker
  (`tools/recipe/src/inputs/root.rs` 87–215). It then starts Ginux with that
  worker as init (`launch.rs` 273–314).
- Inside, the Rhai interpreter and every helper run under Ginux's syscall
  supervision.
- Results come back as an export tar. The coordinator validates it,
  publishes it (`publication.rs` 20–379) and activates the package on the
  host (`packages.rs`).
- The installed program then runs on the host with the user's full rights.

What Ginux enforces today, and what the coordinator enforces:

| Guarantee | Today enforced by |
| --- | --- |
| Network only to `NEEDS.network`, byte cap | Ginux `[network]` mediator, numeric endpoints, the connect-only contract (`needs/contract.rs` 66–106, `inputs/config.rs` 152–167) |
| Memory, storage bytes/objects, CPUs | Ginux `[resources]`/`[storage]` (`config.rs` 128–146) |
| Writes only to `/work`, `/out`; inputs read-only | Ginux storage mediation plus 0555 directories (`root.rs` 186–215). The helpers themselves accept any absolute path |
| Checkpoints of `/work` and `/out` | Ginux snapshot over the control socket (`ginux.rs` 148–176); `retained.rs` pairs it with `ctx.state` |
| Process teardown on cancel or crash | Ginux domain teardown (`launch.rs` 282–292). Helper children run without their own deadline or group |
| Timeout | Coordinator (`launch.rs` 528–554) |
| Download integrity | Recipe: `download` and `github_download_release` require SHA-256 (`helpers/acquire/download.rs` 33–37) |
| Output validation | Recipe: `publication::prepare` |

## 2. What 2026-11-13 showed

These costs all come from where the line sits:

- **Cost per install.** Installing ripgrep, a 5 MB static program, means a
  330 MB image download on first use and unpacking 1.3 GB per run. That took
  30–60 s on an idle machine, and 18 minutes with two ISO builds loading the
  host.
- **Portability.** To start at all, a launch needs a chroot. On a standalone
  host that meant:
  - user namespaces (Ginux `ccc492a2`);
  - an AppArmor profile on Ubuntu 23.10 and later (Recipe `edee1b9`);
  - a `linkat` path for kernels before 6.10 (Ginux `7b9dec91`).

  Each distro family adds to this list.
- **Speed.** Every system call of every helper, including hashing and
  extracting archives, goes through a single-threaded ptrace supervisor
  (`docs/prebuilt-downloads.md`).

## 3. What the sandbox protects at this layer

- **Third-party code during a build or install.** This is the real value:
  `make`, `cargo`, `cmake`, `meson`, shell scripts, and a downloaded program
  run to check itself. A hostile build script cannot read `~/.ssh` or reach
  an undeclared host.
- **Hermetic builds.** Builds see a pinned root and declared inputs only.
- **Not the recipe script.** Rhai can touch files, the network or processes
  only through Recipe's registered helpers. The script is confined by the
  helper API, provided the helpers confine their arguments (they do not yet;
  see 5.1).
- **Not the installed program.** It runs unconfined after activation.
  Whatever the install step was kept from doing, the program can do the
  first time it runs.

So for a recipe that downloads a pinned release, checks its hash and copies
files, Ginux adds almost nothing that hash pinning and a confined helper API
would not give. Yet it costs the most there.

## 4. What the recipes actually run

The helpers fall into three classes, from `tools/recipe/src/helpers/`:

- **Recipe code only.** Paths and strings, file reads and writes, copies,
  links, chmod, hashing, native archive extraction (`extract`, `target_*`
  tree helpers) and the ELF rewriting of `carry_*`.
- **Network through Recipe's own client.** `download` (hash-pinned),
  `download_exact`, `fetch_bounded` and their `_scoped` forms, `http_get`,
  `github_*` and `fetch_sha256`. One `ureq` client, verified TLS, no proxy.
  Only the `_scoped` forms check hosts themselves; the rest rely on Ginux's
  endpoint grant (`acquire/mod.rs` 28–42).
- **Starts a program.**
  - Any program the recipe names: `exec*` and `shell*`.
  - Fixed tools from the image: `git_checkout` (git CLI),
    `target_extract_rpm` (`rpm2archive`, `rpm`), and the dnf/apk/rpm query
    and install helpers.

The user catalog (`recipes/`, 25 recipes):

| Kind | Recipes |
| --- | --- |
| Download, verify, copy only | bat, btop, claude-code, codex, delta, eza, fd, fzf, gh, jq, micro, muse, opencode, pi, rclone, ripgrep, uv, zoxide (18), plus antigravity, which also calls `shell` for `tail`/`head` |
| Fixed tools only | ninja (`rpm2archive`, `sha256sum`) |
| Fixed tools, plus running the downloaded program in a check hook | cmake, go, python (`--version` from `is_built`/`is_installed`), gxx (also runs the image's `gcc -dump*`), desktop-runtime (`glib-compile-schemas`, `python3`) |
| Builds from source or runs installer scripts | none |

The OS producers (`distro/builder/os-recipes/`) are the opposite: they
compile the kernel (`make`), our tools (`cargo`), the desktop apps (`cmake`),
XLibre (`meson`) and the TUIs (`bun`), and they run `apk` and `dnf`. They
need Ginux, and they also do a lot of pure file work that is slow under it.

## 5. The proposed boundary

### 5.1 Host side: the worker and Recipe's helpers

The Rhai worker runs as a host process owned by the invoking user. It works
in host directories that stand in for `/inputs`, `/work` and `/out`, and
Recipe enforces what Ginux enforced:

- **Paths.** Every file helper resolves its path beneath one of the
  invocation's directories, and nothing else, using
  `openat2(RESOLVE_BENEATH | RESOLVE_NO_MAGICLINKS)` (Linux 5.6+) on directory
  descriptors held by the worker. A canonicalize-and-check fallback covers
  older kernels. Inputs are opened read-only and stay hash-checked copies.
  `RUNTIME.paths.*` keep their meaning, so recipes do not change. Recipes
  that spell `/work/...` literally fail with a message naming
  `RUNTIME.paths`.
- **Network.** Recipe's HTTP client is the only way out of a host-side
  recipe. It connects only to the `NEEDS.network` host:port pairs: it
  resolves them once, as the coordinator does today (`inputs/network.rs`
  247–356), connects to those numeric addresses only, and checks redirects
  as the `_scoped` helpers already do. It counts bytes against
  `network_max_bytes` and writes the `network_usage` report itself, instead
  of parsing Ginux audit lines. Payload downloads stay hash-pinned.
- **Resources.**
  - `timeout_secs` and cancellation: as today, from the coordinator.
  - Storage bytes and objects: counted by the helpers that write.
  - `memory_mib`: bounds the payload runs (5.2); Recipe's own code is
    trusted.
  - Rhai's operation and call-depth limits stop a script that loops.
- **Checkpoints.** At a phase boundary no child process is running, because
  helpers are synchronous. A checkpoint is then a reflink copy (or a plain
  copy where reflinks are missing) of the work and output directories,
  recorded through the existing `retained.rs` generation pairing. Restore
  recreates the directories from it. The Ginux snapshot exchange is not used
  on this side.
- **Outputs.** `publication::prepare`'s rules (kinds, bounds, no escapes,
  declared outputs present) check the live output tree instead of an export
  tar.
- **Ownership.** As today, owners and modes in outputs are data in the
  target-metadata manifest (`target.rs`), never real `chown`. The `uid`/`gid`
  options of `exec` belong to the Ginux side.

### 5.2 Ginux side: programs a recipe did not write

Each call that starts a program runs in Ginux: `exec*`, `shell*`,
`git_checkout`, `target_extract_rpm`, the dnf/apk helpers, and a check hook
running a downloaded binary. The call gets:

- the pinned root image, as today;
- the invocation's work and output directories, writable, and its inputs
  read-only;
- the recipe's network grant (or none);
- the remaining time, memory and storage budget;
- a result and output report back to the worker.

This needs one thing Ginux does not have today: **assigning a host directory
to an instance** (writable, with storage accounting). Today Ginux only knows
`memory:` assignments and archive import (`config.rs` 134–146), because the
worker was the init. The alternative, copying the work tree into and out of
each call, is too slow for build trees. This is a Ginux change for our fork
first, to be offered upstream on its own merits.

Starting Ginux for each call is cheap next to a build, but not next to a
`sha256sum`. Section 6 removes the cheap calls.

### 5.3 Fixed tools become Recipe code

Several programs exist only to do what Recipe can do natively:

- `rpm2archive`/`rpm` in `target_extract_rpm`: read the RPM payload with a
  Rust crate, as `cargo xtask release recipe` already writes RPMs with `rpm`.
- `sha256sum --check` in the check hooks of ninja, cmake, go, gxx and python:
  use `verify_sha256`. This is a recipe edit.
- `tail`/`head` in antigravity: string helpers. This is a recipe edit.
  antigravity also calls `carry_libraries`, which copies libraries from the
  image's `/lib64` and `/usr/lib64` (`install/carry.rs` 33). Host-side, it
  would copy the host's libraries instead. So `carry_libraries` must take its
  libraries from a declared input, or stay a Ginux-side call.
- `git_checkout`: a pure-Rust Git client (gitoxide) for pinned fetch and
  checkout. Or git stays a Ginux call, since it talks to recipe-chosen
  servers.

Then only genuinely foreign programs start a Ginux instance.

## 6. Stages

1. **Native fixed tools** (5.3). This is useful today: fewer image
   dependencies and faster runs inside Ginux too.
2. **Choose per recipe, from its source.** Recipe already walks the recipe's
   AST statically (`core/executor.rs` `declared_metadata`,
   `inputs/discover.rs`). If no hook can reach a program-starting helper,
   directly or through `extends`/imports, the invocation runs host-side with
   5.1's confinement. Otherwise it runs exactly as today. This needs no
   Ginux change, and the 18 download-only catalog recipes, plus ninja after
   stage 1, need no image, no namespace, no AppArmor profile and no
   particular kernel.
3. **Mixed runs.** The host worker launches Ginux per program call (5.2),
   once Ginux can assign host directories. Then cmake, go, python, gxx and
   the OS producers run their file work on the host and only their compiles
   and checks under Ginux.
4. **Retire the whole-run mode** once stage 3 covers the OS producers.

Each stage stands on its own and can stop there.

## 7. What changes and what it costs

- **The security boundary moves into Recipe's helpers.** For a host-side
  recipe, the path and network checks of 5.1 are the boundary. Their bugs
  would run with the user's rights, or root's for "for this computer". They
  need the care the Ginux contract gets now: openat2 with directory
  descriptors rather than string checks, no shell, no host programs, and
  archive extraction beneath the output descriptor.
- **Reproducibility.** Host-side helpers are pure Rust, so their results do
  not depend on the host. Everything that runs foreign code stays in the
  pinned image. Stage 1 removes the fixed tools that today come from the
  image.
- **Identity.** The saved-state key and `identity::declared()` gain the
  execution mode, and `ENGINE_BEHAVIOUR` is bumped. Because the OS builder's
  part keys include it, OS parts rebuild once.
- **Tests.**
  - `tests/e2e/worker_entry.rs` asserts "never host execution", and is
    replaced by the static rule of stage 2.
  - The `managed_*`, `needs`, downloads, archives, cleanup and dependency
    suites gain host-side cases.
  - The suites that need Ginux keep it.
- **Standalone release.** For most of the catalog, the image download, the
  AppArmor profile and the kernel quirks matter only when a recipe runs a
  foreign program. `recipe runtime fetch` stays for those.
- **Root installs.** "For this computer" runs the host-side worker as root.
  It must never follow a path outside its directories. The same openat2 rule
  covers it, and it is the strongest reason to keep the helper set small.

## 8. Not this design: confining installed programs

Confining programs while they run is where a sandbox protects the user
every day rather than once at install. Ginux could do that later. It is a
separate and much larger question: compatibility with desktop programs,
portals, how a user grants access. It is not needed to place the install
boundary correctly.

## 9. Decisions

1. Adopt the boundary of section 5 as the direction.
2. Start with stages 1 and 2, before or after the first standalone release.
3. Native Git (gitoxide) or Git as a Ginux call.
4. Whether host-side "for this computer" installs need extra rules beyond
   openat2 confinement, such as refusing to follow symlinks anywhere under
   `/var/lib/recipe`.
