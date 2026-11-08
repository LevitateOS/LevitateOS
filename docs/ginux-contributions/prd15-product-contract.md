# PRD15 / PRD19 — callers, documentation and included products

**In progress; release acceptance remains open.** The included scope is the
PRD19 table in the contribution queue: both x86-64 Linux distros, real package
acquisition/rootfs, separate installed and live products, prebuilt kernels,
complete live-boot/live-tools ISOs, guest installer interaction, installation,
installed operation and recab A/B recovery. Desktop products, macOS, rebuilding
kernels and external AI-provider interoperability are not added by this task.

The user retains upstream precedence and keep-local/wait-for-maintained-source
delivery. No publication or source-adoption choice is authorized. Local product
checks cannot close PRD01/PRD05 or final delivered-source acceptance.

## Active checkpoint

Active skill: `hdae-development`, discovered at
`.agents/skills/hdae-development/SKILL.md`; reload it, required references,
repository instructions and this checkpoint after a context shift. Root
instructions prohibit mandatory process gates and Recipe unit tests; existing
CLI/integration/product owners provide verification.

Evidence is under `.artifacts/ginux-contribution-preparation/prd15/`.
`sources-initial.json` and adjacent patches pin the initial checkout, including
pre-existing local edits. Shared staging is preserved. Current builder/contract/
xtask and their linked consumers compile with `cargo build --locked -p
levitate-xtask` (`caller-build.log`). This establishes compilation only.

Recipe documentation is committed at `cca5bb7` (preceded by `a53c247`) over candidate
`124f378063ce449e3799aad9b43dfac9404754f1` (production Rust unchanged); configured
Ginux is `a498e4f6148d5d94ce4ba8edb79588b7883f6a0f`. The PRD14 compiler and
acquisition images are respectively `a8edd8fa…` and `ffe04d86…`; their durable
input and fresh-host preparation evidence remains in the PRD14 report. These
are provisional local candidates, not a released combination.

Existing product/scenario manifests include successful runs produced in another
checkout and older input combinations. They must retain their original source
bindings. A recent Levitate base-rootfs attempt failed. None of these is being
promoted to current final-source acceptance.

## Current evidence and findings

| Outcome | Receipt/disposition |
| --- | --- |
| Current caller compilation | `caller-build.log`, `builder-build.log`, `caller-pty-build-final.log`; actual builder/contract/xtask and linked consumer code compile. |
| Declaration inspection | All 14 executable builder recipe declarations inspect successfully (`recipe-declarations.json`). Desktop declarations are inspected as current source, not added to executed release scope. |
| Public package acquisition | `preseed-levitate-toolchain.json` and `preseed-acorn.json` pass on the explicit tuple. Both use the actual locked public Fedora RPM/Alpine APK paths with hash checks. The user explicitly authorized missing pinned downloads; locks are unchanged. |
| Runnable examples | Hello and threaded Cargo each pass install, custom readiness, fresh-coordinator reuse and actual published-program execution. `example-results.json` retains all results. Exact author and man-page code blocks also pass (`documented-author-inline.json`, `documented-man-inline.json`). |
| Static commands | `documented-validate`, `documented-inspect`, `documented-diff` and corrected `documented-explain-corrected` pass. The initial probe used nonexistent `filesystem.write`, which correctly failed; the documented `network.connect` operation passes. No implementation defect or false success is inferred from that probe error. |
| Real documented tool producer | Initial source preparation with package `recfstab` fails: it is a binary of package `recstrap`. README now uses the actual workspace builder command with `--repo-root`, prepares `recstrap`, then selects `BIN=recfstab`. Corrected source preparation and managed static compilation pass (`documented-tool-source-corrected.log`, `documented-tool-build.json`). Locks, linker threading and isolation remain enabled. |
| Metadata production | Existing actual CLI case `target_metadata::cli_target_metadata_survives_publication_reimport_and_erofs_production` passes on the current tuple (`target-metadata.json`, retained `metadata/` fixture). The old guide named a removed case and incorrectly attributed deployment to this case; corrected to separate production and deployment receipts. |
| Actual deployment | Both `metadata-vm-{levitate,acorn}/console.log` contain `PRD12_VM PASS`, using the exact new EROFS and current built recstrap with each existing prebuilt kernel. Domains shut down and disappear from session libvirt. This is real metadata deployment, not full distro boot/install acceptance. |
| Helper/man documentation | Registered helper inventory has no missing entries; five target-metadata helpers were undocumented and are now listed. `print`/`debug` are deliberate Rhai built-ins. All three manuals render with groff without warnings; all 20 local Markdown links in the audited Recipe documents resolve. |
| Root privilege/toolchain setup | First privileged xtask attempt fails before Recipe: root has no rustup default. Retained as `preseed-levitate.json`. Retry explicitly selects the already installed operator compiler path and Cargo inputs for the whole coordinator; passes. No launcher-only sudo wrapper or host-execution fallback is used. |

