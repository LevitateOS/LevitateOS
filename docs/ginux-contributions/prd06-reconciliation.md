# PRD06 — local correction and Recipe audit reconciliation

PRD06 is complete as source reconciliation. Every input below has a retained,
upstream, replaced or downstream disposition in the [source ledger](prd06-source-inventory.json).
The ledger includes exact source/base IDs, every diff hunk and its hash, semantic
owners, test versions, preserved edits and explicit replacement tests. Retained
means keep the implementation or regression intent with its named contribution;
it does **not** mean the final integration, publication review or security
acceptance has passed. PRD04, PRD05 and PRD07–PRD16 remain open as applicable.
No Ginux/Recipe implementation, branch, gitlink, upstream PR or remote was changed.
Downstream is an ownership disposition: configured clocks remain required in
the release; scratch policy and optional refactors remain excluded until reviewed.

## Sources and completeness boundary

The read-only GitHub refresh on 2026-11-08 found upstream main
`53a54addea67969abaa240ea478def1b94130eee`. The comparison uses:

| Source | Exact revision / input | Disposition boundary |
| --- | --- | --- |
| Current Recipe integration | `5300f5ef9a8281f4f10f5519d5eb761654e04829` | 53 non-merge commits beyond main; 299 net hunks, all assigned. |
| PRD04 candidate | `5449e2a184bb1147cf618b140a29196b74206b13` | Four additional corrections, 22 hunks; frozen, with failed export acceptance below. |
| Storage foundation | `9802ef036d53ae1ee5276fe59cc719389dda5879` | [PRD03 review](prd03-storage-review.md), 119 feature hunks over `f1c3d3a7727f5a8027b448b4dcad0ad52c896c4c`; 103 prerequisite hunks against main. |
| Original donor | `7cf2638adaf2fd3b783be636fe2a05b2d60458c5` over `3f2fe7dfadd4b40e235d982a6a3d386dc6490177` | All 66 commits and original 261 net hunks, rather than just patch exports. |
| Other historical work | All 49 entries in [P03](p03-catalog.json) | Superseded prototypes and four review corrections individually retained/replaced below. |
| Prepared work | All 47 [P05 entries](p05-package-plan.json), including 37 tracked patch exports | Parent placeholders are not extra implementations. Exact head/base and every stored patch hunk are independently indexed. |
| Other reachable local work | Every registered local/origin ref and worktree HEAD | 62 additional commits, 11 otherwise uncovered ancestors, plus reviewed G07 snapshot `5fa497c1`. No remaining non-main non-merge ancestor lacks a source/packet/diff entry. |
| Dirty/stashed work | Six recovered worktrees, 39 files; stashes `2b887da6` and `a94d56fb` | Every preserved file/hash and dirty hunk accounted for; present recovered files still match preserved bytes. Staging/conflicts remain intact. |
| Recipe / builder | Recipe `b41443880fe585a27bc799d80382c9ff5b07047b`; builder `0044634d616d7888b93d048b4bd39cb0d2db0241` | All 13 historical audit findings reconciled below, with current production callsites. |

The inventory contains **246 commit entries, 7,036 hunk entries, 1,869 current
source symbol entries, 218 distinct Rust test path/names and 197 fixture source
versions**. These counts overlap intentionally: a retained commit, its exported
patch and its final integrated hunk are separately identifiable evidence, not
independent fixes or additional passing tests. Every hunk has an explicit
per-owner disposition; mixed hunks preserve their retained parts while replacing
the old foundation. There are zero unknown owners or unmapped Rust tests.

All **89 upstream-main unit cases** survive in the integration: 88 have identical
function bodies; the unlink/recreated-name case intentionally changes independent
alias reads into shared-offset reads (`o`, then `ld`, then EOF) for #232. Its
old-object, new-object and CLOEXEC assertions remain. All 138 integration unit
cases are indexed; no test is removed to make the comparison pass. Historical
units and fixture versions include cases added then removed before a later head.
The [test retention record](prd06-test-retention.md) identifies the actual missing
oracles rather than treating a test-name count as proof of equivalent coverage.

