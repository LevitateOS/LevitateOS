# Untangle log

## Scope

- **Target:** the 12 non-recipe `rec*` tools, taken as one system: recab, recart, recchroot, recfstab, recguard, recinit, reciso, recpart, recqemu, recshuttle, recstrap, recuki.
  - `tools/recipe` is out of scope (it has its own `tools/recipe/UNTANGLE.md`). `tools/ginux` is out of scope.
  - Callers outside `tools/` (leviso, AcornOS, distro-builder, distro-spec, testing/*, tui, xtask) are not refactor targets themselves, but links between them and the rec* tools count as findings.
- **Additions chosen:** Comments (remove comments that only restate the code) and Tests (remove tests that wouldn't catch a real bug; fix the ones that fail without one).
  - AGENTS.md already prohibits unit tests outside ginux, so "Tests" applies to the integration/CLI/e2e tests in `tools/*/tests/`.
- **Repo rules that bind the refactor:** commit on the checked-out branch (no new branches), submodules first then superproject pointers, no unit tests.
- **Tooling for later phases:** `.artifacts/untangle-rec/scripts/` (gitignored)
  - `skeleton.py <root> <out> tool...` writes body-stripped Rust to `.artifacts/untangle-rec/skeleton/` (~5k lines for ~15k source lines).
  - `depgraph.sh <root>` writes crate deps, reverse deps, binary-name references, intra-crate imports.
  - `history.sh <root> [since]` writes churn, intra-tool co-change, cross-tool pointer co-change.

## Baseline

Taken 2026-11-06.

- **Superproject:** `master` @ `6aa48b82568db7adcfadecbf9519e271dd6f1284` (unrelated dirty files: `docs/ginux-contributions/daybreakblue-handoff.md`, `tools/recipe` pointer, `*.err/*.out` scratch).
- **Submodules** (all on `codex/streamline-development`, all clean, all equal to the superproject pin):

  | tool | SHA |
  |---|---|
  | recab | d1bd863 |
  | recchroot | c19da09 |
  | recfstab | 98926aa |
  | recguard | e93c40d |
  | recinit | 200f521 |
  | reciso | e4e5a95 |
  | recpart | caab64b |
  | recqemu | 83815ed |
  | recshuttle | a84707f |
  | recstrap | 275d700 |
  | recuki | 062e189 |
  | recart | in-tree (superproject SHA above) |

- **Build:** `cargo build -p recab -p recart -p recchroot -p recfstab -p reciso -p recpart -p recqemu -p recshuttle -p recstrap -p recuki` → success. `recinit` is not a root workspace member; `cd tools/recinit && cargo build` → success.
- **Tests:** `cargo test -p <tool>` (only these five have tests):

  | tool | passed | failed | ignored |
  |---|---|---|---|
  | recab (`tests/e2e_sdc.rs`) | 0 | 0 | 2 (destructive, `/dev/sdc`) |
  | recchroot (`tests/integration.rs`) | 15 | 0 | 2 (`test_chroot_echo_command`, `test_command_in_chroot`) |
  | recfstab (`tests/integration.rs`) | 20 | 0 | 0 |
  | recpart (`tests/integration_cli.rs`, `tests/e2e_real_disk.rs`) | 3 | 0 | 3 (real throwaway disk) |
  | recstrap (`tests/integration.rs`) | 20 | 0 | 0 |

  recart, recinit, reciso, recqemu, recshuttle, recuki have no tests. recguard has no code.
- **Behavior snapshots** in `.artifacts/untangle-rec/baseline/`:
  - `help.txt`: `--help` of every binary.
  - `cli-samples.txt`: `recfstab /` (real fstab of this host), `recpart list-disks --json`, `recab status` (exit 21, RECAB_ERR_STATE), `recstrap /tmp` as non-root (cheat-guard banner, exit 8), `recchroot /tmp true` as non-root (exit 8, protected path), `recart ls`.
  - `tests.txt`, `build-ws.txt`, `build-recinit.txt`.

## Map

Data: `.artifacts/untangle-rec/depgraph.txt`, `history.txt`, `skeleton/`.

**Areas** (~15k Rust lines, plus recguard's spec):

| Area | Tools | Purpose | Main callers |
|---|---|---|---|
| A. Install trio | recstrap (1.9k), recchroot (0.8k), recfstab (1.3k) | Arch-style extract / chroot / fstab CLIs run from the live ISO | distro-builder recipes (`recipes/rec*.rhai`, `pipeline/live_tools.rs`, `recipe/tools.rs`), distro-variants products/scenarios TOML, live-overlay docs scripts, distro-spec error/paths |
| B. Disk + slots | recpart (2.4k), recab (1.9k), recguard (spec only) | Partition plan/apply with JSON contract; A/B slot state machine; mutability policy | tui `install/disk-plan`, tui `runtime/recab` + `runtime/postinstall-tools` (spawn binaries, parse JSON) |
| C. Boot images | recinit (2.1k), recuki (0.3k), reciso (0.8k) | initramfs builder, UKI builder, ISO builder | leviso + AcornOS `src/artifact/{initramfs,uki,iso}.rs` as library deps; reciso depends on recuki |
| D. VM + remote | recqemu (1.9k lib), recshuttle (0.6k lib) | QEMU command builder + serial console; SSH transport | leviso `src/qemu.rs`, testing/install-tests `src/qemu/*` (re-exports + local builder with "anti-cheat") |
| E. Artifact store | recart (1.9k, in-tree, axum web UI) | `.artifacts` store management + web UI | nobody (CLI only) |

**Hotspots (1 year, commits per tool):** recstrap 39, recfstab 31, recchroot 26, recab 16, recinit 14, recpart 14, recqemu 11. Within tools: `recstrap/src/{main,helpers,constants,error,rootfs}.rs` all co-change (helpers↔main 7×); `recab/src/lib.rs` (1185 lines, one file) ↔ `main.rs` 6×; `recinit/src/install.rs`↔`systemd.rs` 5×, `modules.rs`↔`tiny.rs` 4×.

**Cross-tool co-change:** superproject pointer bumps move recchroot+recfstab+recstrap together 13–15× (the trio changes as one unit across three repos, three CI configs, three Cargo.locks). recinit+reciso+recuki move together 7–8×.

**Dependency graph, worst parts:**
- recart (a tool) depends on `leviso` and `acornos` (whole distro builders) and `distro-builder`: stable tool → volatile products. `server.rs:936` calls `leviso::rebuild::cache_*_hash`.
- recchroot and recstrap depend on `testing/cheat-test` (a test crate) at runtime; recstrap's Cargo.toml comment says it re-implements `guarded_ensure!` instead of using cheat-guard.
- recinit, reciso, recpart, recstrap, recchroot all depend on `distro-spec` (product facts) — generic tools importing distro facts.
- reciso depends on `distro-builder` (orchestrator) as well as recuki.
- recinit is consumed by leviso/AcornOS but is not a root workspace member.
- recqemu is partly re-wrapped by `testing/install-tests/src/qemu/` (own `QemuBuilder`, `find_ovmf`, `create_disk`) and `leviso/src/qemu.rs`.
- Binary names and flags are referenced by string from many places (distro-builder, distro-variants TOML, distro-spec errors, docs, tui) with no shared contract.

**Leverage looks highest at:** the install trio (most churn, three repos moving in lockstep), recart's dependency direction, distro-spec coupling of generic tools, and recqemu vs its re-wrappers.

## Problems

**Counts:** 281 hidden · 66 needless · 136 surplus · **183 special cases** (the number the refactor works to bring down) · plus 18 grouped comment entries (several hundred individual restating comments) and 44 test entries.

Sources: five area audits (A–E), my own cross-area pass, and two fresh completeness passes over the highest-churn code (recstrap/recfstab/recchroot; recab/recinit/recqemu). About ten headline claims were checked against the code and held. Some entries appear in two areas where a link crosses them (e.g. recshuttle's layout vs recpart/recab); the redesign should treat those as one problem.

**Completeness:** the fresh passes added ~55 entries, all inside existing clusters (wrong-code error mappings, duplicated branches inside one function, several codecs for one format, harness strings vs guest messages, struct-literal coupling). They found no new cluster, and a targeted sweep for their new pattern kinds elsewhere found nothing more. Another pass would likely still add line-level detail, but not a new pattern for the redesign.

### Clusters (patterns that recur across areas; the redesign should start here)

1. **Cross-tool contracts with no owner.** Facts that several tools and their drivers must agree on are free strings in each: slot/partition labels (recpart `ROOT_A`/`STATE` vs recab `system-a` vs recshuttle `system-a`/`var` vs docs), boot-entry naming and loader.conf defaults (recab, recshuttle, distro-spec disagree), the rec-tool CLI syntax (scripted as strings by recpart handoff, recshuttle, recqemu, live-docs.sh ×2, distro-builder, AcornOS), error codes/exit codes (differently numbered per tool, copied into CI greps, READMEs, CLAUDE.md, docs, TUIs), the serial protocol markers (recqemu vs five guest scripts), the live-boot path contract (`/run/live-media`, `/run/live-rootfs.erofs`, `.live-payload-role`, boot-injection fw_cfg), JSON schemas (recpart/recab Rust types vs hand-written schemas vs TS validators; recart server vs app.js).
2. **Generic tools depend on distro policy, and distro-spec holds tool internals.** recinit, reciso, recstrap, recchroot, recpart import distro-spec; recart imports whole distro builders; reciso imports distro-builder. Conversely distro-spec holds recstrap-only values and dead tool-shaped helpers (`ToolError`, `CHROOT_BIND_MOUNTS`, udev lists). Generic tools also embed distro vocabulary (recqemu `levitateos login:` and leviso lock paths, recinit `ID=levitateos`, reciso RHEL `SELINUX_DISABLE`, recstrap `/bin/bash -G wheel`). This is the mechanism behind the 13–15× joint pointer bumps.
3. **Parallel implementations of one job.** recshuttle is a second installer beside recpart (own layout, labels, entries); four QEMU builders (recqemu, distro-builder, xtask, leviso); three live init templates; three busybox fetch paths; three recart ingest paths; fsdbg's hand-copied "must match" lists; disk-plan TUI re-implements recpart inventory and preflight; two near-identical recab TUIs.
4. **Deprecated consumers keep library surfaces alive.** recinit/reciso/recuki library APIs, recart's leviso/acornos deps, and recqemu's Console/Executor surface are used only by deprecated leviso/AcornOS or the disabled install-tests suite, while the active pipeline (`build-release.sh`, xtask scenarios) uses CLIs or its own copies. Two surfaces with different knobs drift apart.
5. **Prose copies drift from code.** Error tables, flags, check counts, protected-path lists, licences, behaviour (resolv.conf "copied", `/proc/mounts`) are restated in README/CLAUDE.md/REQUIREMENTS/docs site and already disagree; recguard is spec-only.
6. **Silent failure as policy.** Swallowed errors and silent skips change what the tools produce without saying so: recstrap SSH-key regen, recfstab blkid/swap, recinit missing files/modules, recart ingest exit 0, recab audit/loader.conf, recpart rollback unmount.
7. **Checks that can't catch bugs.** clap-only tests, root-gated tests that skip silently, ignored destructive e2e that needn't be destructive, fsdbg checks that can't fail, disabled install-tests `Step` suite, cached scenario passes, CI jobs broken by later guards.
8. **Per-repo packaging around a monorepo build.** Submodule CI can't build standalone (superproject path deps), stale per-tool Cargo.lock, release version bumps vs recipe pins, inconsistent metadata, 11 identical AGENTS.md, recinit outside the workspace.

**Probable live bugs surfaced by the audit (recorded, not fixed):** `{{LIVE_OVERLAY_PATH}}` never substituted in leviso/AcornOS templates; recpart-installed systems never match recab's cmdline slot detection; recfstab writes host-absolute swap-file paths; recstrap's own CI e2e is rejected by the host-root-fs guard; recart File Tree requests `out/leviso` instead of `out/levitate`; recab PartialUpdate after a backend failure exits 20 not 22; AcornOS live UKIs get console args twice.

### Cross-area (all rec* tools)

- [hidden] `tools/{recab,recchroot,recfstab,recguard,recinit,reciso,recpart,recqemu,recshuttle,recstrap,recuki}/AGENTS.md`: 11 byte-identical copies (md5 `3b9a9766…`) of the development rules; the root `AGENTS.md` already says it applies recursively. A rule change must be repeated 11 times, and nothing says so.
- [hidden] `tools/{recab,recchroot,recfstab,reciso,recqemu,recstrap,recuki}/CLAUDE.md`: seven per-tool agent guides with different content, alongside AGENTS.md; they restate README/code facts (flags, error codes, status) that change with the code but are not checked against it.
- [hidden] `tools/{recchroot,recfstab,recstrap}/.github/workflows/ci.yml`: three near-identical workflows (recchroot vs recfstab differ only in the smoke script and `binary-name`). Workflow churn: recfstab 14, recstrap 10, recchroot 7 commits/year, the top churn files in their repos.
- [needless] `tools/recstrap/.github/workflows/ci.yml:22,30` (same in recchroot, recfstab): standalone tool repos call `LevitateOS/LevitateOS/.github/workflows/{detect-changes,rust-ci}.yml@master`; a stable tool's CI changes behavior whenever the superproject's master workflow changes.
- [hidden] `.github/workflows/rust-ci.yml:39,58` checks out only the tool repo and runs `cargo build`, but recstrap/recchroot/recfstab… declare `path = "../../distro-spec"` and `../../testing/cheat-test` deps that exist only inside the superproject. Standalone CI and the superproject layout must change together; nothing says so.
- [hidden] `tools/recstrap/.github/workflows/ci.yml:35` greps `E008`, `tools/recchroot/.github/workflows/ci.yml:35` and `tools/recfstab/.github/workflows/ci.yml:36` grep `E001` for `/nonexistent`: CI encodes error-code numbering owned by `distro-spec/src/shared/error.rs` and each tool's error enum.
- [surplus] `tools/{recchroot,recfstab,recstrap}/Cargo.lock`: lockfiles in three workspace members; the root workspace's `Cargo.lock` governs every real build, so these only churn (5–7 commits/year each) and can disagree silently.
- [hidden] Package metadata is per-tool and inconsistent: `rust-version = "1.93"` only in recchroot/recfstab/reciso/recqemu/recstrap/recuki/recart; `license = "MIT"` in `tools/recfstab/Cargo.toml` vs `MIT OR Apache-2.0` everywhere else (recfstab ships `LICENSE`, others `LICENSE-APACHE`+`LICENSE-MIT`); `authors`/`repository` present on some. Root `Cargo.toml:31` defines `[workspace.package]` but no rec* tool inherits from it, so MSRV/edition/license changes must be made per tool.
- [special] Versions: recchroot 0.1.4, recfstab 0.1.5, recstrap 0.1.5 (bumped by their CI `workflow_dispatch: version_bump` release flow), everything else 0.1.0 — release machinery exists for three tools only.
- [special] `.gitmodules`: recstrap, recfstab, recchroot, recshuttle track `branch = master`; recinit, recuki, reciso, recqemu, recab, recpart, recguard track `main` (and all are currently checked out on `codex/streamline-development`). `tools/sync-submodules-default-branches.sh` exists to cope with this.
- [special] `Cargo.toml:3` workspace members list every rec* tool except `tools/recinit`, which leviso and AcornOS still depend on by path; recinit builds in its own target dir and is skipped by `cargo build/check --workspace`.
- [hidden] The set of required rec* host tools is listed in `distro-contract/src/build_host.rs:18-29`, `distro-variants/levitate/build-host/build-host.toml:5-16`, and `distro-variants/acorn/build-host/build-host.toml` — three copies of one list.
- [hidden] `docs/content/src/content/05-rec-tooling/*.ts` (13 pages) hand-copy each tool's flags, exit codes and behavior. Already drifted: `12-recstrap.ts:60-70` lacks `--allow-non-mountpoint`, `--allow-nonempty`, `--allow-host-root-fs` that `recstrap --help` shows (and describes `-f` differently).
- [hidden] Error conventions differ per tool with no shared owner: recstrap/recchroot/recfstab use `E0xx` + numeric exit codes (partly from `distro-spec/src/shared/error.rs`), recab uses `[RECAB_ERR_*]` with exit 21, recpart uses JSON error objects; consumers (CI greps, tui runners, docs) each encode one convention.

### A. Install trio (recstrap, recchroot, recfstab)

- [hidden] Each recstrap validation is spread across five places by layer: check + cheat text in `tools/recstrap/src/main.rs:145-433`, predicate in `helpers.rs`, code/constructor/doc in `error.rs:11-47,86-240`, value in `distro-spec/src/shared/rootfs.rs:77-111`, doc tables in `tools/recstrap/CLAUDE.md:41-61` and `README.md:45-89`. This is why main/helpers/constants/error/rootfs co-change.
- [hidden] Four hand-kept copies of the E001–E017 table: `tools/recstrap/src/main.rs:30-50`, `error.rs:11-47`, `CLAUDE.md:41-61`, `README.md:67-89`.
- [hidden] Four copies of recchroot error numbering: `tools/recchroot/src/main.rs:12-23` (doc table), `:56-73` (variant docs), `:75-101` (two parallel matches for code string and exit int), `README.md:42-53`.
- [hidden] `tools/recstrap/src/error.rs:72-74` exit code = enum discriminant; `:50-69` spells `E0NN` separately.
- [hidden] Same failure, different numbers per tool: not-root is E008/8 in `tools/recstrap/src/error.rs:28` but E007/7 in `tools/recchroot/src/main.rs:70`; protected path E010 vs E008. Drivers (recshuttle, install-tests) must know each mapping.
- [hidden] `tools/recfstab/README.md:59-69` documents exit codes 1–7, but `tools/recfstab/src/main.rs:52` always exits 1; "7 blkid not found" doesn't exist in `error.rs`.
- [hidden] `tools/recstrap/src/main.rs:236-252` reports the host-root-fs guard as `ProtectedPath` (E010) — a different condition with a different override. `tests/integration.rs:466-482` (`test_mnt_is_allowed`) fails as root whenever `/mnt` is on `/`.
- [hidden] `tools/recstrap/.github/workflows/ci.yml:72-75` runs `recstrap --force --rootfs test.erofs /tmp/recstrap-test-target` without `--allow-host-root-fs`; the guard at `main.rs:235` rejects it on a runner whose `/tmp` is on `/` (verified). The CI copy of the CLI contract wasn't updated.
- [hidden] `.github/workflows/rust-release.yml:64-88` bumps each tool's `Cargo.toml` version; `distro-builder/recipes/{recstrap,recchroot,recfstab}.rhai:9` pin `0.1.5`/`0.1.4`/`0.1.5` and `_verify` (`:24-29`) requires an exact `--version` match (verified). A release bump breaks live-tools until the recipe is edited.
- [hidden] `distro-builder/recipes/rec*.rhai:25-26` depend on clap's `#[command(version)]` output being exactly `"<name> <version>"` (`tools/recstrap/src/main.rs:76`, `recchroot/src/main.rs:39`, `recfstab/src/main.rs:10`).
- [hidden] `tools/recstrap/src/main.rs:213` writes `.recstrap_write_test`; `helpers.rs:31` separately ignores that literal in `is_dir_empty`; works only because the write test runs first.
- [hidden] `tools/recstrap/src/rootfs.rs:34` hard-codes offset `1024` though `distro-spec/src/shared/rootfs.rs:111` exports `EROFS_MAGIC_OFFSET` (`constants.rs:11` notes it).
- [hidden] `tools/recstrap/src/helpers.rs:148` hard-codes SSH host-key types/bits; `distro-spec/src/shared/auth/ssh.rs:62` `HOST_KEY_CONFIGS` owns them; `helpers.rs:190` repeats them in a message.
- [hidden] `tools/recstrap/src/helpers.rs:268` writes `useradd -m -s /bin/bash -G wheel`; shell and groups are per-distro in `distro-spec/src/acorn/paths.rs:149` (`/bin/ash`), `levitate/paths.rs:92`, `shared/paths.rs:67-72` (`DEFAULT_USER_GROUPS`). Wrong on Acorn.
- [hidden] `tools/recchroot/src/main.rs:428-429` defaults to `/bin/bash`, `tools/recqemu/src/serial/chroot.rs:24` hard-codes `/bin/bash -c`; both ignore per-distro `DEFAULT_SHELL`.
- [hidden] `tools/recstrap/src/main.rs:519` and `helpers.rs:225` rely on an invariant stated only in `distro-spec/src/shared/auth/mod.rs:130-134` (recstrap copies only the EROFS lower layer, never the live overlay).
- [hidden] `tools/recstrap/CLAUDE.md:106` says wheel uses password auth; `distro-spec/src/shared/auth/mod.rs:55` says NOPASSWD sudo.
- [hidden] `distro-spec/src/shared/rootfs.rs:78` first search path `/run/live-rootfs.erofs` is created by `tools/recinit/templates/init_tiny.template:279` as a symlink to `$ROOTFS_SRC`, which can be a partition device (`:203-210`); recstrap assumes a regular `.erofs` file (`main.rs:364` `is_file()`, `:380` extension check on canonical path, `rootfs.rs:176` `-o ro,loop`).
- [hidden] `leviso/profile/init_tiny.template:252`, `AcornOS/profile/init_tiny.template:263` keep the ISO at `/media/cdrom` to match `distro-spec/src/shared/rootfs.rs:83`; `/run/archiso/bootmnt` (`:81`) matches no producer.
- [hidden] `distro-spec/src/shared/rootfs.rs:96` `MIN_REQUIRED_BYTES` fixed at 2 GiB regardless of image size; copied as "2GB" in `tools/recstrap/README.md:57,93`.
- [hidden] `distro-spec/src/shared/rootfs.rs:90` `ESSENTIAL_DIRS` copied into `tools/recstrap/.github/workflows/ci.yml:54,78-83`; a subset in `testing/install-tests/src/steps/phase3_base.rs:170`.
- [hidden] `tools/recstrap/README.md:65`, `tools/recchroot/README.md:59`, `tools/recstrap/CLAUDE.md:65` hand-copy a protected-path list already drifted from `distro-spec/src/shared/paths.rs:83-106`.
- [hidden] `legitimate_change` texts point at "the protected path list in src/main.rs" (`tools/recstrap/tests/integration.rs:166,195,426`, `tools/recchroot/tests/integration.rs:188,215`); the list lives in `distro-spec/src/shared/paths.rs:83`.
- [hidden] `tools/recchroot/src/main.rs:187-195` (`BIND_MOUNTS`, `OPTIONAL_MOUNTS`) and `distro-spec/src/shared/chroot.rs:13-45` (`CHROOT_BIND_MOUNTS`) both define the chroot mount set and differ (`/dev/pts`, efivars required).
- [hidden] `tools/recchroot/CLAUDE.md:45`, `README.md:31` say resolv.conf is copied; `main.rs:247-284` bind-mounts it.
- [hidden] `tools/recfstab/README.md:47` and `tests/integration.rs:17-20` say it reads `/proc/mounts`; `src/mount.rs:16-19` runs `findmnt`.
- [hidden] `tools/recfstab/src/swap.rs:150-158` returns the host-absolute swap-file path (e.g. `/mnt/swapfile`) as the fstab source while `:163-190` re-roots the target — the generated line points at a host path (verified).
- [hidden] `tools/recfstab/src/swap.rs:130-145` `is_swap_under_root` duplicates `filter.rs:67-73` `is_under_root`; `swap.rs:163-190` `get_swap_target` duplicates `fstab.rs:30-50` `make_fstab_target`; trailing-slash normalization at `lib.rs:77-82`, `swap.rs:137`, `swap.rs:166-170`.
- [hidden] `tools/recfstab/src/device.rs:75-79` hard-codes the four prefixes `IdType` (`:6-17`) enumerates; `lookup_device_id` (`:120`) assumes blkid tag == fstab prefix.
- [hidden] `tools/recfstab/src/lib.rs:85` converts `IdType` to `&str` (`blkid_tag()`); downstream (`device.rs:68`, `swap.rs:150,193`) take strings — valid values are a convention.
- [hidden] `tools/recchroot/src/main.rs:247` `bind_host_resolv_conf` returns placeholder files via an out-parameter; caller must remove them only after `cleanup_mounts` (`:454-457`); order unstated.
- [hidden] `tools/recstrap/src/rootfs.rs:166-169` cleanup correctness depends on declaration order (`_interrupts` before `guard`).
- [hidden] Validation order is a contract: recstrap checks root first (`main.rs:145`; CI `ci.yml:36` and `tests/integration.rs:112-121,403-412` grep E008 for a nonexistent path); recchroot checks protected path before root (`main.rs:312-321`; `tests/integration.rs:192` "E008 should come before E007 now").
- [hidden] `tools/recchroot/src/main.rs:461` passes the child's exit code through and maps signals to 1, colliding with recchroot's own codes 1–8; `tests/integration.rs:240-241` claims exit codes identify error types.
- [hidden] `tools/recchroot/src/main.rs:219` hard-codes `"E005"` in a warning; `ErrorCode::UnmountFailed` never constructed.
- [hidden] Signal handling implemented twice with different sets: `tools/recstrap/src/rootfs.rs:98-122,237-248` (libc, INT+TERM) vs `tools/recchroot/src/main.rs:327-331,443-450` (nix, INT+TERM+QUIT).
- [hidden] Mount/unmount two ways: recstrap shells out to `mount`/`umount` (`rootfs.rs:79,175`); recchroot uses nix `mount`/`umount2(MNT_DETACH)` (`main.rs:217,343`).
- [hidden] Path whitespace trimming in `tools/recchroot/src/main.rs:293-296` and `tools/recfstab/src/lib.rs:57-60`, not in recstrap.
- [hidden] `tools/recstrap/src/validation.rs:17-55` `guarded_ensure!` duplicates the banner format of `testing/cheat-guard/src/lib.rs:174-215` `cheat_check!` (admitted in `tools/recstrap/Cargo.toml:24-27`).
- [hidden] Next-steps install workflow duplicated as free text: `tools/recstrap/src/main.rs:506-528`, `tools/recpart/src/handoff.rs:14-18`, `tools/recshuttle/src/lib.rs:377-406,507-550`, `distro-variants/acorn/ring2/overlays/live/etc/profile.d/live-docs.sh:43-46`, `AcornOS/profile/live-overlay/etc/profile.d/live-docs.sh:43-46` (byte-identical to the previous), `distro-builder/src/artifact/live_overlay.rs:376`, `distro-builder/src/pipeline/live_tools.rs:473`, `AcornOS/src/component/custom/live.rs:30-32`.
- [hidden] The list recstrap/recfstab/recchroot is restated in ≥15 places: `Cargo.toml:13-15`, `distro-spec/src/shared/components/users.rs:33-39,48`, `distro-builder/src/recipe/tools.rs:13,190`, `distro-variants/{acorn,levitate}/ring2/products.toml` (acorn `:101-113`, levitate `:168-178`), `distro-variants/{acorn,levitate}/scenarios/scenarios.toml`, `distro-variants/*/build-host/build-host.toml:14-16`, `distro-contract/src/build_host.rs:27-29`, `testing/install-tests/src/distro/levitate.rs:233-235`, `distro/acorn.rs:131-133`, `xtask/src/tasks/tooling/hooks.rs:98-104`, `AcornOS/src/component/custom/live.rs:102-176`.
- [hidden] Supported Rust targets + default restated in `distro-builder/src/recipe/tools.rs:197-203` and each `distro-builder/recipes/rec{strap,chroot,fstab}.rhai:10,17-21,67-69`; `distro-variants/acorn/ring2/products.toml:103,108,113` repeats `target = musl` per tool.
- [hidden] `distro-builder/recipes/recstrap.rhai`, `recchroot.rhai`, `recfstab.rhai` are byte-identical except name and version.
- [hidden] `distro-builder/src/recipe/tools.rs:110-113,139-141` depend on the tools' `Cargo.toml` shape (strip `dev-dependencies`; copy `README.md` because `readme =` is declared).
- [hidden] `testing/install-tests/src/steps/phase3_base.rs:145-147` passes `--force` "because ext4 contains lost+found", but `tools/recstrap/src/helpers.rs:31` already ignores lost+found.
- [hidden] Check counts disagree: `tools/recstrap/CLAUDE.md:75-85` 9 phases, `main.rs:141-497` 8 phases, `README.md:7,27,45-61` "14", "15" and a reordered 15-row table, `main.rs:450` prints `14`.
- [hidden] `tools/recstrap/README.md:104`, `tools/recchroot/README.md` say MIT; their `Cargo.toml:7` say `MIT OR Apache-2.0`.
- [hidden] `tools/recstrap/CLAUDE.md:27` claims `--release` has LTO+strip — true only inside the workspace (root `Cargo.toml:34-36`).
- [hidden] `distro-spec/src/shared/error.rs:3-5` says recfstab uses the shared error framework; recfstab doesn't depend on distro-spec and redefines everything in `tools/recfstab/src/error.rs:21-155`.
- [needless] `distro-spec/src/shared/rootfs.rs:69-111` holds recstrap-only volatile values (`ROOTFS_SEARCH_PATHS`, `ESSENTIAL_DIRS`, `MIN_REQUIRED_BYTES`, `EROFS_MAGIC`) in the stable spec crate; `distro-spec/src/shared/system.rs:53` `is_mount_point` is recstrap-only. A recstrap tweak needs a distro-spec change + pointer bump — explains the 13–15 joint bumps.
- [needless] `tools/recstrap/Cargo.toml:21`, `tools/recchroot/Cargo.toml:21` pull the whole `distro-spec` (kernel, QEMU, UKI, auth) for `is_root`, `is_protected_path`, a trait and a few constants.
- [needless] `distro-spec/src/shared/error.rs:44-50,91-103` (`ToolErrorCode`, `impl_error_code_display!`) is shallow: each tool still re-implements `RecError`, constructor, `Display`, `Error` (`tools/recstrap/src/error.rs:80-250`, `tools/recchroot/src/main.rs:105-180`).
- [needless] `tools/recstrap/src/validation.rs:58-91` prints developer-facing "CHEAT VECTORS"/"USER CONSEQUENCE" banners to end users on every failed check (`main.rs:145-433`; baseline `cli-samples.txt`); internal notes are user-visible output.
- [needless] `tools/recfstab/src/lib.rs:122-131`, `swap.rs:202-208` library `run()` and `print_swap_entries` print to stdout directly.
- [needless] `tools/recfstab/src/lib.rs:31-46` exports every internal module/helper as public API; only consumer is `main.rs`.
- [needless] `tools/recstrap/src/helpers.rs` grab-bag: rootfs discovery (`:15`), fs predicates (`:26-77`), module loading (`:81-104`), SSH regen (`:107-194`), interactive user setup (`:205-298`).
- [needless] `tools/recstrap/src/main.rs:119-126` `is_on_host_root_fs` lives in main.rs while sibling predicates live in helpers.rs or distro-spec.
- [needless] `tools/recstrap/src/helpers.rs:128`, `rootfs.rs:161` thread `quiet: bool` through library functions to decide printing.
- [needless] `tools/recstrap/Cargo.lock` lacks `distro-spec`, `smallvec`, `leviso-cheat-test`; `tools/recchroot/Cargo.lock` lacks `distro-spec` — stale. Builds use the root lock (`distro-builder/src/recipe/tools.rs:41-42`), and `rust-ci.yml`'s MSRV job deletes it.
- [needless] `distro-variants/*/build-host/build-host.toml:14-16`, `distro-contract/src/build_host.rs:27-29` list the trio as required build-host tools, but they are live-ISO payload built from workspace source (`build_host.rs:16,19` notes question this).
- [surplus] `tools/recstrap/src/constants.rs:1-11` only re-exports four distro-spec constants.
- [surplus] `tools/recstrap/src/helpers.rs:11-12` re-exports `is_mount_point`, `is_protected_path`, `is_root` unchanged.
- [surplus] `distro-builder/src/pipeline/live_tools.rs:214-221` `build_workspace_binary_artifact` forwards to `recipe::tools::build_tool_program`.
- [surplus] `distro-spec/src/shared/error.rs:52-85` `ToolError<C>` unused anywhere.
- [surplus] `distro-spec/src/shared/chroot.rs:13-87` (`CHROOT_BIND_MOUNTS`, `BindMount`, ordering helpers) unused outside distro-spec.
- [surplus] `distro-spec/src/shared/rootfs.rs:111` `EROFS_MAGIC_OFFSET` unused.
- [surplus] `tools/recstrap/src/error.rs:24-26,151-157` `ToolNotInstalled`/E007 dead (`#[allow(dead_code)]`) but documented in four tables.
- [surplus] `tools/recstrap/src/error.rs:128-139` `extraction_failed` dead; `rootfs.rs:136,141,182,188,213,222` build errors ad hoc.
- [surplus] `tools/recstrap/src/rootfs.rs:13-32` `RootfsType` one-variant enum; `if expected == RootfsType::Erofs` always true; `main.rs:380,448,464` only Debug-print it.
- [surplus] `tools/recstrap/src/helpers.rs:42` `path_to_cstring` and `:81` `erofs_supported` are `pub` but module-local.
- [surplus] `tools/recchroot/src/main.rs:66,68` `UnmountFailed` never constructed (exit 5 unreachable).
- [surplus] `tools/recchroot/src/main.rs:464-466` empty "Tests" banner.
- [surplus] `tools/recfstab/src/device.rs:31-38` `IdType::fstab_prefix` dead (identical to `blkid_tag`).
- [surplus] `tools/recfstab/src/filter.rs:61-64` `is_valid_fstype` dead.
- [surplus] `tools/recfstab/src/error.rs:51-62` `ErrorCode::description` dead.
- [surplus] `tools/recfstab/src/swap.rs:14` `SwapInfo.swap_type` never read.
- [surplus] `tools/recfstab/src/device.rs:6` `#[derive(Default)]` on `IdType` unused (`main.rs:38-46` picks default by hand).
- [surplus] `tools/recfstab/src/lib.rs:139` checks `is_swap_under_root` then calls `print_swap_entries` on a one-element slice that checks again (`swap.rs:195`).
- [surplus] `tools/recfstab/src/swap.rs:21` `read_swaps` returns `Result` but can't fail (`:24`).
- [surplus] `distro-spec/src/shared/components/users.rs:43-48` doc describes a `cargo build -p` path that no longer exists.
- [surplus] `leviso/src/preflight/dependencies.rs:54-63` checks `deps/{tool}.rhai`, a path that no longer exists.
- [surplus] `AcornOS/src/component/custom/live.rs:98-192` `copy_recstrap` — third, legacy install path copying from `target/release` and writing a placeholder script that documents `grub-install`/`arch-chroot`.
- [special] `tools/recstrap/src/main.rs:235-253` dedicated `--allow-host-root-fs` guard for one scenario (`--force /home/user`), never implied by `--force`, reusing E010.
- [special] `tools/recstrap/src/main.rs:255-256` `--force` = OR of exactly two flags, deliberately excluding the third.
- [special] `tools/recstrap/src/main.rs:260` `is_mount_point(...).unwrap_or(false)` swallows I/O errors.
- [special] `tools/recstrap/src/main.rs:277` `is_dir_empty(...).unwrap_or(false)` swallows read errors.
- [special] `tools/recstrap/src/main.rs:293-311` statvfs failure skips the space check with a warning (none under `--quiet`).
- [special] `tools/recstrap/src/main.rs:213-217` write-test removal `let _ =`, leftover special-cased in `helpers.rs:31`.
- [special] `tools/recstrap/src/main.rs:383` legacy "squashfs is no longer supported" message.
- [special] `tools/recstrap/src/main.rs:450` hard-coded `14` checks.
- [special] `tools/recstrap/src/main.rs:488-493` SSH key regen failure (a security problem) downgraded to a warning.
- [special] `tools/recstrap/src/main.rs:501-504` user prompt only on TTY without `--quiet`; `Result` discarded.
- [special] `tools/recstrap/src/helpers.rs:31` `is_dir_empty` ignores exactly `lost+found` and `.recstrap_write_test`.
- [special] `tools/recstrap/src/helpers.rs:96-100` `modprobe erofs` result discarded.
- [special] `tools/recstrap/src/helpers.rs:107-114` `ssh_keygen_available` tests only that `--help` spawned.
- [special] `tools/recstrap/src/helpers.rs:132-146` missing `/etc/ssh` or `ssh-keygen` returns `Ok(())`, silent under `--quiet`, so shared keys ship — contradicts `main.rs:489-490`.
- [special] `tools/recstrap/src/helpers.rs:155-156` old keys deleted with errors ignored before generation; later failure leaves no keys.
- [special] `tools/recstrap/src/helpers.rs:168` `bits == 0` sentinel for ed25519.
- [special] `tools/recstrap/src/helpers.rs:218-220` user setup silently skipped if `<target>/root` missing.
- [special] `tools/recstrap/src/helpers.rs:236` only "y"/"yes" accepted.
- [special] `tools/recstrap/src/helpers.rs:248-251` separate empty-username branch though `is_valid_username` rejects empty.
- [special] `tools/recstrap/src/rootfs.rs:130-134` mount dir in `/run` if present else `temp_dir()`.
- [special] `tools/recstrap/src/rootfs.rs:82-93` unmount failure leaves the dir with a warning; `remove_dir` error ignored.
- [special] `tools/recstrap/src/rootfs.rs:191` mount failure message guesses one cause.
- [special] `tools/recchroot/src/main.rs:195,374-414` efivars the single optional mount with its own warn-and-continue loop.
- [special] `tools/recchroot/src/main.rs:257,265-267,424` symlinked resolv.conf skipped, missing `etc` skipped, DNS bind error → warning.
- [special] `tools/recchroot/src/main.rs:456` placeholder resolv.conf removal ignores errors.
- [special] `tools/recchroot/src/main.rs:229-240` symlinked mount target reported as `MountFailed` (E004).
- [special] `tools/recfstab/src/lib.rs:74` canonicalize failure reported as E003 "failed to determine current directory" (`error.rs:102-108`).
- [special] `tools/recfstab/src/lib.rs:137` `if let Ok(swaps)` swallows swap read errors.
- [special] `tools/recfstab/src/swap.rs:141-144` block-device swaps always included, so a live host's swap leaks into the target fstab (baseline `recfstab /` shows `/dev/dm-1 none swap`); `tests/integration.rs:157-171` enshrines it.
- [special] `tools/recfstab/src/swap.rs:33,113-115` zram skip by `/dev/zram` prefix.
- [special] `tools/recfstab/src/filter.rs:35` `RUNTIME_OPTIONS` drops user-chosen `ro`, `noatime`, `relatime`; `:50` one-off `subvolid=` strip for btrfs.
- [special] `tools/recfstab/src/filter.rs:16-17` desktop-specific `fuse.gvfsd-fuse`, `fuse.portal` in the pseudo-fs list.
- [special] `tools/recfstab/src/fstab.rs:73-75` `needs_fsck` excludes vfat and btrfs by name.
- [special] `tools/recfstab/src/fstab.rs:31-34` empty target → "/"; `device.rs:69-72` empty source → "none"; `device.rs:49-52` leading `[` special case.
- [special] `tools/recfstab/src/device.rs:94-96,115` blkid failures silent, falling back to raw `/dev` paths (unstable in fstab; visible in baseline output for `/home`, `/boot`).
- [special] `tools/recfstab/src/swap.rs:85` hand-rolled octal check.
- [special] `distro-builder/src/recipe/tools.rs:14,191` `install-split-pane` mapping inside the rec-tool producer.
- [special] `distro-builder/src/recipe/tools.rs:139-141` README copied only because one manifest key needs it.
- [comment] `tools/recstrap/src/main.rs:141-143,160-162,313-315,413-415,435-437,457-459,471-473,478-480,495-497` 9 phase banners + ~9 one-liners (`:192,212,258,275,292,417,439,468,475`).
- [comment] `tools/recstrap/src/helpers.rs` ~22 restating comments (`:14,30,38-39,41,48,59,63,68,79-80,88-89,95,102,106,131,139,154,158,165-166,180,216,241`); `:196-204` doc misattached to `is_valid_username`.
- [comment] `tools/recstrap/src/rootfs.rs:12,27-28,168,171,197,231` (6); `error.rs:1-3,12-46` (17 variant docs repeating `code()`); `constants.rs:1-4,6,11`.
- [comment] `tools/recchroot/src/main.rs` ~16 restating comments + 3 banners + 8 variant docs (`:50-52,57-72,182-186,194-202,214,286,292,306,318,333,427,453`).
- [comment] `tools/recfstab/src/lib.rs` 12 (`:56,64,84,87,93,98,103,109,112,115,118,136`); `device.rs` ~13; `swap.rs` ~14; `mount.rs:43,59`; `fstab.rs:31`; `main.rs:37`; `error.rs` ~15; `filter.rs:61` misdescribes `is_valid_fstype`.
- [test] `--help`-only tests: `tools/recstrap/tests/integration.rs:25-45` (`:38` asserts removed `--squashfs` absent), `tools/recchroot/tests/integration.rs:21-31`, `tools/recfstab/tests/integration.rs:62-77`.
- [test] `--version`-only: recstrap `:47-53`, recchroot `:33-39`, recfstab `:79-85`.
- [test] clap missing-argument: recstrap `:55-66`, recchroot `:41-52`, recfstab `:87-98`; recstrap `:232-241` clap unknown-flag.
- [test] `tools/recfstab/tests/integration.rs:222-298` six `*_flag_parses` tests prove only clap parsing; `:300-312` tests clap `conflicts_with`.
- [test] `test_exit_code_success_on_help` (recstrap `:391-395`, recchroot `:90-94`, recfstab `:191-195`) duplicate help tests; `test_exit_code_failure_on_error` (`:397-401`, `:96-100`, `:197-201`) duplicate nonexistent-path tests with weaker assertions.
- [test] `tools/recstrap/tests/integration.rs:403-413` duplicates `test_root_check_without_root` and asserts nothing as root.
- [test] `tools/recstrap/tests/integration.rs:110-121,138-464` 12 tests start with `if !is_root() { return; }` — report passing without running in the normal unprivileged case; `rust-ci.yml` never runs `cargo test`.
- [test] `tools/recstrap/tests/integration.rs:466-482` fails as root when `/mnt` is on `/`; `:323-354` gets E010 instead of E009/E011 when `/tmp` is on `/`.
- [test] `tools/recstrap/tests/integration.rs:217-230,450-464` (`/etc`, `/home`) duplicate `/` and `/usr` protected-path tests.
- [test] `tools/recchroot/tests/integration.rs:279-307` two `#[ignore]` tests target `/` (protected) so can never pass, and assert nothing on failure.
- [test] `tools/recchroot/tests/integration.rs:147-179` `test_not_root` as root really bind-mounts `/proc`, `/sys`, `/dev`, `/run` into a tempdir.
- [test] `tools/recchroot/tests/integration.rs:244-260` duplicates `:58-84`; `:130-141`, `:262-273` more copies of the E001 test.
- [test] `tools/recfstab/tests/integration.rs:152-185` accepts success and failure; enshrines the host-swap leak.
- [test] `tools/recfstab/tests/integration.rs:207-220,318-348` depend on the host mount table; `:327-330` re-runs a helper every other `/` test already calls.
- [test] No test covers swap-file output, btrfs subvolumes, escaping, or recstrap `--check`/extraction/SSH regen outside the root-only CI e2e job, which is broken (host-root-fs guard, path deps).

- Added by completeness pass 1 (fresh auditor, no access to the list):
  - [hidden] `tools/recstrap/src/main.rs:30-50` vs `error.rs:31`: the E010 description already differs between copies (`main.rs:43`).
  - [special] `tools/recstrap/src/main.rs:120-126` `is_on_host_root_fs` returns `false` on metadata errors (`_ => false`) — the host-root guard fails open; its sibling device-ID check `is_mount_point` lives in `distro-spec/src/shared/system.rs:53`.
  - [hidden] `tools/recstrap/src/main.rs:145-433`, `rootfs.rs:277`: `guarded_ensure!` cheat/consequence prose names identifiers ("Reduce MIN_REQUIRED_BYTES" `main.rs:303`, "Reduce ESSENTIAL_DIRS list"); renames don't reach it.
  - [hidden] `tools/recstrap/src/rootfs.rs:253-263` vs `276-282`: cheat vectors written twice in one function (rustdoc and macro args).
  - [hidden] `tools/recstrap/src/main.rs:194-195,342-343,372-373`: canonicalize failures build `RecError::new(code, e.to_string())` inline, bypassing `error.rs` constructors; error wording split.
  - [hidden] `tools/recstrap/src/main.rs:105-107,255-256`: "`--force` expands to two flags but never `--allow-host-root-fs`" exists only in help text/comments; a new `--allow-*` flag must remember line 255.
  - [hidden] `tools/recstrap/src/main.rs:317-375`: explicit `--rootfs` and auto-detected branches duplicate the is_file guard and canonicalize-and-map-error code.
  - [hidden] `tools/recstrap/src/main.rs:89,380-386`: ".erofs only" rule restated in arg doc, extension check, and implied by every `ROOTFS_SEARCH_PATHS` entry.
  - [hidden] `tools/recstrap/src/main.rs:422-433`: `ensure_erofs_module()` runs `modprobe` before the `--check` early exit — "pre-flight only" mode has a kernel side effect, by undocumented ordering.
  - [hidden] `tools/recstrap/src/main.rs:13-17,79-82,506-528`: next-steps text exists three times inside recstrap (module doc, `long_about`, final message); runs `/root/setup-initial-user.sh` with `bash` though the script is `#!/bin/sh` (`helpers.rs:263,265,293-294`); `bootctl install` copies `distro-spec/src/shared/boot.rs:249`.
  - [needless] `tools/recstrap/src/main.rs:501-504`: `--quiet` doubles as the non-interactive switch (two concerns, one flag).
  - [surplus] `tools/recstrap/src/helpers.rs:63-66` `is_rootfs_inside_target` one-line wrapper over `Path::starts_with`.
  - [surplus] `tools/recstrap/src/helpers.rs:68-77` `can_read_rootfs` reads 4 bytes at offset 0; `rootfs.rs:29-37` reopens and reads at 1024 — subsumed by the magic check.
  - [hidden] SSH host-key type list `[(rsa,3072),(ecdsa,256),(ed25519,0)]` also in `distro-builder/src/alpine/ssh.rs:29-31` and `testing/fsdbg/src/checklist/qcow2.rs:76-78` (besides `distro-spec/src/shared/auth/ssh.rs:62`).
  - [hidden] `tools/recstrap/src/helpers.rs:205-213` vs `:255`: username rule coded in the function and restated as a regex in the prompt.
  - [hidden] `tools/recstrap/src/helpers.rs:264-274` also ignores `UserSpec::useradd_command` (`distro-spec/src/shared/users.rs:61`).
  - [hidden] Error messages copied verbatim between tools ("target directory '{}' does not exist", `tools/recstrap/src/error.rs:97` vs `tools/recchroot/src/main.rs:123`).
  - [hidden] `tools/recstrap/src/rootfs.rs:97-122,175,208,235-248`: every `Command` must be wrapped in `with_default_signals` by convention.
  - [special] `tools/recchroot/src/main.rs:327-331` ignores signals process-wide and never restores dispositions (recstrap restores).
  - [hidden] `tools/recchroot/src/main.rs:380` optional-mount target derived via `strip_prefix('/')` while `:187-192` use explicit `(src, name)` pairs — one dataset, two representations.
  - [hidden] `tools/recchroot/src/main.rs:269` `File::create` failure for resolv.conf reported as `mkdir_failed` (E003).
  - [special] `tools/recchroot/src/main.rs:461` `status.code().unwrap_or(1) as u8` also truncates i32.
  - [special] `tools/recfstab/src/lib.rs:103-107` for duplicate mount targets the first findmnt row wins — the over-mounted (hidden) one.
  - [hidden] Three escape/unescape codecs that must agree: `tools/recfstab/src/mount.rs:76-83` (findmnt `\xNN`), `swap.rs:75-107` (`/proc/swaps` octal), `fstab.rs:10-24` (fstab octal).
  - [hidden] `tools/recfstab/src/lib.rs:122-131` and `swap.rs:202-208` print the fstab line format (comment, tab fields, blank line) on two code paths.
  - [hidden] `tools/recfstab/src/main.rs:22,26,30,38-46`: a new id type means editing every flag's `conflicts_with_all` list plus the if-chain.
  - [test] `tools/recfstab/tests/integration.rs:142` locks in exit 1 for E004; `tools/recstrap/tests/integration.rs:83-84` returns early when run *as* root.
  - History (recurring change patterns): (1) safety hardening done separately per tool — recstrap `275d700` and recchroot `c19da09` the same day, earlier recchroot `e6b9c81` / recstrap `890c1d6` — no shared owner for "mount, run child, always clean up"; (2) constants and error framework ping-pong into/out of distro-spec (`ca29996`, `22f3f65`, `e676e18`, `bc03213`, then `a4f3ba8` moved `SQUASHFS_MAGIC` back) leaving re-export modules and unused `ToolError`; (3) squashfs→EROFS transition residue (`7968f45`, `0f795d6`, `890c1d6`, `a4f3ba8`, `6029962`, `ccf5eb2`); (4) post-install config creep into "extract only" recstrap (`ad4c4f9`, `f5748d8`, `6484900`, `275d700`); (5) much line churn is test/process policy (`86c30a4`, `bca4767`, `9b4b270`, fmt/clippy `3ecce68`, `4045391`), not behaviour.

### B. Disk + slots (recpart, recab, recguard) and their TUIs

- [hidden] `tools/recpart/src/policy.rs:122,131` and `exec.rs:232-233` label slot filesystems `ROOT_A`/`ROOT_B`; `tools/recab/src/lib.rs:944-953` detects the active slot by substring `system-a`/`system-b` in `/proc/cmdline`, as do `tools/recshuttle/src/lib.rs:280-300,434-456`, `tools/recab/tests/e2e_sdc.rs:268-283`, `docs/content/.../03-installation-disk.ts:167-184`. No shared constant; recpart-installed systems never hit recab's fast path (verified).
- [hidden] `tools/recpart/src/policy.rs:137-143` persistent partition `STATE` at `/state`; docs (`02-installation.ts:31`, `03-installation-disk.ts:184`) describe a `var` partition.
- [hidden] `tools/recpart/src/exec.rs:184-321` rebuilds partition/format/mount from hard-coded indices 1-4, labels, `mkfs.vfat`/`mkfs.ext4` and subpaths `sysroot`/`boot`/`state`, ignoring `plan.partitions` from `policy.rs:106-167` (only copied into `partition_map`, `exec.rs:89,125`). Changing a template in policy.rs has no effect on apply.
- [hidden] `tools/recpart/src/policy.rs:125,134,143` template mountpoints `/slots/A`, `/slots/B`, `/state`; `exec.rs:244-278` mounts root_a at `<mount_root>/sysroot`, never mounts root_b, mounts state at `sysroot/state`. Plan JSON shown to the user (`disk-plan-screen.tsx:911`) disagrees with reality.
- [hidden] `tools/recpart/src/exec.rs:330-369` `mount_map_for_plan` is a second hand-written copy of the topology in `exec.rs:244-316`.
- [hidden] `tools/recpart/src/handoff.rs:22-23` hard-codes slot A = partition 2, B = 3 — third copy of the index layout (`policy.rs:119-135`, `exec.rs:227-228`).
- [hidden] `tools/recpart/src/handoff.rs:10-12` finds the target by a mount path ending `/sysroot`, falling back to `/mnt/sysroot`; relies on `exec.rs:244,294,334,348` and CLI default `--mount-root /mnt` (`cli.rs:65`).
- [hidden] `tools/recpart/src/exec.rs:104-110` rollback finds unmount targets via `program == "mount"` and `args.last()`; any change to mount action shape breaks rollback silently.
- [hidden] `tools/recpart/src/exec.rs:188-319` phase strings `"partition"/"format"/"mount"` mapped back to error codes at `501-508` (typo → `Internal`); also serialized to JSON `steps[].phase` and shown by the TUI. Stringly-typed contract.
- [hidden] `tools/recpart/src/exec.rs:19` `CONFIRM_TOKEN = "DESTROY"` repeated in `tui/apps/install/disk-plan/src/app/plan-runner.ts:52`, `presentation/page-text.ts:4`, `disk-plan-screen.tsx:429,515,1029,1045,1063`, `docs/content/.../10-recpart.ts:89,152`, `tests/e2e_real_disk.rs:122`; implicitly `disk-plan-screen.tsx:280` (`.slice(0, 7)`).
- [hidden] `tools/recpart/src/error.rs:6-19,40-42` exit codes are enum discriminants (`*self as u8`), with E001-E012 strings mapped separately (`23-38`); table copied in `tools/recpart/README.md:62-77` and `docs/content/.../10-recpart.ts:157-178`. Reordering the enum changes exit codes silently.
- [hidden] `tools/recpart/src/types.rs:28-162` output types are copied 3 more times: hand-written JSON schemas (`docs/frontend-handoff/schemas/*.json`), examples (`docs/frontend-handoff/examples/*.json`), and hand-written TS validators (`tui/apps/install/disk-plan/src/domain/plan-model.ts:46-126`, `apply-model.ts:47-219`).
- [hidden] `schema_version === 1` repeated in `plan-model.ts:58`, `apply-model.ts:59,121,201`, `disk-plan-screen.tsx:318,371,476`, mirroring five `*_SCHEMA_VERSION` constants (`types.rs:5-9`) and each schema's `"const": 1`.
- [hidden] Mode values `"ab" | "mutable"` re-declared in `disk-plan/src/cli/parse-args.ts:1,10-15`, `plan-model.ts:13,63`, `apply-model.ts:39,64`, `disk-plan-screen.tsx:504,837-838`, schemas, `tools/recpart/src/types.rs:11-26`, `cli.rs:107-120`.
- [hidden] `tui/apps/install/disk-plan/src/app/plan-runner.ts:179-271` `listHostDisks` re-implements recpart's inventory (`preflight.rs:521-618`, exposed as `recpart list-disks --json`, which `docs/frontend-handoff/README.md:14-16` says frontends must use). Already diverged on loop devices.
- [hidden] `tui/apps/install/disk-plan/src/app/preflight.ts:55-75` re-implements recpart's root/tool preflight (`preflight.rs:52-108`) with a different tool list.
- [hidden] `tools/recpart/src/exec.rs:142` runs `umount` and `exec.rs:553` suggests `partprobe`; neither is in `REQUIRED_TOOLS` (`preflight.rs:52-61`).
- [hidden] `disk-plan-screen.tsx:948,951,1063` hand-render recpart command lines separately from `plan-runner.ts:39-55` (`buildPlanCommand`/`buildApplyCommand`).
- [hidden] `plan-runner.ts:43-55` never passes `--mount-root`; depends on recpart's default `/mnt`.
- [hidden] `plan-runner.ts:62-81,99` fallback JSON extractor for "text before the JSON" exists because recpart writes warnings to stderr (`preflight.rs:585`) and error JSON to stderr (`main.rs:16`); undocumented convention.
- [hidden] `plan-runner.ts:5-7` computes `REPO_ROOT` (six `..`) and exports `RECPART_ROOT`/`RECPART_MANIFEST`, but `spawnRecpart` (`:84`) runs bare `recpart` from PATH; no just recipe builds recpart for `just tui-install-disk-plan` (`justfile:429-443`).
- [hidden] recab status JSON fields (`tools/recab/src/lib.rs:146-154`) re-parsed by hand in `tui/apps/runtime/recab/domain/recab.ts:49-84` (and postinstall-tools copy), asserted as raw strings in `tests/e2e_sdc.rs:491-526`; `Slot` (`lib.rs:108-113`, `rename_all = "UPPERCASE"`) re-declared in `recab.ts:1`.
- [hidden] `tools/recab/src/main.rs:89-113` text status labels copied in `tui/apps/runtime/recab/presentation/runtime-recab-screen.tsx:41-62`.
- [hidden] `tools/recab/src/lib.rs:31-35` exit codes 13/20/21/22 duplicated in `tools/recab/README.md:79-86`, `tests/e2e_sdc.rs:609`; shown raw in `runtime-recab-screen.tsx:91-92,125`.
- [hidden] `tools/recab/src/lib.rs:75-105` `stable_exit_code` also classifies untagged errors by lowercase message substrings; rewording messages at `lib.rs:165,173,181,241,857,892,936,1168,1178` changes exit codes.
- [hidden] `tools/recab/src/lib.rs:76-81` tag check order is part of the contract; `revert_state_or_escalate` (`422-429`) embeds the primary error's tag inside a PartialUpdate error, so a partial update from a backend failure exits 20, not documented 22.
- [hidden] `<prefix>-a.conf`/`-b.conf` naming rebuilt in `tools/recab/src/lib.rs:640,786,1045,1064,1100-1106,1125-1136,1159-1160` and independently in `tools/recshuttle/src/lib.rs:266-267,418-422`.
- [hidden] loader.conf defaults: `tools/recab/src/lib.rs:712,718` (`timeout 5`, `editor 0`), `tools/recshuttle/src/lib.rs:321,427` (same), `distro-spec/src/shared/boot.rs:22,195-218` (`DEFAULT_TIMEOUT = 3`, `editor no`) — three disagreeing owners.
- [hidden] `tools/recab/src/lib.rs:606-607` hard-codes `loader/loader.conf`, `loader/entries` owned by `distro-spec/src/shared/boot.rs:16,19`.
- [hidden] `tools/recab/src/lib.rs:937` hint `RECAB_ENTRY_PREFIX=levitate` and `tests/e2e_sdc.rs:16` `BOOT_PREFIX = "levitate"` vs `distro-spec/src/levitate/paths.rs:68` `OS_ID = "levitateos"`, which `lib.rs:923-926` infers from os-release.
- [hidden] `tools/recab/src/lib.rs:873-884` sets default in loader.conf and (if bootctl) EFI `LoaderEntryDefault`, but `default_slot()` (`865-871`) reads only loader.conf.
- [hidden] `tools/recab/src/lib.rs:835` `RECAB_ACTIVE_SLOT` and `907` `RECAB_ENTRY_PREFIX` read lazily inside backend methods, unlike other overrides read in `from_env` (`510-521,602-632`); `RECAB_ACTIVE_SLOT` missing from `README.md:88-98`.
- [hidden] `tools/recab/tests/e2e_sdc.rs:359,395` fake bootctl keyed on `RECAB_TEST_BOOTCTL_LOG`, passed through the production env.
- [hidden] `tools/recab/src/lib.rs:372-382` `ensure_root` duplicates `distro_spec::shared::is_root` (`distro-spec/src/shared/system.rs:26-29`, used by recpart `preflight.rs:97`); TUI repeats via `id -u` (`disk-plan/src/app/preflight.ts:55-63`).
- [hidden] `tools/recpart/src/error.rs:47-99` `RecpartError` (component/expectation/observed/remediation) parallels `distro_spec::shared::error::ToolError` (`distro-spec/src/shared/error.rs:57-85`), half-used; shape re-declared in `disk-plan/src/app/preflight.ts:4-10`, `apply-model.ts:182-188`, `tools/recguard/REQUIREMENTS.md:220-226`.
- [hidden] `tools/recpart/src/policy.rs:5-8,106-167`, `plan.rs:48-64` duplicate `distro-spec/src/shared/partitions.rs:7-77` (EFI size/label, fs, GPT types, sfdisk format) without using it; `testing/install-tests/src/steps/phase2_disk.rs:106-107` uses distro-spec's version (root label `root` vs recpart `ROOT`), so install tests and recpart use different layouts.
- [hidden] `tools/recpart/src/policy.rs:179-181` derives CLI flag names via `replace('_', "-")`; `policy.rs:60,78,195,215` hard-code flag names; both depend on clap fields in `cli.rs:84-105`.
- [hidden] `tools/recpart/REQUIREMENTS.md:118-130` holds UI requirements (960x1080, 80x24, wizard pages) for `tui/apps/install/disk-plan`; a third state model in `docs/frontend-handoff/state-machine.md:5-28` that the TUI (`disk-plan/src/domain/flow.ts:3-14`) doesn't use.
- [hidden] `tui/apps/runtime/recab/**` and `tui/apps/runtime/postinstall-tools/src/**` are near-identical copies (`domain/recab.ts`, `app/recab-runner.ts`, `flows/index.ts`, `types/tui-kit-shim.d.ts` byte-identical).
- [hidden] `tui/apps/runtime/{recab,postinstall-tools}/types/tui-kit-shim.d.ts` (via `tsconfig.json:16`) hand-declares the `@levitate/tui-kit` API; drift from `tui/kit/core` is not typechecked.
- [hidden] `tui/apps/runtime/recab/flows/index.ts:19-60` hotkeys, `runtime-recab-screen.tsx:165-168` `useHotkeys`, and `cli/parse-args.ts:50-54` help — three copies.
- [hidden] TTY guard for disk-plan lives in `justfile:438-441`, for runtime apps in `cli/run.ts:15-18`.
- [hidden] `tools/recpart/src/handoff.rs:14-18,30-33` hard-codes CLI syntax of recstrap, recfstab (`>> <t>/etc/fstab`), recchroot, `recab status`, `recab set-next`; mirrored in `docs/frontend-handoff/examples/apply-dry-run-ab.json:46-51`.
- [needless] `tools/recpart/src/handoff.rs:1-64` knows recab's slot vocabulary and commands; `ModeContext` (`types.rs:107-114`) uses `Option<String>` slots.
- [needless] `tools/recpart/src/policy.rs:4` `DEFAULT_AB_TARGET_SLOT = "A"` (boot policy in sizing module); `handoff.rs:24-29` re-implements `recab::Slot::other` (`tools/recab/src/lib.rs:116-121`) by string compare.
- [needless] `disk-plan/src/domain/apply-model.ts:68-71,87-90,99-114,149-165` strictly validate `dry_run`, `mounted`, `mode_context.notes`, `warnings`, none of which the screen reads.
- [needless] `disk-plan/src/domain/plan-model.ts:32-35,95-101` requires `label` per partition; never rendered.
- [needless] `tools/recpart/src/types.rs:36-45` `DiskInventory` repeats `DiskTarget` (`28-34`); `preflight.rs:508-519,589-593` copy field by field.
- [needless] `tools/recpart/src/cli.rs:84-105,132-142` `LayoutRequestArgs` copies `LayoutRequest` (`types.rs:75-82`) field for field.
- [needless] `tools/recpart/src/cli.rs:107-120` `ModeArg` mirrors `InstallMode`; `tools/recab/src/main.rs:37-50` `SlotArg` mirrors `Slot` — only for clap `ValueEnum`.
- [needless] `tools/recpart/src/cli.rs:122-130` `json_requested` must list every subcommand because `--json` is per-subcommand.
- [needless] `tools/recpart/src/preflight.rs:134-156` mount-root argument validation reported as `MountFailed` (E007).
- [needless] `tools/recpart/src/preflight.rs:171-193,210-232,358-380,401-429,522-550` call lsblk five times with five copies of the error boilerplate.
- [needless] `tools/recpart/src/preflight.rs:357-394` parses lsblk `RO` as literal `"0"` while `31-50` `LsblkReadOnly` parses it three ways.
- [needless] "whole disk = disk or loop" at `preflight.rs:196,571` and `plan-runner.ts:209,216` (divergent).
- [needless] `/sys/class/block` hard-coded at `preflight.rs:266,343`.
- [needless] `tools/recpart/src/exec.rs:388-397` CreateDir builds its own `"mkdir"`/`-p` record instead of `ExecutionAction::program_name`/`args` (`586-602`).
- [needless] `tools/recab/src/lib.rs:634-636,834,866,874,887,906` re-run `ensure_uefi_ready` per backend method; `entry_prefix()` (re-reads loader.conf, os-release, entries) called repeatedly (`190,202,280,313,639,776`).
- [needless] `tools/recab/src/lib.rs:834`: read-only `recab status` fails exit 20 off UEFI because `active_slot` runs `ensure_uefi_ready`.
- [needless] `isRecord`/`asString`/`asNumber` helpers defined in `apply-model.ts:1-17`, `plan-model.ts:1-30`, `tui/apps/runtime/recab/domain/recab.ts:28-47`.
- [surplus] `tools/recab/src/lib.rs:440-453` `BootBackend`/`StateStore` traits and generic `*_with` functions (`187-351`) have one implementation each — leftovers from removed unit tests (`c6d327a`, `83a6679`).
- [surplus] `tools/recab/src/lib.rs:164-185` `set_next`/`commit`/`rollback` compute and return a `Status` that `main.rs:69-77` discards.
- [surplus] `tools/recab/src/lib.rs:87-103` message-substring fallback duplicates tag classification.
- [surplus] `tools/recab/src/lib.rs:651-652` `NotFound` arm equals the catch-all.
- [surplus] `tools/recab/src/lib.rs:1109-1111` `prefix_from_entry_name` one-line wrapper; `slot_from_entry_name` (`1098-1107`) re-implements the suffix match.
- [surplus] `tools/recab/src/lib.rs:21,533-535` `STATE_SCHEMA_VERSION` written but never checked.
- [surplus] `tools/recpart/src/plan.rs:10-12` `build_plan` only re-exported (`lib.rs:13`); no external Rust consumers of the recpart lib.
- [surplus] `tools/recpart/src/exec.rs:40-48` `execute_plan_with_reporter` always gets `None`; `execute_plan` is a pass-through.
- [surplus] `tools/recpart/src/preflight.rs:508-519` `list_candidate_disks` unused.
- [surplus] `tools/recpart/src/preflight.rs:52,63` `REQUIRED_TOOLS`, `tool_in_path` are `pub` but module-local.
- [surplus] `tools/recpart/src/error.rs:15,17` `HandoffFailed` (E008), `NotImplemented` (E010) never constructed but documented as stable (`README.md:73,75`, `10-recpart.ts:170,172`).
- [surplus] `tools/recpart/src/types.rs:151` `ApplyResult.warnings` always empty, yet schema-required and TUI-validated.
- [surplus] `tools/recpart/src/types.rs:149-150` `ApplyResult.mounted` and `handoff.mount_map` hold the same vector.
- [surplus] `tools/recpart/src/exec.rs:81-94` vs `117-130` build the same `ApplyResult` twice.
- [surplus] `tools/recpart/src/plan.rs:81-113` `validate_partition_fit` unreachable given `required_min_mb` (`policy.rs:91-104`, checked at `plan.rs:23-31`).
- [surplus] `tools/recpart/src/policy.rs:94-95` `unwrap_or(DEFAULT_AB_ROOT_SIZE_MB)` unreachable (`policy_defaults` always `Some`, `16-17`).
- [surplus] `tools/recpart/src/json.rs:1-14` wraps `serde_json::to_string_pretty` to convert the error type.
- [surplus] `tools/recpart/src/exec.rs:35-37` `VerifyPartitions { phase }` — always `"partition"`.
- [surplus] `tools/recpart/src/exec.rs:55` re-checks `ensure_block_device` already validated by `query_disk_target` (`preflight.rs:398`).
- [surplus] `disk-plan/src/domain/flow.ts:62-68` `canAdvance` unused (screen uses `guards.ts:12-32`).
- [surplus] `disk-plan-screen.tsx:77-79` `pageLabel` identity function.
- [surplus] `disk-plan/src/index.ts:4-5`, `plan-runner.ts:6-7` export unused `RECPART_ROOT`/`RECPART_MANIFEST`.
- [surplus] `tui/apps/runtime/recab/domain/recab.ts:13-16` `RecabCommandSpec.command` repeats `args[0]`.
- [surplus] `tui/apps/runtime/recab/index.ts:1-20` and `main.ts:1-7` — two entrypoints that both call `runCli()` (same in postinstall-tools).
- [surplus] `tools/recpart/docs/installer-production-readiness-gaps.md:1-65` stale audit (references nonexistent `src/tui.rs`, `REQ-CONF-001`).
- [surplus] `tools/recpart/README.md:31-50` "Planned UX/Components" describe implemented things and claim idempotent safety checks that don't hold; `tools/recab/README.md:105-108` "Likely Integrations (TBD)" lists implemented features.
- [surplus] `tools/recguard/REQUIREMENTS.md:285-306` coverage minimums + traceability matrix of planned test files for a tool with no code; conflicts with workspace rules.
- [surplus] `tools/recguard/` (README + 307-line spec, no code) restates the `ab`/`mutable` model and diagnostic shape owned by recpart.
- [surplus] `tools/recab/REQUIREMENTS.md:285-293` cites `QUALITY.md`, `tools/recipe/OS_UPGRADES_BRAINDUMP.md` — neither exists.
- [surplus] `tools/recpart/REQUIREMENTS.md:26` TOC points to a renamed section (`:282`).
- [special] `tools/recab/src/lib.rs:944-953` any cmdline containing `system-a`/`system-b` anywhere (e.g. a UKI path) is taken as the slot, before parsing `root=`.
- [special] `tools/recab/src/lib.rs:835-844` `RECAB_ACTIVE_SLOT` production override for "constrained environments" — evaluated after the UEFI check, so it can't help off UEFI.
- [special] `tools/recab/src/lib.rs:850-852` falls back to the loader default as the active slot when cmdline detection fails.
- [special] `tools/recab/src/lib.rs:211-226` `set-next` on the active slot writes state + audit (`set-next-noop-active`), contradicting REQ-SM-010 (`REQUIREMENTS.md:121`).
- [special] `tools/recab/src/lib.rs:228-235` early return when state says pending, without checking the bootctl oneshot is still set.
- [special] `tools/recab/src/lib.rs:237-248` refuses `set-next` unless loader default equals known-good ("run recab commit first").
- [special] `tools/recab/src/lib.rs:879-882` silently skips `bootctl set-default` when bootctl is missing, while `set_trial_slot` (`888-897`) hard-fails.
- [special] `tools/recab/src/lib.rs:676-732` writes loader.conf in place without temp+rename; on later bootctl failure state is reverted (`290-297`) but loader.conf isn't — violates REQ-ERR-010 (`REQUIREMENTS.md:257`).
- [special] `tools/recab/src/lib.rs:408-410` `append_audit_best_effort` swallows audit errors.
- [special] `tools/recab/src/lib.rs:753` missing `/proc/cmdline` treated as "no slot".
- [special] `tools/recab/src/lib.rs:772-774` plain `root=/dev/sdX` never mapped via entries.
- [special] `tools/recab/src/lib.rs:1091-1092` bare `default` line stops loader.conf parsing.
- [special] `tools/recab/src/lib.rs:1100,1103,1131-1136` accept entry names without `.conf` for bootctl-style ids.
- [special] `tools/recab/src/lib.rs:533-535` `schema == 0` silently upgraded.
- [special] `tools/recab/src/lib.rs:359` `compose_status` reports loader default as `known_good_slot` when state has none; `rollback` (`317-325`) then rejects it.
- [special] `tools/recpart/src/plan.rs:53` bootable only when `index == 1 && gpt_type == "U"`.
- [special] `tools/recpart/src/preflight.rs:575-580` skips `zram*` by name; `plan-runner.ts:218-219` has its own zram regex.
- [special] `tools/recpart/src/preflight.rs:196,571` accept `loop` devices as disks (tests/VMs).
- [special] `tools/recpart/src/preflight.rs:31-50` `LsblkReadOnly` handles three util-linux encodings of `RO`.
- [special] `tools/recpart/src/preflight.rs:582-588` disks failing `query_disk_target` dropped from `list-disks` with only a stderr warning.
- [special] `tools/recpart/src/preflight.rs:278,288,327` unreadable holders dirs, `/proc/swaps`, sysfs dirs treated as "no problems".
- [special] `tools/recpart/src/preflight.rs:273-277` holders path built differently for disk vs partitions.
- [special] `tools/recpart/src/preflight.rs:304-311` `O_EXCL` probe only when no other problem found.
- [special] `tools/recpart/src/preflight.rs:239` filters `-` as a mountpoint value.
- [special] `tools/recpart/src/exec.rs:51-59` dry-run skips every safety check (writable, mounted, in-use, tools, root); TUI dry-run "success" says nothing about apply.
- [special] `tools/recpart/src/exec.rs:203,400,513-558` `udevadm settle --timeout=10` plus 250 ms polling up to 10 s for partitions.
- [special] `tools/recpart/src/exec.rs:135-168` rollback unmount discards stdout/stderr; failures folded into `observed`.
- [special] `tools/recpart/src/exec.rs:190-191` omits `wipefs --force` to use its busy check; `docs/frontend-handoff/examples/apply-dry-run-ab.json:21` still shows `--force`.
- [special] `tools/recpart/src/exec.rs:244-278` A/B mode formats B but never mounts it; "first install goes to A" baked in, not taken from `DEFAULT_AB_TARGET_SLOT`.
- [special] `tools/recpart/src/handoff.rs:31-33` emits a shell comment as an entry of `next_commands`, rendered as a command (`disk-plan-screen.tsx:1167-1169`).
- [special] `tools/recpart/src/main.rs:17` error payload serialization failure silently falls back to text under `--json`.
- [special] `tools/recpart/src/policy.rs:96` `DEFAULT_AB_MIN_STATE_MB` used both as min explicit state size and as assumed size of "remaining space".
- [special] `plan-runner.ts:99` takes JSON from stderr on failure, then stdout.
- [special] `plan-runner.ts:164-166,172-176,188-195` swallow udevadm, lsblk, realpath failures.
- [special] `disk-plan/src/app/preflight.ts:12-17` checks tools via `bash -lc "command -v ${command}"` (string interpolation).
- [special] `disk-plan-screen.tsx:203-221` injects a fake `(not found)` candidate so explicit `--disk` is never replaced.
- [special] `disk-plan-screen.tsx:102` hides vendor strings `ATA`/`NVME`/`SCSI`.
- [special] `disk-plan-screen.tsx:280` confirm input: letters only, uppercased, truncated to 7.
- [special] `disk-plan/src/presentation/page-text.ts:7-10` shows dry-run `status === null` as `OK`.
- [special] `tui/apps/runtime/recab/app/recab-runner.ts:32` synthesizes exit 127 on spawn failure, `?? 1` for signals.
- [special] `tools/recab/tests/e2e_sdc.rs:301-309` polls 100 × 50 ms for partition nodes.
- [comment] `tools/recab/src/lib.rs:107,144,156,163,171,179`: 6 doc comments restating item names.
- [comment] `plan-runner.ts:21,77`; `tools/recpart/src/preflight.rs:337` restate code.
- [test] `tools/recpart/tests/integration_cli.rs:15-32` checks only `--help` text (clap parsing).
- [test] `tools/recpart/tests/integration_cli.rs:56-71` checks only `schema_version == 1` and `disks` is an array; host-dependent; passes on empty list.
- [test] `tools/recpart/tests/e2e_real_disk.rs:29-44` `#[ignore]`; `:42` asserts `disk.path == <input>`, which fails for symlinked paths since recpart canonicalizes (`preflight.rs:397`).
- [test] `tools/recpart/tests/e2e_real_disk.rs:46-85` `#[ignore]`, needs a real disk though dry-run touches nothing; checks only non-empty `steps`.
- [test] `tools/recpart/tests/e2e_real_disk.rs:87-180` `#[ignore]` destructive; fourth copy of `sysroot/boot/state` topology (`:139-143,151-155`); never checks labels — where recpart and recab disagree.
- [test] `tools/recab/tests/e2e_sdc.rs:455-456,588-589` both `#[ignore]`; repartition `/dev/sdc` (`:263-299`) only to get PARTUUIDs recab reads from temp files — fake UUIDs would exercise the same code.
- [test] `tools/recab/tests/e2e_sdc.rs:197,226` `RECAB_E2E_DEVICE` must equal `/dev/sdc`, so the override can't take effect.
- [test] `tools/recab/tests/e2e_sdc.rs:357-361` fake bootctl always exits 0; revert, partial-update (22) and tag-order bug never exercised; `:609` hard-codes `21`.
- [test] `tools/recab/tests/e2e_sdc.rs:16,268-283` use `levitate` prefix and `system-a/b` labels matching neither recpart (`ROOT_A`) nor `OS_ID` — can't catch the cross-tool mismatch.
- [test] `tools/recpart/docs/use-cases.md:7-17` "Test Mapping" maps use cases to ignored tests or "Manual CLI workflow".
- Cross-area notes from this area:
  - [hidden] `tools/recshuttle/src/lib.rs:266-300,315-456` independently writes recab's input contract (`{distro_id}-a/b.conf`, `root=LABEL=system-a/b`, loader.conf defaults) and uses `/mnt/sysroot` — a second, divergent installer beside recpart.
  - [hidden] `testing/install-tests/src/steps/phase2_disk.rs:106-107` partitions with distro-spec's 2-partition layout, not recpart; install tests don't exercise recpart.
  - [hidden] `docs/content/src/content/05-rec-tooling/03-recab.ts:12-40` still calls recab a "scaffold" with TODO backend and documents `set-next <a|b>`; `07-recguard.ts:58`, `10-recpart.ts:202` likewise; getting-started pages document `system-a/b`/`var` labels recpart doesn't produce.

- Added by completeness pass 2 (fresh auditor):
  - [surplus] `tools/recab/src/lib.rs:92-93` match `"missing '/sys/firmware/efi'"` and `"missing efivarfs"`, strings nothing produces (real messages at `:1168,1178` say "'…' is missing") — dead classifier arms (verified).
  - [surplus] `tools/recab/src/lib.rs:157-185` the four public entry points each repeat `from_env()` + root check before forwarding to `*_with`.
  - [special] `tools/recab/src/lib.rs:510-521,602-632` nine `RECAB_*` env overrides (boot root, state, audit, os-release, cmdline, efi, efivars, bootctl) read as process globals; only `tests/e2e_sdc.rs:385-396` sets them — test seams in the production config surface.
  - [hidden] `tools/recab/src/lib.rs:699-704` (updater) and `:1080-1096` (reader) parse the loader.conf `default` line separately.
  - [hidden] `tools/recab/src/lib.rs:923-927` entry prefix falls back to os-release `ID`, relying on the installer (recshuttle) naming entries after `distro_id`; unstated.
  - [hidden] `tools/recab/src/lib.rs:206-209` and `:359` compute the `known_good.or(default)` fallback twice.
  - [hidden] `tools/recab/src/lib.rs:213-218,252-343` each operation has three string names (`last_operation`, `save_state` op, audit op); `set-next-noop-active` repeated 3×.
  - [special] `tools/recab/src/lib.rs:533-535` the `schema == 0` upgrade only fires on an explicit 0 (no serde default), so a missing field fails to parse instead.

### C. Boot images (recinit, recuki, reciso)

- [hidden] `AcornOS/profile/init_tiny.template:204,207` and `leviso/profile/init_tiny.template:197,200` use `{{LIVE_OVERLAY_PATH}}`, which `tools/recinit/src/tiny.rs:263-268` never substitutes (only `ISO_LABEL`, `ROOTFS_PATH`, `BOOT_MODULES`, `BOOT_DEVICES`, `LIVE_OVERLAY_IMAGE_PATH`; both callers pass empty `extra_template_vars`) — the live overlay is never mounted on those paths (verified). Placeholder names are an unwritten contract between tiny.rs and three template files.
- [hidden] Three drifted copies of the live init: `tools/recinit/templates/init_tiny.template` (421 lines; partition-role probe, boot injection, `/run/live-media`), `leviso/profile/init_tiny.template` (272), `AcornOS/profile/init_tiny.template` (292; `/media/cdrom`, file probing, bind-mounted overlay dirs). Only `build-release.sh:45` uses the recinit copy.
- [hidden] `tools/reciso/src/lib.rs:114` defaults to `LivePayloadLayout::AppendedPartitions` (rootfs/overlay not copied into the ISO fs, `:322-336`); leviso/AcornOS callers (`leviso/src/artifact/iso.rs:111-119`, `AcornOS/src/artifact/iso.rs:63-65`) take the default while their templates look for `/mnt{{ROOTFS_PATH}}` (`AcornOS/profile/init_tiny.template:142,186`; `leviso/profile/init_tiny.template:135,179`) and their checks require `/live/filesystem.erofs` (`AcornOS/src/artifact/iso.rs:109`, `testing/fsdbg/src/checklist/iso.rs:101`).
- [hidden] `leviso/src/artifact/iso.rs:119`, `AcornOS/src/artifact/iso.rs:65` pass the *directory* `output_dir.join("live-overlay")` to `with_overlay`, which (`tools/reciso/src/lib.rs:145-148`) now means "EROFS image file"; `validate_inputs` bails on non-files (`:283-288`). The old name hides the meaning change.
- [hidden] `tools/recinit/templates/init_tiny.template:277-282` creates `/newroot/run/live-rootfs.erofs` and `live-overlay.erofs` before the overlay mount on `/newroot` (`:302`) and the tmpfs on `$NEWROOT/run` (`:322`) hide them; targets point at `/mnt…`, later moved to `/run/live-media` (`:404`). recstrap's first candidate (`distro-spec/src/shared/rootfs.rs:53,78`) depends on this mount order.
- [hidden] `tools/recinit/templates/init_tiny.template:139` reads `/mnt/.disk_label` that nothing writes; matching works only through the `-d /mnt/boot` fallback (`:140`).
- [hidden] Partition-role protocol spread across: template probes partitions `2 3 4` (`init_tiny.template:186`) for `.live-payload-role` (`:192`); reciso hard-codes indices 2, 3 (`tools/reciso/src/lib.rs:231,237`); role strings written by `distro-variants/{levitate,acorn}/ring2/products.toml:34-37` and `distro-builder/src/artifact/live_overlay.rs:143,702`.
- [hidden] `tools/recinit/src/main.rs:55-56,129` passes `live_overlay_image_path: None` when the flag is omitted (as `build-release.sh:307-313` does); the path resolves only via `..Default::default()` (`main.rs:132`) filling legacy `live_overlay_path` (`tiny.rs:78`) and the fallback at `tiny.rs:256-261`. The active release path depends on the "legacy alias".
- [hidden] Live UKI cmdline from `tools/reciso/src/lib.rs:350-353` (hard-coded `console=ttyS0,115200n8 console=tty0` + `SELINUX_DISABLE`); AcornOS `UKI_ENTRIES` already contain `console=tty0 console=ttyS0,115200n8` (`distro-spec/src/acorn/uki.rs:22,27,32`), so AcornOS live UKIs get consoles twice with primary reversed; `AcornOS/src/artifact/uki.rs:61` assumes otherwise.
- [hidden] Kernel cmdline built in four places: `tools/reciso/src/lib.rs:350-353`, `leviso/src/artifact/uki.rs:71-74` (+`efi=debug`), `AcornOS/src/artifact/uki.rs:62,113`, `build-release.sh:329-331` (`merge_uki_cmdline`); `root=LABEL=root rw` literal in `leviso/src/artifact/uki.rs:72`, `AcornOS/src/artifact/uki.rs:113`.
- [hidden] ISO label from env in `leviso/src/artifact/iso.rs:43-45`, `AcornOS/src/artifact/iso.rs:33` (xorriso `-V`, UKI `root=LABEL=`) but from the constant for the initramfs (`leviso/src/artifact/initramfs.rs:81`, `AcornOS/src/artifact/initramfs.rs:48`) → template default `ROOT_LABEL` (`init_tiny.template:91`).
- [hidden] Busybox URL/SHA/env name copied 3×: `tools/recinit/src/busybox.rs:77-84`, `distro-spec/src/levitate/paths.rs:108-113`, `distro-variants/_shared/ring0/hooks/build-release.sh:46-48`; SHA256 verification reimplemented in shell (`build-release.sh:252-277`).
- [hidden] Busybox download/caching three ways: `leviso/src/artifact/initramfs.rs:67-73` (no SHA check), `AcornOS/src/artifact/initramfs.rs:34-35`, `tools/recinit/src/install.rs:147-148` (cache in output's parent) — leviso keeps two copies with different verification.
- [hidden] Kernel-version discovery ("first dir in read_dir") 5×: `tools/recinit/src/modules.rs:168-188`, `install.rs:226-239`, `leviso/src/artifact/initramfs.rs:209-220`, `leviso/src/artifact/iso.rs:241-256`, template runtime (`init_tiny.template:46`); `build-release.sh:241-242` uses `kernel.release`, so paths may pick different kernels.
- [hidden] `tools/recinit/src/modules.rs:31-43` silently drops module names missing from `distro-spec/src/shared/modules.rs:179-228` `MODULE_PATHS`, so `tiny.rs:211`'s missing-module check never sees them; `LIVE_MODULES`/`INSTALL_MODULES` must change with `MODULE_PATHS`.
- [hidden] Module extension lists disagree: `tools/recinit/src/modules.rs:120-123` (no `.ko.zst`), `:134` probes `.ko.zst`, `:197-200` strips `.ko.zst`, templates decompress only `.xz`/`.gz` (`init_tiny.template:58-61`, all three) — a `.ko.zst` module is copied but can't be insmodded.
- [hidden] Install-initramfs BINARIES copied in `testing/fsdbg/src/checklist/install_initramfs.rs:21-54` vs `tools/recinit/src/systemd.rs:14-42`, drifted (fsdbg says bash removed per TEAM_145; recinit ships `usr/bin/bash`, `systemd.rs:40`).
- [hidden] `tools/recinit/src/systemd.rs:364-367` enables `initrd-udevadm-cleanup-db.service`; `testing/fsdbg/src/checklist/install_initramfs.rs:214-215` says it's intentionally NOT enabled (TEAM_144).
- [hidden] udev helper list 3×: `tools/recinit/src/systemd.rs:497-503` (5), `distro-spec/src/shared/udev.rs:22-29` (6, "used by recinit"), `testing/fsdbg/src/checklist/install_initramfs.rs:152-158` (5).
- [hidden] udev patch list 2×: `tools/recinit/src/systemd.rs:437-442` (4), `distro-spec/src/shared/udev.rs:39-45` (5).
- [hidden] Live initramfs dir list 3×: `tools/recinit/src/tiny.rs:99-111`, `testing/fsdbg/src/checklist/live_initramfs.rs:32-44`, `distro-spec/src/shared/initramfs.rs:8-19` (missing `lib/modules`); mount points also as template literals and unused `MOUNT_*` (`distro-spec/src/shared/initramfs.rs:26-35`).
- [hidden] Busybox applet list 3×: `tools/recinit/src/busybox.rs:18-47`, `testing/fsdbg/src/checklist/live_initramfs.rs:63-101`, `distro-builder/src/artifact/initramfs.rs:41`.
- [hidden] More fsdbg "Must match" copies: install dirs (`install_initramfs.rs:226-256` vs `tools/recinit/src/install.rs:57-91`), generators/tmpfiles/`/etc` files (`install_initramfs.rs:124-130,165-181` vs `systemd.rs:213-218,237-241`, `install.rs:161-184`), merged-usr symlinks (`install_initramfs.rs:111-117` vs `install.rs:252-257`).
- [hidden] `tools/recinit/src/tiny.rs:213-219` hard-codes a required-module list in error text (incl. `erofs`, not in `LIVE_MODULES`), duplicating `distro-spec/src/shared/modules.rs:19-41` and template comments (`init_tiny.template:51-53`).
- [hidden] `tools/recinit/src/install.rs:144` `copy_systemd` must run before `:149` `setup_busybox` (which skips existing links, `busybox.rs:172`), else applets shadow util-linux `mount`/`umount`/`sh`. Unwritten order.
- [hidden] `InstallConfig.modules_path` (`tools/recinit/src/install.rs:26-28`; CLI `main.rs:73-75`) actually means "a root containing `usr/lib/modules`" (`install.rs:108,283`); leviso passes `output_dir/staging` (`leviso/src/artifact/initramfs.rs:130-133`).
- [hidden] `leviso/src/rebuild.rs:145-170` hand-lists recinit sources as install-initramfs inputs, omitting `busybox.rs` and `distro-spec/src/shared/modules.rs`; `:130-139` live inputs ignore `tools/recinit/src/{tiny,modules,busybox}.rs`; `iso_needs_rebuild` (`:226-258`) ignores UKIs, `UKI_ENTRIES`, recuki/reciso and hard-codes five overlay paths.
- [hidden] `distro-builder/src/artifact/iso_utils.rs:76-82` `generate_iso_checksum` writes the name it's given; every caller passes `.tmp` and renames (`leviso/src/artifact/iso.rs:116,164-174`, `AcornOS/src/artifact/iso.rs:35,79`, `build-release.sh:322,357-363`), so `.sha512` contents name `*.iso.tmp`.
- [hidden] `build-release.sh:324-325,359-363` guesses where reciso put the checksum (ALT1/ALT2); `leviso/src/artifact/iso.rs:168-174` recomputes with `with_extension`.
- [hidden] `AcornOS/src/artifact/iso.rs:146-147` derives `base_dir = output_dir.parent()` = `.artifacts/out` (`distro-builder/src/artifact_store/mod.rs:755-770`), so `profile/live-overlay` is never found and the seed overlay is silently skipped.
- [hidden] `build-release.sh:293-300` sed-replaces the literal banner `msg "LevitateOS initramfs starting..."` from `tools/recinit/templates/init_tiny.template:108`.
- [hidden] `build-release.sh:307` runs `cargo run -p recinit`, resolvable only because deprecated `leviso`/`AcornOS` path-depend on it (`leviso/Cargo.toml:23`, `AcornOS/Cargo.toml:17`); `tools/recinit/.gitignore` ignores its own `Cargo.lock`.
- [hidden] `tools/recinit/src/cpio.rs:104-108` writes host inode/uid/gid/nlink and `:48` walks unsorted — non-reproducible, builder-owned initramfs despite the "mtime 0 for reproducibility" comment (`:109`).
- [hidden] `"live/filesystem.erofs"` in `build-release.sh:88-90,313`, `distro-spec/src/shared/iso.rs:118`, `tools/recinit/src/main.rs:51-52`, `tiny.rs:76`.
- [hidden] `staging/boot/vmlinuz` literal in `leviso/src/artifact/iso.rs:236,270`, `leviso/src/artifact/initramfs.rs:170`, `leviso/src/rebuild.rs:203,231`, `AcornOS/src/artifact/iso.rs:30`, `build-release.sh:31`; declared but unused in `distro-contract/src/build_host.rs:40`.
- [hidden] `tools/reciso/src/lib.rs:34,247,346,431,467,523` literals (`efiboot.img`, 200 MB, `EFI/Linux`, `EFI/BOOT/BOOTX64.EFI`) duplicate `distro-spec/src/shared/iso.rs:113,142,148,151`, which fsdbg uses to verify the same ISO.
- [hidden] `distro-contract/src/build_host.rs:17-30` omits what reciso/recinit actually call (`mmd`/`mcopy`, `mkfs.fat`, `dd`, `sha512sum`, `readelf`, `curl`, host `systemd-bootx64.efi`) and lists recinit/reciso/recuki as host tools though the release runs them via `cargo run`.
- [hidden] `xtask/src/tasks/tooling/hooks.rs:85-107` hard-codes the submodule list, missing `distro-contract`, `tools/recab`, `tools/recpart`, `tools/recguard`, `tools/recshuttle`, `shared/tui-kit`, `tui`.
- [hidden] fw_cfg `opt/levitate/boot-injection` and `boot-injection.env` (`init_tiny.template:334,354`) shared with `testing/install-tests/src/boot_injection.rs:7,143` and `xtask/src/tasks/testing/scenarios.rs:925,1274`.
- [hidden] `/live-boot-marker` (`init_tiny.template:308,317`) and `/run/live-media` (`:404`) are path contracts with `testing/install-tests/src/scenarios/mod.rs:1362`, `steps/phase3_base.rs:46`, `tools/recshuttle/src/lib.rs:373,563`, `distro-builder/src/artifact/live_overlay.rs:591-608`; legacy templates use `/media/cdrom`.
- [needless] `tools/recinit/Cargo.toml:23` "standalone rootless initramfs builder" depends on `distro-spec` policy (module lists, `LIVE_OVERLAYFS_ISO_PATH`, `BOOT_DEVICE_PROBE_ORDER`; `modules.rs:12`, `tiny.rs:77-78,92`).
- [needless] `tools/reciso/Cargo.toml:23` depends on the whole `distro-builder` for seven helpers (`lib.rs:20-23`) and on `distro-spec` for `SELINUX_DISABLE`, `LoaderConfig`, paths (`:24-27`).
- [needless] `tools/recinit/src/install.rs:161-164` bakes `NAME="LevitateOS initramfs"`, `ID=levitateos` into every install initramfs.
- [needless] `tools/reciso/src/lib.rs:350-353` adds RHEL-specific `SELINUX_DISABLE` (`distro-spec/src/shared/iso.rs:173-174`) to every distro's UKI.
- [needless] `leviso-elf/src/copy.rs:85-93` `copy_library_to` takes per-caller vocabulary (`dest_lib64_path`, `dest_lib_path`, `private_lib_dirs`); recinit always passes the same values (`tools/recinit/src/systemd.rs:126-130,261-264,540`).
- [needless] `leviso-elf/src/paths.rs:36-38` always searches `usr/lib64/systemd`, yet recinit passes it again (`systemd.rs:113,243`).
- [needless] `tools/recinit/src/lib.rs:39-42` + `src/elf.rs:5-8` re-export `leviso-elf` as recinit API (no outside users).
- [needless] `tools/reciso/src/lib.rs:30` re-exports recuki's `OsRelease`, `UkiConfig`, then copies fields one by one (`:390-392,412-414`).
- [needless] `leviso/src/artifact/iso.rs:137-142` creates `iso-staging/boot/uki`, an old reciso internal (now `iso-root`, `tools/reciso/src/lib.rs:204`).
- [needless] `build-release.sh:45` reaches into `tools/recinit/templates/init_tiny.template` instead of a recinit entry point.
- [needless] `tools/reciso/src/lib.rs:299,345` takes `systemd-bootx64.efi` from the build host, not the target rootfs (leviso qcow2 prefers rootfs, `leviso/src/artifact/qcow2/partitions.rs:27-28`).
- [needless] `tools/recinit/src/install.rs:147-148` library function downloads busybox from the network.
- [needless] Install payload split by layer between `tools/recinit/src/install.rs:93-223` (`/etc` files for systemd/udev, `:157-184`) and `systemd.rs:176-285` — the 5× co-change.
- [needless] Module selection split between `tools/recinit/src/modules.rs:31-43,193-201` and `tiny.rs:201,248-253` (names→paths→names) — the 4× co-change.
- [needless] Library entry points (`TinyConfig`, `InstallConfig`, `IsoConfig`, `UkiConfig`) serve only deprecated `leviso`/`AcornOS` (`leviso/Cargo.toml:5`, `AcornOS/Cargo.toml:5`); the active pipeline uses the CLIs (`build-release.sh:307,350`), and the two surfaces expose different knobs.
- [surplus] `tools/recinit/src/lib.rs:50-98` `InitramfsBuilder` only forwards `verbose`; main.rs-only.
- [surplus] `tools/recinit/src/lib.rs:103-126` `verify_initramfs` unused; 1 KB threshold duplicates `tiny.rs:157`, `install.rs:200`.
- [surplus] `tools/recinit/src/lib.rs:101` `DEFAULT_GZIP_LEVEL = 9` duplicates `distro-spec/src/shared/initramfs.rs:42` and CLI `"9"` defaults (`main.rs:59,86`).
- [surplus] `tools/recinit/src/elf.rs:1-8` pure re-export module.
- [surplus] `tools/recinit/src/lib.rs:38` `pub use cpio::build_cpio` no external users; `distro-builder/src/artifact/cpio.rs:34-49` second `build_cpio`, no users.
- [surplus] `tools/recinit/src/modules.rs:46-52` `ModulePreset::module_names` unused.
- [surplus] `tools/recinit/src/busybox.rs:149-153` `commands: Option<&[&str]>` always `None`.
- [surplus] `tools/recinit/src/tiny.rs:44-48,78` legacy `live_overlay_path` set equal to `live_overlay_image_path` by both callers.
- [surplus] `tools/recinit/src/tiny.rs:68-86` `TinyConfig::default` nonsense defaults reached only by the CLI.
- [surplus] `tools/recinit/src/install.rs:43-54` `InstallConfig::default` unused.
- [surplus] `tools/recinit/src/main.rs:180-187` re-implements `ModulePreset::parse_name` (`modules.rs:55-61`).
- [surplus] `tools/recinit/src/main.rs:212` `modules.is_empty()` unreachable.
- [surplus] `tools/recinit/src/systemd.rs:288-373` second tuple element of `INITRD_WANTS_SYMLINKS` always derivable; recomputed at `:393`.
- [surplus] `tools/recuki/src/lib.rs:172-195` `build_uki_simple`, `build_uki_branded` no callers.
- [surplus] `tools/recuki/src/lib.rs:63-70` `UkiConfig::with_os_release` duplicates `OsRelease::new` (`:75-81`).
- [surplus] `tools/reciso/src/lib.rs:119-172` builder methods `with_prebuilt_uki`, `with_uki`, `with_extra_file`, `without_checksum`, `with_live_payload_iso_files` uncalled (callers set fields directly).
- [surplus] `tools/reciso/src/lib.rs:145-148` `with_overlay` pass-through alias.
- [surplus] `tools/reciso/src/main.rs:183-186,218-226` `RecisoPayloadLayoutArg` mirrors `LivePayloadLayout`.
- [surplus] `tools/reciso/src/lib.rs:439-441` `entry_id` appends `.conf` then it's trimmed.
- [surplus] `tools/recuki/Cargo.toml:16-19`, `tools/reciso/Cargo.toml:19-22` exclude nonexistent `.github/*`.
- [surplus] `AcornOS/src/artifact/uki.rs:57-131` `build_live_ukis`, `build_installed_ukis` exported but never called; duplicate reciso's UKI loop.
- [surplus] `AcornOS/src/artifact/uki.rs:29-36` and `leviso/src/artifact/uki.rs:31-38` identical thin wrappers over `recuki::build_uki`.
- [surplus] `leviso/src/artifact/initramfs.rs:44-47` `busybox_url` duplicates `tools/recinit/src/busybox.rs:122`.
- [surplus] `leviso/src/rebuild.rs:196-284` "backwards compatible" wrappers; qcow2 ones `#[allow(dead_code)]`.
- [surplus] Unused distro-spec constants: `distro-spec/src/shared/udev.rs:39-99`, `modules.rs:266-272`, `iso.rs:154,167`, `initramfs.rs:26-35`.
- [surplus] `distro-spec/src/shared/iso.rs:134-135` `LIVE_OVERLAY_ISO_PATH` legacy alias still fed to recinit.
- [surplus] `distro-builder/src/artifact/iso_utils.rs:204-217` `create_efi_boot_image` unused.
- [surplus] `distro-builder/src/artifact/initramfs.rs` (99 lines) self-described placeholder parallel to recinit.
- [surplus] `testing/fsdbg/src/checklist/live_initramfs.rs:111` `INIT_SHEBANG` unused and wrong.
- [surplus] `tools/recinit/templates/init_tiny.template:79,85,98-100` `DEBUG` set but never read; `debug()` always prints.
- [special] `tools/recinit/templates/init_tiny.template:112-123` sleep 1 s, then 2 s more if neither `/dev/sr0` nor `/dev/sda`; same in both legacy templates.
- [special] `init_tiny.template:140` any device with `/boot` accepted as boot media.
- [special] `init_tiny.template:229` always `/dev/loop0`; a second file-backed payload falls through to direct mount (`:230-241`).
- [special] `init_tiny.template:310-317` "TWO-LAYER (shouldn't happen from ISO)" branch.
- [special] `init_tiny.template:86,325-385` QEMU-test boot-injection channel (fw_cfg, `/dev/sr1 /dev/sr2 /dev/sdb /dev/vdb`, "Stage 01 SSH key handoff") baked into production live init.
- [special] `init_tiny.template:37-39` redirects all output to `/dev/ttyS0` when it exists (AcornOS too; leviso lacks it).
- [special] `AcornOS/profile/init_tiny.template:255-261,270-276` mounts efivarfs in initramfs and falls back to `openrc-init`; leviso drops efivarfs (`leviso/profile/init_tiny.template:248-250`).
- [special] `tools/recinit/src/systemd.rs:146-166` symlinks `libsystemd-*.so*` — RHEL versioned-private-lib workaround (`257-13.el10.rocky`).
- [special] `tools/recinit/src/systemd.rs:425-472` strips `ConditionPathIsReadWrite=/sys` from four udev units by line match.
- [special] `tools/recinit/src/systemd.rs:136-144` copies `ld-linux-x86-64.so.2`, `ld-linux.so.2` by name, x86 only.
- [special] `tools/recinit/src/systemd.rs:97,117,192,221,248,398-400,446,522` silently skip absent source files.
- [special] `tools/recinit/src/install.rs:166-184` hand-writes passwd/group/shadow/nsswitch with fixed udev ids.
- [special] `tools/recinit/src/install.rs:290` forces `builtin_check = true`; tiny makes it configurable (`tiny.rs:207`).
- [special] `tools/recinit/src/install.rs:293-305` missing install modules printed only when verbose; tiny bails (`tiny.rs:211-222`).
- [special] `tools/recinit/src/install.rs:216-219` size warning only when verbose.
- [special] `tools/recinit/src/busybox.rs:118-134` cached busybox not re-verified; verification skipped whenever `BUSYBOX_URL` is set.
- [special] `tools/recinit/src/modules.rs:100-112` missing `modules.builtin` silently disables the builtin check.
- [special] `tools/recinit/src/modules.rs:152-159` copies every `modules.*` though `distro-spec/src/shared/modules.rs:238-250` `MODULE_METADATA_FILES` curates them.
- [special] `tools/recinit/src/cpio.rs:64` drops `GzEncoder::finish()` result; trailer write errors swallowed.
- [special] `tools/recinit/src/tiny.rs:185-190` writes a `/dev/.note` text file.
- [special] `leviso-elf/src/analyze.rs:41-48` readelf failures = "no deps" by matching five stderr substrings.
- [special] `leviso-elf/src/copy.rs:146-148` dangling symlink copied as-is; `:45-47` `copy_dir_recursive` returns `Ok(0)` for non-dirs, relied on by `tools/recinit/src/systemd.rs:200-202`.
- [special] `tools/recuki/src/lib.rs:125,139,155-158` temp `.cmdline`/`.os-release` next to output; removal errors swallowed.
- [special] `tools/recuki/src/lib.rs:149-152`, `main.rs:17-30` `dnf`-only install hints; no fallback when ukify is missing.
- [special] `tools/reciso/src/lib.rs:406-428` implicit default UKI `<label>.efi` when none configured.
- [special] `tools/reciso/src/lib.rs:477-509` `mmd` exit status ignored.
- [special] `tools/reciso/src/lib.rs:199-204,258,467,526` fixed temp names in the output dir (concurrent builds collide); cleanup errors swallowed.
- [special] `tools/reciso/src/lib.rs:34,470` EFI image fixed at 200 MB regardless of UKI count/size.
- [special] `tools/reciso/src/lib.rs:442-451,535-549` explicit loader entries for UKIs systemd-boot already auto-discovers.
- [special] `tools/reciso/src/main.rs:130-145` `--build-uki name:cmdline:filename` via `splitn(3, ':')`; `build-release.sh:342-344` puts product labels in the name.
- [special] `tools/reciso/src/main.rs:107-109`, `tools/recuki/src/main.rs:61-63` `--quiet` doesn't silence library `println!` (`tools/reciso/src/lib.rs:207,227,253,260,313,394`).
- [special] `build-release.sh:347` forces `--live-payload-layout iso-files`, so the template's partition-probe path (`init_tiny.template:173-201`) runs only for deprecated callers.
- [special] `build-release.sh:242-245` modules dir fallback; `:282` 3× curl retry with 2 s; `:292` appends a `# product-artifact-tag:` comment; `:34,323` defines `ISO_SHA` twice.
- [special] `leviso/src/artifact/initramfs.rs:116-148,166-205` rootfs from `downloads/rootfs` ("rootfs-staging lacks initrd.target"), modules "custom" vs "Rocky" by presence of `staging/boot/vmlinuz`, `include_firmware: false`.
- [special] `leviso/src/artifact/iso.rs:143-161,177` missing installed UKIs skipped; hardware-compat critical failures warn and continue; cleanup error swallowed.
- [special] `AcornOS/src/artifact/iso.rs:97-101` verification "SKIPPED" with `Ok` on any `isoinfo` error.
- [special] `distro-spec/src/shared/modules.rs:18` says EROFS is built in, yet `tools/recinit/src/tiny.rs:217` and templates demand the module.
- [comment] `tools/recinit/src/tiny.rs` 13, `install.rs` 16, `systemd.rs` 15, `modules.rs` 7 (+`:64-75` args block), `cpio.rs` 10 (`:15` wrong), `busybox.rs` 4, `lib.rs` 6 (`:11-24` stale doc example), `main.rs` 2 restating comments.
- [comment] Stale template headers: `tools/recinit/templates/init_tiny.template:10-20,398-400` ("GRUB loads kernel", no `set -x`, points at `.teams/TEAM_114`); same in `AcornOS/profile/init_tiny.template:8-21` and `leviso/profile/init_tiny.template`.
- [comment] `tools/recuki/src/lib.rs` 7, `main.rs` 2; `tools/reciso/src/lib.rs` 25 "Stage N" (`:177-186` doc says rootfs copied into ISO); `tools/reciso/src/main.rs` 7.
- [comment] `leviso/src/artifact/{initramfs,uki,iso}.rs`, `AcornOS/src/artifact/{initramfs,uki,iso}.rs` ~40 restating/stale comments (`/live/overlay` flow); `leviso-elf/src/copy.rs` 8; `distro-builder/src/artifact/iso_utils.rs:27-34,106-113,232-242` wrong/ignored doc examples; `distro-spec/src/shared/udev.rs:4-16`, `modules.rs:5-7,265` stale consumer notes.
- [test] recinit, recuki, reciso have no tests; the active release path (`build-release.sh:314,318,352`) only checks files exist, so placeholder/layout mismatches go undetected.
- [test] `testing/fsdbg/src/checklist/live_initramfs.rs:272-300` can't fail (`LIVE_MODULES_BUILTIN` ⊇ `LIVE_MODULES`).
- [test] `testing/fsdbg/src/checklist/install_initramfs.rs:474-503` can't fail (`INSTALL_MODULES_BUILTIN` == `INSTALL_MODULES`; `/{module}/` substring match).
- [test] `testing/fsdbg/src/checklist/install_initramfs.rs:317-323` any regular `/init` passes; `live_initramfs.rs:247-253` any `/init` symlink passes.
- [test] `testing/fsdbg/src/checklist/install_initramfs.rs:99-102` slice checks commented out; `:33,41` omit shipped binaries; `:201-218` never checks the wants link it says must be absent; `:513-518` skips resolving same-name symlinks.
- [test] fsdbg DIRS/APPLETS/BINARIES/UNITS/WANTS lists are hand copies of recinit constants — they catch copy drift, not wrong contents.
- [test] `AcornOS/src/artifact/initramfs.rs:91-99` module check commented out ("TODO").
- [test] `AcornOS/src/artifact/iso.rs:107-111` requires `/live/overlay` (no build produces it) and `/live/filesystem.erofs` (absent under reciso's default).
- [test] `leviso/src/artifact/iso.rs:156-161` hardware-compat verification warn-only.
- Cross-area notes from this area:
  - [hidden] `distro-builder/src/artifact/{cpio,initramfs,filesystem}.rs` duplicate recinit pieces; `distro-builder/src/artifact/live_overlay.rs:143,702` owns the `"overlay"` role string the template reads.
  - [hidden] `testing/fsdbg/src/checklist/iso.rs:89-103` requires rootfs/overlay inside the ISO fs (conflicts with reciso's default); `:91` comment disagrees with `INITRAMFS_LIVE_ISO_PATH`.
  - [hidden] `distro-spec/src/{levitate,acorn}/uki.rs` each define entries (consoles only in acorn's); `build-release.sh` re-derives them from env + `merge_uki_cmdline` (`distro-variants/_shared/ring0/hooks/release-artifacts.sh:103-131`).
  - [hidden] Host-tool lists in `leviso/src/preflight/host_tools.rs:20-29`, `AcornOS/src/preflight/host_tools.rs:21`, `distro-contract/src/build_host.rs:17-30`, `distro-variants/*/build-host/build-host.toml`.
  - [hidden] leviso qcow2 (`leviso/src/artifact/qcow2/partitions.rs:27-28`, `qcow2/mtools.rs`) has its own systemd-boot + mtools path beside reciso.
  - The recinit+reciso+recuki co-change (7–8×) matches two consumer families: deprecated leviso/AcornOS (library API) and `build-release.sh` (CLIs); both encode reciso/recinit internals.

- Added by completeness pass 2 (fresh auditor; install.rs line numbers re-anchored to the real file):
  - [hidden] `tools/recinit/templates/init_tiny.template:259-261` now says "Rootfs payload not found (missing partition role …)", but the install-tests fail-fast pattern is still `"Rootfs payload partition not found"` (`testing/install-tests/src/distro/levitate.rs:69,109`, `openrc_base.rs`) — that boot failure no longer fails fast (verified).
  - [needless] `tools/recinit/src/tiny.rs:21-66` `TinyConfig` built with full struct literals by both callers; every new field (`914e2ad`, `4091fc6`) forces edits in leviso and AcornOS.
  - [hidden] `tools/recinit/src/tiny.rs:91-96` `default_boot_devices` is `pub` in a private module and not re-exported, so leviso and AcornOS each redo the `BOOT_DEVICE_PROBE_ORDER` → `Vec<String>` conversion.
  - [hidden] `tools/recinit/src/tiny.rs:119-163` and `install.rs:116-208` copy the same build skeleton (scratch root beside output, clean, cpio to `.tmp`, `<1024` check, rename, remove).
  - [hidden] `tools/recinit/src/modules.rs:96` writes `lib/modules/<kver>`; install builds rely on the `lib → usr/lib` symlink created earlier (`install.rs:242-266`) and the tiny template reads `/lib/modules` (`init_tiny.template:46`) — unstated layout/order convention.
  - [hidden] `tools/recinit/src/busybox.rs:18-47` `BUSYBOX_COMMANDS` claims to be what the init script uses, but the template calls everything as `busybox X` and also uses `touch`, `tr`, `rmdir`, which aren't listed.
  - [hidden] `tools/recinit/src/busybox.rs:57-61` curl runs without `--fail`, so an HTTP error page can be saved as busybox (verified); leviso calls `download_busybox` directly (`leviso/src/artifact/initramfs.rs:67-70`) without a checksum.
  - [hidden] Busybox cache filename `busybox-static` in `tools/recinit/src/busybox.rs:116`, leviso `downloads_dir.join("busybox-static")`, and `build-release.sh:47`.
  - [surplus] `tools/recinit/src/install.rs:57-91` `INSTALL_DIRS` creates `bin`, `sbin`, `lib`, `lib64`, which `create_install_directory_structure` (`:242-266`) then removes to make symlinks.
  - [hidden] `tools/recinit/src/install.rs:105-109,129,283` the `usr/lib/modules` path and the `modules_path`-vs-`rootfs` choice computed three times.
  - [hidden] `tools/recinit/src/install.rs:154` init symlink target `/usr/lib/systemd/systemd` must match `SYSTEMD_FILES[0]` (`systemd.rs:15`).
  - [hidden] `tools/recinit/src/systemd.rs:113,123-131,243,257-265,515,533-541` the 7-argument `copy_library_to(..., "usr/lib64", "usr/lib", …)` call and `extra_lib_paths` value repeated three times.
  - [hidden] `tools/recinit/src/systemd.rs:146-166` libsystemd symlinks exist to undo leviso-elf's `private_lib_dirs` placement (`leviso-elf/src/copy.rs:101-110`).
  - [hidden] `tools/recinit/src/systemd.rs:437-442` `patch_udev_units` repeats unit names from `INITRD_UNITS` and `INITRD_WANTS_SYMLINKS`.
  - [hidden] `tools/recinit/templates/init_tiny.template:117,155` block-device glob list repeated, and differs from `{{BOOT_DEVICES}}`.
  - [special] `tools/recinit/templates/init_tiny.template:62-66` insmod failure logged as "already loaded or failed" and boot continues.
  - [surplus] `tools/recinit/templates/init_tiny.template:86,383-385` `boot_injection=required` cmdline flag — no producer passes it.
  - [special] `tools/recinit/templates/init_tiny.template:224-256` nested `case` with three branches running the same `mount -t erofs`.
  - [special] `tools/recinit/templates/init_tiny.template:322-323` tmpfs mounts `|| true`.
  - [hidden] `tools/recinit/templates/init_tiny.template:372-375` `SSH_AUTHORIZED_KEY(S)` payload keys and `;` separator are a protocol shared with payload writers in install-tests and xtask.
  - [hidden] `tools/recinit/templates/init_tiny.template:186,334,351` payload partitions 2–4 and injection devices `/dev/sr1 /dev/sr2 /dev/sdb /dev/vdb` depend on recqemu's `cdparts0` scsi-hd view and extra-CD numbering (`tools/recqemu/src/lib.rs:303-329`).
  - History: the template is the most-changed recinit file and every change is a cross-repo boot-protocol change (`eef2671` payload roles + symlinks, `fd14dcc` boot-injection channel, `914e2ad` overlay rename, `ca51d5d` `/squashfs`→`/rootfs`) needing matching edits in reciso/distro-builder, recqemu, install-tests/xtask and build-release.sh, plus two forked templates. Install-initramfs lists grow one item per boot failure (`1be2c66`, `d2db50a`, `f7d0260`), each with drifting twins in fsdbg and distro-spec. Struct-literal callers force leviso/AcornOS edits per new field, leaving legacy aliases.

### D. VM + remote (recqemu, recshuttle) and their re-wrappers

- [hidden] `tools/recqemu/src/serial/sync.rs:21-22` assumes guest echo is off via `stty -echo`; the levitate guest script (`distro-spec/src/shared/auth/files/00-levitate-test.sh:1-30` and leviso copy) never runs `stty` (verified); only `distro-variants/acorn/ring2/overlays/live/etc/profile.d/00-acorn-test.sh:25` does.
- [hidden] `tools/recqemu/src/serial/exec.rs:28` `PROMPT_MARKER`, `sync.rs:42-43`, `boot.rs:28` hard-code `___PROMPT___`/`___SHELL_READY___`, which must match `00-levitate-test.sh:23,27,30` (×2 copies), `distro-builder/src/artifact/live_overlay.rs:654-667` (inline `00-live-test.sh`) and the acorn scripts.
- [hidden] distro-builder's live ISO installs `00-live-test.sh` (`distro-builder/src/artifact/live_overlay.rs:653-668`) which emits only `___SHELL_READY___`, never `___PROMPT___`, so every recqemu `exec` (`exec.rs:75`) waits out 500 ms. Three guest marker sets (levitate, live, acorn); recqemu assumes levitate.
- [hidden] `distro-spec/src/shared/auth/files/00-levitate-test.sh` and `leviso/src/component/custom/live/overlay/etc/profile.d/00-levitate-test.sh` are byte-identical (verified); consumed by `testing/install-tests/src/distro/levitate.rs:215` and `leviso/src/component/custom/live/mod.rs:16,75`.
- [hidden] `tools/recqemu/src/serial/sync.rs:26-27` defines `___START_n___`/`___DONE_n___`; `sync.rs:37-38` re-spells prefixes for filtering; `exec.rs:81-84,129-157` rely on ` $?` digits after the done marker.
- [hidden] `tools/recqemu/src/serial/sync.rs:39-45`, `exec.rs:11-12` filter `___SYNC_`, `___SYNC2_`, `___CMD_START_`, `___CMD_END_` that no LevitateOS guest emits.
- [hidden] `tools/recqemu/src/serial/exec.rs:14-15,33` refer to a `sync_shell()` protocol that no longer exists.
- [hidden] `tools/recqemu/src/serial/boot.rs:49-53` installed-boot success patterns duplicate `distro-variants/levitate/scenarios/scenarios.toml:54`, which is what `testing/install-tests/src/qemu/serial/mod.rs:111-124` actually uses.
- [hidden] `tools/recqemu/src/patterns.rs:19-54` CRITICAL_BOOT_ERRORS and `:58-92` BOOT_ERROR_PATTERNS are hand-copied (`:59` "Include all critical errors"); copied again (already drifted) in `testing/install-tests/src/distro/levitate.rs:37-115`, `distro/openrc_base.rs:14-80`, `leviso/src/qemu.rs:28-39`, `distro-builder/src/qemu.rs:20-31`.
- [hidden] `tools/recqemu/src/patterns.rs:96` SERVICE_FAILURE_PATTERNS duplicated in `testing/install-tests/src/distro/levitate.rs:117-119`, `openrc_base.rs:83`; DistroContext copies never reach recqemu (`boot.rs:154,209`).
- [hidden] `tools/recqemu/src/serial/boot.rs:137-181` and `:191-242` — phase detection, service-failure tracking, error dump and success+sleep duplicated line for line inside one function.
- [hidden] `leviso/src/qemu.rs:153-162` re-implements recqemu's reader thread (`console.rs:71-78`); `:164-284` re-implements `wait_for_boot_with_patterns` with its own constants (30 s stall `:167`, 100 ms poll `:214`).
- [hidden] "receive line → push to `output_buffer`" on `Console`'s `pub(crate)` fields repeated in `tools/recqemu/src/serial/auth.rs:227-233,266-271,343-345`, `exec.rs:41-42,53,106`, `boot.rs:133-135,188-190`.
- [hidden] `tools/recqemu/src/serial/auth.rs:229-232,267-270` duplicate "keep last 50" trimming; context-dump tail lengths differ per site (`boot.rs:113,165,225,247`, `auth.rs:194`: 30, 30, 30, 20, 25).
- [hidden] `tools/recqemu/src/lib.rs:441-464` and `:469-503` carry the same per-distro OVMF candidate tables (`:450-461`, `:489-502`); copied in `distro-builder/src/qemu.rs:194-226`, `xtask/src/util/repo.rs:46-49`, `distro-builder/src/recipe/qemu.rs:23-24`, `justfile:8-9`; `leviso/src/qemu.rs:75-81` hard-codes install hints; `tools/recqemu/README.md` lists another stale set.
- [hidden] `tools/recqemu/src/lib.rs:443,470,477` read `OVMF_PATH`/`OVMF_VARS_PATH`, set by `justfile:8-9` and `xtask/src/util/tools_env.rs:23` (only `OVMF_PATH`), so VARS falls back to a sibling-file guess (`:477-487`); `:442` wrongly says the recipe sets it.
- [hidden] `create_disk` (`tools/recqemu/src/lib.rs:511-536`) and `find_ovmf` (`:441`) hand-rolled again in `AcornOS/src/qemu.rs:47-54` and `xtask/src/tasks/testing/scenarios.rs:765,899`.
- [hidden] `/dev/kvm` probe in `tools/recqemu/src/lib.rs:87,539-541`, repeated or skipped (unconditional `-enable-kvm -cpu host`) in `distro-builder/src/qemu.rs:101,272`, `AcornOS/src/qemu.rs:29`, `xtask/src/tasks/testing/scenarios.rs:769-771,902-904`.
- [hidden] `tools/recqemu/src/lib.rs:292-315` attaches the ISO twice (scsi-cd `cdrom0` + scsi-hd `cdparts0`) and starts extra CD-ROMs at `scsi2`; `xtask/src/tasks/testing/scenarios.rs:909-917,931-939` copies the layout without `cdparts0` but still uses `scsi2`; `distro-builder/src/qemu.rs:129-132` uses AHCI.
- [hidden] hostfwd `127.0.0.1` (`tools/recqemu/src/lib.rs:363`), `root@127.0.0.1` (`tools/recshuttle/src/lib.rs:61`), guest port 22 (`testing/install-tests/src/qemu/session.rs:50,104`) must agree; nothing ties them.
- [hidden] `tools/recqemu/src/serial/console.rs:53-55` `Console::new` needs a child from `build_piped` (`lib.rs:420-426`, `testing/install-tests/src/qemu/builder.rs:148-156`); `leviso/src/qemu.rs:131-146` builds its own stdio and can't use Console.
- [hidden] `tools/recqemu/src/lib.rs:48` and `testing/install-tests/src/qemu/builder.rs:21` derive `Default` that differs from `new()` (no KVM detection, no `.nodefaults()`).
- [hidden] Two calling conventions for recchroot: `tools/recqemu/src/serial/chroot.rs:24` (`recchroot '<p>' /bin/bash -c '<cmd>'`) vs `tools/recshuttle/src/lib.rs:383,399,406,527,543,550` (`recchroot /mnt/sysroot -- <cmd>`); acorn uses ash (`testing/install-tests/src/distro/openrc_base.rs` `chroot_shell`).
- [hidden] `tools/recqemu/src/serial/utils.rs:29` assumes bash (`set +H`) and leaves the path unquoted.
- [hidden] `tools/recshuttle/src/lib.rs:403-409,547-553` wrap `enable_serial_getty_cmd` in `sh -c '{}'`; `testing/install-tests/src/distro/openrc_base.rs:103-105` passes single quotes, breaking the redirect. Same for password (`:397-402,541-546`) and hostname (`:390-395,534-539`).
- [hidden] `tools/recshuttle/src/lib.rs:558-566` inserts "Copy initramfs" at fixed index 6 — right only for MutableSingleRoot (`:344-410`); in ImmutableAb (`:462-554`) it lands before "Mount boot".
- [hidden] `/mnt/sysroot/boot/loader/entries/{id}-a.conf`/`-b.conf` built in `tools/recshuttle/src/lib.rs:266-267` and `:417-424`, and again in `testing/install-tests/src/scenarios/mod.rs:587-604`.
- [hidden] `tools/recshuttle/src/lib.rs:254-303` checks labels `system-a`/`system-b`/`var` and indices 2/3/4 that must match its own mkfs (`:476-484`) and sfdisk (`:466`).
- [hidden] `console=tty0 console=ttyS0,115200n8` repeated 6× in `tools/recshuttle/src/lib.rs:328,337,434,441,449,456`; must match `00-levitate-test.sh:9`, recqemu `-serial mon:stdio` (`lib.rs:393`) and DistroContext getty commands.
- [hidden] `tools/recshuttle/src/lib.rs:373,503,563` hard-code live ISO layout (`/run/live-media/boot/vmlinuz`, `initramfs.img`) in an SSH transport crate.
- [hidden] `tools/recshuttle/src/lib.rs:140-161` `run_install_plan` and `:205-223` `verify_checks` are the same loop.
- [hidden] `tools/recshuttle/src/lib.rs:111-138` `wait_ready` and `testing/install-tests/src/scenarios/mod.rs:927-948` `verify_live_boot_ssh_login` are two copies of one SSH readiness poll.
- [hidden] SSH key resolution with `~/.ssh/id_ed25519` fallback in `tools/recshuttle/src/lib.rs:29-37` and `xtask/src/tasks/testing/scenarios.rs:716-735` (only recshuttle honours `LEVITATE_SSH_PRIVATE_KEY`); different SSH option sets (`recshuttle :47-61` vs `xtask :737-751`).
- [hidden] `testing/install-tests/src/scenarios/mod.rs:950-1018` and `xtask/src/tasks/testing/scenarios.rs:1310-1336` — two copies of the guest SSH debug probe.
- [hidden] `testing/install-tests/src/scenarios/mod.rs:505-509` matches issue-text strings emitted by `distro-builder/src/artifact/live_overlay.rs:136,645`, `distro-builder/src/pipeline/overlay.rs:329,353`.
- [hidden] `testing/install-tests/src/scenarios/mod.rs:564,571` compute `include_initramfs` as `init_system_name() != "OpenRC"` twice; verify list (`:572-582`) must agree with recshuttle's insert (`lib.rs:558-566`).
- [hidden] `testing/install-tests/src/scenarios/mod.rs:552-561,705-715,744-754` repeat "load facts → default_password or bail".
- [hidden] `testing/install-tests/src/scenarios/mod.rs` kill/wait paired by hand (`:346-347,493-494,623-625,667-668,720-721,779-780`); in `run_automated_login` (`:702,716,719`) and `run_daily_driver_tools` (`:743,755,771`) a `?` returns before kill, leaking QEMU.
- [hidden] `testing/install-tests/src/qemu/session.rs:40-137` four near-identical spawn functions (builder chain, boot injection, spawn, `Console::new`, 2 s sleep).
- [hidden] Port allocation by bind-and-drop in `testing/install-tests/src/qemu/session.rs:150-159` and `xtask/src/tasks/testing/scenarios.rs:1219`.
- [hidden] `testing/install-tests/src/qemu/serial/mod.rs:31-49` copies `recqemu::CommandResult` into `ExecResult`; `testing/install-tests/src/executor.rs:11-29` re-declares the same five fields and `success()` as `tools/recqemu/src/serial/console.rs:21-38`.
- [hidden] Two boot-wait paths with different pattern sources: `testing/install-tests/src/qemu/serial/mod.rs:60-66` (recqemu hard-coded) vs `:91-125` (DistroContext + contract).
- [hidden] `xtask/src/tasks/testing/scenarios.rs:925` hard-codes `opt/levitate/boot-injection` (owned by `testing/install-tests/src/boot_injection.rs:7`); `:580` `___SHELL_READY___`; `:609-658` re-implements recqemu's private `strip_ansi_codes` (`tools/recqemu/src/serial/ansi.rs:15-88`).
- [hidden] `tools/recqemu/src/process.rs:17-21` kills by filenames current producers no longer create (`leviso-install-test.qcow2`, `levitateos.iso`; real: `disk.qcow2` at `testing/install-tests/src/scenarios/mod.rs:35`, `levitateos-x86_64.iso` at `distro-spec/src/levitate/paths.rs:102`); `process.rs:46,74` hard-code "leviso".
- [hidden] QEMU sizing defaults: `tools/recqemu/src/lib.rs:36-42` (4G, 4, 20G) vs `distro-spec/src/shared/qemu.rs:11,17` (8 GB, 256 GB, used by `leviso/src/qemu.rs:64,86`), `testing/install-tests/src/scenarios/mod.rs:526` (20G), `bin/qmp.rs:77` (10G), `xtask/src/tasks/testing/scenarios.rs:772-775,905-908`; `leviso/src/qemu.rs:63` comment stale.
- [hidden] `leviso/src/qemu.rs:107-108` says AHCI CD-ROM; recqemu builds virtio-scsi (`lib.rs:291-313`).
- [hidden] `testing/install-tests/src/distro/levitate.rs:32` cites `00-live-test.sh` as the `___SHELL_READY___` source while `tools/recqemu/src/serial/boot.rs:26` cites `00-levitate-test.sh`.
- [hidden] `testing/install-tests/src/executor.rs:3-4,31-34` says QMP implements Executor; `qemu/qmp/mod.rs:25-30` says it must not; `bin/qmp.rs:4` points to a command `bin/install-tests.rs:56-64` disables.
- [hidden] `tools/recqemu/README.md`, `CLAUDE.md`, and `docs/content/src/content/05-rec-tooling/11-recqemu.ts` describe an API different from `src/`.
- [needless] `tools/recqemu/src/lib.rs:520-524` generic library error text hard-codes superproject tooling (`recipe install --build-dir .artifacts/tools ... qemu-deps`); drifted from `xtask/src/util/repo.rs:12-14`.
- [needless] `tools/recqemu/src/serial/boot.rs:22-58` LevitateOS success patterns (`levitateos login:`) in the generic Console.
- [needless] `tools/recqemu/src/process.rs:16-80` install-test vocabulary (filenames, `/tmp/leviso-install-test.lock`, "Another install-test is already running") in the generic crate.
- [needless] `tools/recqemu/src/serial/boot.rs:78` `track_service_failures: bool` toggles two behaviours only the installed-boot workflow understands.
- [needless] `tools/recqemu/src/serial/boot.rs:92-98` sends a newline every 3 s "for headless OpenRC paths" for every caller.
- [needless] `tools/recqemu/src/serial/exec.rs:113-126` checks boot fatal patterns against every command's output.
- [needless] `tools/recqemu/src/serial/console.rs:43-48` `pub(crate)` fields manipulated directly by `auth.rs`, `boot.rs`, `exec.rs`.
- [needless] `tools/recqemu/src/serial/console.rs:48`, `boot.rs:61-63,89` per-boot `failed_services` stored as long-lived Console state.
- [needless] `tools/recshuttle/src/lib.rs:6-21,254-576` distro install policy (layouts, entries, labels, fstab, getty, hostname, password) in an "SSH transport helpers" crate (`Cargo.toml:5`).
- [needless] `tools/recshuttle/src/lib.rs:13-21` `InstallPlanSpec` takes raw shell fragments and an `include_initramfs` bool meaning "not OpenRC" (`testing/install-tests/src/scenarios/mod.rs:564`).
- [needless] `tools/recshuttle/src/lib.rs:143` `println!` progress inside a library loop.
- [needless] `testing/install-tests/src/qemu/session.rs:32,41` take a DistroContext they ignore.
- [needless] `leviso/src/qemu.rs:13-15,43,113` QEMU launcher depends on `distro_builder::artifact_store` and distro-spec to find the ISO.
- [surplus] `testing/install-tests/src/qemu/builder.rs:39-140` 17 setters forwarding to `recqemu::QemuBuilder` (two renamed).
- [surplus] `testing/install-tests/src/qemu/builder.rs:148-156,184-189` re-implement `build_piped` (`tools/recqemu/src/lib.rs:420-426`).
- [surplus] `testing/install-tests/src/qemu/builder.rs:39-56,176-212` `.kernel()`, `.initrd()`, `.append()`, `build_direct_boot_debug()` uncalled; the "anti-cheat" panic guards an impossible combination.
- [surplus] `testing/install-tests/src/qemu/patterns.rs:1-5` re-export nobody imports.
- [surplus] `testing/install-tests/src/qemu/serial/mod.rs:18` and `tools/recqemu/src/serial/mod.rs:24` export `generate_command_markers`, `is_marker_line`; no outside callers.
- [surplus] recqemu's `create_disk`, `find_ovmf`, `find_ovmf_vars`, `kill_stale_qemu_processes`, `acquire_test_lock` re-exported across three layers (`testing/install-tests/src/qemu/builder.rs:13,16`, `qemu/mod.rs:21-24`, `lib.rs:37-40`) while `session.rs:18-19,44` and `scenarios/mod.rs:526,659,694,735` call `recqemu::` directly.
- [surplus] `testing/install-tests/src/qemu/serial/mod.rs:29-71` `impl Executor for Console` reachable only from `Step::execute`, which never runs (`bin/install-tests.rs:56-64`); so recqemu `wait_for_boot` (`boot.rs:21-33`), `wait_for_installed_boot` (`:39-59`), `failed_services` (`:61-63`), `exec_chroot` (`chroot.rs:16-29`), `write_file` (`utils.rs:15-32`) are effectively dead (`#[allow(dead_code)]` at `boot.rs:21,39`).
- [surplus] `tools/recqemu/src/serial/console.rs:81-88` `replay_output_buffer`, `:100-130` `attach_stdio` — no callers.
- [surplus] `tools/recqemu/src/serial/auth.rs:48-70,178-208` `AuthConfig`/`authenticate_with_config` speculative (private, always defaults).
- [surplus] `tools/recqemu/src/serial/auth.rs:87-88,108-143,359-362,386-391` unreachable states (`AuthResult.success`, `LoginState::Complete` "should not reach here", loop tail).
- [surplus] `tools/recqemu/src/process.rs:41-80` `acquire_test_lock` no callers (verified: only re-exports); `:56-61` `#[cfg(not(unix))]` speculative.
- [surplus] `tools/recqemu/src/process.rs:16-30` `kill_stale_qemu_processes` one caller (`testing/install-tests/src/bin/qmp.rs:62`) and its patterns match nothing.
- [surplus] `tools/recqemu/src/lib.rs:420-426` `build_piped` no callers (install-tests uses its own wrapper's).
- [surplus] `tools/recqemu/src/lib.rs:105-108` `cpu`, `:189-192` `kvm`, `:214-217` `display`, `:42` `DEFAULT_DISK_SIZE` no callers; `memory`, `smp`, `serial_file`, `vga`, `build_interactive` leviso-only; `nodefaults` wrapper-only; `qmp_socket` `bin/qmp.rs`-only.
- [surplus] `tools/recqemu/src/lib.rs:27,29` `Console`, `CommandResult` exported at root and via `pub mod serial`.
- [surplus] `tools/recqemu/src/serial/sync.rs:9-14` `timestamp_micros` `pub` but single-use.
- [surplus] `tools/recqemu/src/serial/console.rs:31` `CommandResult.stalled` always `false` (`exec.rs:100,123,154,178`) yet checked (`console.rs:37`, `testing/install-tests/src/executor.rs:21,27`).
- [surplus] `testing/install-tests/src/qemu/session.rs:32-35` `spawn_live`, `:63-85` `spawn_live_with_disk` — no callers.
- [surplus] `testing/install-tests/src/scenarios/mod.rs:920-925` `spawn_live_qemu_with_ssh` forwards to `session::spawn_live_with_ssh`.
- [surplus] `testing/install-tests/src/scenarios/mod.rs:1020-1027` `ssh_exec` wraps `recshuttle::ssh_exec_default_key` and adds the same context recshuttle adds (`tools/recshuttle/src/lib.rs:65-70`).
- [surplus] `tools/recshuttle/src/lib.rs:82-85` `ssh_exec_default_key` one-line wrapper; `:100-105` `with_key_path` no callers.
- [surplus] `testing/install-tests/src/qemu/qmp/capture.rs:16-18`, `qmp/input.rs:9-170` (`KeyCode`, `send_key`, `send_text`), `qmp/client.rs:18-19,24-68,250-284` unused.
- [surplus] `distro-spec/src/shared/auth/files/00-levitate-test.sh:10` exports `LEVITATE_TEST_MODE`, read by nothing.
- [surplus] `tools/recqemu/CLAUDE.md` lists "`fsdbg` - Future" consumer.
- [special] `tools/recqemu/src/serial/exec.rs:75` discards `wait_for_prompt` result — silent 500 ms sleep per command when no prompt marker.
- [special] `tools/recqemu/src/serial/exec.rs:165-166` drops any output line containing `root@` or `# ` (loses real output).
- [special] `tools/recqemu/src/serial/exec.rs:138-143` done marker accepted only with digits after it, to skip echoed command lines.
- [special] `tools/recqemu/src/serial/exec.rs:113-126` aborts on any line matching fatal patterns, including echo/stale boot output/"core dumped" in legit output.
- [special] `tools/recqemu/src/serial/auth.rs:224` sleeps 300 ms for getty.
- [special] `tools/recqemu/src/serial/auth.rs:301,317` linear backoff though `:18` says exponential.
- [special] `tools/recqemu/src/serial/auth.rs:302` `login:` line without the username treated as rejection (relies on getty echo).
- [special] `tools/recqemu/src/serial/auth.rs:329` login success = any line ending in `#` or `$`.
- [special] `tools/recqemu/src/serial/auth.rs:340` ignores the marker only when the line starts with `echo `; an echoed prompt line `root@host:~# echo ___LOGIN_OK___` still matches.
- [special] `tools/recqemu/src/serial/auth.rs:342,366-372` 100 ms drain sleep; newline every 500 ms timeout.
- [special] `tools/recqemu/src/serial/boot.rs:178,240` 500 ms settle sleep after success.
- [special] `tools/recqemu/src/serial/boot.rs:51-52` accepts `multi-user.target` "even if login prompt has VT quirks"; `levitateos login:` to avoid "Login Prompts".
- [special] `tools/recqemu/src/serial/boot.rs:137-151` boot phase from broad substrings (`"EFI"`, `"loader"`, `"KASLR"`) only to pick an error message.
- [special] `tools/recqemu/src/serial/boot.rs:95-96`, `process.rs:24-26` swallow write/pkill errors; `process.rs:29` sleeps 100 ms hoping processes died.
- [special] `tools/recqemu/src/lib.rs:372-377` adds `-vga none` under nographic "to avoid vgabios ROM files"; `:392-396` `serial_stdio` silently wins over `serial_file`.
- [special] `tools/recqemu/src/lib.rs:266-270` CPU `host` vs `qemu64` — same decision as `distro-spec/src/shared/qemu.rs:36` `QEMU_CPU_MODE`.
- [special] `tools/recqemu/src/lib.rs:443-448,470-487` silently ignores env OVMF paths to missing files; guesses VARS next to CODE.
- [special] `tools/recqemu/src/lib.rs:333` disk without `format=` (qemu probes); `xtask/src/tasks/testing/scenarios.rs:764,807-846` sniffs qcow2 magic to compensate.
- [special] `tools/recqemu/src/lib.rs:518-524` special-cases `qemu-img` NotFound to refuse a raw fallback that no longer exists.
- [special] `tools/recqemu/src/serial/utils.rs:18-29` "Bug #1 fix" `%` escape and `set +H` point fixes.
- [special] `tools/recshuttle/src/lib.rs:236-245` `poweroff` exits counted as success if output contains "connection"/"closed"/"reset by peer"/"broken pipe"; caller ignores the result (`testing/install-tests/src/scenarios/mod.rs:623`).
- [special] `tools/recshuttle/src/lib.rs:571-576` `partition_path` `if` for `/dev/nvme`/`/dev/mmcblk`; `:164-175` fixed device probe list skipping `sr*|loop*|ram*`.
- [special] `tools/recshuttle/src/lib.rs:259-261` no runtime policy checks unless ImmutableAb.
- [special] `tools/recshuttle/src/lib.rs:324-342,431-460` UUID vs PARTUUID and initrd split by `include_initramfs` (= "OpenRC").
- [special] `tools/recshuttle/src/lib.rs:129` 1 s poll; `testing/install-tests/src/scenarios/mod.rs:929-939` 60 × 1 s.
- [special] `testing/install-tests/src/qemu/session.rs:58,83,112,135` 2 s sleep after spawning QEMU.
- [special] `testing/install-tests/src/scenarios/mod.rs:666,702,743` hard-coded 90 s installed-boot stall vs ctx-provided live timeout; login 15 s (`:716,755`), exec 5 s (`:719,771,1362,1367`).
- [special] `testing/install-tests/src/scenarios/mod.rs:505-509` live-tools-only success patterns.
- [special] `testing/install-tests/src/scenarios/mod.rs:539-543` waits for serial markers before an SSH-only install "to drain serial output".
- [special] `testing/install-tests/src/scenarios/mod.rs:767-770` real validation only for `sudo`/`doas`.
- [special] `testing/install-tests/src/scenarios/mod.rs:379-381` missing tool = exit 127 or "not found" in output.
- [special] `testing/install-tests/src/scenarios/mod.rs:573` `/mnt/sysroot/bin/busybox` as "Root filesystem" check for every distro.
- [special] `testing/install-tests/src/scenarios/mod.rs:467-474` UX-only checks when `expected_install_experience == "ux"`.
- [special] `testing/install-tests/src/scenarios/mod.rs:950-997` diagnostics mix OpenRC, systemd and `anaconda-sshd.service`.
- [special] `testing/install-tests/src/scenarios/mod.rs:612-618` reuses `run_install_plan` for sync/unmount teardown.
- [special] `testing/install-tests/src/scenarios/mod.rs:346,493,623-625,642,667,720,779` `let _ =` on kill/wait/shutdown/`finish_failed`.
- [special] `testing/install-tests/src/qemu/builder.rs:192-212` `check_anti_cheat` panics on one flag combination.
- [special] `testing/install-tests/src/bin/qmp.rs:105-110,132,142`, `qmp/client.rs:225` polls and sleeps (50×100 ms, 30 s "for boot", 2 s, 50 ms/keystroke).
- [special] `leviso/src/qemu.rs:136` `.smp(2)`, `:167` 30 s stall.
- [special] `leviso/src/qemu.rs:19-25` any `Reached target` or `login:` = success; `:229` any line containing "init" = init started.
- [comment] `tools/recqemu/src/lib.rs:253-411` 16 section comments in `build()`; `:35-41,84,92-240` ~30 setter/constant docs restating names; `:442` wrong ("set by recipe").
- [comment] `tools/recqemu/src/patterns.rs:9-14,21-53,80` ~35 trailing comments repeating the string.
- [comment] `tools/recqemu/src/serial/auth.rs` banners `:44-46,72-74,104-106,145-147,152-154`, 14 inline restatements (`:223-352`), field docs `:50-58,88-120`.
- [comment] `tools/recqemu/src/serial/exec.rs:40,49,71,77,80,108,128,134,159,164` (10); `boot.rs` 13 (`:25-239`), `:211-213` describes code that doesn't exist; `ansi.rs` 7; `console.rs:1-10,22-31,45-47,233`; `utils.rs:1-6,18,24`; `chroot.rs:3-4,12-14,22`; `serial/mod.rs:1-12`; `lib.rs:1-4`.
- [comment] `testing/install-tests/src/qemu/builder.rs:38-138` (17), `qemu/mod.rs:1-13`, `qemu/serial/mod.rs:1-16`, `scenarios/mod.rs` (9 + banners), `qmp/client.rs` and `qmp/input.rs` doc blocks; `leviso/src/qemu.rs` 12; `00-levitate-test.sh` 6 (×2 copies).
- [test] `tools/recqemu/src/lib.rs:8-23` ` ```ignore ` doctest that wouldn't compile; `serial/auth.rs:23-34` ` ```ignore ` doctest using an unreachable type.
- [test] `testing/install-tests/src/bin/install-tests.rs:56-64` disables `install-tests run`; every `Step::execute` in `testing/install-tests/src/steps/` is a compiled-but-dead suite kept alive by `Executor for Console`.
- [test] `testing/install-tests/src/bin/qmp.rs:54-160` "smoke test" always reports success; never inspects screenshots.
- [test] `testing/install-tests/src/scenarios/mod.rs:1450-1452` `… || true` checks; `:1406,1433` pipelines whose status comes from `head` — missing tools pass.
- [test] `testing/install-tests/src/scenarios/mod.rs:769,379-381,1397-1456` installed tools checked only with `command -v`; "not found" detection defeated by stderr redirect.
- [test] `testing/install-tests/src/scenarios/mod.rs:194-201,871-898` skip a previously passing scenario when the fingerprint (ISO path+mtime or run id) is unchanged — harness/recqemu/recshuttle changes masked by cached passes.
- [test] `testing/install-tests/src/scenarios/mod.rs:206-208` BuildPreflight returns a constant success string.
- [test] `tools/recqemu/src/serial/auth.rs:340` used by the automated-login scenario (`scenarios/mod.rs:716`) can match the echoed command, passing without the command running.
- Cross-area notes from this area:
  - [hidden] `distro-builder/src/qemu.rs:14-560` is a fourth QEMU builder (own `find_ovmf` `:194-226`, patterns `:14-31`, AHCI `cdparts0` `:129-132`, boot loop `:228`); `AcornOS/src/qemu.rs` wraps it and shells out to `qemu-img` itself.
  - [hidden] `xtask/src/tasks/testing/scenarios.rs` builds raw QEMU command lines (`:754-822,892-985`) with unconditional KVM and has its own ANSI stripper, SSH args, port allocator, debug probe, boot-regression patterns (`:1338-1366`); uses neither recqemu nor recshuttle.
  - [hidden] `tools/recshuttle/src/lib.rs:412-555` own A/B layout (512M EFI, 6G+6G roots, `system-a`/`system-b`/`var`) and sfdisk script, disagreeing with recpart and recab — the shipped partitioner is never exercised by the e2e install.
  - [hidden] recshuttle drives `recstrap --force` (`:377,507`), `recfstab` (`:379,509`), `recchroot … --` by CLI string; recqemu drives `recchroot '<p>' /bin/bash -c` (`chroot.rs:24`).
  - [hidden] Guest serial protocol scripts in five places: `distro-spec/src/shared/auth/files/00-levitate-test.sh`, `leviso/.../00-levitate-test.sh`, `distro-builder/src/artifact/live_overlay.rs:653-668`, `AcornOS/profile/live-overlay/etc/profile.d/00-acorn-test.sh`, `distro-variants/acorn/ring2/overlays/live/etc/profile.d/00-acorn-test.sh` (last two identical); also `leviso/src/rebuild.rs:241`, `leviso/src/artifact/qcow2/config.rs:196`.
  - [surplus] Contract live-boot `success_patterns = []` (`distro-variants/*/scenarios/scenarios.toml:4,52-54`) unused; DistroContext hard-codes `["___SHELL_READY___"]` (`testing/install-tests/src/distro/levitate.rs:34`, `acorn.rs:27`).
  - [hidden] `distro-spec/src/shared/qemu.rs` constants compete with recqemu defaults and xtask literals.
  - [hidden] `xtask/src/tasks/tooling/hooks.rs:90-105` repo list includes `tools/recqemu` but omits recshuttle, recab, recart, recpart.
  - [hidden] recshuttle has no README and no license file though its `Cargo.toml` declares a license.

- Added by completeness pass 2 (fresh auditor):
  - [needless] `tools/recqemu/src/lib.rs:292-313` the generic `cdrom()` attaches the ISO a second time as scsi-hd `cdparts0` solely so the LevitateOS initramfs can find payload partitions (`tools/recinit/templates/init_tiny.template:182-201`) — one consumer's boot protocol in a generic builder.
  - [hidden] `tools/recqemu/src/serial/sync.rs:9` marker ids use `timestamp_micros % 1_000_000`, so markers can collide across commands.
  - [surplus] `tools/recqemu/src/patterns.rs:19-92` `CRITICAL_BOOT_ERRORS`/`BOOT_ERROR_PATTERNS` are reached only via `Executor::wait_for_live_boot`/`wait_for_installed_boot`, which nothing calls (install-tests uses `*_with_context`).
  - History: recqemu serial code gets repeated reliability patches (`df42828` drain-before-recv copy-pasted into a second loop, `a78a667` console nudge + fw_cfg + hostfwd, `0a8de28` renamed in two files), and the builder grows test-specific switches in step with install-tests and the initramfs. recab was written in one burst (Mar 1–2 2026); its single-impl traits and `RECAB_*` overrides are seams left by deleted unit tests (`c6d327a`), as are recqemu's unreachable `AuthConfig`/`timestamp_micros` (`83815ed`).

### E. recart (`tools/recart`)

- [hidden] `tools/recart/web/app.js:262` calls `loadTree(dir)` with the distro crate-dir (`leviso`/`AcornOS`, from `server.rs:222-233`), but outputs live under the output name (`levitate`/`acorn`), mapped only in `distro-builder/src/artifact_store/mod.rs:763-767`. The File Tree asks for a directory that doesn't exist (verified).
- [hidden] `tools/recart/src/server.rs:347-352` `out_rel_path` strips `out_dir.parent()`; correct only because `central_output_dir_for_distro` (`distro-builder/src/artifact_store/mod.rs:768-771`) nests exactly one level under `out`. On failure it yields an absolute path that `sanitize_relative_path` (`server.rs:1271-1289`) rejects when `app.js:179` builds a download link.
- [hidden] `tools/recart/web/app.js:298-299` "Copy Path" joins `summary.out_root` + `row.rel_path`, relying on `server.rs:284` and `server.rs:347-352` staying aligned.
- [hidden] API routes are written twice: `tools/recart/src/server.rs:97-118` and hard-coded paths/verbs throughout `tools/recart/web/app.js` (lines 179, 227, 256, 275, 290, 315, 328, 384, 404, 435, 452, 469, 475-476, 522, 545, 561, 582).
- [hidden] JSON field names of response structs (`server.rs:194-261,531-545,641-648,692-720`) are re-typed as bare property reads in `app.js` (≈107-476); a Rust rename breaks the UI silently.
- [hidden] `tools/recart/src/server.rs:697,719` serialize `distro_builder::artifact_store::IndexEntry` straight to the browser; `app.js:419-444` reads its fields and `ArtifactFormat`'s serde spelling (`distro-builder/src/artifact_store/mod.rs:36`) becomes UI text (`app.js:231,439`). The on-disk index schema is the web API schema.
- [hidden] Auth token convention split across languages: header `X-Recart-Token` (`server.rs:775-779`, `app.js:36`), query `token` (`server.rs:124`, `app.js:30`), sessionStorage `recart_token` (`app.js:31,35`).
- [hidden] Input-key filenames `.kernel-inputs.hash`, `.rootfs-inputs.hash`, `.initramfs-inputs.hash`, `.install-initramfs-inputs.hash` are literals in `tools/recart/src/main.rs:159,187,216,245,276-279,344,372,401,432-434`, `server.rs:366-375,453-460,965,1006-1028,1149-1172,1212-1220`, and must match what `leviso/src/rebuild.rs:66,78,134,150`, `AcornOS/src/rebuild.rs:26-167` and `AcornOS/src/main.rs:153-269` write.
- [hidden] Store kind names (`kernel_payload`, `rootfs_erofs`, `initramfs`, `install_initramfs`, `iso`, `iso_checksum`) are free strings across `tools/recart/src/main.rs` (~20 sites from 27 to 459), `server.rs` (~40 sites, 383-1202), `distro-builder/src/artifact_store/mod.rs:363,446,843`, `AcornOS/src/main.rs:209,249,326,347,438`. No shared enum.
- [hidden] The kind → (key file, output path) mapping is restated in every recart workflow — CLI ingest (`main.rs:157-324`, `342-479`), summary (`server.rs:359-376`, `447-461`), web ingest (`server.rs:963-1090`), web restore (`server.rs:1145-1204`) — and again in the builders (`leviso/src/rebuild.rs:56-172`, `AcornOS/src/rebuild.rs:17-169`).
- [hidden] ISO input-key derivation (which key files, in what order) is copied 5× (`main.rs:275-280`, `main.rs:431-435`, `server.rs:371-376`, `457-461`, `1209-1224`); `iso_key_files` exists but summary and CLI ingest don't use it. No builder computes this key; ISO store identity exists only in recart.
- [hidden] Kernel payload paths `staging`, `staging/boot/vmlinuz` are literals in `main.rs:158,160,343,345`, `server.rs:364,451,964-966,1150`, duplicating `distro-builder/src/artifact_store/mod.rs:366,836`, `leviso/src/rebuild.rs:203,231`, `AcornOS/src/rebuild.rs:42,131`.
- [hidden] `tools/recart/src/server.rs:748-754` rebuilds the blob layout `blobs/sha256/<2>/<sha>` by hand, copying private `ArtifactStore::blob_path`/`blobs_dir` (`distro-builder/src/artifact_store/mod.rs:97-99,136-140`); the blob endpoint bypasses the index.
- [hidden] `tools/recart/src/server.rs:666-684` (`api_store_kinds`) reads `<store>/index/*` directly, copying private `index_dir`/`list_kinds` (`distro-builder/src/artifact_store/mod.rs:101-103,698-714`).
- [hidden] Checksum naming `foo.iso` → `foo.sha512` hard-coded at `main.rs:491,497` and `with_extension("sha512")` at `server.rs:363,450,1063,1195`; owner is `distro-builder/src/artifact/iso_utils.rs:84` + `distro_spec::shared::ISO_CHECKSUM_SUFFIX` (`distro-spec/src/shared/iso.rs:88`).
- [hidden] Artifact filenames copied as literals: skip messages `main.rs:211,240,269,396,425` and fallbacks `server.rs:1230,1238,1246` (`filesystem.erofs`, `initramfs-live.cpio.gz`, invented `distro.iso`); owners in `distro-spec/src/shared/rootfs.rs:47`, `shared/paths.rs:54`, `levitate/paths.rs:116`.
- [hidden] Distro labels `"LevitateOS"`/`"AcornOS"` at `server.rs:226,230` copy `distro_spec::{levitate,acorn}::OS_NAME`.
- [hidden] Adding/removing a distro touches 11 recart sites (`Cargo.toml:17-19`, `main.rs:124-137,143-327,329-482`, `server.rs:222-233,271-280,905-930,932-948,1018,1166,1209-1224,1226-1248`) plus `distro-builder/src/artifact_store/mod.rs:763-767`; removing IuppiterOS (commit 49b8914) deleted 165 lines from main.rs and 96 from server.rs.
- [hidden] Three implementations of "ingest existing outputs": `main.rs:143-327` (leviso), `main.rs:329-482` (acorn), `server.rs:950-1137` (web). Already diverged: CLI stores empty metadata, web records `"distro"` (`server.rs:956-960`).
- [hidden] `tools/recart/src/server.rs:354-442` `summary_leviso` and `444-521` `summary_acorn` are near-identical copies.
- [hidden] "Prune then GC" sequencing repeated at `main.rs:101-102` and `server.rs:809-810`; `ArtifactStore::prune_keep_last` doesn't own it.
- [hidden] keep-last default `3` in `main.rs:35`, `web/index.html:88`, `web/app.js:559`; the `>= 1` rule in both `app.js:559` and `distro-builder/src/artifact_store/mod.rs:596-598`.
- [hidden] Store paging default 30 in `app.js:61` and `server.rs:706`; server clamps to 200 and returns no total, so `app.js:574-577` lets "Next" page past the end.
- [hidden] Which actions need mutation rights is decided in three places: `server.rs:766-782`, `app.js:86-89,265,283`, and early returns in `app.js:518,533,557,580`.
- [hidden] Output listing default 500 / cap 5000 (`server.rs:552`); the UI never passes `limit` and ignores `truncated` (`app.js:328-330`), silently cutting large dirs.
- [hidden] `main.rs:152-155,338-340` and `server.rs:852,932-948` call `leviso::rebuild::cache_*_hash` / `acornos::rebuild::cache_*_hash` to rewrite key files from *today's* inputs before ingesting outputs built earlier; the store key depends on current builder input lists, not on what produced the file.
- [hidden] `main.rs:69` `ArtifactStore::open` runs `ensure_layout` (`distro-builder/src/artifact_store/mod.rs:72-95`), so read-only `status`, `ls`, `serve` create `blobs/ index/ tmp/ locks/` on disk.
- [hidden] `main.rs:505-516` `find_repo_root` = first ancestor with `.git` and `Cargo.toml`; inside a submodule (e.g. `tools/recipe`) it picks the wrong root. One of several root-detection copies (`xtask/src/util/repo.rs:5`, `distro-builder/src/bin/workflows/layout.rs:5`, `distro-contract/src/variant.rs:314`, `testing/install-tests/src/lib.rs:71`).
- [hidden] `server.rs:1` imports `find_iso_checksum_file`/`iso_input_key` from the binary's `main.rs:484-503`; store/ISO domain helpers live in the CLI entry file.
- [hidden] `.artifacts/out` shown to users in `web/index.html:14`, `web/app.js:325`, `server.rs:1300` and in comments `server.rs:28,247,525,623,637`, copying `DEFAULT_STORE_DIR`/`DEFAULT_OUTPUT_SUBDIR` (`distro-builder/src/artifact_store/mod.rs:26,32`).
- [hidden] Two sources for the set of kinds: Store pane lists `index/` dirs (`server.rs:666-684`, `app.js:475`); Outputs pane uses hard-coded per-distro lists (`server.rs:354-521,905-930`).
- [needless] `tools/recart/Cargo.toml:19-20` + `main.rs:152-155,338-340`, `server.rs:936-944`: the store tool depends on the whole `leviso` crate (marked "retired legacy builder", `leviso/src/main.rs:1`) and the whole `acornos` builder, only to call seven `cache_*_hash` functions.
- [needless] `server.rs:263-280,849-856,894-897` use legacy crate-dir names (`leviso`/`AcornOS`) as distro identity and join them onto `repo_root`; the API is bound to legacy crate layout, not to the distro ids active producers use (`distro-builder/src/bin/artifact_paths.rs:3-5`).
- [needless] `server.rs:905-948,1018,1166,1209-1248`: general per-kind functions (`ingest_one_kind`, `restore_one_kind`) take a `distro_dir` string and re-branch on `"leviso"`/`"AcornOS"` inside.
- [needless] `server.rs:697,719,739` expose distro-builder's internal `IndexEntry` as HTTP bodies.
- [needless] `server.rs:1033-1090,1176-1204` re-implement store get-then-ingest for ISO inline because store helpers (`distro-builder/src/artifact_store/mod.rs:792-809`) take a key *file* and the ISO key is derived.
- [needless] `server.rs:325-336,1257-1264` (`store_presence`) compare inode/dev of `stored.blob_path` with the output, depending on the store's internal choice to hardlink blobs (`distro-builder/src/artifact_store/mod.rs:265-277,985-988`).
- [surplus] `server.rs:303-305` `read_key_file` only converts the error type of `distro_builder::artifact_store::read_input_key_file`.
- [surplus] `server.rs:166-168` `content_type` passes through to `HeaderValue::from_static`.
- [surplus] `main.rs:484-487` `iso_input_key` only converts `&[PathBuf]` to `&[&Path]` for `distro_builder::cache::hash_files`.
- [surplus] `server.rs:149-157` hand-rolled `hex_encode`, used once.
- [surplus] `server.rs:159-164` `now_unix` duplicates private `distro-builder/src/artifact_store/mod.rs:889-894`.
- [surplus] `server.rs:1329-1350` `sha256_file_async` duplicates `distro-builder/src/artifact_store/mod.rs:923-939`.
- [surplus] `server.rs:1250-1255` `validate_hex_64` duplicates `is_hex_64`/`validate_sha256` (`distro-builder/src/artifact_store/mod.rs:964-973`).
- [surplus] `main.rs:518-536` `fmt_bytes` and `web/app.js:9-20` `fmtBytes` are the same algorithm; a third is `distro-builder/src/artifact/rootfs.rs:136` `format_size_human`.
- [surplus] `server.rs:393-402,478-481` loop tuples carry `kind` and `store_kind`, always equal.
- [surplus] `server.rs:772-774` unreachable `else { return Ok(()) }`: `token` is `Some` exactly when `mutations_enabled` (`server.rs:83-87`) — two fields for one state.
- [surplus] `server.rs:759-764` `MutateResp.ok` is always `true` (`791,812`); the UI ignores the whole response (`app.js:561-567,582`).
- [surplus] Response fields the UI never reads: `DistroSummaryResp.{distro,generated_at_unix}` (`server.rs:257,259`), `StorePresence.out_nlink` (`241`), `OutLsResp.{root,truncated}` (`542,544`), `OutLsEntry.mtime_unix` (`537`), `Sha256Resp.{rel_path,abs_path,size_bytes,computed_at_unix}` (`643-647`), `StoreEntriesResp.{kind,offset,limit}` (`694-696`), `IngestResp.results` (`838`), `RestoreResp.restored` (`884`).
- [surplus] `server.rs:109,717-741` `/api/v1/store/:kind/entry` returns an `IndexEntry` the UI already has from `/entries` (`app.js:450-455`).
- [surplus] `server.rs:1369-1375` content-type arms: `"iso"` equals the default; `html/css/js/json` arms serve types the output store doesn't produce.
- [surplus] `server.rs:1266-1269` `#[cfg(not(unix))]` `hardlink_info` stub for a Linux-only toolchain.
- [surplus] `server.rs:1226-1248` `_ =>` fallbacks in `distro_{rootfs,initramfs,iso}_name` reachable only for unsupported distros.
- [surplus] `main.rs:298-299,453-454` `if checksum.exists()` re-checks what `find_iso_checksum_file` (`main.rs:489-503`) already guaranteed.
- [surplus] `server.rs:25` `#[derive(Clone)]` on `AppState` unused (shared via `Arc`, `89,118`).
- [surplus] `web/app.js:65,472` `state.storeRootAbs` assigned, never read.
- [surplus] `main.rs:42,106-108` `Ingest` subcommand and `server.rs:114-116,841-873` `ingest_existing`: two front doors to one operation.
- [surplus] `leviso/src/rebuild.rs:260-274` `cache_{kernel,rootfs,initramfs,install_initramfs}_hash` have no caller except recart.
- [special] `main.rs:496-500` checks the "Older/buggy case" `foo.iso.sha512` checksum name.
- [special] `server.rs:1017-1024,1165-1168` `if distro_dir != "leviso"`: install_initramfs special-cased to one distro, hard-coding `distro_spec::levitate::INITRAMFS_INSTALLED_OUTPUT` (`1029,1173`) inside generic per-kind dispatch.
- [special] `distro-builder/src/artifact_store/mod.rs:763-767` `"leviso" => "levitate"`, `"AcornOS" => "acorn"` table exists because legacy crate dirs differ from output names; recart is its main consumer.
- [special] `main.rs:172,201,230,259,294,312,357,386,415,448,467` print `[WARN]` on each ingest failure and continue; `recart ingest` exits 0 when ingests fail.
- [special] `server.rs:859-865` turns per-kind ingest errors into `status: "error"` rows; `app.js:275-281,522-529` ignore them and report "Ingest complete".
- [special] `app.js:256-263,545-553` restore ignores `RestoreResp.restored` and always says "Restored X".
- [special] `server.rs:932-948`, `main.rs:151-155,337-340` "best-effort" hash-key writes; callees discard write errors (`leviso/src/rebuild.rs:46`, `AcornOS/src/rebuild.rs:72,154,167` `let _ =`).
- [special] `server.rs:554-555,1293-1297,1307-1308,1330-1331`: not-found goes through `anyhow::Context` into `ApiError::Internal` → HTTP 500 + `[ERROR]` log (`server.rs:60-67`); this is what the `loadTree("leviso")` mismatch hits.
- [special] `server.rs:270-280` vs `847-856,893-897`: summary rejects unknown distro dirs; ingest/restore don't (`default_kinds_for_distro` returns `vec![]`, `928`), so ingest silently succeeds with no results.
- [special] `server.rs:567-580,585` symlinks classified by target type but `size_bytes` only set for regular files.
- [special] `server.rs:601-607` dirs-first sort hard-codes `"dir"`, also matched in `app.js:371,376,379`.
- [special] `server.rs:363,450,1062-1063,1194-1195` invent a non-existent `.sha512` path when no checksum exists.
- [special] `app.js:96-98` clipboard failure falls back to `window.prompt`.
- [special] `main.rs:136-138` bails only if neither `leviso/` nor `AcornOS/` exists — tied to two legacy dirs.
- [comment] `main.rs:157,185,214,243,272,342,370,399,428`: 9 section comments restating the next line.
- [comment] `server.rs:527` restates (and duplicates) the 500 default at `552`; `server.rs:567,601` restate code; `server.rs:28,247,525,623,637` repeat `.artifacts/out` facts; `web/app.js:75` restates `shaCache`.
- [test] recart has no tests; nothing checks that `app.js` routes/fields match `server.rs`.
- Cross-area notes from this area:
  - [hidden] Active producers write `.artifacts/out/<distro_id>/releases/<product>/<run_id>/…` (`distro-builder/src/pipeline/io.rs:78-83,112-117`, `distro-builder/src/bin/artifact_paths.rs:3-19`) and kernels to `.artifacts/kernel/<distro>/current`; recart only knows the flat legacy `.artifacts/out/{levitate,acorn}/<file>` + `staging/` layout.
  - [hidden] `.artifacts/` has no single owner: store (`index/ blobs/ tmp/ locks/`, distro-builder), `out/` (builders), `kernel/` and `tools/` (xtask), recipe (`tools/recipe/src/publication.rs:57`). No `index/` or `blobs/` exist in this checkout — the store appears unused.
  - [hidden] `AcornOS/src/main.rs:140-290,317-480` has its own restore→build→store copies of the kind/key/output triples and never stores `kernel_payload` or `iso`; those reach the store only via recart.
  - [special] `distro-builder/src/artifact_store/mod.rs:722-724` says lock files must never be unlinked, but `ArtifactLock::drop` (`883-887`) unlinks them.
  - [hidden] `leviso/src/rebuild.rs:74,145-167` use sibling crates' source files (`../distro-spec/src/shared/*`, `../tools/recinit/src/*`) as rebuild inputs — recinit source layout is a hidden input to leviso caching.
  - [hidden] `docs/content/src/content/05-rec-tooling/04-recart.ts` repeats subcommands and the leviso/AcornOS dirs; no xtask/justfile wiring calls recart.

## Fixes

## Progress

- 2026-11-06: Audit (phase 1) complete. Code untouched. Next: untangle-redesign.
