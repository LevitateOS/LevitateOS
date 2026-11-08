# Untangle refactor — handoff for whoever continues (Codex or Claude)

Written 2026-11-08 ~03:00 when the Claude token budget ran low. Several Claude subagents may have stopped mid-fix and left **uncommitted edits in the working trees**. This file says how to find that work and finish everything.

## Read first
1. `AGENTS.md`: the repo rules. No branches; no unit tests; commit straight to each repo's one branch; submodules first, then the superproject pointers.
2. `UNTANGLE.md`:
   - `## Scope`: the goal. LevitateOS and AcornOS are true siblings with the same features. The only intended differences are sudo/doas, Plasma/LXQt and LTS/latest kernel.
   - `## Fixes`: G01–G32. Each has its design, "Done when", "Decisions" and the problems it solves.
   - `## Progress`: what landed. Each ticked entry lists its commits, what changed, the checks it passed and what is left.
3. `.artifacts/untangle-repo/redesign/area-*.md`: the detailed design behind each fix (each fix names its area keys, e.g. `TV-10`).
4. `.artifacts/untangle-repo/refactor/IMPLEMENTER.md`: the working rules every implementer followed: quiet hours, pathspec commits, the per-repo branch table, progress notes.

## Status at handoff
- **Landed** (see Progress): G01, G02, G03, G04, G05, G09, G10, G11, G13 (recipe and Rust halves), G14, G17, G18, G19, G20, G22, G23, G24, G27, G31, and the early part of G32 (builder and install-tests docs).
- **In flight when written.** Each may be partly committed, with the rest uncommitted in the working tree:

  | Fix | Paths | Committed so far |
  |---|---|---|
  | G06 → G25 → G26 (one agent, in order) | `testing/install-tests`, `testing/fsdbg`, `tools/recqemu`, `tools/recshuttle`, xtask scenario/run/status/doctor/setup code, builder `run_history.rs`, `pipeline/{io,paths}.rs` | G06 and G25 have commits; G26 none |
  | G07 → G12 → G08 (one agent) | rootfs recipes `fedora-rpm-*`, `alpine-*`, `recipes/sources/**`, the package and `[live]`/`[installed]` sections of `distro.toml`, the contract's capability list, the tool lists in test-scripts, `xtask/src/tasks/base.rs` | G07 and G12 have commits; G08 none |
  | G21 | `xtask/src/tasks/tui/**`, `install-split-pane`, builder `tui_binary.rs`, `install_ux.rs`, `tui_apps.rs` (new, untracked), the contract `install_ux` type | commits exist. **xtask HEAD does not build until `distro/builder/src/tui_apps.rs` is committed** |
  | G29 (Recipe as the package manager on both) | `recipes/install-tools.rhai`, Recipe config/state in the rootfs, docs/content package and update pages | none |
  | G16 (release assembly in Rust; hooks deleted; cmdline as a build fact) | `distro/variants/*/hooks`, `_shared/hooks`, builder `bin/workflows/release_hook.rs`, a new release module, busybox recipe, contract cmdline | none |
  | G15 (one busybox installed initramfs for both) | recinit side landed (`d81ebe9`); remaining: rootfs packages and modules (agent E), release hook (G16), recstrap `plan.rs` and install-tests `preflight.rs` "always initramfs" (see the G15 Progress entry) | recinit side done |

- **G07, G12 and G08** landed (see Progress). G08 left: the xtask wiring for `base lock acorn`, the doctor row for both distros, and removing the stopgap `rpm_base` field (steps in the G08 entry); AcornOS keymaps via `loadkeys` (recstrap and `system-setup.sh`).
- **G28** recipe and docs half landed (see Progress). The Alpine NetworkManager packages are agent E's; `recsetup --copy-network` is a follow-up.
- **Not started:** G30 (desktop as a product layer), the rest of G32 (root `README.md`, `architecture.md`, `xtask/README.md`, docs/content architecture pages, tui docs; once the code has settled). TO-14 (each tool declares the external commands it runs, and the builder checks them; format proposed in `merged-progress-G09.md`).
- **Leftovers named in Progress entries.** Each "Left:" line names the fix it belongs to; sweep them after the fixes. For example:
  - drop the `RUST_TARGET` define in `distro/builder/src/recipe/tools.rs`;
  - delete `PrepareQemuTools`, `recipe/qemu.rs` and `qemu-tools.rhai` once G25's setup change is in;
  - delete the recuki binary after its docs page `13-recuki.ts` goes;
  - commit `docs/content/src/distros.ts` after a fresh `cargo xtask gen`.