The documentation now covers API 2/NEEDS, typed complete install/reuse outputs,
dependency-owned authority and graph narrowing, exact image/worker selections,
root-private publications, recovery/retention, global/vDSO clocks, zero sysinfo,
per-process address space and real CPU affinity, numeric endpoint authority,
bounded poll limitations and whole-second archive times. `docs/runtime-limits.md`
is shipped through the existing documentation asset wildcard. External-provider
interop is explicitly outside acceptance; the sample is configuration syntax.

The live-tools scenario previously tested only probe/split smoke. A focused
guest PTY interaction is being added to that existing owner, with its actual
terminal transcript retained in each run. It is compiled but **not yet verified**.
Do not count it as support until both new products run it successfully.

## Subsequent product findings and required corrections

- **TUI ownership decision:** the user explicitly requires Recipe-owned TUI
  acquisition and compilation. The current pinned host-Bun producer is not an
  accepted exception. The work must move out of `tui_binary.rs`; successful
  existing host-Bun products cannot close this criterion. Both installed/system
  and live/install binaries are included; desktop scope is not added.
- **Bun acquisition:** new `distro/builder/recipes/bun-toolchain.rhai` owns
  public version 1.4.2, both archive pins, declared GitHub endpoints, checked ZIP
  extraction and exact typed compiler/runtime publication. Its fresh actual
  install passes (`bun-toolchain-acquisition.json`). Official release metadata
  confirms both pins; `bun-source-upstream/availability.json` identifies compiler
  source `744846f844374847c902b5e7fd59b4342a51ef99`. The published compiler's
  SHA-256 is `a83d263767d839e4d2649ca8e35d07159c7afc99afdc96d731ced29e056dda0c`;
  it matches the prior cached compiler byte-for-byte. The musl runtime is
  `16b72935ffd7a503b978c186874539c92aade4e3515b70a5abf5db2581fdef7d`.
  This proves acquisition, not compilation or complete TUI migration.
- **Actual Bun compilation is blocked:** three retained managed reproductions
  fail resolving `/work/build` or `/inputs`, including explicit immutable cwd,
  PWD and source input. The lower-noise `bun-direct.stderr` records `readlink`
  returning EACCES followed by openat EACCES. Exact Bun source
  `src/sys/lib.rs::get_fd_path` resolves Linux descriptors via
  `/proc/self/fd/N`. Configured Ginux currently denies it. The user **approved a
  bounded configured-only readlink view of descriptors already owned by the
  caller, returning domain paths only**. Native policy and host-proc denial stay
  unchanged. No generic procfs, native ABI expansion, publication or maintained
  fork adoption is authorized. Final security verification remains required.
