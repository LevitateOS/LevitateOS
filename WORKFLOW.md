# Delivery Workflow

This repository now uses GitHub Issues as the live backlog and pull requests as
the normal delivery path for the default branch.

The goal is not to restart the repo. The goal is to migrate from ad-hoc direct
pushes to a PR-gated workflow that still works with the current submodule-heavy
layout.

## Default Rules

- Do not push directly to the default branch.
- Every code change must link to a GitHub Issue.
- Every PR must name one primary owner path.
- Every PR must include acceptance commands or a concrete verification path.
- Reviews are expected before merge. The default branch is intended to be
  review-gated, including AI review agents where configured.

## Think By Owner, Not By Submodule

Use the primary owner path first. Only think in submodules when the owner path
actually lives inside a submodule repository.

### Superproject-first paths

Open the PR in `LevitateOS/LevitateOS` when the change primarily touches:

- `distro-variants/**`
- `xtask/**`
- `.github/**`
- `BACKLOG.md`
- `WORKFLOW.md`
- root docs and repo policy files that are not themselves submodules

### Submodule-owned canonical paths

Open the first PR in the owning submodule repository when the change primarily
touches one of these paths:

- `distro-builder/` -> `LevitateOS/distro-builder`
- `distro-contract/` -> `LevitateOS/distro-contract`
- `distro-spec/` -> `LevitateOS/distro-spec`
- `testing/install-tests/` -> `LevitateOS/install-tests`
- `testing/fsdbg/` -> `LevitateOS/fsdbg`
- `testing/rootfs-tests/` -> `LevitateOS/rootfs-test`
- `tools/recipe/` -> `LevitateOS/recipe`
- `tools/recab/` -> `LevitateOS/recab`
- `docs/content/` -> `LevitateOS/docs-content`
- `docs/website/` -> `LevitateOS/website`
- `docs/tui/` -> `LevitateOS/docs-tui`

### Legacy compatibility paths

These are not the default place for new work:

- `leviso/`
- `AcornOS/`
- `IuppiterOS/`
- `RalphOS/`

Only use them for explicit compatibility fixes or retirement work.

## Change Types

### Superproject-only change

Use one branch and one PR in `LevitateOS/LevitateOS`.

### Submodule-only change

1. Open the code PR in the submodule repository.
2. Merge the submodule PR after review.
3. If the superproject must consume the new revision, open a pointer-update PR
   in `LevitateOS/LevitateOS`.

### Cross-repo change

1. Open the submodule PRs first.
2. Open one superproject integration PR that updates the submodule pointers and
   any top-level orchestration/docs/tests.
3. In the superproject PR body, list the linked submodule PRs explicitly.

Do not hide cross-repo work inside a direct pointer bump with no linked review
history.

## PR Shape

Every PR should state:

- linked issue
- primary owner path
- target repository
- what changed
- submodule PRs, if any
- acceptance commands

Use `.github/pull_request_template.md`.

## Automatic AI Review

- The default-branch ruleset automatically requests GitHub Copilot code review.
- Automatic review runs on draft pull requests and on each new push to the pull
  request.
- Repository-specific review guidance lives in `.github/copilot-instructions.md`.
- AI review is advisory and does not replace the required human approval.
- If an automatic AI review does not appear, verify that GitHub Copilot code
  review is available to the repository or author and that the relevant quota
  and policy settings allow the review to run.

## Merge Rules

- Prefer small PRs with one clear owner.
- Do not mix unrelated submodule bumps into the same PR.
- Do not merge until the required review and required checks pass.
- After merge, delete the branch.

## Migration Notes

- Existing submodules stay in place for now.
- The migration is about delivery discipline first, not immediate repo
  restructuring.
- If the submodule model remains too expensive later, that should be handled as
  a separate structural migration with its own issue and acceptance criteria.
