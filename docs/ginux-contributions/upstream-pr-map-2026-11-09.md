# Upstream PR map — 2026-11-09

A proposed order for everything we would send to `gmkurtzer/ginux`. It is a
map for approval: no branch was cut, nothing was pushed and no PR was edited.

Upstream main is `55b719db`. Our fork's main is `1b7257be`, which carries
24,511 added lines over upstream in 86 files: about 17,000 Rust, 6,800 tests,
800 docs and build.

Sizes below are approximate. Packet sizes come from the packet stack recorded
on 2026-11-06 and have drifted; items marked † were measured on current main.
Titles are working titles in upstream's style.

## What decides the order

1. **Six open PRs gate everything after stage B.** #232, #233, #234, #236,
   #238 and #243 are unmerged. The maintainer has said review follows his own
   foundation work; his Gate 13 PR (#255) is open now.
2. **Our main is not in PR shape for storage.** The storage packets (G03b,
   G03c) failed review and were redesigned as `launch_storage.rs` and
   `launch_paths.rs` (candidate `9802ef03`). Main never adopted that redesign:
   it still runs on `fs_hooks.rs`. Every PR from stage D on touches storage,
   so each must be ported onto the redesign before it can be cut.
3. **The prepared standalone candidates are stale.** Storage, catalog,
   zero-fill and TCP all sit on `53a54add`, 60 upstream commits back. Upstream
   has since replaced the contract schema (88 files, +7,805/−1,971).
4. **Upstream now has its own catalog 2.** Ours is 1002 and is approved only
   downstream. How configured-launch operations enter upstream's catalog is a
   maintainer decision, and stage C cannot be cut without it.

## Stage A — already open (13 PRs, nothing to prepare)

All mergeable with green CI except #207.

| PR | Subject | Role |
| --- | --- | --- |
| #232 | vfs: share offsets between duplicated descriptors | foundation |
| #233 | launch: mkdirat for the workload directory | foundation |
| #234 | process: adopt orphans after the last parent thread exits | foundation |
| #236 | init: keep host directories out of chrooted workloads | foundation |
| #238 | core: refuse signal-driven I/O | foundation |
| #243 | launch: enforce the resolved instance contract | foundation, on #238 |
| #241, #242, #244–#248 | native-mode fixes | independent of this stack |
| #207 | console: bounded polling | conflicting; review paused by the maintainer under #179 |

## Stage B — can be opened on main once rebased (2 PRs)

| # | PR | Size | Tests | Blocker |
| --- | --- | --- | --- | --- |
| B1 | fix(vfs): zero-fill gaps past EOF and empty writes | ~45 | red/green VFS units | conflicts with #232; send after it merges |
| B2 | feat(devices): bounded supervisor TCP endpoint (G28) | ~820 | 5 TCP units | **you decided to keep this local**; needs your reversal |

## Stage C — configured contract and storage (5 PRs, after stage A's six)

| # | PR | Size | Tests | Depends on |
| --- | --- | --- | --- | --- |
| C1 | feat(contract): configured-launch operations in the catalog | ~350 | contract units, CLI controls | maintainer decision on the catalog |
| C2 | refactor(vfs): overlay slot states as an enum | small | existing VFS units | #232 |
| C3 | feat(vfs): typed seek, modes, append, exclusive create | ~700 | VFS units | C2, B1 |
| C4 | feat(vfs): directories and stable inode identity | ~750 | VFS units | C3 |
| C5 | feat(launch): assigned ephemeral memory storage | ~2,500 | `test-launch-storage` | C1, C4, #233, #236, #243 |

C2–C5 are the redesign candidate (`9802ef03`, 4,004 lines in 15 files) split
at its natural seams. C5 is still large; `launch_paths.rs` (378 lines) could
go first as its own PR.

## Stage D — descriptors and tasks (9 PRs)

| # | PR | Size | Tests | Depends on |
| --- | --- | --- | --- | --- |
| D1 | feat(launch): assigned null device (G04) | ~600 | `test-launch-devices` | C5 |
| D2 | feat(launch): public descriptor table (G05) | ~700 | `test-launch-fds` | D1 |
| D3 | feat(launch): anonymous pipes (G05) | ~750 | `test-launch-pipes` | D2 |
| D4 | feat(process): cancel reserved tasks, inspect waits | ~200 | process units | #234 |
| D5 | feat(launch): fork, wait, kill and exec for configured tasks (G06b) | ~2,500 | `test-launch-tasks`, `test-launch-exec` | D3, D4 |
| D6 | feat(launch): native threads sharing a process (G07) | ~1,500† | `test-launch-threads` | D5 |
| D7 | feat(launch): poll and ppoll over owned descriptors (G08a) | ~450 | `test-launch-poll` | D5 |
| D8 | feat(launch): local packet pairs and FIONBIO (G08b) | ~320 | poll and pipe fixtures | D7 |
| D9 | feat(launch): eventfd and epoll | ~600† | `test-launch-events` | D7 |

**Today's fix lands mostly in D5.** It is the first PR with children, so the
first where a parked call can miss SIGCHLD, where a creation can lose its
parent, and where exec injects syscalls. D5 carries the creation reservation,
the park rule and the signal hold, with the `park-read`, `park-wait`,
`birth-orphan` and `exec-signal` scenarios. D6 adds the thread half of the
creation rule and the `race-thread-exit` regression. D7 and D9 add
`park-poll` and `park-epoll`. D5 should be split further before review, most
naturally fork/wait/kill first and exec second.

## Stage E — data, namespace, transfer (13 PRs)

| # | PR | Size | Depends on |
| --- | --- | --- | --- |
| E1 | feat(vfs): byte-storage owner (G09a) | ~290 | C4 |
| E2 | feat(launch): growable storage (G09b) | ~280 | D5, E1 |
| E3 | feat(vfs): directory descriptions (G09c) | ~490 | E1 |
| E4 | feat(launch): directory and image cwd operations (G09d) | ~580 | E2, E3 |
| E5 | feat(launch): bounded namespace capacity (G18) | ~320 | E4 |
| E6 | feat(launch): pwrite (G17a) | ~50 | E4 |
| E7 | feat(launch): exec of owned ELF files (G10) | ~1,170 | E4 |
| E8 | feat(launch): scripts and interpreter chains (G11) | ~1,160 | E7 |
| E9 | feat(launch): private mappings of owned files (G12) | ~670 | E8 |
| E10 | feat(launch): owned symlinks (G13) | ~520 | E8 |
| E11 | feat(launch): per-process cwd (G14) | ~250 | E10 |
| E12 | feat(launch): hardlink identity (G15) | ~440 | E10 |
| E13 | feat(launch): unified image and owned traversal (G16), private backing limits (G17b) | ~350 | E11, E5, E9 |

E9 carries the `map-signal` scenario. Tests: `test-launch-directories`,
`test-launch-owned-exec`, `test-launch-scripts`, `test-launch-storage`.

## Stage F — persistence (4 PRs)

| # | PR | Size | Tests | Depends on |
| --- | --- | --- | --- | --- |
| F1 | feat(launch): flush and quiesced export (G19) | ~530 | export units | E9 |
| F2 | feat(launch): checkpoint control and CI wiring (G20, G31) | ~560 | `test-launch-checkpoints` | F1 |
| F3 | feat(launch): sealed startup restore (G21) | ~480 | `test-launch-restore` | F1 |
| F4 | feat(launch): link identity in archives (G22) | ~400 | archive units | E12, E13, F3 |

## Stage G — consumer semantics (9 PRs)

| # | PR | Size | Depends on | Hold |
| --- | --- | --- | --- | --- |
| G1 | feat(launch): virtual file locks (G23) | ~390 | E12 | |
| G2 | feat(vfs): inode times and archive mtime (G24) | ~260 | F4 | clock policy |
| G3 | feat(launch): bounded vectors and chunked I/O (G25) | ~250 | E6 | |
| G4 | feat(launch): child address-space and CPU bounds (G26a, G26b) | ~220 | D5 | policy-held |
| G5 | feat(launch): virtual uname and sysinfo (G27) | ~120 | — | held under #179 |
| G6 | feat(launch): virtual uid and gid ownership (G30) | ~90 | E12 | |
| G7 | feat(launch): configured clocks and bounded sleep (G02 configured form) | ~400† | D5 | you kept clock policy held |
| G8 | feat(launch): descriptor path names and O_PATH references | ~500† | D2, E4 | |
| G9 | feat(launch): own-stack view | ~300† | D5 | |

G7 carries the sleep rule from today's fix and the three `sleep-*` scenarios.

## Stage H — networking (1 PR)

| # | PR | Size | Tests | Depends on |
| --- | --- | --- | --- | --- |
| H1 | feat(launch): numeric TCP through public descriptors (G29) | ~1,020 | `test-launch-network` | B2, D7, E11, its own contract entry |

## Totals

13 open, 43 new: B 2, C 5, D 9, E 13, F 4, G 9, H 1. With D5 and C5 split
as suggested, 45.

## Standing holds this map does not override

- No stack is published ahead of its base; each PR is cut against real main
  after its predecessors merge. Every push, PR, title, body or comment needs
  your explicit go-ahead.
- B2 (TCP) stays local by your decision, which also blocks H1.
- G4, G5 and G7 are policy-held around #179; #163, #178 and #219 are paused
  by the maintainer and not to be raced.
- E7 and E8 were recorded as security-held, and D6 as unready because of a
  shared-path race. Both statuses predate the argument guard and today's
  work, so they need a fresh look, not an assumption either way.
- #224, #226 and #227 belong to other contributors and are not duplicated.

## Decisions needed before any branch is cut

1. **Port main onto the storage redesign first, or send main's storage as it
   is?** Recommended: port. The `fs_hooks.rs` form already failed review, and
   every later PR would inherit it.
2. **Raise the catalog question with the maintainer** (C1) before storage
   work is rebased, since its answer shapes C5 and H1.
3. **Does B2 (TCP) stay local?** If so, H1 has no upstream path.
4. **Is one PR per packet the right grain?** 43 PRs is a long queue for a
   maintainer who has merged nothing of ours since 2026-11-05. Stages E and G
   could be folded to roughly half that if he prefers fewer, larger reviews.

## Basis and limits

Live PR state was read from GitHub on 2026-11-09. Packet contents, sizes and
dependencies come from `pr-inventory.md`, `p04-dependencies.md`,
`prd07-delivery-waves.md` and the `g*-review.md` files, extracted by a
subagent that read the review files by head and tail only. The capabilities
added after G31 (events, descriptor paths, O_PATH, stack view, configured
clocks) have no packet record; their sizes are file measurements and their
boundaries are my proposal.
