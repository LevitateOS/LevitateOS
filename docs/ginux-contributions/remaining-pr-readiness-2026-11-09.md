# Remaining upstream PR readiness — 2026-11-09

**Title follow-up complete:** #238 now names both balanced/native paths;
#244 names the remaining pidfd selector/error/audit corrections; #248 names
test-only virtual cgroup API/containment coverage. #207 is untouched. #250
remains open; closure as superseded by merged #256 is recommended, with no
closure or comment performed.

**Description follow-up complete:** under the user's subsequent authorization,
the bodies of #233/#236/#238/#242/#243/#244/#245/#246/#247/#248 now match their
published revisions. Useful original failure and boot context is retained as
historical evidence. #207 is untouched; all titles, heads, bases, states,
comments and fork refs are unchanged. No source was pushed in this follow-up.

**Publication complete:** the user authorized the eight exact source updates
below, and their atomic lease-guarded push succeeded. Git and GitHub verified
every resulting head. All **24 hosted checks passed**: changes, static-and-unit
and launch-integration for each PR. Upstream remained
`55b719db9c52889543fbb177fab4cf2b2f90955e` at publication.
#207 remains separate and unchanged. #245/#247 are already published and have
passing CI; their source and public text were preserved.

This supplements the [original complete review](open-pr-updates-2026-11-09.md)
and [#245/#247 corrections](pr245-pr247-followup-2026-11-09.md). Old revision
tables remain historical. This report governs the eight current candidates.

## Exact publication table

All rows updated existing `LevitateOS/ginux` branches and their existing upstream
PRs. These rewritten histories were published using the exact remote leases below.
#243 contains #238 as its parent dependency; the other seven descend directly
from upstream main. #248 changes purpose to retained tests under the upstream
virtual cgroup owner, and must be presented as that test-only replacement.

| PR | Existing fork branch | Previous published head (lease used) | Published exact head |
| --- | --- | --- | --- |
| [#233](https://github.com/gmkurtzer/ginux/pull/233) | `fix/launch-mkdirat` | `f751cd6f04f7d95487b0fba5e1c01b6540ab43a7` | `e3b93f44a64c9443af1826b039df4766cf7da524` |
| [#236](https://github.com/gmkurtzer/ginux/pull/236) | `fix/stdio-directories` | `1f43d7fa1d2ea30975629356492e640e095b4d4e` | `f031ba75a8a106c11a342cfe2079787d672f33ab` |
| [#238](https://github.com/gmkurtzer/ginux/pull/238) | `fix/fcntl-signal-routing` | `efaeeca84829e0c232709da8fcf91d5248952e32` | `fd91bb33b9ba84fda9bd09efe4f8b3f4eae0a42e` |
| [#242](https://github.com/gmkurtzer/ginux/pull/242) | `fix/native-sigchld-delivery` | `bf0e00bafb19932b1b2476fb6230d053ee07916b` | `ed65a5fe95db56dc07dde91eb944759fcaa6d541` |
| [#243](https://github.com/gmkurtzer/ginux/pull/243) | `fix/launch-contract` | `067d992f29d11f7b79e78753258721fc94a2f2c3` | `607e93ea65b9f65242a825d2225a114bb936518e` |
| [#244](https://github.com/gmkurtzer/ginux/pull/244) | `fix/native-waitid-pidfd` | `ef05b4b576cd09733f6b37b4e70624d6da7e541a` | `705e676ce74850677384c2802c4549f54e69eb9d` |
| [#246](https://github.com/gmkurtzer/ginux/pull/246) | `fix/native-pidfd-get-info` | `4d3582f2716610177318855b0def5cdb5d3f4e00` | `806ed7d4631a4abe7f2c92418a8c6646660b6d61` |
| [#248](https://github.com/gmkurtzer/ginux/pull/248) | `fix/native-cgroup-mkdir` | `14881d130803c3f7eccd6ef02eaa51ca2d5a4088` | `3ae81ac81ca2c53d49e79706218060127e4a9593` |

## What is included

- #233: shared mkdirat syscall constant and missing-workdir regression, adapted to current upstream ownership.
- #236: inherited host-FD cleanup and standard-directory refusal, including O_PATH stdin; retained launch/native regressions and unchanged public limits.
- #238: signal-driven fcntl/ioctl refusal at the actual native admission owner, with 32-bit request classification and safe denial controls.
- #242: deliver SIGCHLD while choosing PTRACE_SYSCALL only for an awaited exit stop, otherwise PTRACE_CONT; retained blocked-poll and busy-userspace handlers.
- #243: current hierarchical contract enforcement, deny precedence, audit provenance/disable/cap and explicit refusal of unsupported controls. Rebased onto the refreshed #238. No downstream storage/catalog-1002 policy is added.
- #244: preserve upstream P_PIDFD implementation; normalize id_t and correct invalid/non-child errors while finishing denial audit records. Retain positive, WNOWAIT and high-bit alias tests. No inherited #242 runtime patch.
- #246: PIDFD_GET_INFO returns ENOTTY for the 32-bit request family, including changing size fields/high register bits, before reaching host information; unrelated-ioctl and child-wait controls remain.
- #248: retain cgroup mkdir/EEXIST/procs/events/kill, absolute and relative mkdirat, and mirror-symlink containment tests under upstream NativeVirtualKernelState. The obsolete host-materialization implementation is dropped. Only the Python harness and C fixture differ from current main.

The native runners now capture audit stderr separately and concatenate it only
after process completion. #245 hosted CI demonstrated that a shared pipe can
interleave an audit fragment into an otherwise correct fixture line. #236's
existing stream override helper is preserved; all assertions remain intact.
The other affected native runners use the same correction already published
in #245. This follow-up changes no production Rust/Cargo source or C fixtures
relative to the previously reviewed candidates. Every rebuilt launcher matches
its prior exact hash. #233 needs no capture change.

## Exact evidence

Each of the eight committed standalone candidates freshly passed:

- `git diff 55b719db9c52889543fbb177fab4cf2b2f90955e --check`;
- `make check build build-launch-fixture`, including formatting and strict Clippy;
- `sudo -n make test-launch` for #233;
- both `test-launch` and `test-native-supervision` for #236/#238/#243;
- `sudo -n make test-native-supervision` for #242/#244/#246/#248.

[Command receipts](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-remaining-readiness-2026-11-09/results.json), [full per-PR diffs and identities](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-remaining-readiness-2026-11-09) and [publication table](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-remaining-readiness-2026-11-09/publication-table.json) bind every result to its exact source. Local revised PR drafts are retained for review; none replaced public prose.

Combined validation, excluding #207: `6cc7c338ea82a898e50a6d85e0d36e75a7d81c84` on `codex/pr-remaining-validation`, launcher SHA-256 `925b7b7b678296dbe178e71b4446761aca41faaf5136b21f48d2474c0fd571d1`. This is the prior #207-free joined tree plus the single harness capture correction. Production source, C fixtures and launcher bytes are unchanged.

That exact combined tree freshly passed `make check test-unit build
build-launch-fixture` (105 unit tests, zero failures) and
`sudo -n make test-launch test-native-supervision` (all launch controls,
23 native checks and three clone-snapshot controls). No assertion was removed
or relaxed. Earlier same-production-source VFS and orphan receipts remain
valid in the #245/#247 report; they were not rerun or presented as fresh passes
in this task.

[Joined commands](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-remaining-readiness-2026-11-09/joined-results.json), [static/unit/build log](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-remaining-readiness-2026-11-09/joined-0.log), [runtime log](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-remaining-readiness-2026-11-09/joined-1.log), [full joined diff](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-remaining-readiness-2026-11-09/joined.patch), [single follow-up diff](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-remaining-readiness-2026-11-09/joined-followup.patch).

#207 is not an ancestor of any of these eight candidates or the joined check.
The joined core policy blob equals #238; the channel hook blob equals #243.
The existing separate #207 local head remains `401aa878` and its public head
remains `5ae058f2`. No native poll policy expansion is bundled here.

## Other open PRs

| PR | Disposition |
| --- | --- |
| #232/#234/#241 | No source update needed on unchanged main. Preserve their useful implementations/tests and earlier exact clean-merge, VFS and orphan evidence. Their relevant units also pass on the current joined tree. |
| #245/#247 | Already published at `9508612f` / `6f5baf89`, all three CI checks passed for each. No further update proposed. |
| #250 | Superseded by merged #256. Its workflow/classifier/test blobs are identical to main. No duplicate implementation update; no closure/comment authorized. |
| #207 | Separate local proposal, still subject to upstream policy direction. No source or public action proposed here. |

## Publication boundary and public text

The user authorized exactly these eight source updates and normal automatic
PR CI. Upstream main and every lease matched immediately before publication.
The atomic push completed, every resulting head matched through Git and GitHub,
and all other origin branches remained identical to the pre-push snapshot.
All three hosted checks completed successfully on every approved revision.
No manual workflow dispatch or rerun occurred.

During the source-only publication, public titles, descriptions and comments
were preserved. At that point the prose was historical: notably #243 describes the older schema, #244 the superseded
whole implementation/base dependency, and #248 host-backed materialization.
The prepared local drafts described the current candidates. That source-only
push did not authorize rewriting human-authored prose; the subsequent explicit
description-update instruction authorized the completed follow-up above. #248 must be identified to
the user as a test-only replacement when approving its source push. Any later
public explanation/title/body edit or superseded-PR disposition requires its
own explicit permission.

No manual hosted dispatch/rerun, PR creation, comment, closure, tag, release,
maintained-source update or root publication occurred. These bounded checks
do not establish full systemd/VM boot, all adversarial schedules or release/global
security acceptance. macOS remains indefinitely deferred.

[Preservation receipt](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-remaining-readiness-2026-11-09/preservation.json) confirms unchanged #245/#247/#207, maintained integration, frozen PRD04/export sources and binaries, and protected human staging. [Proposed command, not executed](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-remaining-readiness-2026-11-09/proposed-push.sh).

Active skill: hdae-development via
`.agents/skills/hdae-development/SKILL.md`; reload the full skill and required
references after any context shift. The authorized source publication and subsequent description updates are
complete. No further source revisions or other upstream actions are inferred.

## Publication receipt

[Exact authorization](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-remaining-publication-2026-11-09/authorization.md), [atomic push log](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-remaining-publication-2026-11-09/push.log), [verified Git heads](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-remaining-publication-2026-11-09/origin-after.txt), [verified PR heads and unchanged text](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-remaining-publication-2026-11-09/api-after.json), [24 successful hosted checks](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-remaining-publication-2026-11-09/checks-final.json), [unchanged other PRs](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-remaining-publication-2026-11-09/other-prs-after.json) and [preservation proof](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-remaining-publication-2026-11-09/preservation.json).

| PR | Published head | Automatic CI |
| --- | --- | --- |
| #233 | `e3b93f44` | [3/3 passed](https://github.com/gmkurtzer/ginux/actions/runs/42683874583) |
| #236 | `f031ba75` | [3/3 passed](https://github.com/gmkurtzer/ginux/actions/runs/42683877334) |
| #238 | `fd91bb33` | [3/3 passed](https://github.com/gmkurtzer/ginux/actions/runs/42683877994) |
| #242 | `ed65a5fe` | [3/3 passed](https://github.com/gmkurtzer/ginux/actions/runs/42683876130) |
| #243 | `607e93ea` | [3/3 passed](https://github.com/gmkurtzer/ginux/actions/runs/42683876669) |
| #244 | `705e676c` | [3/3 passed](https://github.com/gmkurtzer/ginux/actions/runs/42683878955) |
| #246 | `806ed7d4` | [3/3 passed](https://github.com/gmkurtzer/ginux/actions/runs/42683875553) |
| #248 | `3ae81ac8` | [3/3 passed](https://github.com/gmkurtzer/ginux/actions/runs/42683876635) |

## Description follow-up receipt

The refreshed #243 body describes catalog 2, hierarchical selectors, deny
precedence, audit bounds, unsupported-control refusal and its #238 dependency.
#244 identifies upstream as the managed-child wait owner and describes only
the remaining id/error/audit corrections and retained tests. #248 explicitly
identifies its test-only replacement and virtual cgroup owner. #245 and #247
now describe the published immutable-clone and lifetime-based reap fixes.
The other refreshed bodies retain the original bug context while updating
configuration, regression coverage and current-source limitations. Original
Fedora/Rocky/OpenRC boot observations are explicitly historical, not presented
as fresh results for these heads. Each body links its exact current head and
successful hosted checks.

These were description-only edits through the GitHub connector. Every body was
checked against fresh metadata immediately before mutation and read back
exactly afterward. Existing titles, base/head refs and SHAs, states, maintainer
permissions and comment counts matched throughout. Untouched #207/#232/#234/
#241/#250 were read back unchanged; all fork branch heads match the pre-edit
snapshot. No code changed, new tests ran, workflow was manually dispatched,
comment was posted or PR was created/closed. Existing exact-source CI receipts
were reused and labelled accordingly.

[Original bodies](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-description-updates-2026-11-09/before.json), [replacement bodies](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-description-updates-2026-11-09/proposed.json), [verified readback](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-description-updates-2026-11-09/after.json), [unchanged PRs](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-description-updates-2026-11-09/untouched-after.json), [reused CI evidence](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-description-updates-2026-11-09/checks.json) and [preservation proof](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-description-updates-2026-11-09/preservation.json). Per-PR body diffs are retained in the same evidence directory.

## Title follow-up and #250 disposition

Under the user's subsequent title-update instruction, the verified current
titles are:

- #238: `fix(core): refuse signal-driven I/O in balanced and native paths`
- #244: `fix(init): correct pidfd wait selectors, errors and audit completion`
- #248: `test(init): retain virtual cgroup API and containment coverage`

All other titles already match their current contributions. #248's body also
lost only the obsolete sentence saying its original title was retained; the
rest of the approved descriptions are preserved. Titles and that minimal
consistency edit were read back exactly. Heads, bases, states, comments and
all origin branches are unchanged. #207 remains separate and untouched. No
source push, CI dispatch or new test execution occurred for metadata edits.

#250 is still open. #256 explicitly replaced it and is merged at
`55b719db9c52889543fbb177fab4cf2b2f90955e`. Fresh comparison confirms that
current main and #250 have identical workflow, classifier and classifier-test
blobs. Close #250 as superseded by #256; no duplicate rebase or implementation
update is needed. The user requested a recommendation, so #250's title, body,
state and comments were left unchanged. Any closure or public comment needs
its own instruction.

[Before metadata](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-title-updates-2026-11-09/before.json), [verified title updates](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-title-updates-2026-11-09/after.json), [untouched PRs](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-title-updates-2026-11-09/untouched-after.json), [#250 equivalence and disposition](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-title-updates-2026-11-09/250-disposition.json), [preservation receipt](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-title-updates-2026-11-09/preservation.json).
