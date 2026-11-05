# Prepared Ginux contributions — 2026-11-05

Three local, standalone candidates are ready for user review. Nothing has been
pushed or opened. Recommended first submission: the failed-open fix; the other
two can follow at the user's pace.

All three commits have the same immediate parent, upstream `main` at
`2489517e89c3b62c5203ccd0f6d1eb28cf8500af`. Each changes only
`crates/ginux-vfs/src/lib.rs`, includes focused regression coverage, and retains
the existing tests and CI. Each worktree is detached; no branch was created or
switched. Their patches also apply together without conflicts.

| Candidate | Local commit | Change size | Validation |
| --- | --- | --- | --- |
| Failed opens preserve contents and capacity | `397471245b8335c3ab8494197af81b26e3125987` | +55 / -5 | static checks, 57 unit tests, release build, VFS CLI |
| Unlinked files retain bytes until their last close | `4070b3a1a19ee83d9521c43043f27c07000cf239` | +137 / -6 | static checks, 58 unit tests, release build, VFS CLI |
| Links resolve before parent components | `2a60d36170c1c31636eee55322e035403188122c` | +125 / -64 | static checks, 56 unit tests, release build, VFS CLI |

The combined tree passes static checks and all 61 unit cases. On the exact
unmodified upstream VFS source, its four existing VFS tests pass and all six new
regressions fail. Most added lines in the lifetime fix are its three regression
scenarios. The CLI checks ran on the Linux host using the existing prepared
Rocky rootfs and a freshly verified 9,827-entry index; the Rocky VM wrappers
were not rerun. GitHub CI has not run because these candidates remain local.

## Review files

- [Failed-open draft](../.artifacts/ginux-pr-preparation-20261105/01-open-failure/body.md), [patch](../.artifacts/ginux-pr-preparation-20261105/01-open-failure/candidate.patch), [checks](../.artifacts/ginux-pr-preparation-20261105/01-open-failure/checks.log).
- [Unlink-lifetime draft](../.artifacts/ginux-pr-preparation-20261105/02-unlink-lifetime/body.md), [patch](../.artifacts/ginux-pr-preparation-20261105/02-unlink-lifetime/candidate.patch), [checks](../.artifacts/ginux-pr-preparation-20261105/02-unlink-lifetime/checks.log).
- [Path-resolution draft](../.artifacts/ginux-pr-preparation-20261105/03-path-resolution/body.md), [patch](../.artifacts/ginux-pr-preparation-20261105/03-path-resolution/candidate.patch), [checks](../.artifacts/ginux-pr-preparation-20261105/03-path-resolution/checks.log).
- [Manifest and exact worktree paths](../.artifacts/ginux-pr-preparation-20261105/manifest.json), [baseline failures](../.artifacts/ginux-pr-preparation-20261105/all-regressions-baseline.log), [combined checks](../.artifacts/ginux-pr-preparation-20261105/combined-checks.log).

Each candidate directory also contains its title, plain diff, build/CLI logs and
initial reproduction. The temporary combination checkout is not a candidate;
its archive request was refused by the app as owned by another task, so it was
left intact. Its exact path and disposition are in the manifest. The three candidate worktrees are retained for inspection:

- `/home/vince/.codex/worktrees/ginux-pr-open-failure/ginux`
- `/home/vince/.codex/worktrees/ginux-pr-unlink-lifetime/ginux`
- `/home/vince/.codex/worktrees/ginux-pr-path-resolution/ginux`

## Disposition of larger contributions

- **Already upstream:** invalid-command rejection was independently merged in
  [PR #165](https://github.com/gmkurtzer/ginux/pull/165). Do not resubmit that
  local commit merely because its patch ID differs.
- **Already open:** [PR #207](https://github.com/gmkurtzer/ginux/pull/207) covers
  bounded console polling. Its current review requests holding head `e7f5f552`
  while Gregory decides the balanced-profile contract; no revision is requested.
- **Policy decision pending:** clock/sleep, cwd and descriptor compatibility
  changes touch the groups discussed in [issue #179](https://github.com/gmkurtzer/ginux/issues/179).
  Its maintainer comment asks to hold implementation until Gregory chooses the
  contract. Existing local experiments are preserved, not repackaged as new
  policy PRs in this batch.
- **Further extraction and design review needed:** assigned writable storage,
  virtual devices/channels, configured child/thread ownership, generated exec,
  file mappings, snapshots/export/restore and resource enforcement are larger
  capability proposals. The integration branch contains shared prerequisites;
  its successful combined tests do not prove any arbitrary extracted feature
  works independently on main. The older fork branches also contain prototype
  demo/library wiring and cannot be sent as independent configured-launch PRs
  simply by changing their base.

For those larger proposals, prepare the next coherent capability after its
prerequisites are upstream, then base it directly on current main. This preserves
standalone PRs without carrying another pending contribution's commits. No
11,000-line integration dump or policy files are included in this package.

The Ginux contribution commits and draft text contain no workspace instructions
or downstream policy. Existing human-authored upstream PR and comment text was
only read and remains unchanged.

Active skill: `hdae-development` at
`.agents/skills/hdae-development/SKILL.md`; reload after context shifts.
Current boundary: local preparation complete; user review precedes publication.
Exact next action: review `01-open-failure`; refresh the chosen candidate against
upstream again immediately before any separately authorized publication.
