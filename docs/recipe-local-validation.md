# Local Recipe integration validation

This delivery implements the [ordered task queue](recipe-simplification-implementation-plan.md).
The [interface record](ginux-recipe-interfaces.md) describes current behavior.
All changes stay local: no Ginux PR, push, remote-text edit or maintainer message
is part of this work. Ginux contribution candidates remain in that queue's backlog.

## Workspaces and revisions

The superproject stays on `master`; Recipe stays on `ginux-recipe/r06-launch`.
Local Ginux work is in
`/home/vince/.codex/worktrees/ginux-recipe-local-integration/ginux`, branch
`codex/ginux-recipe-local-integration`. Its upstream reconciliation base is
`2a82f61e`, joined by `8b5cf5dd`. The original `tools/ginux` checkout and branch
are not switched. Its superproject gitlink is deliberately not used to imply
that the unmodified checkout contains this separate worktree's changes.

Current validation baseline supervisor: `93a1a56d`, immutable binary
`/tmp/ginux-t24-hidden-devices`, SHA256
`641db1d511ae0ba9ce538eb0bf764ab63cae20a5e015a8cb949c519009503df3`.
Final local Ginux HEAD is `7cf2638adaf2fd3b783be636fe2a05b2d60458c5`.
The additive corrections are `74835089` (bounded native reads above1MiB) and
`7cf2638a` (preserve owned virtual uid/gid0 or unchanged IDs; other ownership
changes fail explicitly). Immutable `/tmp/ginux-owned-root` SHA256 is
`e3d4b6ec6216f012aee5e657f7bd2364afccf7fc4a160f3a6d83573a68ee54d0`.
Full `make check test-unit` passes again at that HEAD; affected storage, FD,
checkpoint, exec, scripts, network and device suites pass. Logs:
`/tmp/ginux-owner-{check,storage,fds,checkpoints,affected-runtime}.log`.
The full Recipe run uses the stated baseline; later focused results are
separately recorded rather than relabeled as a whole-suite final-HEAD run.

The initial complete-run CLI/worker binary is `/tmp/recipe-t25-bin`, SHA256
`48f9c4ba2a30f857ea23a7481b3f28bfe4f2c510653a5ae562fcec3033edd3ff`.
Actual CLI harness `/tmp/recipe-t25-e2e` SHA256
`e9bf97b2cad48d330edbf3dc20990d92a133167733336c221560aab5a12968a4`.
It includes the current preseed closure fixture, committed as `a6ac832`.
Coherent implementation commits and source HEADs are recorded in the task
checkpoint; immutable executable hashes distinguish runs during parallel builds.


Final selected superproject revisions are Recipe`022e91c`, distro-builder`97c55e3`,
distro-contract`0d066d9`, AcornOS`c3ddd4a`, leviso`fbad656` and docs-content`06ffaa0`.
AcornOS already contained `d1777d3` before this batch; its unrelated untracked
lockfile is preserved. Original untracked task outputs are not committed.
Ginux's intended tested local dependency is the full7cf2638a worktree commit
above; its old checkout/gitlink is intentionally not rewritten. Both original
Ginux stashes remain. Standalone HTTP/TLS fixtures were individually stopped
successfully after validation; diagnostic/provenance artifacts remain.

## Prepared images and provenance

| Prepared image under `.artifacts/ginux-env` | Assembled tar SHA256 |
| --- | --- |
| `prepared-image` (GNU compiler) | `2e99d8dc80324b185267fcbf646eb407a469751f69a0d794ffc00977e8755fba` |
| `prepared-image-git` | `8bd5ad2e715c163ea201cc629856fb7ee970104f31e7d899913e185e56d3117b` |
| `prepared-image-git-musl` | `9f220633d34efff68b1535375fbb4fa3c4c2c8843917859fd68d8f6edec27ff4` |
| `prepared-image-acquisition` (adds cpio) | `c089f95c462582cc0fcc2a8b2ba86c7d2d64c22c82efa499ce20e3704a7fe15e` |
| `prepared-image-network-fixture` (test-only CA) | `fcc70d8c3052ff988a2223839a1050e5305c19ad6344f80cb3bd369c8b3132b8` |

