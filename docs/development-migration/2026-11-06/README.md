# Development migration — 2026-11-06

This is a recovery snapshot for moving development to another computer. It is
not a release, a passing CI claim or approval to submit Ginux PRs. The Ginux
implementation goal remains paused. Its task list, F02/F03 reviews and
DaybreakBlue handoff record the exact unfinished work and failed/unrun checks.
The user's migration request authorizes committing and pushing backup work to
our origin repositories; upstream PR publication still requires separate review.

## Start on the new computer

Clone the superproject, including its submodules:

```sh
git clone --recurse-submodules git@github.com:LevitateOS/LevitateOS.git
cd LevitateOS
```

Use the submodule commits recorded by the superproject. The manifest also lists
all local branch heads captured during inventory; origin contains the pushed
branches. A local branch already behind origin was not force-pushed backwards.
No branch was switched and no remote history was rewritten for this migration.

## Recover prepared Ginux work

The proposed changes are still represented by the committed patches and reviews
in `docs/ginux-contributions/`. The bundle also retains their otherwise local
commit/composition objects, held G07, old detached review heads, staged blobs and
both saved stashes. Import its recovery references into the fork checkout:

```sh
git -C tools/ginux fetch ../../docs/development-migration/2026-11-06/ginux.bundle \
  'refs/migration/*:refs/migration/*'
```

These are recovery references; this command does not check out a branch or open
a PR. `manifest.json` maps every reference to its exact object. To inspect any
saved candidate, use its recorded commit with `git show`, or ask the next agent
to create an isolated checkout. In particular, F02 is the local combined
`dfebda7c29b628fc41f50635b07e8cb48d6f883a` and corrected G12 is
`8bb50aaa50e80428ff417654d35affd468255775`. G07 remains unready and excluded from
that combined snapshot. No ordinary pass substitutes for its security acceptance.

Extract the development evidence and raw worktree snapshots into a chosen
scratch directory:

```sh
mkdir -p /tmp/levitate-recovery
tar -xzf docs/development-migration/2026-11-06/recovery.tar.gz -C /tmp/levitate-recovery
```

The archive contains:

- `worktrees/*/snapshot.json`, changed files and the exact index for six unfinished
  Ginux checkouts. This preserves staged/unstaged differences and unresolved merge
  entries. Restore only into a fresh checkout of the snapshot's recorded head:
  overlay its changed files, apply its recorded deletions, and copy its index to
  that checkout's own Git index path. Never copy a snapshot index onto an unrelated
  working branch. The stored base objects are in ginux.bundle.
- `evidence/ginux-contribution-preparation/` and `evidence/latest/`, with paused
  states, receipts, failures, selected logs, candidate patches and reproduction
  helpers. Absolute old paths need adaptation on the new computer; they are
  evidence of the original machine, not instructions to overwrite live paths.
- `helpers/`, plus saved UI/Recipe untangle scripts and analysis under
  `other-development/`.
- `recipe-snapshot-overlay/`, with the exact tracked dirty overlay and normalized
  standalone lock used for the fresh F03 verification snapshot.
- `inventory/`, recording the original repository, worktree, branch and stash
  state. Original local files and indexes were left intact after capture.

`restore-verification.json` records fresh-store bundle import and exact status
reconstruction for all six snapshots, including the conflicted indexes.

## Retired repositories

The13 bundles under `retired/` retain local branch/tag histories from removed
submodules. They are recovery material only; obsolete repository origins were
not modified. The manifest records their former paths and remotes. For example:

```sh
git clone --branch main docs/development-migration/2026-11-06/retired/AcornOS.bundle /tmp/AcornOS-recovered
```

## What to recreate

SSH/API credentials, private keys and local trust configuration must be provisioned
separately. Compiled binaries, Cargo/Bun caches, downloaded packages, prepared
rootfs images and other large build outputs are not in this source migration.
Recreate them through their existing producers and pinned manifests. Ginux F03's
image identity and executable/source hashes are saved for comparison. Do not copy
test CA trust into production trust. Keep the original computer until any needed
non-Git build assets have been transferred or reproduced successfully.

All new archives/bundles have SHA256 checksums in the manifest. `push-results.json`
records the origin verification completed after saving the source changes.
