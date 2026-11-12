# Development Guidelines

Keep development fast. Fix the actual behavior, use judgment, and keep process proportional to the change.

These rules apply to every directory and submodule, recursively, across Git repository boundaries. Nested guides may add domain details but must not reintroduce unit tests or mandatory development bureaucracy, except for the ginux unit-testing exception below.

## Recipe is the product
- Recipe is the star of these systems, not one tool among others. LevitateOS and AcornOS are the systems whose software is acquired, built, installed and updated through Recipe, inside Ginux; recab owns the system image. Everything else in this repository serves that.
- There is no vacancy to fill. Whenever a need amounts to getting, building, installing, updating, removing or isolating software, during the build or for the person using the installed system or its desktop, the answer is Recipe. Never propose, compare against as an option, or add another package manager, app store, bundle format or sandbox for it: no Flatpak, Snap, AppImage, Nix, Homebrew, Discover or other store back-ends, and no dnf or apk workflow for users.
- Where Recipe cannot do the job yet, say exactly what Recipe lacks and extend Recipe (or Ginux, or recab). A gap in Recipe is work on Recipe, never a reason to bring in a substitute.
- The same holds for the other tools here (ginux, recab, recstrap, recpart, recinit, the TUIs): before naming an outside tool for a job, check whether one of ours owns it.

## Working style
- ginux (`tools/ginux` and all of its branches and worktrees) is Greg Kurtzer's external upstream project (`gmkurtzer/ginux`); `LevitateOS/ginux:main` is our maintained delivery repository as authorized on 2026-11-08. Upstream Ginux leads: retain its history, reconcile incoming upstream changes, and replace or drop our changes when upstream provides the behavior. Keep Recipe-required changes available in our repository while upstream takes its own direction. No upstream PRs or comments are authorized by this delivery decision. Respect upstream ownership, conventions, existing tests, and CI coverage. Keep LevitateOS-specific policies and exceptions in this superproject's instructions; do not put them in ginux files, commits, or pull requests. Do not remove upstream tests or weaken validation to impose LevitateOS workspace rules.
- Act on authorized work without repeated permission requests, plans, audits, checklists, decision matrices, or approval gates.
- Bureaucracy is prohibited: do not introduce mandatory paperwork, review ceremonies, coverage targets, or process-only blockers.
- Use the existing owner of a behavior. Prefer a small direct change over a new framework or parallel implementation.
- Preserve unrelated edits. Do not commit secrets, caches, or generated junk.
- No branches. Every repository, the superproject and each submodule, has exactly one branch: `master` or `main`, as `.gitmodules` names it. Do not create, push, check out or rename any other branch, for features, experiments, pull requests, agents or anything else; there are no pull-request or review-branch workflows. Commit and push straight to that branch. Work that is not ready stays uncommitted. The one exception is `tools/ginux`: contributions to the upstream project go as pull requests from branches of our fork, as upstream requires.
- A submodule checkout ahead of the superproject pin is normal while nothing in the superproject needs the newer code. A superproject commit that uses new code from a submodule (xtask calling a new builder command, say) must carry that submodule's pointer: push the submodule first, then include its path in the same commit's pathspec. Local checks cannot catch a missing pointer, because cargo builds the checked-out submodule; CI checks out the pins and fails. Look at CI after pushing.
- Commit early and often. Commit each coherent, working step as it lands rather than batching a session's work into one large commit at the end.

## Validation
- Exception: the ginux repository (`tools/ginux`, including its branches and worktrees) permits unit tests, assertion-based unit doctests, their runners, and unit-test CI coverage. Preserve its existing tests and coverage; add or update focused tests when useful. The following unit-test prohibition and removal requirement do not apply to ginux.
- Unit testing, including assertion-based unit doctests, is prohibited. Do not add, retain, run, require, or replace unit tests with renamed equivalents, snapshots of isolated helpers, or mock-only unit suites.
- Remove unit tests and their runners rather than marking them ignored or reporting them as passing.
- Use compilation, typechecking, and focused integration, CLI, boot, install, or runtime checks when they help establish the changed behavior. Choose checks appropriate to the work; do not run a full suite for every edit or commit.
- Do not build after each change. Rootfs, ISO and kernel builds and VM scenario runs take hours, and the next change invalidates them. A change to a declaration (a package list, a capability, a pin, a kconfig line, a recipe, docs) is done when it is edited and committed: make it, check it by reading and with checks that take seconds, commit it, say it is unbuilt, and take the next request. Start a build or a scenario run only when the user asks for one; it then covers every change since the last build. Never make the user wait on a build to get the next change in.
- No mandatory build, lint, test, or policy-audit pipeline on local commits. Keep those commands available explicitly.
- Report failures and unverified behavior honestly. Fix causes; do not fabricate success, suppress failures, or silently substitute outputs.

## Build and runtime
- The supported execution target is native Linux x86_64. Work on additional CPU architectures is deferred indefinitely and must not be added as a requirement or delivery blocker for Recipe, recab, or the distros unless the user explicitly changes this scope.
- Filesystem iteration uses the `live-tools` product. Named integration/e2e scenarios run independently against existing artifacts; do not reintroduce numbered composition, numeric aliases, or ordered test gates.
- Recipes own source/package facts and acquisition/build/install behavior. Rust/TOML owns orchestration, output locations, and contracts. Normal public recipes run without required defines.
- Build payload through its producers. Keep build commands explicit; scenario commands consume existing artifacts.
- Use `.artifacts` for current outputs and tooling.
- Every VM runs under libvirt: scenarios and `cargo xtask run` boot transient domains on the user session (`qemu:///session`), named `<distro>-<scenario>`, so they can be watched and managed in Cockpit (Virtual Machines). Do not start QEMU directly. Test LevitateOS and AcornOS both this way: `cargo xtask test all levitate` and `cargo xtask test all acorn`, plus `cargo xtask test ab-update <distro>` for each.
- Reuse existing kernel artifacts during ordinary development. Rebuild kernels explicitly with `cargo xtask build kernel <distro>` (or `--all`).
- Keep runtime checks that prevent invalid artifacts or destructive operations. Fail with useful errors.
- For releases, produce reproducible artifacts and validate declared products, provenance, and relevant integration/e2e behavior. Development convenience is not release evidence.

## Code ownership
- Active work lives in `distro/*` (builder, contract, variants), `gui`, `testing/*`, `tools/*`, `tui`, and `xtask`.
- Keep JavaScript/TypeScript ESM-only.
- Run workflows through `cargo xtask` (`cargo xtask --help` lists them; `just` has short aliases). Direct commands are fine.
- When asked to commit everything, commit and push changed submodules first, then the superproject pointers.

<!-- hdae:context-recovery:start -->
## HDAE context recovery

For every task that uses HDAE, and after any context compaction, reset, resume,
or handoff, read `.agents/skills/hdae-development/SKILL.md` completely before
continuing. Then read the skill-directed references and authoritative HDAE
project checkpoint. Do not rely on compacted chat as a substitute.
<!-- hdae:context-recovery:end -->