Each producer output contains `provenance.json`. The committed
[acquisition manifest](../tools/recipe/bootstrap/ginux-acquisition-image.json)
and [bootstrap instructions](../tools/recipe/bootstrap/README.md) record the
exact ordinary inputs, publisher-metadata pin verification, source URLs and
producer command. The cpio input is official Rocky2.15-3.el10, SHA256
`ffd887348d611f779ae02cef89d7e1cae53adfe7901d31840bb9d5730915bb5a`.
The Rust1.99 musl component is separately pinned and selected, rather than
substituting GNU output for musl. Generated images, caches, test CA keys and
private diagnostic directories are not committed. Fixture trust is never
installed into host trust or the ordinary production image.

## Reproduction

Build only explicit producers/binaries from the workspace:

```sh
cargo build --offline -p levitate-recipe --bin recipe --bin recipe-image
cargo test --offline -p levitate-recipe --test e2e --no-run
cargo check --offline -p levitate-xtask -p distro-builder -p distro-contract \
  -p levitate-recipe -p acornos -p leviso --all-targets
```

Ginux's native bootstrap needs the documented privileges. Run the coordinator
and inherited channels with those privileges; wrapping only the launcher in
plain `sudo` closes required descriptors. A representative actual combined run
uses immutable copied artifacts so a parallel build cannot replace them:

```sh
sudo -n env \
  RECIPE_TEST_BIN=/tmp/recipe-t25-bin \
  RECIPE_WORKER_BIN=/tmp/recipe-t25-bin \
  GINUX_BIN=/tmp/ginux-t24-hidden-devices \
  GINUX_ROOTFS="$PWD/.artifacts/ginux-env/prepared-image-git-musl/rootfs.tar" \
  GINUX_ROOT=oci:recipe-compiler@sha256:9f220633d34efff68b1535375fbb4fa3c4c2c8843917859fd68d8f6edec27ff4 \
  /tmp/recipe-t25-e2e --test-threads=6
```

The initial complete132-case run finished **120passed/12failed** in1859.87s,
`/tmp/recipe-t25-final-managed.log`; it is not a green full-suite result. Eleven
failures were stale query/state fixture expectations. One exposed a real
finalization bug: the coordinator killed a successful worker's still-exporting
supervisor after10 seconds, leaving an empty result file. Isolated large-payload
controls passed, but an actual-supervisor delayed-completion marker reproduced
premature killing reliably (`/tmp/recipe-t25-completion-marker-repro.log`).

Recipe `022e91c` lets valid final export use the remaining execution deadline,
keeps error/cancellation teardown short, and prevents a killed/nonzero launcher
from publishing even after worker success. Query fixtures `dc24694` use API2
and unchanged-source/JSON-state assertions; `f01ae3a` verifies disabled recovery
without incorrectly forbidding durable diagnostics.

The final concurrent regression batch passes **26/26** in147.38s, covering all
corrected failures, actual2GiB+4096 payload,5000 nonempty files, mode/link/mtime
restart, all16 query cases, cancellation/deadlines/partial frames and two new
real-supervisor completion/exit-status cases. Log:
`/tmp/recipe-t25-final-corrections-managed.log`, CLI SHA256
`6fd538039524e054cf333f3f85e9a16bdb41d7b03db31d9db8f4cf18229ccbda`,
harness SHA256 `8f0aa949d1499faf2586ea36648dd24877d7ac32f9ea695619c0cd6643069200`.
The final catalog has134 scenarios. This is complete baseline coverage plus
focused corrected-path revalidation, not a second all134-case run.

