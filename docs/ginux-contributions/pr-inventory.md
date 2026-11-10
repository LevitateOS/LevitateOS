# Ginux PR inventory — 2026-11-06

Use the [consolidated production-readiness queue](../ginux-contribution-task-list.md)
for current work and completion criteria. This inventory records candidate
history and upstream/source facts; historical packet status is not final
production acceptance.

## PRD08 configured contract successor — 2026-11-08

[PRD08](prd08-public-contracts.md) records the user-approved versioned downstream
contract, all native/upstream delivery holds, and complete clock acceptance.
Clean local `4aa51c1d` over `5300f5ef` confines configured large reads and adds
zero-filled configured sysinfo. All six clock-dependent Recipe cases pass;
actual five-minute and realtime-adjustment checks pass under session libvirt.
Carry its four-file delta into PRD05; storage/PRD04 composition, source packaging,
final security and publication remain separate. Original pinned candidates and
upstream PRs were not changed.

## PRD06 reconciliation — 2026-11-08

The [complete reconciliation](prd06-reconciliation.md), [source ledger](prd06-source-inventory.json)
and [test retention record](prd06-test-retention.md) supersede old inventory
completion claims. They account for all 246 source commit entries, 47 packet
entries and every useful hunk/test against current main `53a54add` and storage
`9802ef03`, including native #241/#242/#244–248 and downstream boot scratch.
PRD06 is complete as accounting. PRD04 remains failed/open for export publication
after backing-close error; storage integration, test ports, Recipe callsite/lease
joins and final security/product acceptance remain open. No publication occurred.

## PRD03 storage candidate — 2026-11-08

The current prepared candidate is clean local `9802ef036d53ae1ee5276fe59cc719389dda5879`
at `/home/vince/levitate-development/ginux-storage-prd03`, feature base `f1c3d3a7`,
main `53a54add`. See the [complete pre-review](prd03-storage-review.md) and
[unpublished PR draft](prd03-storage-pr-draft.md) for the full patches, exact
prerequisite/source chain, each storage-spec resolution, four red controls and
final-source checks. All upstream unit tests and CI steps remain; 138 units,
40 storage launches and every local launch-integration CI command pass.

Publication remains blocked by open #232/#233/#238/#243/#236, separate zero-fill
and the unpublished explicit catalog-2 prerequisite, followed by merged-source
refresh and user authorization. Catalogs 1/2 keep authority/digests; storage
selects catalog 3. PRD05 must reconcile this foundation with Recipe's catalog-2
renderer and later runtime owners. Existing PRD04/PRD09 findings remain open.
The donor `545380dd` and older facts below are history, not the current candidate.
No push, PR, upstream comment, Recipe integration or gitlink replacement occurred.

## General engineering continuation — 2026-11-08

The user's latest instruction returns upstream Ginux PR preparation and creation
to Codex, superseding older Claude-exclusive PR ownership. Publishing a PR
requires explicit user permission; pushes and upstream comments also remain
subject to explicit authorization. Codex must pre-review any proposed Ginux
contribution before publication: inspect
the complete diff and dependency chain against current main, trace behavior
through its owners, check failure paths, contracts, Recipe compatibility,
tests and CI, and report concrete file/line findings. Passing CI alone does
not justify approval. The user clarified that this is a pre-publication
review requirement, not a request for recurring review. No schedule was made.
Pushes, PR creation and upstream comments still require explicit authorization.

Read-only refresh found main `53a54addea67969abaa240ea478def1b94130eee` and
storage prerequisites #232, #233 and #243 open. The local configured-storage
candidate remains `545380dd49ea432d9e84e6b956f129f48c72822e`; no feature PR is
published. Its replacement scope remains provisional until merged-source
comparison and affected checks establish which local fixes/tests it replaces.
The old preparation tables below are historical, not current readiness claims.

The user authorized a configured-only local clock contract. Local integration
commit `a7ce73cd` implements realtime/monotonic reads and bounded interruptible
sleep without changing native policy or publishing upstream. Six formerly
blocked Recipe sleep cases pass with exact-source receipts. See
[F03](f03-review.md) for the current consumer/CI checkpoint and storage blocker.

