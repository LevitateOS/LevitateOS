# PRD10 — current release catalog pending; prior results retained

Updated 2026-11-09. **PRD10 remains open.** The current exact optimized candidate
is Ginux e4d95de1 / `21a54e60…`, Recipe 176395a / matching CLI-worker
`e7073fc1…`, image `ffe04d86…`. Its current inventory is **152 e2e + 1
library_backend + 1 image_producer = 154 required cases**. Fresh library_backend
and image_producer each pass on that tuple. The complete 152-case release e2e
rerun is executing; no complete current-combination success is claimed.
[The active checkpoint](prd15-product-contract.md) owns the full identity map,
receipts, retained runner-setup failure and actual ongoing run. Older results
below are historical, never added to the new count.

The maintained baseline 28f4368f/e7bfc166 is publicly obtainable and independently
reproduced under [PRD14](prd14-bootstrap-packaging.md). The source-choice question
is resolved. Later correction delivery, final-source acceptance and PRD05's
actual prerequisite merges remain open; current read-only inspection still finds
main 53a54adde and those PRs open/unmerged. No publication or new user hold is
inferred.

## Historical complete 150-case e2e combination

On the older frozen combination below, **150 e2e + 1 library_backend + 1
image_producer = 152/152 passed**, with zero failed, missing, ignored or
interrupted required cases. All 149 original e2e names remained and one
stalled-Git case was added. These receipts establish that source only; later
152-case e2e catalogs and final delivery require their own evidence.

## Frozen source and executed artifacts

| Component | Exact identity |
| --- | --- |
| Recipe | Clean `8db564c526c1baa302dbdea0f27e247c91f7116c`, tree `88cdb003b15e601ccf10ead660f97451986753b3`. Nine changed files: integration fixtures/coverage and a lease-ownership comment. |
| CLI and matching worker | Both SHA-256 `4cb2b900995a76f4055420790ee0a49e0b9b1c2da9c33f1b941747dd3104e620`. Fresh release build; production executable bytes remain identical to PRD11/12. |
| Ginux | Source `f7f0d14cc5abc1874b755e6990e4bd87bfacf050`; launcher `0dd243fa90ea12117448dfdfbfd170ac8260e0ab3116f116acaa37c40c263ce3`. No Ginux source/native-policy change in this task. |
| Compiler/acquisition image | `oci:recipe-compiler@sha256:c089f95c462582cc0fcc2a8b2ba86c7d2d64c22c82efa499ce20e3704a7fe15e`; exact same rootfs archive. |
| e2e harness, 150 names | `17c357b58a2b8e46ccab063c6979ea01c8d0b0e0ff1bcdf73215937c5ed7da86`. |
| library_backend harness, 1 name | `fcc46cac08fd564f20e275134b4ce34d89378c8699374aafece998950d5dcad9`. |
| image_producer harness, 1 name | `679ee5fd0c428730cc3da5fa6e2a0ed396b123530cc45b7ff63450686a8d0309`; actual recipe-image executable `9725d0b153f84d7647dd7c58dff35fad1597ccdfff87fdb49b79fa5d08212d2d`. The embedded producer path is separately hashed. |
| Producer recipe inputs | Immutable snapshot of committed builder `4fec6bae95bd4c355ce71ec58410f08e1852d531`, selected explicitly by `RECIPE_TEST_BUILDER_RECIPES`. Concurrent desktop work is not silently consumed. |
| Actual EROFS producer | distro-builder source `cc403cf1e477c74e8860fa477d4d9efba5844d3c`, binary `2d365e854fca3ee6bb98744babdbf3a53692ab96201deb42d1a1341d4bb0fe29`. Normal compression/metadata checks retained. |

Evidence root: `.artifacts/ginux-contribution-preparation/prd10/`.
`combination-final.json` selects immutable `bin-final/` tools. `final-source.json`
binds actual binary owners, source archives, compiler/lock, catalog/results,
materialization and resource receipts. Observed mutable checkout heads in the
configuration are distinct from these immutable binary/input owners. The
optional retained recstrap binary is from `3ea733f` (SHA `30b8235a…`); none of
these three catalogs executes it. Concurrent recstrap checkout changes are not
attributed to that binary or claimed tested.

Builds were locked/offline release builds on Linux/x86-64 with Rust 1.99.0;
`corrections-3-build.log` records the last successful build. Committing the
verified files changed no compiled source bytes: `build-source-hashes-3.json`
was checked before clean-source pinning. Complete Recipe/Ginux/builder source
archives and the matching `final-Cargo.lock` accompany the evidence. No
superproject gitlink was adopted by this task.

