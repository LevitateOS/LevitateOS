# Backlog Policy

GitHub Issues are the only live backlog for this repository.
GitHub pull requests are the only normal delivery path for the default branch.
See [`WORKFLOW.md`](WORKFLOW.md) for the branch, PR, and submodule rules.

Architecture docs, migration docs, audits, and checklists may explain context,
history, and design intent, but they must not be the primary place where open
work is tracked.

## Rules

- Every active work item must have a GitHub Issue.
- Every issue must name exactly one primary owner path.
- Do not organize work by git submodule unless the work is literally submodule
  maintenance.
- Legacy crates are compatibility-only owners:
  - `leviso/`
  - `AcornOS/`
  - `IuppiterOS/`
  - `RalphOS/`
- Migration docs may keep historical checklists, but they should not be treated
  as the live source of truth once issues exist.
- When a doc still contains unresolved work, create issue(s) first, then update
  the doc to point at those issue numbers instead of carrying the live backlog.

## Canonical Owner Taxonomy

Use one of these as the issue's primary owner:

- `distro-builder`
- `distro-contract`
- `distro-spec`
- `distro-variants/levitate`
- `distro-variants/acorn`
- `distro-variants/iuppiter`
- `distro-variants/ralph`
- `testing/install-tests`
- `testing/fsdbg`
- `testing/rootfs-tests`
- `xtask`
- `tools/recipe`
- `tools/recab`
- `docs/content`
- `repo-admin`
- `legacy-compat`

If a task spans multiple areas, choose the owner that must change first and
list the rest as related paths.

## Suggested Labels

- `owner:distro-builder`
- `owner:distro-contract`
- `owner:distro-spec`
- `owner:distro-variants/levitate`
- `owner:distro-variants/acorn`
- `owner:distro-variants/iuppiter`
- `owner:distro-variants/ralph`
- `owner:testing/install-tests`
- `owner:testing/fsdbg`
- `owner:testing/rootfs-tests`
- `owner:xtask`
- `owner:tools/recipe`
- `owner:tools/recab`
- `owner:docs/content`
- `owner:repo-admin`
- `owner:legacy-compat`
- `kind:epic`
- `kind:task`
- `kind:bug`
- `kind:docs`
- `kind:migration`
- `kind:debt`
- `priority:p0`
- `priority:p1`
- `priority:p2`
- `priority:p3`
- `state:blocked`

## Required Issue Shape

Every issue should state:

- primary owner path
- target repository (`LevitateOS/LevitateOS` or a named submodule repo)
- problem or outcome
- why it matters
- acceptance command(s) or verification path
- related docs or source paths

## Migration From Docs To Issues

1. Create issues from [`docs/ISSUE_SEED_BACKLOG.md`](docs/ISSUE_SEED_BACKLOG.md).
2. Add labels and milestone/project state in GitHub.
3. Update the originating doc to say `Tracked in: #<issue>`.
4. Stop adding new unchecked lists to docs unless they are explicitly archival.
5. Delete or freeze the seed backlog doc once the issues exist.
