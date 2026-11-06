# Ginux PR inventory — 2026-11-06

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