## Complete successful coverage

The unfiltered run is **`ram-runs/final/` only**. It has all three enumerated
catalogs, `identity.json`, 152 individual case receipts/logs, `results.json` and
`integrity.json`. The result file SHA-256 is
`dd75b4b52db8916b9be4bacfc05dd8c217f90319a7bfbd78fc77029fa85be7c0`.
Every receipt identifies the same combination digest. End-of-run source/input
and artifact changes are both empty; every case log and binary hash was
rechecked afterward. Separate ignored-catalog queries also return zero names.

The e2e catalog covers real lifecycle/dependencies/imports/inheritance, readiness
and checked failures, acquisition/archive/download/Git behavior, current producer
recipes, NEEDS parsing/authority/caller narrowing/output validation, process and
worker boundaries, cancellation/deadlines, leases/retention/concurrent destination
locking, checkpoint recovery, changed inputs/targets and publication preservation.
It includes every-byte validation of the 2,147,487,744-byte product and all 5,000
named nonempty members; existing link/mode/timestamp assertions remain intact.

The new R-03 case serves a silent numeric loopback HTTP origin. It records the
actual Git request and transport-grandchild command/process identities, then
requires the managed 15-second deadline, descendant termination, durable failure
and worker diagnostics, no final publication, unchanged source, and a subsequent
successful invocation using the same destination. The complete case passes in
the final run. Git's 600-second HTTP low-speed option is not presented as a
whole-process bound.

The library case calls the public engine with spoofed worker environment and
still verifies actual Ginux execution, assigned domain paths, the selected
launcher and unchanged source. Its exact successful default-XDG diagnostic
log is copied into `library-final-diagnostics/`; the original retained diagnostic
is intentional application state and its lease was verified unheld.

The image-producer case invokes the real recipe-image CLI twice and compares
outputs/provenance, component selection, hardlinks and normalized mtimes. It
also verifies digest/path refusals without publication and preservation of an
existing destination. These are local composition fixtures; they do not claim
a fresh bootstrap of the full compiler image. The target-metadata e2e case
checks actual package extraction, publication/reimport/composition, target tar
metadata and the real EROFS producer. Actual recstrap deployment remains the
separate exact-image evidence in [PRD12](prd12-lifecycle.md), not a new deployment
claim for this run's generated image.

## Findings and concrete dispositions

| Finding / prior failure | Correction and final disposition |
| --- | --- |
| Kernel-acquisition fixture substituted an obsolete Acorn digest; production correctly refused its tiny valid fixture. | Substitute the current target-specific literal pin, requiring valid 64-hex syntax. All valid/digest/truncation/layout/unsafe-link controls and previous-source/product checks remain; full final case passes. Production pins unchanged. |
| Long retained paths exceeded the USTAR link-name field before CLI execution. | Use tar's long-link encoding; retain literal hostile regular paths and every host-sentinel/rejection assertion. Full final boundary case passes. |
| Library fixture omitted required literal NEEDS. | Add empty NEEDS; retain all spoofed-environment and isolation assertions. Final library case passes through the configured supervisor. No capability added. |
| Current rootfs producers require admin_rule_file/admin_rule; the fixture supplied neither and never reached package integrity validation. | Exact refusal reproduced in `rootfs-refusal/`. Supply all five declared control files and print actual diagnostics on failure. Both producers' same-size-corruption checks now pass with real integrity refusals. |
| Two state tests timed out at the helper's 45-second bootstrap wait and panicked with a live coordinator. | Use the existing 300-second bootstrap allowance and kill/wait the fixture child on error. Preserve the exact checkpoint interruption/state assertions, destination lock-wait and alpha/beta product checks. Both complete final cases pass with no observed live descendants/scratch. |
| Three multi-invocation baseline cases hit the runner's 900-second outer limit. | Record the cutoffs honestly. Final orchestration allows 3600 seconds per whole test; actual Recipe execution/death bounds remain unchanged. All three complete final cases pass. |
| Original scratch observation inspected only fixture roots and missed abandoned scratch beside them. | Final runner scans the whole case directory. Five separate real-coordinator recoveries reclaimed 16 abandoned directories; original failures/interruption statuses were not upgraded. Final scan finds none. |
| Producer recipes changed during the baseline; five paths failed source-integrity comparison. | Pin the committed 4fec6ba recipe snapshot with an explicit test-only selector. The complete final run reports no changed source/input paths. Independent desktop changes remain untouched and untested. |
| Stale source comment promised supervisor lease retention, while Ginux closes inherited duplicates. | Document coordinator ownership through teardown. Existing active-scratch/concurrent-cleanup/death/reclamation checks pass on the joined local runtime. No inherited/public FD exception or limit increase. Final integration/security-race acceptance remains separate. |
| New Git fixture first marked build ready (skipping acquisition), then counted prune.lock as an invocation log. | Require source acquisition before build and select invocation directories, retaining every transport/deadline/reaping/diagnostic/publication/reuse oracle. Both failed preparation attempts remain; the whole corrected case passes in the final 152-case run. |

