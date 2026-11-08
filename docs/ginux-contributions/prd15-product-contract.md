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

Recipe documentation is committed at `a53c247` over candidate
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

Next: identify current product input/provenance and failure owners, exercise
the public producer paths with the explicit candidate tuple, audit author/
operator/helper/man/interface/examples documentation against code, and record
each included product outcome, fixing in-scope failures and rerunning affected
checks. Installer PTY, all named scenarios and A/B remain unverified on the
current combination until actual receipts establish them.
