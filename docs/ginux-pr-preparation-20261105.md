# Prepared Ginux contributions — 2026-11-05

Three local, standalone candidates are ready for user review. Nothing has been
pushed or opened. Recommended first submission: the failed-open fix; the other
two can follow at the user's pace.

All three commits have the same immediate parent, upstream `main` at
`2489517e89c3b62c5203ccd0f6d1eb28cf8500af`. Each changes only
`crates/ginux-vfs/src/lib.rs`, includes focused regression coverage, and retains
the existing tests and CI. Each worktree is detached; no branch was created or
switched. Their changes also merge together without conflicts.

| Candidate | Local commit | Change size | Validation |
| --- | --- | --- | --- |
| Failed opens preserve contents and capacity | `c2a18bb07b3adfa811fee57dc777cbf9ec757a53` | +92 / -7 | static checks, 58 unit tests, release build, VFS CLI |
| Unlinked files retain bytes until their last close | `be56d00121cf7fb1be3463dfd48343949c82aebd` | +152 / -6 | static checks, 59 unit tests, release build, VFS CLI |
| Links resolve before parent components | `2a60d36170c1c31636eee55322e035403188122c` | +125 / -64 | static checks, 56 unit tests, release build, VFS CLI |

Review found and fixed two additional failures: an image read error left an
empty overlay hiding the original, and recycling a deleted slot could resurrect
an image file addressed through a symlinked directory. Each correction has a
regression that failed before the fix. The path-resolution implementation needed
no changes; extra absolute/relative/dangling-link and bound probes passed.

Each candidate passes the current CI workflow’s build and test commands locally on
Linux: static checks, unit tests, full release/fixture builds, launch, native
supervision, process-native and gate1 setup checks. The VFS CLI also passes with
the prepared Rocky rootfs. These are local results; GitHub Actions and its macOS
runner have not run because the candidates remain unpublished. No CI coverage
was removed or changed.

A fresh upstream source tree with the three changes merged passes static checks
and all 63 unit cases. Against unmodified upstream VFS, its four original tests
pass and all eight added regression tests fail. The previous combination
worktree was preserved; the fresh validation source is recorded in the manifest.

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