Final committed Recipe is `022e91c5049350bf449847a2bc1365d067f1c290`.
Delivery executables: `/tmp/recipe-local-final-bin`, SHA256
`94d6b95654608f7f3abc34c7d4a40afa3578e628e11d6784094fc783f228972f`,
and `/tmp/recipe-local-final-e2e`, SHA256
`784e74d3ac37f6fc97da1cb9c1ef3f8247c3bca7c89d95936f077c26b0a264c5`.
After the26-case run, two post-worker supervisor-error messages were corrected;
no other runtime logic changed. The committed binary builds and the shipped
hello demo passes again (`/tmp/recipe-local-final-build.log`,
`/tmp/recipe-t27-hello-final.log`).

Separate real public-library execution with spoofed worker markers passes1/1
(`/tmp/recipe-t25-library.log`); actual image-producer composition/rejection
passes1/1 (`/tmp/recipe-t25-image-producer.log`). All six affected Rust packages
pass all-target compilation (`/tmp/recipe-t25-final-joined-source-check.log`);
the final diagnostic-only edit also passes all Recipe targets.

Actual managed TLS (trusted, wrong hostname, untrusted CA), HTTPS pinned Git and
publisher-pinned Alpine acquisition→extraction→execution→publication pass again
with the final baseline binaries. Exact commands/case outputs are retained under
`/tmp/recipe-lane-b-network/`; final logs are
`final-{tls,https-git,alpine-source}-93a1a56d.log`.
The production Alpine version output is
`apk-tools 3.0.9-r0, compiled for x86_64.`. The verified package SHA256 is
`ea77b2533566e70c9566c03748199b79348010c3199ba02d97e0634cf0fa9ef7`.
Actual RPM→rpm2cpio→cpio extraction→named publication also passes at final
`7cf2638a` with the acquisition image: mode0755 uid/gid0,26,671,584 bytes,
SHA256 `cba3b1f95298902f87c9730c61f746651282f326b126bd1ba964b4c61f866cde`,
independently matching the pinned RPM metadata. Source RPM is unchanged;
no ownership flag/error was suppressed. Proof:
`/tmp/recipe-lane-b-network/rpm-repro/published-validation.json` and
`stderr-owned-root-corrected.log`. All four actual musl live-tool
builds now pass; split-pane correction log
`/tmp/recipe-t14-split-pane-musl-large-read.log`,4 acknowledged checkpoints,
1,582,848-byte static executable SHA256
`a929913760b34d7daa2bf4b0897baff24beb106a7408e17df07b2b6f8d3e6393`.
This proves the version/help entry, not interactive PTY or VM behavior.

QEMU's full15-RPM selected-product pipeline now passes in distro-builder
`97c55e3`: verified acquisition, checked conversion/extraction, strict version
checks, named publication and atomic `.artifacts/tools/current` selection. Its
runtime library closure is explicit, and the unused privileged host bridge
helper is explicitly excluded. Actual selected host consumers report10.0.0,
empty stderr, matching executable hashes/firmware, and create/inspect a disposable
1MiB qcow2 file. Evidence: `/tmp/recipe-qemu-typed-libraries.log` and
`/tmp/recipe-qemu-selected-consumer.json`. Unsuppressed Ginux epoll warnings during
inside-domain version startup do not establish QEMU workload execution in Ginux.
The actual host-consumer checks have no such warning. No VM boot is claimed.

## Working demonstration

The committed [hello recipe](../tools/recipe/examples/hello.rhai) is the same
current-contract example as the authoring guide. With the configured launcher
and verified image, run:

```sh
recipe install tools/recipe/examples/hello.rhai \
  --build-dir .artifacts/ginux-env/demo/hello-work \
  --state-dir .artifacts/ginux-env/demo/hello-state \
  --json-output .artifacts/ginux-env/demo/hello-result.json
```

This actual local invocation passed with one acknowledged checkpoint and intact
source. Its named program is mode0755, exact payload
`#!/bin/sh\nprintf 'hello\\n'\n`, SHA256
`f869aed15d808e5e860a3202973185b70255b03ebd936437b381aa3eed6c18c5`.
Evidence: `/tmp/recipe-t27-hello-final.log` and `hello-result-final.json` (earlier
`hello-result.json` and publication are retained). Products/state remain
owned by the coordinator and publication parents remain private; privileged
production does not automatically make a product accessible to an ordinary user.

