# Upstream integration — 2026-11-09

Local integration of the requested upstream pull and authorized Recipe/caller
migration are complete and locally verified. This is not release or security acceptance.
No push, PR, upstream comment, tag, hosted dispatch or release is authorized
or performed by this task.

Active skill: `hdae-development` through
`.agents/skills/hdae-development/SKILL.md`; reload the complete skill, its
required reference, root AGENTS.md and this checkpoint after a context shift.
The root instructions require proportional maintenance, without new process
or branch gates. The user explicitly authorized migrating Recipe and callers
to upstream's new format after the compatibility conflict was identified.

## Selected sources and replacement

- Fetched `gmkurtzer/ginux:main` from `53a54addea67969abaa240ea478def1b94130eee`
  to `55b719db9c52889543fbb177fab4cf2b2f90955e`.
- Ginux main is local merge `ef1b538f940d8ae99b5e798c19cada9976381d86`, with
  parents `e4d95de10520aa8e91bc2ec69e64da004443e2da` and the new upstream main.
  Both the active integration worktree and `tools/ginux` select it. Upstream
  history, new virtual proc/sys implementation, contract maintenance workflow,
  instance CLI and CI changes are retained.
- Recipe's functional migration is
  `2a786826c73b8aa9111b1a6620a30fa1e65ef6b5`. Local master
  `9f16113807dad92b2b2edf21d4b0d78765ee505c` adds only the matching Ginux CI
  source pin and setup documentation. Rust, Cargo files and test harnesses are
  identical between these two Recipe commits. Runtime results identify the
  actual compiled functional source rather than relabeling it as the CI commit.
- `bootstrap/managed-ci.json` selects Ginux ef1b538f. This commit is not yet
  public, so hosted acquisition/execution remains pending authorized delivery.
  Historical public delivery manifests and parent gitlinks are not relabeled.

| Behavior owner | Disposition |
| --- | --- |
| Legacy `[userspace.*]` / profile configuration | Replaced by upstream `[contract]` / `[contract.*]`; removed syntax stays rejected. |
| Versioned catalog and configured runtime | Catalog 1002's operation membership, handlers and bounds retained; upstream catalog 2 remains separate. |
| Recipe derived grants, digest pin, denial explanation | Migrated to upstream global selectors and explicit exclusions; receipts expose actual `allow` and `deny` arrays. |
| Configured/native parser interaction | Storage/network duplicate-field bits moved to unused u128 bits 82–86; normalized endpoints and limits retained. |
| Newly available native typed policy fields | Configured launch rejects requested native-only filesystem, device, privilege, process and kernel controls before workload entry. Twelve focused refusal controls retained. |
| Configured clock, owned FD/stack/O_PATH/events and storage/transfer/export owners | Retained. `source-reconciliation.json` accounts for 61 byte-identical downstream paths and 22 reconciled paths. |
| Existing tests/callers | Migrated schema, denial injection and native fixture selection; no tests deleted. Audit test independently asserts the unchanged 65,536-byte trace budget, whole records and bounded diagnostics despite the larger normalized preamble. |
| Contract host tests/document generator | Use the existing host RUSTFLAGS/target convention, avoiding the freestanding runtime linker flags. Validation is preserved. |
| Gate 11 / #249 | Its original head is now an ancestor of actual upstream main; the earlier locally included source is retained through actual merged history. |
| Required storage PRD05 chain | Still incomplete. The six tracked prerequisite heads are not ancestors of this upstream; incoming changes do not replace ginux-vfs or introduce launch_paths/launch_storage. This is not a squash/equivalence or GitHub merge-state acceptance claim. |

Both old and migrated Recipe bases were expanded against all **193** operations,
with and without network grants: no admitted/denied action or operation status
changes. New explicit syscall exclusions use upstream's EPERM semantics where
old missing syscall grants returned ENOSYS; their authority is still denied.
Semantic default denials remain default denials. Provenance/rule indexes and
therefore effective digests change as expected:

- without network: `41fb0de923722522be3fab629695f72629951b7be074111a1be83d972db7a31e`;
- with network: `92b6add40edad919cf394ff902670715d7b4385a9286751191910025d5a5e30d`.

Do not pair migrated Recipe with the previous launcher. Old receipts and cache
identities retain their original source/contract meaning; these results do not
upgrade earlier release or security receipts.

## Exact verification

Evidence directory:
`/home/vince/.codex/worktrees/prd15-owned-fd-path/ginux-prd11/.artifacts/upstream-2026-11-09`.

- Ginux launcher SHA-256:
  `a7cc61f211d158150730e819f0f1a4b7d082b7c396a7afc6b0787ab8a6e65df4`.
  The updated `tools/ginux` checkout builds to identical bytes.