Starting point for review → fix → PR. Built from live git/GitHub state plus the
earlier preparation record (`docs/ginux-contribution-task-list.md`,
`.artifacts/ginux-contribution-preparation/upstream-contract-refresh-2026-11-06/current-inventory.json`).

## Where things are

| What | Location / ref |
| --- | --- |
| **Review worktree (new)** | `/home/vince/Projects/ginux-review`, detached at upstream main `ff926d0c` |
| Upstream | `gmkurtzer/ginux` main = `ff926d0c` (#231 "Gate 10: configurable contract and audit") |
| Our fork | `LevitateOS/ginux` (`origin` in the submodule) |
| Superproject pin | `tools/ginux` @ `2eac10bf`, detached, 5 local commits on an old base (see "Legacy") |
| Integration source (donor) | `origin/codex/ginux-recipe-local-integration` @ `7cf2638a`, 66 own commits beyond old main |
| Prepared patches | 37 local commit objects (no branches), listed below; patches/manifests under `.artifacts/ginux-contribution-preparation/` |
| Combined test tree (F02) | `.artifacts/development-migration/2026-11-06/f02-source` @ `dfebda7c` (all 37 applied together) |

## Upstream right now

Main moved a lot today: #228/#229 (Gates 9–9.75: domain spec, network controls) and
#231 (contract catalog, profiles, `EffectiveContract`/`AuditBudget`). Every
prepared patch except G05a predates #231; most predate #228/#229 too.

Open upstream PRs:

| PR | Author | Topic | Relation to us |
| --- | --- | --- | --- |
| #207 | LevitateOS | bounded console polling (= our **G01**) | ours, open, policy-held (#179) |
| #224 | meleficarumvaldis | clone tid store fault handling | overlaps G06 area — don't duplicate |
| #226 | meleficarumvaldis | kill tracees with kill(2) | overlaps G06 area — don't duplicate |
| #227 | hafid-logic | release overflow-checks | unrelated |

Already merged from us: #216, #217, #218 (VFS fixes).

## Fork branches (`LevitateOS/ginux`)

| Branch | State | Suggested action |
| --- | --- | --- |
| `codex/bounded-console-poll` | head of open #207 | keep |
| `codex/vfs-open-failure`, `codex/vfs-path-resolution`, `codex/vfs-unlink-lifetime` | merged as #216–#218 | delete |
| `codex/ginux-recipe-local-integration` | donor for all packets | keep until packets are out |
| `ginux-recipe/i04-m5-writable`, `ginux-recipe/i04-m7-alias` | old memory-loader prototypes, superseded | delete after confirming |

## Prepared patches (37, ~18.4k+/1.6k− total)

"Base" says what the commit currently sits on. Only **G05a** is on today's main.
The six marked ✅ were checked with `git merge-tree` and **apply cleanly to `ff926d0c`**
(textually, not re-tested). Everything else is stacked on other unmerged packets.

### Wave 1 status (2026-11-06)

Reviewed twice, the second time by an independent cold reviewer, then reworked. Each
is one commit on `ff926d0c` (local branches in the review worktree, not pushed). Every
regression fails on main and passes on its branch. All seven `check.yml` steps pass on
every branch (`make check`, `make test-unit`, build, `test-launch`,
`test-native-supervision`, `test-process-native`, `test-gate1-setup`), as they do on
main. The macOS runner was not exercised locally. PR text is in `wave1-pr-drafts.md`.

**Published 2026-11-07** from the fork branches, after the user's go-ahead:
G05a → [gmkurtzer/ginux#232](https://github.com/gmkurtzer/ginux/pull/232) (`5e79a172`),
G03a → [gmkurtzer/ginux#233](https://github.com/gmkurtzer/ginux/pull/233) (`44b3f89c`),
G06a → [gmkurtzer/ginux#234](https://github.com/gmkurtzer/ginux/pull/234) (`78a534c1`).
The published heads differ from the table below only by a `Co-Authored-By` trailer; their trees are identical.

| Packet | Branch | Head | On main | On branch | Rework after review |
| --- | --- | --- | --- | --- | --- |
| G05a | `fix/vfs-dup-offsets` | `f8b3f7f7` | 4 VFS tests fail | 87 units pass | `description`/`offsets` naming, stricter tests, release test, message states the bug and that no runtime caller exists yet |
| G03a | `fix/launch-mkdirat` | `0191e47a` | launch exits 2, workdir unavailable | 83 units, launch passes | one shared `SYS_MKDIRAT_HOST` in `supervisor.rs` instead of a second copy of 258; message covers default `/work` and stray file |
| G06a | `fix/process-orphan-adoption` | `9413e5ab` | 6 unit tests fail; native orphan gets `getppid=2`, init `wait4` = ECHILD | 89 units; `orphan_reparent`/`orphan_zombie` 20/20 each | dropped live-init guard, split tests, added already-exited-orphan test and two native scenarios; message leads with the zombie slot leak |

Known limits recorded in the drafts: G03a creates the workdir as root before the privilege
drop; G06a queues no SIGCHLD to init for already-dead adoptees, and systemd mode still
returns ECHILD for adopted children because host `wait4` sees the real parent.

### Wave 2 preparation (2026-11-07)

- **G03b needs a redesign, not polish.** It was rebased onto `fix/vfs-dup-offsets` as local
  branch `prep/vfs-writable-metadata` (`06645ea8`, 25 VFS tests pass). A cold review found:
  - `descriptor_flags` returns Linux ABI values (`0o2000`, 0/1/2) instead of the crate's
    own `O_*` flags.
  - `seek` takes raw whence numbers and reports a bad whence or negative offset as
    `InvalidPath`; there is no `InvalidArgument` error.
  - A file's inode changes on copy-up, so `fstat` on an earlier fd disagrees with the path.
  - `chmod` rejects setuid/setgid/sticky bits (`mkdir` masks them instead), and mode is
    never enforced on overlay files.
  - Directories take whole file slots, can't be removed (no `rmdir`) or opened, and
    aren't counted by `overlay_files()`.
  - Overlay states are magic numbers 0–3, and `append` lives on the descriptor rather
    than the shared description.
  - Its only caller is G03c, which uses every new API, so it is one feature split across
    two patches. Every later packet (G04 onward) builds on this API, so a redesign
    ripples up the stack.
- **SIGCHLD follow-up to #234 dropped.** `rt_sigaction` is ENOSYS in the native lane and the
  default disposition discards SIGCHLD, so a virtual SIGCHLD to init is unobservable today.
  It becomes relevant only once upstream mediates signal handlers.

### Published 2026-11-07: #243 launch contract enforcement

#237 (Gate 10.5: `[userspace.contract]` recipes, `ginux instance validate|inspect|explain|diff`,
`[userspace.filesystem]`, pre-boot hooks, root drops to 65534) merged as `53a54add` from head
`af8852cd`, later than the `e29a2130` we pre-stacked on. It still does not enforce the contract
in `ginux launch`.

The fix is `fix/launch-contract` on `53a54add` (pushed to our fork;
the old main-based tip `2d70a97d` and `fix/launch-contract-on-237` are superseded):
`b9b24355` (enforce the resolved instance contract) and `067d992f` (honor a disabled audit
mode). Code identical to the e29a2130 version; only doc context and wording changed. All 7 CI
steps pass, commit 1 passes on its own, and the new test-launch.py fails on `53a54add`.
Four cold reviews; the fourth (after #237 merged) found no bug in the enforcement path. Since
the main-based version it:
- refuses explicit `InstanceDeny`/`RecipeDeny`/`ExactDeny` of what launch cannot enforce:
  console input, `/proc`, `/sys`, cgroup and clock reads, plus vDSO-served clock syscalls;
- treats operations the catalog never grants, and the network families, as enforced, so the
  example recipes' `dev.mmio.direct`/`dev.dma.direct`/`network.packet.raw` denials launch. A
  compile-time assertion keeps every socket and io_uring syscall out of launch;
- writes `policy-rule=` in audit records, as native does;
- lets an explicit syscall deny win over a family that is only not granted (shared
  `runtime_rule`, so native too);
- widens the shared families with mremap, madvise, rt_sigreturn, pidfd_send_signal,
  setreuid and setregid. Every shipped profile/recipe grants each of these with its family;
- documents what launch cannot enforce, that minimal-v1/strict-v1 give no console output,
  fstat under a `filesystem.path.read` deny, the shared-mapping write residual, that launch's
  own bootstrap (chroot, privilege drop, workdir creation, exec of init) precedes the
  contract, and that legacy `[contract]` selectors audit `policy-rule=0`.

The signal family relies on #238 closing the fcntl/ioctl signal-driven I/O path; the PR body
says so. Published as [gmkurtzer/ginux#243](https://github.com/gmkurtzer/ginux/pull/243).

Pre-existing on main, found by the fourth review, candidates for separate PRs:
- `ginux launch` accepts and ignores `[hooks] pre_boot`, `[volumes]`, `[userspace.filesystem]`
  and `[access]`; its workdir `mkdirat` runs as root before the drop and ignores `write_deny`.
- README ("drops to UID/GID 1000") and the launch header (`tracee-uid=1000`) are wrong since
  #237: root drops to 65534/65534.
- Native classifies console read/write only on its protected console fd, so writes to fd 0-2
  or a dup escape a `dev.console.write` deny in systemd mode.
- Legacy `[contract]` instance allow/deny selectors always record `source_rule` 0
  (`apply_selector` never sets it). Fixing it changes digests.

### Published 2026-11-07: #236 chrooted stdio directories

[gmkurtzer/ginux#236](https://github.com/gmkurtzer/ginux/pull/236), branch `fix/stdio-directories`
(`1f43d7fa`: rebased onto `53a54add` after #237 conflicted on imports; its close_range setup
failure moved from 133, now process-native's identity drop, to 127; force-pushed). `ginux launch` and `ginux native` refuse to start when stdin,
stdout or stderr is a directory (or can't be checked), and `ginux native` now closes every
other inherited descriptor (its loop stopped at 256). Found by the 4th storage review; on main
a host directory as stdin let the workload read host files, and in native systemd mode
`fchdir(0)` led to a full escape. Tests fail on main and pass; all 7 CI steps pass locally.

Not in the PR, pre-existing and recorded only here:
- A root with procfs mounted exposes host process roots via `/proc/<pid>/root`. The README
  says so as a limit.
- `fcntl` had no argument filter in balanced, so `F_SETOWN`/`F_SETSIG` on a stdio pipe let a
  workload signal a host process that shares uid 1000 (reproduced: it killed a uid-1000
  `sleep`). Reworked after a cold review as `fix/fcntl-signal-routing` (`efaeeca8`): new
  `ginux_core::signal_driven_io` also covers ioctl FIOASYNC/FIOSETOWN/SIOCSPGRP, which native
  systemd mode admits, and the terminal case where O_ASYNC alone signals the foreground group.
  Core, launch and native tests fail on main and pass; all 7 CI steps pass; merges cleanly.
  Published as [gmkurtzer/ginux#238](https://github.com/gmkurtzer/ginux/pull/238).


### Prepared, not published (2026-11-07, on upstream main 59e2f7aa)

Upstream merged #235 (Gate 10.25: the "domain" model is now "instance"; `make check` fails on
the old word anywhere in product files). Everything below is rebased onto it and renamed. #233
conflicted with the rename; it was rebased and force-pushed (`f751cd6f`), and CI is green again.
#232 and #234 still merge cleanly.

| Candidate | Branch | Base | State |
| --- | --- | --- | --- |
| Launch contract fix | `fix/launch-contract` (`b9b24355`, `067d992f`) | main `53a54add` | Published as #243 |
| VFS zero-fill fix | `fix/vfs-truncate-gap` (`6f11d011`) | main | Ready: test reads `secretx` on main |
| Configured storage | `prep/configured-storage` (`545380dd`) | #232, #233, contract, zero-fill | Review-3 findings fixed; all 8 CI steps green, 120 units |

**Launch contract fix.**
- Commit 1 checks the resolved contract at every syscall entry.
  - It classifies path calls and console writes. Console reads are not classified, because
    any fd may be read, so a dup of stdin would evade it.
  - Descriptors and ioctl requests are compared as 32-bit values.
  - Audit entry records, including those of lane-completed calls, carry the operation and
    its policy source.
- Commit 2 honors disabled audit.

**Storage, after review 3:**
- Resolution follows the kernel's bounds where it can: 40 links, EBADF for bad dirfds, and a
  NULL path accepted with `AT_EMPTY_PATH`. The remaining limits (255-byte paths and link
  targets, no `/proc` links, `..` not checked for search permission) are documented.
- An assignment through an image link or file above its last component is rejected at
  startup. That removes the precondition for B1.
- A workdir held in memory is not created in the image.
- The scratch area falls back from below the stack pointer to the live stack, saved and restored.
- `writev` checks its whole iovec array before writing anything.
- One table of path calls drives both launch classification and storage dispatch.
- Linux errno and order fixes:
  - `O_RDONLY|O_TRUNC` truncates;
  - `x/` with `O_CREAT` gives EISDIR;
  - `AT_REMOVEDIR` on a file gives ENOTDIR;
  - `W_OK` on an unassigned ancestor gives EROFS;
  - flags are validated before path lookup.
- The zero-fill commit no longer drops #232's tests.

**Publication order:**
- #232 and #233 merge first.
- The contract fix and the zero-fill fix are independent.
- Storage opens after all four have merged.

**Open item, not ours yet:** on #207, hafid-logic flagged that bounded poll validates the
pollfd array from a PEEKDATA snapshot while the kernel reads the live array. The maintainer has
paused review. Fix it when #207 work resumes.

### Outside the storage chain (checked 2026-11-07)

Nothing standalone is left to prepare right now:
- Upstream bug issues #163 (getrandom bound on flags), #178 (oversized read/write fail
  instead of short transfer) and #219 (sysinfo leaks host data) all have a maintainer-
  recommended design, but each is explicitly held ("no additional work is requested here
  right now"), and the reporter appears to hold a PR for #178. Do not race them.
- #223/#225/#222 are already covered by other contributors' PRs #224/#226/#227.
- G27/G28/G26a remain the only other patches on main, all policy-held.
- Pre-existing quirks seen during review (`open("/x/", O_CREAT)` gives NotFound; launch
  workdir created as root before the privilege drop) are unreachable or moot upstream
  today: VirtualFs has no runtime writer and launch rejects storage assignments.

**Parallel work in progress:** a Codex run, still executing
`docs/ginux-contribution-task-list.md`, refreshed G03b, G03c, G04 and G05 on current
upstream between 23:48 and 00:21 (superproject commits 2ea15f1…d540732; new bases
`85ee433a`, `3801fd45`, `8880ddc8`). It builds on the G03b API that failed review above
and does not know about #232–#234.

### Standalone on main (first-wave candidates)

| ID | Commit | Size | Title | Holds |
| --- | --- | --- | --- | --- |
| G05a ✅ | `25d4f05a` | 1f +95/−21 | fix(vfs): share offsets between duplicated descriptors | none beyond CI; tested on `ff926d0c` |
| G03a ✅ | `c28dd132` | 2f +4/−1 | fix(launch): use mkdirat when creating the workload directory | re-test on `ff926d0c` |
| G06a ✅ | `01f22e06` | 1f +118 | fix(process): adopt orphans after the last parent thread exits | re-test; check vs #224/#226 |
| G27 ✅ | `1eb3e189` | 6f +119/−16 | feat(kernel-views): project configured uname identity | #179 identity policy; base `3f2fe7df` is old |
| G28 ✅ | `0a3cf5f5` | 3f +769 | feat(devices): bounded supervisor-owned TCP backend | network policy; must reconcile with #229 network controls |
| G26a ✅ | `1d0824e6` | 5f +103/−7 | feat(init): apply configured process bounds before exec | resource-limit contract; base old |

### Stacked (need prerequisites merged upstream first)

Main spine: **G05a → G03b → (+G03a) G03c → G04 → G05 → (+G06a) G06b**, then branches off it.

| ID | Commit | Size | Title | Depends on | Notes |
| --- | --- | --- | --- | --- | --- |
| G03b | `5fb12ab4` | 1f +425/−6 | feat(vfs): bounded writable metadata and positioning | G05a | |
| G03c | `e9c0a4d4` | 10f +1140/−28 | feat(launch): assigned ephemeral memory storage | G03b, G03a | must join #231 contract/audit owners |
| G04 | `09fdb1b0` | 12f +601/−119 | feat(launch): assigned virtual null devices | G03c | |
| G05 | `51b2c751` | 17f +1297/−76 | feat(launch): unify public descriptors and owned pipes | G04 | |
| G06b | `7240bb21` | 19f +2323/−351 | feat(launch): supervise configured child and exec lifecycles | G05, G06a | largest; task-loop join with #231 |
| G08a | `fe27c693` | 12f +434/−10 | feat(launch): poll owned descriptors via registered owners | G06b | base is an older G06b head |
| G08b | `bc34cf66` | 5f +320/−53 | feat(channels): local packet pairs, shared nonblocking flags | G08a | G01/G02 policy, G07 |
| G09a | `648ec32a` | 1f +289/−25 | feat(vfs): separate owned byte storage, truncation | G03b | pair with G09b |
| G09b | `a2a70d1c` | 7f +281/−13 | feat(launch): grow memory files within a shared bound | G06b, G09a | |
| G09c | `1e34ef36` | 1f +491/−2 | feat(vfs): own directory descriptions and mutations | G09a | pair with G09d |
| G09d | `ab47378f` | 11f +582/−53 | feat(launch): owned directory and image cwd ops | G09b, G09c | |
| G10 | `38f7ee40` | 14f +1165/−131 | feat(launch): execute owned ELF via private backing | G09d | security-held |
| G11 | `0798d991` | 15f +1162/−57 | feat(launch): owned scripts, bounded interpreter chains | G10 | base is **old** G10 head |
| G12 | `8bb50aaa` | 11f +666/−81 | feat(storage): map owned files via read-only backing | G11 | needs G07 for nonleader threads |
| G13 | `5c3ae8e5` | 8f +516/−58 | feat(storage): owned symbolic link operations | G11 | |
| G14 | `ec6f9c48` | 5f +252/−34 | feat(launch): per-process owned cwd | G13 | |
| G15 | `8ab24d4b` | 11f +441/−72 | feat(storage): shared inode identity across hardlinks | G13 | |
| G16 | `4682b0b0` | 13f +274/−85 | feat(launch): join image and owned path traversal | G14 | |
| G17a | `94baebbe` | 4f +49/−8 | feat(storage): positioned writes (pwrite) | G09d | |
| G17b | `6fcdd618` | 7f +80/−27 | fix(launch): private backing FDs without widening public limits | G12, G18 | |
| G18 | `f58ba480` | 7f +324/−37 | feat(storage): configurable namespace/data capacity | G09d | |
| G19 | `a36e7644` | 10f +527/−24 | feat(launch): flush owned files, export after quiesced success | G12 | |
| G20 | `7afa833d` | 11f +563/−36 | feat(launch): snapshot capture over private control | G19 | includes G31 CI wiring |
| G21 | `33bf08a7` | 9f +482/−7 | feat(storage): restore snapshots before startup | G19 | includes G31 CI wiring |
| G22 | `7b2adc4f` | 7f +401/−77 | feat(storage): link identity across bounded archives | G13, G15, G18, G17b, G21 | |
| G23 | `14c33321` | 8f +389/−10 | feat(storage): file locks with virtual ownership | G15 | needs G07 + real SQLite/Cargo check |
| G24 | `b33fbd24` | 12f +261/−8 | feat(vfs): explicit inode timestamps, archive mtime | G22 | |
| G25 | `9afd60cc` | 8f +253/−25 | feat(init): bounded owned vectors, chunked writes | G17a | native clamp `74835089` excluded (policy) |
| G26b | `2ed520c4` | 5f +116/−6 | feat(init): task-owned affinity, process bounds | G06b, G26a | |
| G29 | `26fbfef2` | 12f +1022/−13 | feat(launch): configured TCP via public descriptors | G08a, G28 | Git fails at missing getcwd (needs G14) |
| G30 | `2186548d` | 5f +88/−7 | feat(launch): virtual root ownership on assigned storage | G15 | |

Freshness: G05a is on `ff926d0c`; G03a…G10 (13 packets) were refreshed on `8a475e8b`;
the remaining ~23 still sit on `3f2fe7df`-era bases, and several (G08a, G11, G18,
G17a, G19, G22…) sit on *older* heads of packets that have since been refreshed,
so the stack is internally inconsistent and needs rebuilding bottom-up.

## Not in the patch set

| Item | State |
| --- | --- |
| G01 bounded console poll | open as #207; held on #179 policy |
| G02 global clocks / interruptible sleep | local only; held on upstream clock contract |
| G07 thread ownership | **unready**; unresolved shared-path race. Blocks real consumers (Bash child, tar progress spinner → `ENOSYS` on reader thread) and G12/G23/G26 joins |
| G31 checkpoint/restore CI | folded into G20/G21 |
| `74835089` native descriptor clamp | excluded; policy-held (#178/#163) |

## Legacy / preserved state (nothing to submit directly)

- **Superproject pin `2eac10bf`**: 5 commits "vfs: … (G06/G06a/G07/G07a/G07b)" — an older,
  differently numbered series (+1941/−217 in `fs_hooks.rs`, `ginux-vfs/src/lib.rs`).
  Classified in P03 as superseded demo/native file owner. Not PR material.
- **Six restored worktrees** under `.artifacts/development-migration/2026-11-06/worktrees/`:
  `10-limits-review` (9 M, 1 conflict), `11-upcoming-review` (8 M, 2 conflicts),
  `2-…` (8 unstaged), `3-…` (1 staged — the already-merged VFS fixes),
  `8-i04m5` (6 M, 2 untracked), `9-i04m7` (2 M). All mapped to owners in P03;
  recovery snapshots, not starting points.
- Stashes: none present now (P01 recorded two identical, superseded interpreter stashes).

## Observations for review

1. **Wave 1 is small and real**: G05a, G03a, G06a are tiny standalone fixes that apply
   cleanly to today's main. G27/G28/G26a also apply but each needs an upstream policy
   answer first, and G28 overlaps #229's new network controls.
2. **The rest is one long stack** (~15 deep on the spine). Upstream review will gate
   it wave by wave; G05a → G03b → G03c is the critical path.
3. **#231 changes the ground under G03c/G06b**: configured launch must route through
   upstream's new contract/audit owners rather than its own dispatch.
4. **G07 is the main functional gap**: until threads work, real consumers (Bash, tar,
   SQLite/Cargo, nonleader mappings) can't be validated end-to-end.
5. The earlier prep process was very heavy (R01–R04/F01–F04 receipts per packet,
   DaybreakBlue handoffs). Its test evidence is useful; its ceremony isn't required here.


### Codex PR #207 conflict update — 2026-11-07

At the user's explicit request to resolve #207's merge conflicts, Codex pushed
`5ae058f211905c6cb0c92a874e5bc97f333a23a3` to `codex/bounded-console-poll`.
It merges upstream main `ff926d0c` into existing head `e7f5f552`, retaining both
parents and history. The main.rs resolution preserves upstream native/systemd
mediation and passes the currently traced `host_pid` to bounded polling validation
in the balanced path. PR body/comments were unchanged. Both required GitHub CI
jobs passed ([run](https://github.com/gmkurtzer/ginux/actions/runs/42314014072));
this did not resolve the existing policy hold or reported shared-array race.
Claude now owns all further upstream work. Codex will push nothing further to the
fork and will only change its local Ginux integration source for Recipe consumers.


### Codex local Recipe integration — 2026-11-07

Codex's own integration source is
`/home/vince/levitate-development/ginux-recipe-integration` (separate from the
preserved F02 source and Claude's worktrees). Local commits for extraction:

- `91e244c1`: per-task descriptor syscall results/arguments and poll snapshots.
- `78f96df1`: bounded libc threads sharing process descriptor/cwd/executable
  resources, blocked-IO handle pins, fork/wait ownership and last-thread teardown.
  Ordinary thread/process/exec checks and the actual Recipe Bash reader pass.

These are local consumer fixes, not refreshed upstream packets or a publication
readiness claim. G07's shared-argument security hold remains pending. Known B1–B3
storage defects are unchanged. No fork push, PR or upstream comment was made.


Codex local integration update: `8981bf77` now descends from upstream main
`59e2f7aa` (instance rename); shared-thread fixes rebase as12c176d4/4a8d50f7.
Recipe's direct `[instance]` renderer isf70b35d. Ordinary local checks and the
Bash consumer pass; actual Cargo linker EAGAIN is still under diagnosis.
This changes no upstream packet, fork branch, PR or publication status.


Codex local consumer fixes after the instance rebase: `12c4ea80` reports one
bounded task-creation resource failure; `062fb5ef` uses32 configured task slots
after the actual concurrent Cargo linker tree exhausted16. Exact-bound/reuse
checks and the unchanged real Cargo/cache/compiled-program consumer pass.
Both commits are local extraction material only; no upstream action was taken.


Codex local extraction note: `e9a2f8a0` admits musl's exact native pthread clone
flags0x7d0f00 alongside glibc0x3d0f00 through the existing shared-thread owner.
The real static Rust Cargo/cache consumer now starts/joins its thread; default
linker threading stays enabled. This is local consumer evidence only. No fork
push, upstream PR/comment or storage redesign was performed.


Codex local extraction note: `6a19a70a` completes blocking pipe writes across
the unchanged64KiB buffer, retaining per-task vector/progress state and waking
peer readers. Actual rpm2cpio previously delivered only4096 of170444 bytes; the
unchanged RPM pipeline now passes strict payload/mode/ownership/link checks.
The new write/writev regression fails on its parent and passes on this commit.
Recipe0c4f0d4 separately fixes fixed-PATH/explicit-tool input collisions. Actual
managed APK acquisition/execution also passes. No storage redesign, upstream
publication or fork push occurred.


Codex local extraction note: `436c4ae5` tracks configured process-group umask
(shared by threads, copied on fork, preserved on exec) and applies it to owned
creation. GCC otherwise interpreted ENOSYS as the prior mask and produced mode
0645. The unchanged actual SQLite consumer now verifies0755, lock contention,
sum33 and integrity; Git/RPM/APK/GNU/musl also pass this source. No native-lane
change or upstream storage redesign/publication was made.


Codex local verification note: G02 remains held by the user's explicit choice.
The ordinary Recipe sleep/deadline case on436c4ae5 fails with realtime-clock
ENOSYS; another ordinary shared-destination case needs the same guest sleep
capability and is unrun. Independent verification continues. No clock grant,
upstream implementation, PR/comment or fork push was made.


Codex local review checkpoint: Ginux436c4ae5 / Recipe runtime0c4f0d4, fixture
source2ad833c.100 independent e2e cases and six actual consumers pass; G02's
failed sleep case and its unrun dependent case stay held by user choice, and
33 monitored e2e cases plus mixed targets remain pending. Current upstream is
still59e2f7aa; #232–#234 are open. Local corrections are committed for inspection;
Claude retains all upstream extraction/review/publication ownership. No push,
PR/comment or security dispatch occurred.