## How to resume
1. **Inventory the half-done work:** in the superproject and every submodule (except `tools/recipe`, `tools/ginux`), run `git status --short` and `git diff --cached --stat`. Map each changed file to its fix with the path table above. `git log --oneline --since=yesterday --grep='^Gxx:'` in each repo shows what each fix already committed.
2. **Finish one fix at a time, in this order:** G21 (it unblocks xtask's build), G06/G25/G26, G07/G12/G08, G16, G15, G29, G28, G30, TO-14, G32, then the leftovers. For each fix:
   - read its `### Gxx` section and its area designs;
   - compare the uncommitted diff with "Done when";
   - complete or fix it;
   - `CARGO_BUILD_JOBS=2 cargo check --workspace --all-targets` and `cargo xtask check`;
   - commit by pathspec per repo, then the superproject pointers;
   - add its Progress entry to `UNTANGLE.md` (style of the existing ones: commits, what changed, verified, behaviour changes, bugs fixed, what's left).
   - If a half-done diff is wrong or unclear, `git stash` it under a message naming the fix and redo the fix from its design. Never discard work silently.
3. **Integration check after each fix:** export every repo's HEAD into a scratch tree (`git archive HEAD` per repo) and run `cargo check --workspace --all-targets` there with a separate `CARGO_TARGET_DIR`. Committed HEADs must compile together, not only the dirty tree.
4. **Daytime batch only (after 10:00; build fans are too loud at night).** This is listed at the end of `## Progress` ("Deferred to a daytime batch") and in the G11 entry:
   - force-rebuild both rootfs sources, then base-rootfs, live-boot, live-tools and installed-boot for both distros, through `/tmp/iso-run-fixed.sh target/debug/distro-builder release build iso <product> <distro>` (as root, with the Ginux/Recipe env);
   - check the `REC_*` lines in each image's `/etc/os-release`;
   - rebuild and republish both kernels under the converged kconfig (the memory note on kernel publishing has the steps);
   - run every scenario on both distros (`cargo xtask test all <distro>`, `test ab-update <distro>`) once G26 has removed preflight's privileged Recipe kernel check;
   - compare against `## Baseline`. Every difference must be a recorded behaviour change or bug fix.
5. **Re-audit:** fresh reviewers, using `.artifacts/untangle-repo/BRIEF.md`'s checklist on the changed code. Add what they find under a fix, apply, and repeat until nothing worth fixing is left. Then the final report: fixes landed, special cases before (613) and after, bugs fixed, what was left as is.

## Open items for the user
- **Recipe on the installed system (G29):** `/usr/bin/recipe` ships on both, but `recipe install` cannot work on a target yet. It needs a Ginux launcher and a root image (the compiler image is 1.27 GB), and it publishes under `$PWD/.artifacts` instead of into the running system. Closing that is Recipe work (`tools/recipe`, out of scope). Also a one-line `tools/recipe` change, `libc::syscall(SYS_renameat2…)` in `publication.rs:292`, would let the musl link shim in `install-tools.rhai` go.
- **Bug being fixed at handoff:** `/home` was inside the A/B slot, so updates hid user files. The fix makes `/home` a symlink to `var/home` on STATE (see `progress-home-fix.md`, or the Progress entry once merged).
- Recipe's e2e tests (`tools/recipe/tests/e2e/preseed_recipes.rs`, out of scope) still expect the old Alpine preseed outputs and `sources/alpine.rhai` (G08). They need updating by whoever owns Recipe.
- Delete two stray remote branches pushed by mistake (both contain only commits already on the right branch):
  - `git -C . push origin --delete main` (superproject; its branch is `master`);
  - `git -C distro/builder push origin --delete master` (builder; its branch is `main`).
- Archive the GitHub repos `LevitateOS/hardware-compat`, `rootfs-tests` and `leviso-elf` (decision D17).
- Recipe's own release workflow (`tools/recipe`, out of scope) now needs a `version-bump` input from its caller after G27's shared workflow change.
- About 15 hardware kconfig options that never took effect are commented "NOT BUILT YET" (G11's progress note); enabling them is a product call.
- The user-visible defaults D1–D16 in `## Fixes` → "Decisions for the user" were applied. The user wanted room to change their mind, so list what changed for them.

## Prompt to give the next agent
> You are continuing the LevitateOS untangle refactor in `/home/vince/Projects/LevitateOS`. Read `UNTANGLE-HANDOFF.md` first and follow its "How to resume" section exactly. Then read `AGENTS.md`, the `## Fixes` and `## Progress` sections of `UNTANGLE.md`, and `.artifacts/untangle-repo/refactor/IMPLEMENTER.md`. Out of scope: don't touch `tools/recipe` or `tools/ginux`. No branches: push only to each repo's own branch, which is listed in IMPLEMENTER.md. No unit tests. Commit by pathspec, and leave other people's staged changes alone. Before 10:00, don't build ISOs or boot VMs. Start with the inventory (step 1), then finish the in-flight fixes in the given order, one commit set and one Progress entry per fix. Keep the changes concrete and minimal: no new frameworks and no repo merges. Stop and report when a fix's design turns out to be wrong in the code, rather than forcing it.