For real compilation use `distro-builder artifact prepare-tool-source recfstab
.artifacts/recfstab-source`, then the canonical `distro/builder/recipes/recfstab.rhai`
with `--input source=...`; the recipe compiles inside Ginux. For QEMU preparation
use `cargo xtask setup`; boot scenarios consume the selected existing product.

## Verification limits

Prepared kernel source/prebuilt cases validate the caller/lifecycle migration;
no kernel was rebuilt for it. Small prepared rootfs/preseed fixtures and actual
package transfers do not claim a full Fedora ISO materialization or VM boot.
Repair mechanics use a controlled provider, not external credentials.
Ginux's inherited per-process address-space/CPU-affinity limits are not aggregate
RSS or CPU-time accounting. Endpoint grants do not isolate cohosted virtual names.
All demonstrated extraction/build/finalization failures have been resolved with
recorded actual rechecks. Full-ISO, VM, interactive PTY and broader Linux metadata
coverage remain outside this implementation's demonstrated scope.

## Future Ginux contribution candidates

The [complete contribution task list](ginux-contribution-task-list.md) covers
these candidates and their earlier local foundations, dependency order, review,
validation and the user checkpoint before publication. Its work packets are not
a finalized PR count.

These are local records, not PR drafts ready to publish. Re-evaluate against
then-current upstream, extract the smallest coherent change and make each
proposal standalone once its prerequisites are available upstream. No pending
PR should be used as an implicit base.

| Purpose | Local implementation references | Prerequisite and actual evidence |
| --- | --- | --- |
| Owned directory cwd and shared hardlink identity | `6a45d8f8`, `9e2a39e8` | Existing configured owned storage; real relative/fork/alias/unlink cases |
| Cross-image/owned traversal | `1da0ce13`, hidden-device correction `93a1a56d` | Shared VFS/assignment owners; existing exec/device cases preserved |
| Scalable bounded storage and archive identity | `38fa14fd`, `e3374593`, `53a2c9cc` | Owned storage backing and private FD ownership; actual5000 nonempty files and2GiB+ payload |
| SQLite/Cargo file locking | `26784843` | Owned open descriptions; actual simultaneous Cargo cache users then compile/run |
| Timestamp preservation | `d21d26c8` | Inode identity and archive format; actual hardlink/symlink/restart timestamps |
| Bounded TCP assignments and readiness | backend `8ff83d64`–`9a70427b`, dispatch `66de2880` | Maintainer network policy plus device/descriptor owners; actual grants/TLS/Git/budgets/revocation |
| Correct private/public FD budgets | `d5357101` | Public descriptor ownership; lower operator limits and actual many-file use |
| Vectored and bounded regular IO | `88fa4749`, `7d556b4e`, `74835089` | Existing owned/native IO owners; actual APK/RPM/large-input failures and corrected cases |
| Explicit inherited process bounds | `0c571cbb` | Configured native bootstrap; actual AS exhaustion, fork/exec inheritance and affinity denial |
| Virtual platform identity | `9cf265ba` | Existing identity view; real RPM tool and raw ABI checks, no host uname |
| Duplicate task-death accounting | `07c949db` | Ptrace/subreaper lifecycle; actual Git failure cleanup and repeated native process cases |
| Virtual root ownership preservation | `7cf2638a` | Existing owned metadata; actual cpio and invalid-ID/missing-object/nofollow cases |
| Checkpoint/restore CI coverage | `5e20a65f` | Existing checkpoint fixture/feature; adds coverage while retaining every upstream unit/static job |

Any later concrete corrections receive their exact commits in the final checkpoint. Some capabilities share prerequisites and are
not independent proposals today; this table does not turn the entire local
integration into one large upstream request. It also does not duplicate existing
PRs #216–#218 or authorize editing their human text.