- **Isolated correction source:** managed detached worktree
  `/home/vince/.codex/worktrees/prd15-owned-fd-path/ginux-prd11`, clean base
  `a498e4f6148d5d94ce4ba8edb79588b7883f6a0f`. Implement the bounded view there,
  using the public descriptor table and existing filesystem/image path owners;
  do not edit the concurrently integrating original worktree. No correction is
  implemented or verified yet at this checkpoint.
- **Mutable launcher incident:** Acorn live-tools and Levitate base-rootfs
  stopped at a later launch when catalog 2's digest changed from `121cb495…` to
  `ac6e693e…`. Those runs fail and remain preserved. The shared launcher path was
  rebuilt during the runs; subsequently observed binary hash is `e5d87cb5…`,
  from a conflicted/modified worktree, not a reproducibly pinned candidate.
  Recipe's guard correctly refused it; no new digest was accepted. The
  previously sealed PRD13 executable was copied unchanged into
  `prd15/bin/ginux` (0555), verified as `0dd243fa…`. `run.py` now verifies the
  selected image/worker/launcher hashes before and after each run. Prior receipts
  retain their original paths and limitations. Current upstream main read over
  GitHub remains `53a54addea67969abaa240ea478def1b94130eee`; this incident is not
  evidence of a new upstream main or authority to adopt the conflicted source.
- **Actual product progress:** new Acorn base-rootfs
  `A1LtZzYkkhvNVgkJ` and live-boot `A1LvCQzI8EW3uRlu` succeed. They still use
  the host-Bun boundary which must now be replaced. Levitate's real Fedora rootfs
  recipe succeeds, but its base ISO fails later at the changed launcher; no
  complete Levitate ISO pass is claimed. Acorn live-tools fails before completing
  its tool set. Full current-product VM/install/A/B acceptance has not run.
- **Kernel mismatch:** freshly acquired/verified prebuilt Acorn matches every
  selected bundle entry. Levitate's old selection has the same release string
  but 265 differing entries against the pinned release, including kernel image
  hashes `ce542acd…` versus pinned `05e651ed…`. Red comparison is retained in
  `kernel-levitate-comparison.json`. The normal prebuilt-only caller refresh
  succeeds (`refresh-selected-kernel-levitate.json`), without compiling a kernel.
  Both corrected selections match every freshly verified bundle entry
  (`kernel-*-comparison-selected.json`). The refresh completed before the
  ongoing Levitate build reached kernel assembly. Its affected metadata
  deployment control also passes on the new kernel
  (`metadata-vm-levitate-pinned/console.log`). Version/config-shape status alone
  is not byte-pin provenance; release reproduction must refresh/verify the
  prebuilt selection explicitly.
- **Older-ISO PTY diagnostic:** the current scenario correctly rejects the old
  Acorn ISO for missing `recab-tui`. An independently opened older ISO through
  `cargo xtask run live-tools acorn --ssh` was used only to debug actual PTY
  input. First oracles expected an exit-code label and a login-help banner which
  that UI does not emit. Both failures are retained. Corrected interaction
  passes shell command/file handshake, docs-pane input/exit, terminal-host quit
  and fallback-root-shell command execution (`diagnostic-old-pty-final.log`).
  This does not certify the new product, all apps, Ginux PTYs or a full
  UI-driven disk install. The diagnostic guest was closed normally.

Exact next action: implement the approved self-FD readlink view in the isolated
Ginux candidate, add focused ordinary owner/lifetime and consumer checks, rerun
the actual Bun compilation, then finish and wire the Recipe-owned TUI producer.
Rebuild affected complete products and run both distros' required libvirt
scenarios/actual PTY controls on that exact combination. Preserve existing red
runs; obtain required final-source/security evidence before closure.

Next: identify current product input/provenance and failure owners, exercise
the public producer paths with the explicit candidate tuple, audit author/
operator/helper/man/interface/examples documentation against code, and record
each included product outcome, fixing in-scope failures and rerunning affected
checks. Installer PTY, all named scenarios and A/B remain unverified on the
current combination until actual receipts establish them.
