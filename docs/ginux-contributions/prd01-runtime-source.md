# PRD01 — corrected maintained source delivered; final acceptance remains separate

The maintained-source choice is resolved. The authorized PRD16 source delivery
now publishes Ginux `e4d95de10520aa8e91bc2ec69e64da004443e2da` and Recipe
`b185e0363ceba1290201ba84593aefb313a85cc4`; anonymous HTTPS fetches verify all
sixteen selected owning commits and exact trees. [The current selection](delivery-sources.json)
and [delivery record](prd16-delivery.md) own final pins, public availability and
product provenance. Upstream Ginux leads; current upstream main and unmerged
Gate 11 #249 history remain included. No upstream PR/comment was published.

Fresh independent source builds reproduce the tested launcher **21a54e60…**,
matching CLI/worker **e7073fc1…** and image producer **aa13ac9b…**. The pinned
acquisition image remains **ffe04d86…**, produced from tracked manifests and
ordinary public inputs. No private worktree or migration executable supplies
source/build output. PRD16 records final parent verification/publication status.
Security, required upstream merges, hosted Linux and production acceptance remain
separate; source delivery does not close those criteria.

The earlier 28f4368f/e7bfc166 fresh-Rocky/package baseline is preserved in
[its original manifest](delivery-sources-2026-11-08-baseline.json) and
[PRD14](prd14-bootstrap-packaging.md). Those results are not relabeled as running
the corrected pair. Later complete product/catalog receipts retain their actual
source and executable identities in the [product checkpoint](prd15-product-contract.md).

## Historical old-pin investigation

The findings below predate the maintained-source decision and verified
28f4368f/e7bfc166 fresh-host delivery. Old pin, availability and authorization
statements describe that earlier investigation only; they do not describe the
current repositories or override the decision above.

## Exact source and acquisition findings

Read-only GitHub inspection on 2026-11-08 finds:

| Source | Exact revision | Availability and disposition |
| --- | --- | --- |
| Checked-in `tools/ginux` | `2eac10bf8c9e7a7e72e1ae94990aa2425b2f3ba0` | Obtainable from its configured fork; fresh clone/build succeeds. It has no configured `launch` command and is unsuitable for current Recipe. |
| Published `LevitateOS/ginux:main` | `2489517e89c3b62c5203ccd0f6d1eb28cf8500af` | Fresh origin clone confirms this head. It contains none of the configured-v2/export/control/restore strings used by Recipe. Moving to this head does not close the gap. |
| Upstream `gmkurtzer/ginux:main` | `53a54addea67969abaa240ea478def1b94130eee` | The [PRD05 merge check](prd05-integration.md) records missing actual storage prerequisites. Current main is not the required integrated runtime. |
| Tested local Ginux | `5300f5ef9a8281f4f10f5519d5eb761654e04829` | Clean local integration, but GitHub returns “No commit found” in the configured origin. It is a donor, not an obtainable release pin or the completed storage/security integration. |
| Checked-in `tools/recipe` | `e37109c98e83ad0adcb291cfb056f8250e02274d` | GitHub cannot resolve it. A fresh independent clone followed by exact `git fetch origin <sha>` fails with “not our ref”. Recursive checkout cannot obtain this pin from the configured origin. |
| Tested local Recipe | `b41443880fe585a27bc799d80382c9ff5b07047b` | Clean at inspection; the same API and actual fresh-fetch checks fail. It is not yet remotely obtainable. |
| Published `LevitateOS/recipe:master` | `43d9dd4e1296799467f42b2c026aa107c3022d8d` | Obtainable origin head, not substituted for the tested Recipe source. |

The superproject inspected at `8924fce1` pins Ginux/Recipe to the first and fifth
rows. `.gitmodules:61–64` selects `LevitateOS/ginux` and `main`; its presence as a
contribution fork does not authorize a maintained downstream release branch.
No gitlink or `.gitmodules` change has been made in this task.

The tracked 2026-11-06 migration bundle is recovery material, explicitly not a
release. Imported into a new empty object store, it does not contain exact
`5300f5ef`, PRD04 `5449e2a1` or PRD08 `4aa51c1d`. The tracked patch/review queue
and historical snapshots are preservation inputs, not an already maintained
source selection/build procedure for all current corrections. They cannot
silently replace approval for a downstream delivery approach.

## Fresh old-pin build and incompatible CLI

A fresh HTTPS clone of `https://github.com/LevitateOS/ginux.git`, without local
alternates, private worktrees or imported object stores, checks out exact
`2eac10bf` and builds successfully with:

```sh
env -u RUSTFLAGS cargo build --locked --offline -p ginux-init \
  --target x86_64-unknown-linux-gnu --release
```

The build uses the installed host Rust/target and no previous launcher binary.
Its resulting launcher SHA-256 is
`f6ebf180c5f2ea31437277b7fe5212d696acee0bef72862b87da5650130c1895`.
This proves source acquisition/build for the old pin, not Recipe compatibility.
It does not establish fresh host toolchain/dependency provisioning.

The old `config` command accepts both its original profile and the text
`configured-balanced-v2`. The initial expected rejection was wrong; the old
parser stores an arbitrary profile identifier without resolving this contract.
Both outputs and the failed expectation are recorded. A configuration parse
alone cannot be used as a release capability check.