Baseline `initial/` has 151 receipts: 142 passed, seven failed statuses and two
interrupted statuses; three have timeout=true (one was labeled failed because
scratch also remained). It also failed source-integrity comparison. Preparation
runs under `ram-runs/corrections*` are retained for their own source/harness
identities. None supplies any part of the final 152-pass total. No required test
was removed, ignored, relabeled as a unit or replaced with a helper-output stub.

## Evidence preservation and resources

The final run used two case workers and a task-owned 16-GiB tmpfs with
nosuid/nodev; concurrency inside individual tests was unchanged. This reduced
bootstrap disk contention without changing product storage/resource limits,
execution deadlines, linker threading, isolation or metadata checks.

`materialization-receipt.json` verifies 21,585 entries and 26 hardlink groups
before backup and after disk materialization: every regular file's bytes/size,
mode, UID/GID, nanosecond mtime, all xattrs, symlink bytes and hardlink
relationships match. Absolute evidence paths remain unchanged. The owned tmpfs
and verified duplicate backup were removed. `final-resource-observation.json`
finds no scoped process, invocation-scratch, host-mount or loop reference. Every
final case separately records no live sampled descendant after grace and no
owned scratch invocation. Retained evidence and normal library diagnostic
retention are intentional data, not open runtime leases. These observations do
not prove every adversarial interleaving; PRD09 remains open.

`SHA256SUMS` seals **35,240 regular files**, with 36 symlinks recorded separately
without traversal and zero special files. Logical byte coverage is
39,964,371,843 (including sparse/oversized negative fixtures). Manifest SHA-256:
`b575dff9dc33ca4af11daf33c2880366dcda3e95890f4b4b1d68d0463b3c9c90`.
Build caches and seal outputs are explicitly excluded; `manifest-summary.json`
records scope. Complete manifest verification exited 0 with an empty error log
in `manifest-verification.log`.

## Reproduction and remaining acceptance

From the project root, verify the sealed evidence:

```sh
sudo -n sha256sum -c .artifacts/ginux-contribution-preparation/prd10/SHA256SUMS --quiet
```

Replay requires the clean Recipe `8db564c` checkout and unchanged pinned producer
snapshot. To replay the preserved exact combination into a **new** evidence
directory:

```sh
sudo -n python3 .artifacts/ginux-contribution-preparation/prd10/run-catalog-final.py replay-next
```

The runner selects `combination-final.json`, enumerates all three harnesses,
refuses ignored/missing names, checks every artifact, records each exact command
and verifies source/artifact stability. Do not overwrite frozen executables,
their embedded recipe-image target path or existing receipts. A new build needs
its own target directory, repinned artifacts and new full ledger. Build logs,
source archives, lock, compiler identity and the actual tmpfs/materialization
procedure are retained; this is not a fresh-checkout bootstrap claim.

PRD01/05's actual maintained delivery/source join is still absent. Carry these
fixtures/oracles into it. Any changed source/artifacts require a complete
successful catalog on that resulting combination; reuse this whole ledger only
when exact identities genuinely remain the same. Never assemble a new full-suite
claim from individual passes on different revisions. Hosted CI/MSRV/portability,
fresh bootstrap/release reproducibility, security acceptance and both distros'
full boot/install/PTY/A-B outcomes remain in PRD13/14/09/19. No push, PR, upstream
comment, gitlink adoption, source-delivery choice or Daybreak execution occurred.

Active skill: `hdae-development` at `.agents/skills/hdae-development/SKILL.md`.
Reload fully with required reference, repository instructions and this checkpoint
after context shift. Current task: PRD10; complete local catalog passed, final
delivery acceptance open. Exact next action: resolve the existing PRD01/05
maintained-source/integration dependency, preserve these corrections and evidence,
and validate its resulting exact combination. Existing unrelated edits/staging
are preserved.