The untracked `UNTANGLE.md`/`.untangle` work in `ginux-pr235` is a separate
architecture audit and its map scripts/drafts, explicitly containing no product
implementation. It stays downstream and untouched; its untriaged architecture
findings are not certified closed by this reconciliation. Arm64-only upstream
#239/#240 are outside this local x86-64 correction set. Merge commits are ancestry,
not additional implementation hunks.

## Upstream and replacement decisions

Merged #216/#217/#218 own failed-open rollback, unlinked-object lifetime and
component-ordered path traversal in current main. Preserve those owners and
current tests. The old aggregate donor diff is replaced by the current composition,
including its negative incoming-native hunks: do not replay deletions of protected
copyout helpers, read-only/cross-page wait buffers or fallback tests. The native
runner and process fixture are byte-identical to main; seven native wait/copyout
function blocks are also identical. Individual retained feature commits are
indexed separately from this superseded aggregate diff. Do not replay the staged combined VFS patch or resurrect the old
lexical normalizer. Exact whole-commit `git patch-id --stable` found no additional
byte-equivalent local commit on main; that does not override these verified
semantic replacements or imply pending PRs have merged.

| Work | Current upstream/source fact | Disposition |
| --- | --- | --- |
| G01 console | [#207](https://github.com/gmkurtzer/ginux/pull/207), head `5ae058f2`, open | Retain existing PR and configured readiness joins; policy/race acceptance remains PRD08/09. No duplicate PR. |
| Shared offsets | [#232](https://github.com/gmkurtzer/ginux/pull/232), `5e79a172`, open | Retain its three focused tests and shared Description owner; already in PRD03 prerequisite chain. |
| Workdir creation | [#233](https://github.com/gmkurtzer/ginux/pull/233), `f751cd6f`, open | Retain syscall correction and missing-workdir regression; PRD03 prerequisite. |
| Orphans | [#234](https://github.com/gmkurtzer/ginux/pull/234), `78a534c1`, open | Retain its six-case suite as well as current aggregate process tests; neither suite licenses dropping the other. |
| Directory stdio | [#236](https://github.com/gmkurtzer/ginux/pull/236), `1f43d7fa`, open; local `1104d5ea` | Same correction owner; preserve inherited-stream controls. |
| Signal-driven I/O | [#238](https://github.com/gmkurtzer/ginux/pull/238), `efaeeca8`, open; local `719aac91` | Retain upstream native policy correction plus separate configured `158e11a6` refusal; both paths matter. |
| Overlay proof value length | [#241](https://github.com/gmkurtzer/ginux/pull/241), `a0a304f2`, open; local `432ff7be` | Retain existing correction, no duplicate; not a storage-export fix. |
| Native SIGCHLD | [#242](https://github.com/gmkurtzer/ginux/pull/242), `bf0e00ba`, open; local `c150b802` / `0c92e2e6` | Retain one owner; host-PID siginfo limit remains explicit below. |
| Resolved contract / disabled audit | [#243](https://github.com/gmkurtzer/ginux/pull/243), `067d992f`, open | Retain upstream fix. Older local domain/instance variants are replaced by PRD02's configured catalog-2 composition and PRD03's catalog-3 selection. Preserve upstream catalog 1/native policy. |
| Native waitid PIDFD | [#244](https://github.com/gmkurtzer/ginux/pull/244), `ef05b4b5`, open | Retain reviewed branch and tests; scratch `c9225b86` is not a second delivery. |
| Native clone3 cgroup | [#245](https://github.com/gmkurtzer/ginux/pull/245), `dec645c3`, open | Retain existing fix/tests; virtual membership semantics remain bounded below. |
| PIDFD_GET_INFO | [#246](https://github.com/gmkurtzer/ginux/pull/246), `4d3582f2`, open | Retain ENOTTY correction/tests; do not mistake it for a general ioctl authority fix. |
| Native waitid reap | [#247](https://github.com/gmkurtzer/ginux/pull/247), `52f6474c`, open | Retain existing fix/tests and explicit orphan/ignored-SIGCHLD limitation. |
| Native cgroup mkdir | [#248](https://github.com/gmkurtzer/ginux/pull/248), `14881d13`, open | Retain existing fix/tests; three synthesized files are not general cgroup support. |

Open #224's protected clone-TID stores and #226's native kill/reap cleanup are
upstream-owned overlaps, not missing work to duplicate. #227's release overflow
checks are separately upstream-owned. Every pending PR still needs current-main
refresh and exact candidate review in PRD07; listing it here is not delivery.

## Semantic disposition of the retained implementation

The ledger lists every commit individually. The following joins explain how its
owners compose; they are not instructions to replay whole old files.

| Owner / useful implementation | Disposition and exact destination | Tests to preserve |
| --- | --- | --- |
| G03/G03b/G03c basic storage | **Replaced** by PRD03 typed Whence/flags/Slot, live copy-up identity, modes, directory operations, shared append and error ordering at `9802ef03`. Replace raw seek/advisory-mode and old cwd/dir implementations, not later capabilities. | PRD03's 56 VFS cases, complete storage fixture and four failing-base controls; old rollback/unlink/path tests remain authoritative. |
| G04 assigned devices | **Retained** in `device_hooks`, explicit null device, namespace hiding and FD classification. | Assigned/unassigned/hidden-device and stream controls. |
| G05/G08 public descriptors/channels | **Retained** in `descriptors`/`channel_hooks`, composed with #232 descriptions. Keep pipes, nonblocking, packet channels, bounded waits/readiness and partial progress. | Alias/slot-reuse/CLOEXEC, poll snapshots, parked peer close/retry, packet/native/private FD fixtures. |
| G06/G07 task/thread ownership | **Retained** current `launch_tasks`/`DescriptorGroup`, replacing incomplete G07 snapshot. `12c176d4`, `4a8d50f7`, `12c4ea80`, `062fb5ef`, `e9a2f8a0` keep per-task calls/poll state, shared resources, first creation diagnostic, 32-task bound and musl flags. | Task/exec/thread, reservation rollback, orphan suites, compiler process-tree and nonleader cases. The 32-task cap remains explicit. |
| Full blocking pipe writes | **Retained**, `6a19a70a`, in existing channel retry/wait owner; do not regress to a short successful compiler-pipe write. | Buffer-spanning blocking write, cancellation and partial-progress cases in pipe/thread fixtures. |
| Creation masks | **Retained**, `436c4ae5`, process-owned mask copied at fork/shared by threads. PRD03 foundation's fixed 022 is not the final integration behavior. | Parent/child/thread and SQLite creation-mask fixture. |
| Shared argument/kernel-use holds | **Retained**, `f9a584e9`, `e0519b48`, `1c2b1333`, `ad364bd7`, `cde7dc22`, `a0419e24`. Seize before guest runs; hold peers until path/exec/map/SIGCHLD/TID kernel use completes. | All associated path/exec/mapping/scratch/SIGCHLD/cross-page TID tests, including nonleader and nested child-TID controls. Prior security passes are source-bound. |
| Empty-path and descriptor reopen | **Retained**, `c7b4b73a`, `6d290d48`, under descriptor/path owners; use actual selected object's rights, not a second string-based namespace interpretation. | Reopen confinement, descriptor access and exact mapping dispatch `b4c69551`. |
| G09 backing, richer namespace | **Retained** generic `OverlayStorage`, bounded growth/memfd identity, truncate, directory cookies/rename; port onto PRD03 without restoring old mode/seek semantics. Include index extent/NUL hardening `b168c2c5`, `2914deaf`. | Growth/bounds/quota, copy-up rollback, holes/zero-fill, directory identity/cookies, malformed index. T-02 remains a required port. |
| G10/G11 executable/script ownership | **Retained** `exec_image`/`exec_script`, sealed memfds, pinned metadata, bounded interpreter chains, vector rollback/CLOEXEC and held pin release. Both interpreter stashes are replaced by these owners. | Owned ELF/script/interpreter, source-byte, cross-page, fork/shared path, retirement and failed-exec controls. |
| G12 mappings | **Retained** `file_mapping`, readonly transfer and kernel mapping/backing lifetime. Manual old loader/map ledger is replaced. | Current mapping/protection/alias/nonleader/failure cleanup plus old partial-page/stale-handle/offset oracles. T-01 is absent in current fixture and retained explicitly. |
| G13–G16 namespace identity | **Retained** symlink component walker, cwd refs, hardlink pins and image/owned boundary selection; adapt to PRD03 `launch_paths`/directory owner. | file/../, missing/../, file/., alias/rename/unlink/cwd/fork/leader-exit and cross-boundary tests. |
| G17a/G17b/G18 positioning/capacity | **Retained** pwrite plus captured public bounds/private reserve and mmap-backed namespace metadata; no giant stack-array revival. PRD04 inherited-FD fix is retained with the lease join below. | Shared-offset/append/negative-offset, namespace scale, FD limits, inherited-pressure and backing lifetime. Public/hard limits must not widen. |
| G19 export | **Retained** archive writer, flush and pinned completed inode/parent (`e354efd5`); `ccba6747`/`3dae449a` close reporting remains useful. **F-01 is failed**: final backing close is checked after final export publication. | Archive/flush/mutation/cancellation/reference-close controls and focused final-close retest after correction. |
| G20/G21 checkpoints/restore | **Retained** private control framing/quiescence and sealed restore snapshot (`76b086e1`), structured close errors (`881d46fa`). | Framing/event/cancel, recovery generations, archive corruption/replacement, hardlink and close-error controls. |
| G22–G25 archive/locks/time/I/O | **Retained** coupled producer/consumer hardlink format, virtual locks, inode timestamps and bounded vectored/chunked I/O. Do not split archive reader/writer changes or drop partial-progress behavior. | Hardlink/mtime round-trip, lock split/reuse/contention, vectors/large reads/writes, pipe partial progress. |
| G26/G27 resources/queries | **Retained** child resource bootstrap, task AS/affinity, configured identity/query and large image read `86e95083`; retain `8a7eb37c` memory-vs-network dispatch fix. | Width, caller ownership, finite 64MiB read bound and image/RPM/query fixtures. These are not aggregate RSS/CPU accounting. |
| G28/G29 bounded TCP | **Retained** backend plus syscall/FD/readiness/policy joins; formatting-only `020f969b` stays with feature. | Backend loopback, endpoint/quota/revoke/drop/failure plus aliases/readiness/peer-death. |
| G30 metadata | **Retained** virtual uid/gid and target/archive round-trip, independent of host privilege. PRD03 common-class mode model remains explicit. | Ownership, special-bit metadata, reimport and target metadata consumers. |
| Signals and clocks | **Retained** bootstrap mask clear `d24cd514`, caller-thread SIGPIPE `e4a43468`; **downstream** configured-only clock/sleep `a7ce73cd` uses catalog 2. | Signal-mask, thread-directed pipe/TCP SIGPIPE and six previously blocked Recipe sleep cases. Native G02 remains policy-held. |
| Contract/config/wiring | **Retained** `8981bf77`, `50204a29`, `5300f5ef` at existing instance/contract/runtime gate. Catalog 3 adds storage; old catalogs/digests/native policy stay intact. G31 separate CI-only packet is **replaced** by feature-owned checkpoint/restore CI. | Contract deny/source/audit-off, old-profile equivalence and each feature's existing Make/CI invocation, without deleting main jobs. |
| Four PRD04 commits | **Retained**, inherited descriptor retirement, regular read/readv semantic policy, pre-restore assignment prefix checks, sticky structured backing-close errors at `5449e2a1`. | Authorized Daybreak items 1–4 and selected 6 pass; item 5 failed. Keep original failed artifact/oracle. |
| Optional typed-handle dirty experiment | **Downstream**, source-preserved owner-local G05/G08 simplification; no separate refactor or requirement to port it wholesale. | Preserve original bytes; useful behavior assertions stay with current public FD owner. |
| Old prototypes | **Replaced** native/demo file/process/exec/raw-channel/manual-mapping/binary-retained-storage/cancellation implementations with current upstream native or configured owners. | Keep regression intent, not stale raw-token APIs or a second persistence format. See test retention record and per-commit rationale. |
| Native boot scratch | **Downstream**, `b75edda7` excluded as-is: changes catalog-1 wildcard authority, audit bound 8192→1,000,000, console dup targets, wait4(0) approximation and restart behavior. `366fa4fb` TCGETS2 and reviewed #244–248 remain separately retained. | Preserve boot/TCGETS2/native scenarios as evidence. A reviewed/versioned policy and correct semantics are required before any scratch hunk enters release source. |
| Historical plans/cache removal | **Downstream** evidence; generated bytecode removed by `dbb356f9` stays removed. Do not put LevitateOS plans/policy into Ginux contributions. | Source objects, tracked patches and original receipts remain available. |

## All historical Recipe audit findings

The [2026-02-10 audit](../history/recipe-engine-audit.md) is history. Its checked
boxes and WONTFIX statement do not describe current acceptance. These rows are
the current disposition, using exact Recipe/builder revisions above.

| Historical finding | Current source, evidence and disposition |
| --- | --- |
| 1. stdout corruption | **Replaced/closed on current pair.** `helpers/internal/cmd.rs::streamed` routes both streams to Diagnostics; capture keeps stdout and forwards stderr; worker protocol owns JSON. Fresh `cli_shell_helpers_preserve_status_capture_and_payloads` and captured-failure CLI checks pass with parseable output and retained diagnostics. The old fd-dup workaround is not the owner. |
| 2. No Rhai limits | **Closed source condition, retained broader resource acceptance.** `core/invocation.rs:10` sets 2M operations/64 call levels/64MiB strings; `worker/run.rs:313` calls this engine. Every public executable RecipeEngine operation enters managed launch. These settings are not proof of total process-tree RAM/CPU accounting; PRD12 owns the declared resource controls. |
| 3. Developer absolute paths | **Replaced/closed for current production recipes.** All 17 active tracked `.rhai` files have no `/home/vince` or `ctx::persist`. Immutable RUNTIME/input discovery and owned work/tool/product paths replace persisted host ctx. Old rocky/alpine sources are no longer active producers. `/home/<name>` target-home metadata is not a developer checkout dependency. Final portability/product acceptance remains PRD13–16. |
| 4. Shell concatenation/quoting | **Open R-01.** Checked argv helpers exist (`helpers/util/process.rs`), but do not repair shell callers. Builder `linux.rhai:108,111,122,235` concatenates unquoted source/build/staging; `:128`, `rootfs-producer.rhai:63,72`, `linux-deps.rhai:61–63` and `fedora-rpm-source-rootfs.rhai:40,44` use single-quote concatenation without escaping apostrophes. Static command-v/which selections are fixed strings; all dynamic-path sites still need argv or correct quoting. Concrete acceptance below. |
| 5. Temp directory leak / old WONTFIX | **Replaced; R-02 join open.** `lib.rs::with_temporary_destination` owns TempDir; `temporary.rs::TemporaryDirectory` owns private scratch/lease cleanup and stale-directory recovery; no old keep-per-failure mechanism is accepted. `launch.rs:245` deliberately inherits four lease FDs. PRD04 closes them as undeclared. Coordinator-held leases and parent-death kill mitigate lifetime; the changed cross-owner contract has not been validated. Old WONTFIX is withdrawn as a current disposition. |
| 6. Ignored shell status | **Replaced/closed for identified callers.** Removed `AcornOS/deps/packages.rhai` no longer installs packages. `shell`/`shell_in` and output forms throw on failure; status helpers explicitly return status. Every active production `shell_status` result is tested (tool detection or link verification). Fresh captured-failure check fails the hook and preserves stderr/cwd; shell helper CLI test verifies explicit 7/1 status. Keep this explicit status API; do not silently change it to throw. Quoting is R-01. |
| 7. Extraction traversal/symlinks | **Replaced owner; final security acceptance open.** `helpers/build/extract.rs:67,102,135,303` validates member/link paths and rejects symlink components for tar/zip rather than delegating unvalidated shell extraction. Fresh valid contained-symlink tar CLI passes. Existing unsafe archive and link tests remain retained, but were not adversarially rerun here; PRD09 must revalidate exact final extraction/publication boundaries in authorized context. No blanket “safe extraction” checkbox. |
| 8. Age-based stale lock | **Replaced/closed on current pair.** `retained.rs:43` uses OS advisory destination locking, bounded/cancellable wait; no two-hour stale-file algorithm. Fresh concurrent distinct-input/same-destination CLI check observes the second waiting and preserves alpha/beta products. PRD10 reruns after runtime joins. |
| 9. Git timeout | **Replaced timeout owner; retained R-03 coverage.** `helpers/acquire/git.rs:24` explicitly calls low-speed flags HTTP-only bounds. `launch.rs:441,454,815` imposes an absolute managed execution/finalization deadline and reaps launcher; public RecipeEngine hooks all use this path. Prior exact PRD02 managed-deadline CLI passes. Do not claim Git's flags bound a whole process. A dedicated stalled-Git acquisition receipt on final source remains required in PRD10/12. |
| 10. No extends E2E | **Closed on current pair.** Fresh `managed_check_reads_staged_base_and_imported_module` executes base+plain child and overriding child, resolves imported facts, checks child ctx/shared closure and unchanged source bytes. This proves the real worker path, not only parsing. Saved ctx is now `ctx.state` plus filesystem checkpoint, whose PRD02 recovery E2E passes separately. |
| 11. AST += semantics / Rhai pin | **Retained and verified.** Cargo.toml pins `=1.19.0` with internals/merge rationale; `core/executor.rs:56` documents base-first statements and child name/arity override. Fresh extends test verifies both inherited and overridden hooks. Any version bump needs that acceptance again. |
| 12. import vs extends | **Replaced/closed on current pair.** `recipes.rs::find_related`, prepared-input discovery and `worker/run.rs::PreparedModules` use one prepared closure/search rule. Fresh combined base/import/override CLI succeeds. Recursive extends is explicitly rejected; arbitrary dynamic imports are not promised. |
| 13. .packages-version parallel race | **Replaced/closed for current producer.** No current tracked production `.rhai` uses `.packages-version`; old packages recipe removed. Rootfs producers operate in owned per-invocation work/output and destination publication is locked. Fresh destination serialization preserves separate products. This does not promise locking for arbitrary shared paths a future recipe invents. |

## Concrete open work; no acceptance hidden by old boxes

| ID | Current evidence and required disposition | Owner / closure evidence |
| --- | --- | --- |
| **F-01 export after close failure** | Daybreak on frozen `5449e2a1` item 5 returns supervisor-error yet leaves final 2,560-byte export SHA256 `25a31d551a178f06c7235a4fc090b369bf13975895cec24a879caba429947143`. `launch_tasks::teardown` exports before filesystem/backing teardown reports failure. Sticky error reporting is useful but insufficient. | **PRD04 open/failed**, G19/G17b. Prepare a separate corrected revision/binary hash without changing the frozen candidate; withhold final publication until all required cleanup succeeds. Focused authorized Daybreak item-5 and affected export controls must prove absence of final export on failure, successful export unchanged, workload-time error/cancel/reference cleanup correct. User already requested this correction; it has not run in PRD06. |
| **T-01 alias mapping offset** | Historical `3fac8e05` assertion is missing from current FD fixture despite its byte/alias mapping check. | **PRD05/07**, G12: port exact oracle in test retention record; public alias mmap must leave shared position 3 even after host fd number reuse. |
| **T-02 chunked copy-up rollback** | Tracked G09a test exercises 9,000 bytes and failure after 4,096, then capacity reuse. Neither current integration nor PRD03 includes this exact oracle. | **PRD05/07**, G09a/b: port full success/failure test to combined typed/generic backend; fail in middle of copy, preserve original descriptions/data and reclaim all provisional storage. |
| **T-03 orphan test join** | Current aggregate process cases do not contain #234's exact six named cases. All six are retained in its open head; no claim they ran in current integration. | **PRD05/07**, G06a: carry incoming suite plus reservation/current process tests; adapt only actual duplicate names, preserve distinct grandchild/live-init-thread oracles. |
| **R-01 production shell quoting** | Actual dynamic-path callsites listed above remain. Helper existence and shell nonzero handling are not a fix. | **PRD10/14/15** builder/Recipe callsites: use existing checked argv API for simple commands and explicit safe shell handling for necessary redirection. Run real affected commands with spaces/apostrophes/metacharacters in accepted path inputs; preserve exact argv/environment/cwd and failure propagation. Security-specific variants belong to PRD09. This task records, not implements, those changes. |
| **R-02 inherited leases** | Recipe `launch.rs:245` keeps four coordinator-owned directory leases in supervisor; PRD04 `launch.rs:539` keeps stdio/control/result only. No exploit/race is asserted; source contracts disagree. | **PRD05/10**, Recipe temporary/launch + G17b: explicitly choose supervisor retention or prove coordinator lifetime plus kill/reap safely replaces it; validate active scratch survives concurrent cleanup, coordinator death settles launcher/workers, next invocation cleans abandoned scratch, diagnostic retention and original FD bounds still hold. Coordinate declaration with the configured contract, not an unbounded inherited-FD exception. |
| **R-03 stalled Git deadline** | HTTP flags are insufficient alone; managed deadline owner exists but no dedicated acquisition-stall receipt is claimed here. | **PRD10/12**: exact final Recipe/Ginux pair, stalled Git child/grandchild, bounded failure/reap, no final product, retained diagnostics and reusable destination lock. |
| **N-01 native ioctl authority/audit** | #246 only handles PIDFD_GET_INFO; its review records other unknown/pidfs ioctls passed through while audit reports denied. This is a current authority/audit issue, not fixed by ENOTTY for one request. | **PRD08/09**, native syscall/audit owner: enumerate actual permitted/denied requests, make action match execution, verify host-namespace/FD consequences in authorized context. Keep pending until concrete reviewed policy and exact-source receipts. |
| **N-02 native behavior limits** | #242 SA_SIGINFO still exposes host child PID; #245 does not implement virtual cgroup membership; #248 synthesizes only `cgroup.procs`, `cgroup.events`, `cgroup.kill` and lacks relative directory-FD mkdirat; #247 leaves ignored-SIGCHLD/orphan slots unreaped. Scratch `b75edda7` changes authority and substitutes wait/restart semantics. | **PRD07/08/19**, native owners: preserve limits and each existing native fixture, correct what required distro boot/install acceptance exposes, verify both distros under libvirt. No general Linux ABI/cgroup guarantee or silent catalog-1 growth. |
| **S-01 final-source joins/security** | PRD03 foundation differs deliberately from later runtime in typed flags/seek, mode/copy-up and directory/cwd ownership; generic backing, capacity, locks, exec/map/export/restore, configured clocks/catalog and all argument holds must compose. Source retention is not final acceptance. | **PRD05**, then **PRD09–16**. Exact integrated candidate, affected entire runtime/Recipe catalog and authorized security validation; retain aggregate-resource, native-clock/public-console, platform/CI limitations in PRD08/12/13. |

No new product decision is necessary to classify these inputs. R-02 and native
public-contract decisions stay open with their owners; this record does not
choose a broader ABI or an inherited-descriptor exemption. There is no pending
user answer needed for PRD06 itself. Earlier user authorization and Daybreak
selection remain in force for their separate tasks.

## Evidence and reproduction

The [ledger builder](prd06-build-inventory.py) reads Git objects/diffs, tracked
patches and recovered snapshots; it never mutates a Ginux checkout/index/ref.
It writes the review ledger and evidence files; declaration ranges are navigation
aids, not independently applicable extraction boundaries. Its evidence directory is
`/home/vince/Projects/LevitateOS/.artifacts/ginux-contribution-preparation/prd06/`.
It contains complete zero-context patches, the unabridged inventory,
`upstream.json`, `patch-equivalence.json`, six CLI logs, `recipe-receipts.json`,
`run_recipe_audit.py` and `SHA256SUMS`. The committed source ledger keeps every
hunk/source/test disposition reviewable even without opening those raw patches.

```sh
cd /home/vince/Projects/LevitateOS
python3 docs/ginux-contributions/prd06-build-inventory.py
cat .artifacts/ginux-contribution-preparation/prd06/build-inventory.log
sha256sum -c .artifacts/ginux-contribution-preparation/prd06/SHA256SUMS
sudo python3 .artifacts/ginux-contribution-preparation/prd06/run_recipe_audit.py
```

The rebuild enumerates live refs; later branch movement requires reconciliation
rather than assuming this snapshot still describes it. Snapshot hashes and
main-case preservation are checked explicitly. Raw patches reconstruct each
record using its source/base; `git show SOURCE:PATH` retrieves a test version.
The evidence manifest excludes itself; rerunning runtime checks changes receipt
elapsed times and therefore requires a new manifest, not fabricated old hashes.

Six fresh ordinary CLI cases passed: staged base+import/override, static inherited
metadata, captured shell failure/stderr/cwd, explicit shell status/capture, valid
contained-symlink tar extraction, and concurrent destination serialization. They
ran through Recipe `b4144388`, Ginux `5300f5ef`, prepared image SHA256
`9f220633d34efff68b1535375fbb4fa3c4c2c8843917859fd68d8f6edec27ff4`:

- Recipe binary: `e7b03c58a4901256b4f13aab6dd1c629e979cd71fae3a3d2a9f77ab75f1cf524`.
- Ginux binary: `972ccb5acf378c33c339006ce204b1be9f082ce482e3db9144fd58ec3de2aec5`.
- CLI harness: `5379c0211c5aaea77c2e47f0a3ba5aaec6f897e16e9fed07b3783c358f4cac81`.

These checks add current-source evidence for the audit; existing PRD02/03 ordinary
receipts and returned Daybreak PRD04 receipts retain their original source and
limits. No adversarial/security runner, whole Recipe suite, final integrated
candidate or VM acceptance was executed here.

Active skill: `hdae-development` at `.agents/skills/hdae-development/SKILL.md`.
Reload the full skill, required reference, instructions and current queue after
context shift. PRD06 reconciliation is complete; next authorized engineering
work is PRD04's separate F-01 correction and focused Daybreak retest, or PRD07's
independent prepared material. PRD05 waits on its actual dependency chain. No
publication is authorized.