Source dispatch has no `Command::Launch` and sends unknown commands to its
Gate 1 demonstration (`crates/ginux-init/src/main.rs:145–169`). The exact Recipe
`launch ... --result-file ... --export-file ...` CLI control selects that demo,
then exits 1 with `payload did not stop after exec`, producing neither result
nor export. The demo hard-codes `/var/tmp/ginux-managed-payload` at line 1886;
it is not the configured Recipe worker. No payload was supplied there, no
successful demo is claimed, and no temporary executable workaround was installed.
The wrapper's initial demo-PASS assertion also failed; this is recorded rather
than hidden or counted as a configured runtime pass.

Recipe README's blanket assertion that an older launcher necessarily rejects
the profile during configuration validation is corrected in documentation-only
Recipe commit `cdcc054`. It now states the actual required interface and the old
workspace pin's incompatibility. Runtime behavior and the tested `b4144388`
source remain unchanged; the documentation commit is local and unpublished.

## Launcher, worker, image and callers

The existing owners must remain the bootstrap path:

- `tools/recipe/src/backend.rs:93` requires `GINUX_BIN` and checks an executable
  file. It neither acquires Ginux nor proves its source/capabilities.
- `tools/recipe/src/inputs/config.rs:110` renders configured catalog 2, writable
  memory assignments, assigned null and bounded network/storage policy. The
  typed storage join requires the explicit catalog-3 migration from PRD05.
- `tools/recipe/src/launch.rs:270` requires configured launch/result/export,
  optional inherited control and restore, including the four declared scratch
  leases. Ginux/Recipe lease lifetime still needs the recorded R-02 join.
- `tools/recipe/src/inputs/root.rs:194` stages the selected Recipe executable as
  `/inputs/recipe-worker`: explicit `RECIPE_WORKER_BIN`, otherwise the current
  executable named `recipe`, then PATH. The worker is built from Recipe source;
  there is no separately published worker binary that cures the missing source.
- `distro/builder/src/recipe/mod.rs:27` passes launcher/image/worker selections
  through. Its `binary` owner builds Recipe when no explicit `RECIPE_BIN` is
  supplied; it does not build/acquire the selected Ginux runtime or image.
  `xtask` delegates product production to the builder; its setup handles host/VM
  prerequisites rather than supplying this runtime combination.
- `tools/recipe/src/bin/recipe-image.rs` is the explicit existing image producer.
  It composes already acquired, digest-verified base/Rust/RPM inputs and emits
  a fresh `rootfs.tar` plus provenance. It is not an input downloader.

`tools/recipe/bootstrap/ginux-acquisition-image.json` currently names fifteen
inputs: the assembled pinned Rocky base, Rust GNU distribution, matching musl
standard library and twelve RPMs. Manifest SHA-256:
`9677220a43627d23b24b3ff5491ba702a014a4b1ff9188078a24a29e3b8bed6c`.
The [bootstrap guide](../../tools/recipe/bootstrap/README.md) documents the
existing producers/pins. A final bootstrap must explicitly obtain every selected
input and verify the produced digest; existing downloaded files/images are not
fresh acquisition evidence. No multi-gigabyte download or full image composition
was substituted while the required Recipe source itself is unobtainable.
Worker ELF/loader compatibility with that image must be checked on the final
build, rather than inferred from a successful host compile.

## Concrete delivery choice, not yet selected

**Preferred, already authorized direction:** keep the upstream-first route.
After the owner lands its foundation and required capabilities, adopt actual
verified upstream source, reconcile PRD05 and select the exact maintained commit.
Then align both obtainable submodule pins, bootstrap commands and builder
selections, and verify the complete fresh combination. PRD01 waits meanwhile.

**Alternative requiring explicit user approval:** prepare a maintained downstream
`recipe-runtime` ref in `LevitateOS/ginux`, pinned by exact commit in the
superproject. Keep an explicit upstream base, upstream-first replacement rule,
unchanged catalog 1/native policy and versioned configured additions. The local
preparation would reconcile PRD03 typed storage, PRD04's separate corrections
and PRD08's accepted clock/identity/IO boundaries with the later runtime and
consumer owners. It would not simply designate `5300f5ef` as a release.

That alternative also requires an obtainable Recipe revision and matching
locked coordinator/worker build. It must provide explicit source/input acquisition
and build commands from a fresh checkout with clean output locations, record
source/toolchain/image/binary identities, and run actual Recipe lifecycle,
checkpoint/restore/publication and required consumers against the resulting pair.
Distinct current findings/tests and release requirements remain required.

Approving preparation of that approach would **not** authorize any push, PR,
upstream comment, tag, release upload or superproject publication. Those actions
need review of the exact finished candidate and separate permission. No such
candidate is complete today, and neither delivery option closes the required
implementation/security/acceptance work through a packaging decision.

## Evidence, status and recovery

All current receipts are under
`.artifacts/ginux-contribution-preparation/prd01/`: remote source results,
`source-trace.json`, fresh clone/build logs and receipts, both profile controls,
failed old launch/result/export control, exact Recipe fetch failures, published
fork string search, and empty-store recovery object checks. The initial sandbox
clone failed DNS; the permitted network retry succeeded. No upstream write occurred.

Fresh required Recipe acquisition: **failed**. Fresh old Ginux acquisition/build:
**passed**, required runtime compatibility: **failed**. Complete fresh launcher/
worker/image combination and final-source capability validation: **not run**,
blocked by source availability and the delivery choice. No production readiness
or PRD01 completion is claimed. Existing unrelated edits/staging stay preserved.

Active skill: `hdae-development` at
`.agents/skills/hdae-development/SKILL.md`; reload the full skill/reference and
current queue after a context shift. Current task: PRD01. Exact next action:
obtain the user's delivery direction before any downstream source adoption;
continue only work independent of that choice while waiting. Upstream-first
remains the standing preference, not a user-imposed stop on local preparation.