- Recipe CLI and matching worker SHA-256:
  `9f47819f8b815502e12db9d5609fa3a413f3928c01f0c532623eaf023d67056f`.
  Build uses the actual workspace lock; this is local integration evidence,
  not a new standalone release-package or fresh-host receipt.
- Image producer SHA-256:
  `d8fe795b02b25620af408a7654114f8ae4b9b3e07783d13306c99ce07448dde0`.
- Acquisition image SHA-256:
  `ffe04d8671e45036ac77acc958fb6cfd3c24a939075dc7e6c5437b69d7f05c15`.
  The new producer plus updated tracked CI pin and upstream OCI producer
  reproduce this digest using the retained verified public inputs offline;
  RPM signatures and expected image digest are checked by the actual wrapper.
- `final-ginux-results.json`: **23/23** targets pass on ef1b538f, including
  strict static checks, **148** unit tests, generated contract coverage,
  CI-change/benchmark scripts and all 18 selected Linux runtime targets.
  The configured contract runner includes **59** cases, including new refusals.
- Recipe release build, fmt and strict Clippy pass. **22 distinct focused
  integration cases pass** on the same compiled source/binaries: all 14 NEEDS
  cases, six additional managed consumer/state/metadata cases, library_backend
  and image_producer. `validation-summary.json` selects their actual successful
  receipts. Initial missing builder/repository environment and missing deliberate
  worker-spoof environment caused two harness setup failures. Both are retained
  and corrected environment reruns pass on unchanged harness/production bytes;
  neither failed attempt is counted as a pass. This is not a full-suite claim.
- All four canonical upstream operator wrapper commands validate/inspect/explain/
  diff pass. Both actual Recipe renders are compared with the authority-expansion
  inputs, and all artifact hashes are rechecked after execution.
- `consumer-results.json`: **6/6** actual consumers pass: SQLite contention,
  concurrent GNU/musl Cargo/cache and built threaded programs, pinned Git
  HTTP/HTTPS, RPM and APK. Every consumer retains its own artifact identities.
- Previous failed merge/caller attempts remain in their original logs. The
  initial freestanding host-test link failure is corrected; native fixture
  failures were invalid field selection and missing executable mode, both
  corrected without changing runtime policy. Duplicate injected deny fields
  are corrected. A full final Ginux run passes after these corrections.

Useful reproduction commands (from their owning checkouts):

```sh
make build check test-unit test-contract test-ci-changes test-bench-compare
sudo make test-launch-contract test-launch test-native-supervision
sudo make test-launch-storage test-launch-devices test-launch-pipes test-launch-fds
sudo make test-launch-tasks test-launch-exec test-launch-poll test-launch-directories
sudo make test-launch-owned-exec test-launch-scripts test-launch-checkpoints
sudo make test-launch-restore test-launch-network test-launch-threads test-launch-image-io
cargo build --release --locked --bins --tests --message-format=json
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
```

Recipe per-case commands, selected environment, build JSON, source/binary hashes
and logs are retained in the evidence directory. Only integration harnesses run;
no Recipe unit tests run. The full earlier 155-case pass is not claimed on this
new combination. Upstream VM/systemd maintenance/CLI scenarios, full product
boot/install/PTY/A-B, hosted jobs, final-source security acceptance, standalone
packaging and fresh public checkout reproduction are not established here.
macOS remains indefinitely deferred.

Frozen PRD04 worktrees remain unchanged: `5449e2a1` / launcher `95bc570e…`
and `2e88c9ad` / launcher `576ee03c…`. Their historical focused export acceptance
is preserved; broader final-source security acceptance remains open. Existing
staging and unrelated worktrees/edits are preserved. No new parent pin or
provenance publication is performed.

## Current next action

This bounded pull/migration task is complete. No running verification session
remains for it. `validation-summary.json` owns the final selected results and
limitations; frozen executables and complete Ginux/Recipe diffs are retained
with the receipts. The evidence manifest verifies **294/294** retained regular files, with three
symlinks described separately. Manifest SHA-256:
`091cbf7945fba51dcff25e8b576d80725c79b06236d1e742754106a1d629081a`.

Keep PRD05, PRD09 and PRD17 incomplete under their existing criteria. Their next
steps are the still-required upstream/storage reconciliation and final release
and security acceptance on an authorized selected delivery. Source pushes,
publication, the new release version and ISO reproducibility decisions and
security authentication/results remain separate open requirements. This task
does not authorize publishing the new local Ginux/Recipe heads or changing final
superproject delivery pins/provenance before durable availability.
