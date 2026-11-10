# PRD02 — configured contract enforcement

Completed for the user-approved local integration, 2026-11-08. This closes
PRD02's contract reconciliation and local verification, not runtime distribution,
storage-redesign integration, NEEDS implementation or production acceptance.
The [current queue](../ginux-contribution-task-list.md) owns those remaining tasks.

## Sources and decision

GitHub read-only inspection found upstream main
`53a54addea67969abaa240ea478def1b94130eee` and open, unmerged
[#243](https://github.com/gmkurtzer/ginux/pull/243) at
`067d992f29d11f7b79e78753258721fc94a2f2c3`, depending on open
[#238](https://github.com/gmkurtzer/ginux/pull/238) at
`efaeeca84829e0c232709da8fcf91d5248952e32`. Both maintainer comments say review
will follow the foundation work; neither requests additional changes. No PR,
branch, comment or remote metadata was changed. The shell GitHub fetch failed
DNS resolution; connected GitHub tools provided the current metadata and the
already-present exact Git objects supplied the complete diffs.

Local baseline Ginux was clean `a7ce73cd`; Recipe was clean `d740fcf`.
The #238 commit is not an ancestor, but its signal-driven-I/O guard and tests
are already present semantically: `ginux-core::signal_driven_io`, balanced/native
argument checking and `DescriptorTable::enter` refuse those requests. They were
retained and verified; no outside-process signal experiment was performed.

Initial enforcement exposed missing configured syscall catalog entries.
Extending catalog 1 failed its existing wildcard-expansion and native-dispatch
tests. The user explicitly chose: **"Use an explicitly versioned local
configured contract/catalog; preserve upstream catalog 1 and native policy."**
The final implementation leaves catalog 1's source table, resolved authority
and digest unchanged. `configured-balanced-v2` explicitly selects catalog 2;
self-contained contracts may also explicitly select `catalog = 2` using the
existing version-1 contract syntax. This does not implement Recipe NEEDS.

## Final local source pair

| Component | Exact source / executable |
| --- | --- |
| Ginux | `5300f5ef9a8281f4f10f5519d5eb761654e04829`, `/home/vince/levitate-development/ginux-recipe-integration` |
| Ginux binary SHA-256 | `972ccb5acf378c33c339006ce204b1be9f082ce482e3db9144fd58ec3de2aec5` |
| Recipe | `b41443880fe585a27bc799d80382c9ff5b07047b`, `tools/recipe` |
| Matching CLI/worker SHA-256 | `e7b03c58a4901256b4f13aab6dd1c629e979cd71fae3a3d2a9f77ab75f1cf524` |
| Prepared compiler image | `prepared-image-git-musl/rootfs.tar`, SHA-256 `9f220633d34efff68b1535375fbb4fa3c4c2c8843917859fd68d8f6edec27ff4` |

Both source changes are locally committed and clean. A post-commit Ginux build
produced exactly the tested binary. No source was pushed or published and the
old `tools/ginux` gitlink was not replaced. Catalog 2 must be accounted for in
PRD01/PRD06/PRD07 delivery and revalidated after PRD05's source changes; it is
not represented as an accepted upstream feature.

## Findings and concrete dispositions

Line references below refer to the exact owning source pair above.

| Finding | Owner and disposition |
| --- | --- |
| Resolved contract was ignored by configured handlers | **Fixed.** `ginux-init/src/launch.rs:404` resolves the complete `InstanceConfig::effective_contract`; `launch_tasks.rs:880` applies it before clock, task, mapping, descriptor, filesystem or network effects. Internal supervisor replay remains inside its already-authorized transaction. |
| Family/syscall precedence differed from the reported policy | **Fixed using #243's shared owner.** `ginux-contract/src/lib.rs:1906` implements `runtime_rule`: semantic denial wins, except an explicit syscall denial wins over an ungranted semantic family; grants never override syscall denial or implementation restrictions. Native supervision uses the same rule without widening its implementation policy. |
| #243's register-only classification misses local aliases and live networking | **Adapted.** `launch.rs:1366` classifies original public calls; `descriptors.rs:263` retains console/TCP identity across dup/fork/close/reuse. TCP denies are enforced on creation and owned socket I/O rather than assuming networking is unreachable. Path mutations, virtual credentials and memory/signal families are classified before their owners run. |
| Strict enforcement would break supported calls absent from catalog 1 | **Fixed through the approved explicit version.** `ginux-contract/src/lib.rs:170` adds ten catalog-2 operations: lstat, readv, pipe, nanosleep, link, sched_getaffinity, clock_getres, faccessat, renameat2 and execveat. The 182-operation upstream table is unchanged; the explicit configured table has 192 operations. Versioned resolution, normalized parsing, inspection and catalog diffs agree. |
| Recipe selected a profile that omitted its configured extensions | **Fixed.** `tools/recipe/src/inputs/config.rs:110` selects `configured-balanced-v2`. Older launchers reject the unknown profile before workload execution. Schema 1, assignments, numeric endpoints, resource bounds, stderr audit and stop policy remain present. |
| Disabled audit mode was ignored; entry records lacked provenance | **Fixed.** `launch.rs:433` selects disabled mode, `launch.rs:849` suppresses decision writes, and `launch.rs:1351` records operation/source/rule. The launch header identifies the instance and digest. The 64 KiB budget still drops whole records; diagnostics remain available when decision audit is disabled. |
| Explicit restrictions could be accepted without enforcement | **Refused at startup.** `launch.rs:1321` rejects explicit vDSO-clock, proc/sys/cgroup semantic denials the backend cannot enforce. `launch.rs:529` rejects unsupported filesystem write-deny rules, pre-boot hooks and shared-network rules. It does not silently ignore them or invent grants. |
| Legacy selector rule indexes are always zero | **Retained, explicit upstream limitation.** Legacy `[contract]` source-rule indexes remain 0 so catalog-1 digests stay stable. Source and deciding operation are reported correctly; self-contained contracts have actual zero-based indexes. NEEDS provenance must use the self-contained contract path in PRD18. |

## Actual Recipe configuration and enforcement trace

`inputs::prepare_invocation` calls `NetworkPolicy::resolve`, then
`InstanceConfig::for_invocation` and `render`. Explicit files/trees are immutable
snapshots under `/inputs`; the worker and invocation arguments are coordinator
owned. Memory assignments cover `/work`, its build/tool/tmp children and `/out`.
No writable host directory is granted. Host resolution is bounded and produces
at most eight numeric endpoints and an immutable hostname map. Empty networking
renders `backend = "none"`; mediated networking retains its 16-connection and
explicit cumulative byte limits.

`launch::launch` passes that staged config directly to `ginux launch`, with the
prepared root, fresh result/export paths, checkpoint FD and optional restore.
Ginux's parser validates and resolves the selected profile or self-contained
contract; the launch entry check then constrains all workload handlers. Storage
assignment/path and byte/object owners, descriptor authority, numeric TCP
endpoint/connection/byte owners and per-process AS/affinity limits remain
additional restrictions. Contract allowance cannot bypass those checks.
Recipe retains the supervisor's stderr separately from worker transport and
requires the authoritative Ginux result before publication.

## Exact-source verification

Ignored evidence is under the Ginux source's
`.artifacts/recipe-integration/prd02-contract/`. `summary.json` and its verified
SHA-256 manifest identify sources, binaries, image, commands and receipts.
Artifacts include both complete upstream diffs, failing baseline controls,
intermediate failures, final logs, fixtures and the actual runners.
The 740-file manifest verified successfully; its SHA-256 is
`064230a9f0ef2b0e6f565c6b654b8b68d55eb4b51b3fa54cbcbd212a0e1bbc4d`.

- **47 real contract controls pass:** allowed counterparts, explicit/family/
  exact/default denial, deny precedence, no network grant, console aliases,
  unknown calls/catalogs/selectors, version-2 opt-in, disabled audit and
  unenforceable-deny startup refusal. Baseline `a7ce73cd` ignores `syscall.open`
  denial and disabled audit; both retained controls fail. The old launcher
  rejects the new profile. Original and final catalog-1 inspection agree exactly.
- **14 full runtime runners pass:** contract controls, configured launch,
  native supervision, storage, descriptors, pipes, tasks, threads, directories,
  exec, owned exec, scripts, poll and TCP. Ordinary checkpoint and restore
  runners also pass, including >2 GiB data and 5,000 nonempty files.
- **50 Ginux unit tests pass** across contract/core/instance. Existing catalog-1
  wildcard/native assertions remain intact. `make check`, release build and the
  unchanged catalog-1 ABI-document check pass. The 47-case runner is wired into
  the existing launch CI job; no hosted job was run here.
- **Eight focused Recipe e2e cases pass** with the matching final CLI/worker:
  lifecycle/source preservation, transitive dependency compilation, deadline
  diagnostics, before/after-ack recovery, verified download, redirect grants and
  byte budget, target metadata through publication/reimport/EROFS/deployment,
  and checked-command failure. Recipe release/e2e compilation, fmt and bins
  Clippy with warnings denied pass. These are eight exact selections, not a
  claim that the whole 136-case catalog passed on this pair.

Intermediate failures were diagnosed rather than suppressed: the prototype
catalog-1 expansion was replaced with explicit catalog 2; a stale audit-record
regex was updated to require provenance; the new round-trip test's input
profile was corrected; the standalone doc tool was rerun with host `RUSTFLAGS`
instead of freestanding linker flags. Harness argument/path errors were setup
failures, not accepted runtime results. Final receipts contain the successful
commands and exact identities.

## Supported limits and remaining ownership

Global realtime/monotonic reads and bounded sleep retain the previously approved
local clock contract. The host vDSO can answer reads without a syscall; an
ungranted clock/proc/sys/cgroup semantic operation alone does not withhold every
effect. Explicit unenforceable denials fail startup. Validation/inspection
establish config/table validity; launch additionally checks backend feasibility.
Contract digests are upstream's deterministic identity, not a cryptographic
trust primitive; source/image/executable SHA-256 receipts supply provenance.

No new product decision remains open for PRD02. Upstream acceptance and durable
runtime delivery belong to PRD01/PRD06/PRD07; redesigned-storage revalidation
belongs to PRD05. PRD08 still owns clock-boundary and public-policy acceptance;
PRD04/PRD09 own the existing descriptor/storage/security findings. PRD10–PRD17
remain open for final release acceptance. PRD18's NEEDS migration and declared
per-recipe contracts are required later and are not implemented here.

Active skill: `hdae-development` at
`.agents/skills/hdae-development/SKILL.md`; reload the complete skill, required
reference, repository instructions and current queue after a context shift.
Next queued task: PRD03's configured-storage candidate and prerequisites,
accounting for this contract enforcement before comparing/integrating owners.
