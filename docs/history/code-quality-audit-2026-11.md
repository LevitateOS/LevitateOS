# LevitateOS code quality & maturity audit

## Executive summary

**Overall maturity: early beta.** The active ring pipeline compiles and is mostly clippy-clean: `justfile` runs `distro-builder`, which runs `distro/variants/_shared/ring0/hooks/build-release.sh`, which calls recinit and reciso. `distro-contract` gives the build a typed manifest layer. Unit-test policy compliance is close to complete. Most of the risk sits at the edges: tools that touch disks and the host, what goes into shipped images, and checks that report success without checking anything. Release maturity is low. Lockfiles are not committed, superproject CI does not run on most changes, and releases carry no provenance.

Top themes:
- **Destructive-op boundaries are unsafe.**
  - The disk-plan TUI can re-run `recpart apply --confirm DESTROY`.
  - recpart cannot partition a blank disk, and it wipes disks that are in use.
  - recchroot writes into the host's resolv.conf and `/run`, and its mounts propagate back to the host.
- **Supply chain of shipped images.**
  - apk runs with `--no-check-certificate` and `--allow-untrusted`, and the host's resolv.conf ends up in the rootfs.
  - Tool downloads have no checksums.
  - The ISO ships the host's own bun and systemd-boot, plus debug-profile binaries.
- **Checks that pass when they should fail.** Examples: live-tools probes that always exit 0, evidence markers that are never checked, a stale cached PASS, an unconditional fsdbg PASS, broken EROFS listing, firmware globs that count directories, and a test macro (`cheat_aware`) that discards `Err` results.
- **Mandatory process gates that AGENTS.md bans.** A nested `cargo xtask policy audit-legacy-bindings` runs before distro-builder, kernel and scenario commands, and kernel builds are refused outside a build-hours window.
- **Legacy code is only partly retired.** leviso and AcornOS (about 13k LOC), recart's dependency on them, the docs/tui and shared/tui-kit forks, and dead modules in distro-builder and install-tests all still compile or run in CI.
- **Several sources of truth for the same facts.** Kernel version and sha256, identity, artifact names, tool lists, QEMU builders, A/B layout, run-manifest format and output paths are each defined in two to five places, and some copies have already drifted.
- **CI and release are structurally broken.**
  - `Cargo.lock` and `bun.lock` are gitignored.
  - The CI path filter skips submodule bumps.
  - The license check runs without submodules, the TUI matrix builds a deleted directory, and lint covers 8 of about 26 crates.
  - Standalone submodule CI cannot resolve its `../../` path dependencies.
  - The release workflow auto-publishes on every push.
- **Public docs describe things that do not exist.** The recipe CLI reference lists commands that were never implemented, llms.txt lists helpers that are not registered, the homepage claims a local LLM, and several READMEs still say the base is Rocky.

## Top priorities

### 1. The disk-plan TUI can re-run or double-run the destructive apply (critical) — #254 #255 #256 #372 #373
- **Why it matters:**
  - The real `recpart apply` runs from a `useEffect` keyed on `page`.
  - `confirmToken` is never cleared, and the effect's cleanup does not kill the child process.
  - Arrow keys in the nav pane skip `canAdvancePage`, so they bypass the plan, dry-run and preflight checks.
  - The parser expects one line of JSON, but recpart pretty-prints. Plan, dry-run and apply therefore always report "no JSON payload", so the user never sees the plan they are confirming.
  - The disk-list effect re-runs on every selection change and can snap the target back to an earlier disk.
- **Where:** `tui/apps/install/disk-plan/src/presentation/disk-plan-screen.tsx:176-203,365-424,463-465,493-560`; `src/app/plan-runner.ts:54-63,89-115`; `tools/recpart/src/json.rs:4-5`
- **Fix:**
  - Start apply once, from the confirm keypress, through a ref-guarded `startApply()` that holds the child handle.
  - Clear the token after any attempt.
  - Send every forward move through `canAdvancePage`.
  - Show the disk model and serial on the confirm page.
  - `JSON.parse` the whole stdout, or have recpart emit compact JSON under `--json`.
  - List disks once on mount, with a cancel flag.
  - Check with a PTY e2e run against a loop device.
- **Effort:** M

### 2. recpart apply fails on blank disks and will wipe disks that are in use — #174 #371 #175 #182 #384 #82
- **Why it matters:**
  - The wait for partition device nodes runs right after `wipefs`, before sfdisk has created anything, so a never-partitioned disk fails with E005.
  - `wipefs -a --force` overrides wipefs's own busy check, and the only guard is "is anything mounted". LVM PVs, md members, dm-crypt and swap pass.
  - Mounts are not rolled back on failure.
  - `is_protected_path` is an exact string match, and recpart does not canonicalize before calling it.
  - One failing device aborts `list-disks`, and zram devices are offered as targets.
- **Where:** `tools/recpart/src/exec.rs:93-100,133-142,398-415,448-477`; `tools/recpart/src/preflight.rs:133-155,209-254,462-476`; `distro/spec/src/shared/paths.rs:83-97`
- **Fix:**
  - Wait once, after sfdisk plus `udevadm settle`, and check that the PARTUUIDs match the plan.
  - Drop `--force`. Check `/sys/block/*/holders`, `/proc/swaps` and an `O_EXCL` open before wiping.
  - Unmount in reverse order on failure.
  - Canonicalize inside `is_protected_path`.
  - In `list-disks`, skip or flag devices that fail the query, and skip zram and held devices.
  - Check with the ignored destructive e2e against a freshly zeroed disk.
- **Effort:** M

### 3. The chroot and bootstrap tools leak into the host — #139 #370 #141 #376 #140 #375 #142 #143
- **Why it matters:**
  - recchroot calls `fs::copy` on resolv.conf through the target's absolute symlink. Because host `/run` is already bind-mounted, the write lands in the host's `/run/systemd/resolve/stub-resolv.conf`.
  - rbind mounts keep shared propagation, so mounts made in the chroot and the lazy unmounts at cleanup propagate to the host.
  - SIGINT, SIGTERM and SIGQUIT stay ignored in the child, so Ctrl-C does nothing inside the chroot.
  - recstrap reads the password with echo on and writes it unescaped into a 0755 script, where a `'` in the password injects shell code.
  - `--force` turns off both the mount-point guard and the non-empty guard, and the protected-path list is exact-match only.
- **Where:** `tools/recchroot/src/main.rs:186-224,264-293,340-364`; `tools/recstrap/src/helpers.rs:249-285`; `tools/recstrap/src/main.rs:92-94,209-241`
- **Fix:**
  - Leave a symlinked target resolv.conf alone, or bind-mount the host file over it read-only and unmount it at cleanup.
  - Set `MS_SLAVE|MS_REC` after each rbind and mount a fresh proc.
  - Reset signal dispositions in `pre_exec`.
  - Pipe `user:pass` to `chpasswd` on stdin instead of writing a script.
  - Split `--force` into separate overrides, and refuse the host root device even with them.
- **Effort:** S to M

### 4. Supply chain of shipped images — #0 #46 #387 #59 #5 #415 #2 #348 #1
- **Why it matters:**
  - The Acorn ring3 rootfs is installed with `--allow-untrusted` into an empty `etc/apk/keys`.
  - The live-tools apk step uses `--no-check-certificate` and leaves the host's `/etc/resolv.conf` inside the image.
  - Package names are interpolated into `sh -lc`.
  - 7zz is downloaded and run without a checksum, and the QEMU tool RPMs come over plain HTTP with no checksum and are put first on `PATH` and `LD_LIBRARY_PATH`.
  - The ISO ships the host's `bun` (found on PATH) and the host's `systemd-bootx64.efi`.
  - recstrap, recfstab, recchroot and install-split-pane are built in debug profile from a hardcoded `target/debug` path, which breaks when `CARGO_TARGET_DIR` is set.
- **Where:**
  - `distro/builder/src/pipeline/live_tools.rs:260-308,310-397,398-509`
  - `distro/builder/recipes/alpine-live-source-rootfs.rhai:357-371`
  - `distro/builder/recipes/dvd-source-deps.rhai:11,31-45`
  - `distro/builder/recipes/qemu-deps.rhai:10-18,52-67`
  - `tools/reciso/src/lib.rs:299-306,345`
  - `distro/variants/acorn/ring2/products.toml:116-117`
- **Fix:**
  - Install `ALPINE_KEYS`, then drop `--allow-untrusted` and `--no-check-certificate`.
  - Swap resolv.conf in only for the apk call, with a guard that restores it on error too.
  - Run `apk add -- "$@"` with the packages as argv, and validate package names when the manifest loads.
  - Add a pinned sha256 to every recipe download and switch them to https.
  - Acquire bun and systemd-boot through pinned recipes, and pass their paths into reciso.
  - Build with `--release` and take the binary path from `cargo --message-format=json`.
- **Effort:** M

### 5. Commit the lockfiles and build with them — #93 #319 #399 #414 #261 #260 #320
- **Why it matters:**
  - `.gitignore:47` ignores `Cargo.lock` and `.gitignore:78-79` ignore `bun.lock` and `bun.lockb`, so every CI run and release resolves dependencies fresh. The root currently resolves clap 4.6.7 while the submodule locks pin 4.5.54.
  - The install-docs bundle in the ISO is built from caret ranges, with a plain `bun install`.
  - Stale per-member locks are committed and ignored. `disk-plan/bun.lock` even belongs to the old blessed-based frontend.
  - `hashFiles('**/Cargo.lock')` hashes nothing in a clean checkout.
- **Where:** `.gitignore:47,78-79`; `.github/workflows/ci.yml:56`; `distro/builder/src/pipeline/live_tools.rs:427-449`; `tui/apps/install/disk-plan/bun.lock`; `tools/recstrap/Cargo.lock`; `tools/recchroot/Cargo.lock`; `leviso/Cargo.lock`; `testing/install-tests/Cargo.lock`
- **Fix:**
  - Remove the ignore lines and commit the root `Cargo.lock` and `bun.lock`.
  - Delete the member locks of crates that only build inside the workspace.
  - Use `--locked` and `bun install --frozen-lockfile` in the release paths.
  - Pin bun to the `packageManager` version.
- **Effort:** S

### 6. Remove the mandatory process gates — #3 #95 #206 #382 #419
- **Why it matters:** AGENTS.md bans mandatory policy-audit pipelines and process-only blockers.
  - Every distro-builder command, including the nested calls the release hooks make, every `xtask kernels build`, every scenario and every qmp run shells out to `cargo xtask policy audit-legacy-bindings`. That nests cargo up to three levels deep and needs a source checkout plus a toolchain.
  - `enforce_build_hours()` calls `process::exit(3)` outside 23:00–10:00, and nothing overrides it.
- **Where:** `distro/builder/src/bin/distro-builder.rs:57-62`; `distro/builder/src/bin/workflows/build.rs:38-56`; `distro/builder/src/bin/workflows/commands.rs:39`; `xtask/src/app.rs:4,146-164`; `xtask/src/tasks/kernels/common.rs:37-59`; `testing/install-tests/src/lib.rs:46-69`; `testing/install-tests/src/bin/scenarios.rs:54-57`; `testing/install-tests/src/bin/qmp.rs:46`
- **Fix:**
  - Delete the guard functions and their call sites, and keep `just policy-legacy` as an explicit command.
  - Delete the build-hours gate.
  - Return errors instead of calling `process::exit`.
- **Effort:** S

### 7. Superproject CI does not test what changes — #89 #90 #396 #424 #92 #321 #322 #397 #91 #263 #282 #333 #398 #109 #337 #402
- **Why it matters:**
  - The change filter matches only `*.rs`, `Cargo.*` and workflow files. Submodule pointer bumps (the usual way code lands here) and changes to rhai, ring TOML, hooks and kconfig never trigger the ISO job.
  - The scenario steps need `.artifacts/tools/.tools`, which CI never provisions, and they ignore the host's OVMF.
  - The license check runs cargo-deny without submodules, against a `deny.toml` that still has v1 advisory keys and a dead ring clarify block.
  - tui-matrix builds the deleted `tools/recpart/frontend`, typechecks only the deprecated forks, and never touches `tui/`.
  - Lint runs clippy in 8 crates (leviso among them), not xtask, distro-builder, distro-contract or rec{ab,part,qemu,shuttle}. There is no fmt check, and xtask currently has 11 warnings.
- **Where:** `.github/workflows/detect-changes.yml:29-37,64`; `.github/workflows/ci.yml:24-27,43-48,82-88,130-175`; `.github/workflows/license-check.yml:12-13`; `deny.toml:20-39`; `.github/workflows/tui-matrix.yml:5-53`; `xtask/src/util/repo.rs:19-67`; `xtask/src/util/tools_env.rs:9-24`
- **Fix:**
  - Drop the paths filter for the superproject, or add the gitlink paths plus `distro/variants/**`, `**/*.{rhai,toml,sh}`, `justfile` and `**/kconfig`.
  - Make the tools prefix optional (fall back to recqemu's `find_ovmf` and `OVMF_PATH`), or add `just tools-install` to CI.
  - Use `submodules: recursive` and move deny.toml to `version = 2`.
  - Point tui-matrix at `tui/**`.
  - Replace the lint list with `cargo clippy --workspace --all-targets -- -D warnings` plus `cargo fmt --all --check`, in CI only.
- **Effort:** S to M

### 8. install-tests reports false passes and leaks state — #196 #197 #198 #199 #200 #201
- **Why it matters:**
  - Tool probes such as `mkfs.ext4 -V 2>&1 | head -1` and `X --version || true` always exit 0.
  - Each scenario declares an evidence script and pass marker, but only live-boot.sh runs, and the marker is never checked.
  - The input fingerprint (ISO path plus mtime, or run_id) leaves out the contract, the scripts and the harness version, so a cached PASS survives edits to any of them.
  - The QEMU `Child` is dropped on early `?` returns, leaving QEMU running with the qcow2 lock held.
  - Boot waits have only a stall timeout, and the 3 s newline nudge keeps resetting it, so there is no overall deadline.
  - Installed scenarios boot the shared install disk read-write.
- **Where:** `testing/install-tests/src/scenarios/mod.rs:194-201,325-338,376-388,510-517,653-780,871-898,1029-1053,1406-1452`; `testing/install-tests/src/qemu/session.rs:56-137`; `tools/recqemu/src/serial/boot.rs:94-127`
- **Fix:**
  - Probe with `command -v X && X --version >/dev/null 2>&1`.
  - Add one `run_evidence()` helper that requires both exit 0 and the marker.
  - Hash the contract section, the scripts and the crate version into the fingerprint.
  - Wrap `Child` in an RAII kill-and-wait guard.
  - Add a total boot deadline.
  - Boot installed scenarios from per-run qcow2 overlays with a copied vars file, and flock the state file.
- **Effort:** M

### 9. Retire the legacy crates, the recart coupling and the TUI forks — #28 #342 #29 #30 #35 #36 #47 #192 #193 #330 #345 #416 #264 #279 #407 #417
- **Why it matters:**
  - leviso's binary only bails. Its library is dead apart from `rebuild.rs`, it has about 2,350 LOC that no `mod` declaration reaches, and it carries `#![allow(dead_code)]` plus heavy unused dependencies.
  - AcornOS is broken: it looks for the kernel and the seed overlay at wrong paths, and its error messages point at commands that no longer exist. Both crates are still workspace members.
  - recart calls `leviso::rebuild::cache_*_hash`. That overwrites the key with the hash of the current inputs and stores old outputs under it. Ingest failures print a warning and still exit 0, and the distro list is hardcoded to leviso and AcornOS.
  - docs/tui and shared/tui-kit are diverged forks that CI targets and that leviso builds from.
- **Where:** `leviso/src/lib.rs:11-20`; `leviso/src/rebuild.rs:43-127`; `leviso/Cargo.toml:9-35`; `AcornOS/src/main.rs:124-198`; `tools/recart/Cargo.toml:17-19`; `tools/recart/src/main.rs:143-516`; `tools/recart/src/server.rs:222-235,932-1248`; `.gitmodules:21-23,109-111`
- **Fix:**
  - Have recart store only keys recorded at build time, from distro/variants and the run layout, and drive one table of ingest steps for both the CLI and the server. Exit non-zero on any failure.
  - Then drop leviso and AcornOS from the members and delete them, along with their uncompiled modules, `leviso/deps/`, kconfig copies, profiles and init templates.
  - Remove the two TUI submodules and point the leviso docs recipe, `LICENSE:17` and the docs at `tui/`.
- **Effort:** L

### 10. Make the variant contract the only owner of variant facts — #45 #48 #65 #94 #64 #66 #49 #203 #79 #331
- **Why it matters:**
  - The ring1 and ring0 manifests declare `initramfs_installed`, `installed_uki` and `metadata_facts`, and nothing produces them.
  - Kernel version, sha256 and localversion exist in both `distro-spec` and `build-host.toml`. xtask builds from the deprecated copy and hardcodes `linux.rhai`, even though Levitate declares `linux-prebuilt.rhai`.
  - Validation is opt-in: xtask kernels, the artifact workflows and the scenarios use contracts that were never validated.
  - 7 ViolationCodes are never emitted.
  - Scenario tool lists, patterns and users are hardcoded in install-tests.
  - distro-spec, which is marked DEPRECATED, is still a dependency of 12 active crates, and its UKI cmdline has already drifted from the TOML.
- **Where:** `distro/variants/*/ring1/transforms.toml:21-42`; `distro/variants/levitate/ring0/release.toml:16-20`; `distro/spec/src/shared/kernel.rs:44-55`; `xtask/src/tasks/kernels/common.rs:22-35,66-72,139-149`; `distro/contract/src/variant.rs:298-312`; `distro/contract/src/error.rs:46-53`; `testing/install-tests/src/distro/{levitate.rs:230-252,acorn.rs:122-152}`; `distro/spec/src/{levitate,acorn}/paths.rs`; `distro/spec/src/acorn/uki.rs:17-50`
- **Fix:**
  - Have the loader return a `ValidatedContract` newtype, so consumers cannot take an unvalidated one.
  - Drive xtask kernels from `contract.build.kernel` and delete the distro-spec kernel and identity constants.
  - Read the tool, pattern and user lists from the contract in install-tests.
  - Either produce the declared ring1 transforms and metadata_facts, plus a post-build check that every declared output exists, or remove the declarations.
  - Implement or delete the dead violation codes.
  - Relabel distro-spec, or move the parts still in use.
- **Effort:** L

### 11. A/B boot correctness and a single A/B layout contract — #170 #171 #172 #177 #178 #173 #176 #374
- **Why it matters:**
  - Rollback never clears the systemd-boot one-shot, so the next boot still goes to the trial slot.
  - loader.conf is rewritten before bootctl runs. When bootctl fails, only state.json is reverted, which breaks the project's own REQ-ERR-010.
  - Pending-trial state is never reconciled with the firmware, so `set-next` on the same slot silently does nothing.
  - The "active" slot falls back to the loader default, so `commit` can mark the wrong slot good.
  - Writes are not fsynced and nothing locks against concurrent runs.
  - recpart, recshuttle and recab disagree on labels, sizes and the persistent mount. The e2e tests install through recshuttle's own sfdisk script, so the partitioner that ships is never tested end to end.
  - recshuttle copies the initramfs at a fixed step index, which in the A/B layout is before the ESP is mounted.
- **Where:** `tools/recab/src/lib.rs:211-370,412-431,568-589,676-731,769-775,846-953`; `tools/recpart/src/policy.rs:117-258`; `tools/recshuttle/src/lib.rs:344-360,462-500,558-576`
- **Fix:**
  - Add `clear_trial()` (`bootctl set-oneshot ""`).
  - Snapshot loader.conf and restore it on failure, or run bootctl first.
  - Read `LoaderEntryOneShot` and `LoaderEntrySelected` from efivarfs.
  - Require a definite active slot before mutating.
  - fsync and rename, and flock `/var/lib/recab`.
  - Define the layout once in distro-spec, and have recshuttle call `recpart apply --json`.
- **Effort:** L

### 12. Recipe engine data and scope bugs — #113 #114 #115 #116 #117 #118
- **Why it matters:**
  - Tar hard links go through `Entry::unpack`, so they resolve against the process CWD and are checked against the wrong base.
  - ctx persistence turns arrays, maps and floats into strings, and can write keys that are not identifiers into the recipe source as broken syntax.
  - A child recipe with no ctx of its own writes install state into the shared base file, which is not locked.
  - Check hooks treat any Rhai error (typo, missing variable) as "needs work".
  - The dependency resolver skips `build()` and nested deps, so its cycle check can never fire.
  - Scope constants are copy-pasted five times and have drifted: `remove`, `cleanup` and checks lack BUILD_DIR, ARCH or RPM_PATH.
- **Where:** `tools/recipe/src/helpers/build/extract_api.rs:94-199`; `tools/recipe/src/core/ctx.rs:15-97`; `tools/recipe/src/core/executor/private/{state.rs:12-101,flow.rs:28-48,421-453,actions.rs:27-251}`; `tools/recipe/src/core/build_deps.rs:54-256`; `tools/recipe/src/core/lock.rs:14`
- **Fix:**
  - Create hard links explicitly under `dest`.
  - Write a real round-trip serializer that refuses values it cannot represent.
  - Never persist state into a base recipe.
  - Count only explicit `throw`s as "needs work", and always re-verify after install.
  - Run deps through the full lifecycle, recursively.
  - Add a single `build_scope()`.
  - Cover these with CLI e2e scenarios.
- **Effort:** M

### 13. recinit can ship a broken or non-reproducible initramfs — #146 #377 #147 #148 #149 #159 #378
- **Why it matters:**
  - The `GzEncoder` returned by `finish()` is dropped, so flush and trailer errors are lost, and the only check afterwards is size of at least 1 KiB.
  - Busybox is fetched with curl without `--fail`, written straight to the cache, and never re-verified on a cache hit.
  - Unknown custom module names are dropped silently, and the empty-list check can never fire.
  - Missing systemd binaries are skipped, and `/init` is symlinked to them anyway.
  - Archive order is unsorted, and uid, gid, inode and nlink come from the build host.
- **Where:** `tools/recinit/src/cpio.rs:41-115,190-211`; `tools/recinit/src/busybox.rs:52-137`; `tools/recinit/src/modules.rs:31-43`; `tools/recinit/src/main.rs:203-217`; `tools/recinit/src/systemd.rs:14-105`; `tools/recinit/src/install.rs:151-219`
- **Fix:**
  - `let f = writer.finish()?.finish()?; f.sync_all()?`, then run `gzip -t` before the rename.
  - Download to a temp file, verify, then rename, and verify on every cache hit.
  - Make `module_paths()` return a `Result`.
  - Split required files from optional ones.
  - Write uid and gid 0 with synthetic inodes, and sort entries.
- **Effort:** S to M

### 14. Verification checks that pass when they should fail — #215 #216 #217 #218 #219 #221 #222 #223 #225 #226 #227 #228 #239 #240
- **Why it matters:**
  - **cheat-test macros:** `#[cheat_aware]` drops the function signature, so `Err` results are thrown away.
  - **fsdbg ISO checks:** it adds an unconditional "loader.conf editor policy" PASS, and the ISO rootfs check passes if `fdisk` output merely contains `{iso}2`.
  - **fsdbg EROFS reader:** `dump.erofs --ls -r` uses an option that does not exist, and the fallback returns an empty listing.
  - **fsdbg rootfs and auth checklists:** they accept only CPIO, but the real rootfs is EROFS. The auth audit never checks file modes or setuid bits.
  - **fsdbg symlink and qcow2 checks:** dangling enable-symlinks pass, symlinks are resolved one hop and against the host, and a truncated CPIO is accepted.
  - **fsdbg module checks:** module matching is substring-based (`xloop.ko` satisfies `loop`).
  - **rootfs-tests:** the sudo check passes on commented-out sudoers lines, and the `recipe list` check falls back to `--help`.
  - **hardware-compat:** firmware globs count directories, and a Critical `=y` requirement is met by `=m`.
- **Where:**
  - `testing/cheat-test/src/lib.rs:258-462`
  - `testing/fsdbg/src/checklist/{iso.rs:160-176,280-283,328-342; qcow2.rs:775-809; install_initramfs.rs:474-503; live_initramfs.rs:273-299; rootfs.rs:96,488-514; auth_audit.rs:134-376}`
  - `testing/fsdbg/src/{erofs.rs:62-170,cpio.rs:186-334}`
  - `testing/rootfs-tests/src/tests/{admin.rs:150-176,packages.rs:62-77}`
  - `testing/hardware-compat/src/{lib.rs:51-70,kernel.rs:30-33}`
- **Fix:**
  - Re-emit the full signature from the macros.
  - Never add a pass the code did not compute.
  - Check the EROFS magic at the expected partition offsets.
  - Parse the real `dump.erofs` output.
  - Add a read-only index trait for CPIO, EROFS and ISO, so the checklists work on every format.
  - Use one resolver that follows symlinks component by component.
  - Require `TRAILER!!!` in CPIO archives.
  - Count only regular files in firmware globs, and make m-for-y a Fail when the requirement is Critical.
- **Effort:** M

### 15. Release provenance and the release workflow — #21 #39 #108 #132 #251 #341 #400 #401
- **Why it matters:**
  - `BuildRunMetadata` holds no git revision, dirty flag, output sha256, kernel release or source versions.
  - `SOURCE_DATE_EPOCH` is not used anywhere.
  - CI uploads the ISO without its `.sha512`, and there is no ISO release workflow.
  - `rust-release.yml` makes a patch bump, pushes to the default branch, tags and runs `cargo publish` on every non-bump push. It has no `concurrency:` group and no `--locked`, and uses `action-gh-release@v1`. This affects leviso-elf, recipe, hardware-compat and others. The bump input works, but only through the caller's `workflow_dispatch`.
- **Where:** `distro/builder/src/bin/distro-builder.rs:34-46`; `distro/builder/src/bin/workflows/{build.rs:99-115,release_hook.rs:89-142}`; `.github/workflows/rust-release.yml:3-148`; `.github/workflows/ci.yml:119-125`; `distro/elf/.github/workflows/ci.yml:22-44`
- **Fix:**
  - Record superproject and submodule revisions, a dirty flag, per-output sha256 and the kernel release in the run manifest.
  - Export `SOURCE_DATE_EPOCH` and pass it to xorriso, mkfs.erofs and mkfs.fat with deterministic UUIDs.
  - Upload the ISO together with its `.sha512`.
  - Release on a tag or `workflow_dispatch` only, with `concurrency`, `--locked`, gh-release v2 and SHA256SUMS.
- **Effort:** M to L

## Themes

### Destructive-op & host safety
| Severity | Finding | Location | Fix | Effort |
|---|---|---|---|---|
| medium | Recipes run `rm -rf`, `7z x` and `chmod` on unquoted concatenated paths, and hide failures with `2>/dev/null` and or-true (#4 #425) | distro/builder/recipes/fedora-dvd-source-rootfs.rhai:223-235,344; custom-source-rootfs.rhai:51-52; linux.rhai:160-161 | Use native rm/extract helpers or a `shell_quote()` helper; stop suppressing errors | M |
| medium | The sync script resets branches with `checkout -B origin/$branch`, discarding unpushed commits (the superproject is currently 5 ahead) (#87) | tools/sync-submodules-default-branches.sh:48,63,94 | Refuse when the branch is ahead or the tree is dirty; use `switch` then `merge --ff-only` | S |
| medium | Window mode serves VNC on 0.0.0.0 with no password, giving a root shell (#88) | xtask/src/tasks/testing/scenarios.rs:96-99,1047-1051,1166-1174 | Bind to 127.0.0.1 by default; require an explicit opt-in for wider binds | S |
| medium | recipe `rm()` deletes by glob recursively with no guard against `/`, `/*` or an empty prefix (#120) | tools/recipe/src/helpers/install/filesystem.rs:40-52 | Refuse root, top-level and $HOME targets and empty leading components | S |
| medium | The autofix patch path check can be bypassed (quoted paths, `x/`/`y/` prefixes, rename and copy headers); the default cwd is the caller's git repo (#125) | tools/recipe/src/core/autofix/{parser.rs:3-33,patch.rs:168-182,prompt.rs:53-73} | Get the authoritative path list from `git apply --numstat -z --check`; default to the recipe's own dir | M |
| medium | recstrap mount cleanup does not run on Ctrl-C; it uses a fixed `/tmp/recstrap-erofs-mount` and calls `remove_dir_all` after an unchecked umount (#144 #386) | tools/recstrap/src/rootfs.rs:52-104 | mkdtemp with mode 0700; `remove_dir` only after a successful umount; handle signals | S |
| medium | reciso and recinit `rm -rf` fixed-name scratch dirs next to the output (#150) | tools/reciso/src/lib.rs:198-208,467-526; distro/builder/src/artifact/iso_utils.rs:35-39; tools/recinit/src/tiny.rs:121-125 | Use a `TempDir`; never delete paths the tool did not create | S |
| low | Run pruning deletes by the run_id read from the manifest, ignores status, never prunes manifest-less dirs, and pid bits overlap in run ids (#11 #388) | distro/builder/src/run_history.rs:34-83,115-131 | Prune by directory entry name; validate it is one component; skip runs still building | S |
| low | `product prepare` removes subdirs of any user-supplied dir; the migration-era cleanup still runs (#12) | distro/builder/src/pipeline/io.rs:6-43; source.rs:126-149 | Require `.artifacts` or a marker file; delete `cleanup_legacy_provider_dir` | S |
| low | The justfile interpolates `{{target}}`, `{{inject}}` and `{{distro}}` raw into bash (#96) | justfile:108,121,139,179-184 | `set positional-arguments` or `quote()` | S |
| low | `product-erofs` runs `rm -rf` on a path built from unvalidated arguments (#98) | justfile:334-342 | Validate against `^[a-z0-9-]+$`, or let distro-builder own the dir | S |
| low | The tui inspect preset targets host `/dev/sda`, and the input wrapper sends keystrokes twice (#101) | xtask/src/tasks/tui/inspect.rs:124-128,238-244 | Use a loop or fake device; send once | S |
| low | reciso `--extra-file` destinations and UKI filenames can escape the staging root (#151) | tools/reciso/src/lib.rs:216-224,386 | Reject absolute paths and `..`; require a single `.efi` component | S |
| low | The root-only recab binary honours `RECAB_BOOTCTL_BIN`, `RECAB_STATE_PATH` and `RECAB_ACTIVE_SLOT` in production (#179) | tools/recab/src/lib.rs:509-521,601-632,835-915 | Put the overrides behind `--root` or a feature flag | S |
| low | The recab destructive e2e is keyed to `/dev/sdc` (it is gated by `#[ignore]` plus an ACK variable) (#389) | tools/recab/tests/e2e_sdc.rs:12,222-286 | Require a by-id path or serial, or use a loop device | S |
| low | recshuttle interpolates hostname, password, title and disk path unquoted into remote shell commands (#184) | tools/recshuttle/src/lib.rs:346-350,390-401,433-441,540-545 | Add `shell_quote()`; pass the password to chpasswd on stdin | S |
| low | recart mutation endpoints do not whitelist the distro path segment, and handlers do blocking fs work on the async runtime (#194) | tools/recart/src/server.rs:547-620,784-790,841-903 | Reuse the summary whitelist; use `spawn_blocking` | S |

### Supply chain & shipped-artifact integrity
| Severity | Finding | Location | Fix | Effort |
|---|---|---|---|---|
| medium | The release hook curls busybox straight to its final path, so a bad file wedges every later run; it runs `cargo run` mid-release. recinit's curl has no `--fail`, custom URLs skip the checksum, and initrd-release is hardcoded to LevitateOS (#53 #352 #164) | distro/variants/_shared/ring0/hooks/build-release.sh:46-48,264-313,349-350; tools/recinit/src/busybox.rs:52-82; tools/recinit/src/install.rs:146-164 | Acquire busybox through a recipe; temp file, verify, rename; pass resolved binary paths and branding in | M |
| medium | `LoaderConfig` defaults to `editor: true`, so the boot cmdline is editable (for example `init=/bin/sh`) (#81) | distro/spec/src/shared/boot.rs:195-220; tools/reciso/src/lib.rs:442-449; leviso/src/artifact/qcow2/partitions.rs:57 | Default to false; build the cmdline in one helper | S |
| medium | The EROFS image is written straight to its final path, and source files without owner-read are silently chmodded (#10) | distro/builder/src/artifact/rootfs.rs:71-134,156-209 | Write a temp file and rename it; fail or log permission changes | S |

### Install tools correctness
| Severity | Finding | Location | Fix | Effort |
|---|---|---|---|---|
| medium | The reciso `with_overlay` alias now means "image", so the leviso and AcornOS callers break at runtime (#152) | tools/reciso/src/lib.rs:145-148,279-289 | Remove the alias so callers fail to compile | S |
| medium | recfstab drops `ro`, `noatime` and `lazytime` from the generated fstab (#155) | tools/recfstab/src/filter.rs:35-59 | Strip only runtime-only options (seclabel, subvolid, rw) | S |
| low | reciso uses a fixed 200 MB FAT image, duplicates the systemd-boot path, and hardcodes the console args (#154) | tools/reciso/src/lib.rs:32-34,299-353 | Size from the summed inputs; take boot settings from config | M |
| low | recinit templates are not escaped or checked for leftover placeholders, and three init templates have diverged (#160) | tools/recinit/src/tiny.rs:73,263-273; tools/recinit/templates/init_tiny.template; leviso/profile/init_tiny.template | Quote values and fail on leftover `{{`; keep one template | M |
| low | The recinit `live_overlay_path` alias means the CLI cannot turn the overlay off. The release hook relies on the default (#161) | tools/recinit/src/tiny.rs:44-48,77-78,256-261 | Make the hook pass the flag first, then default to None | S |
| low | The kernel modules dir is picked from the first readdir entry, in three copies (#162 #360) | tools/recinit/src/{modules.rs:168-188,install.rs:225-239}; leviso/src/artifact/initramfs.rs:166-221 | Take kernel.release explicitly, or fail when there is more than one | S |

### Build pipeline & orchestration correctness
| Severity | Finding | Location | Fix | Effort |
|---|---|---|---|---|
| medium | leviso-elf `copy_dir_recursive` follows directory symlinks and writes through destination symlinks; this is on the live overlay path (#27) | distro/elf/src/copy.rs:56-70 | Classify with `file_type()`; unlink a destination symlink before copying | S |
| medium | The readelf analysis matches English stderr, so it depends on the locale, and it returns Ok(empty) for unreadable files (#31) | distro/elf/src/analyze.rs:30-48 | Set `LC_ALL=C`; resolve symlinks inside the source root; warn; consider goblin | M |
| low | `copy_library_to` classifies on the absolute path, unwraps, and follows dangling symlinks (#32) | distro/elf/src/copy.rs:102-148 | Use `strip_prefix`, contexts and `symlink_metadata` | S |
| low | The host `ldd` decides target library deps, and failures and missing libraries are skipped silently (#17 #346) | distro/builder/src/executor/binaries.rs:17-28,78-136 | Use leviso-elf rooted at the source rootfs; fail on missing libraries | M |
| medium | The repo root is resolved three ways, so `build-all` breaks outside the repo root (#14) | distro/builder/src/bin/workflows/{layout.rs:5-17,build.rs:57-74,artifacts.rs:162} | Resolve once in `main` and pass it down | S |
| medium | `LD_LIBRARY_PATH` pointing at bundled Rocky libraries is exported to every just recipe and to nested cargo (#99) | justfile:5-9; xtask/src/util/tools_env.rs:19-21; xtask/src/tasks/testing/scenarios.rs:871-883 | Set it only on the QEMU child process | S |
| medium | Interactive installed-boot writes to the install disk and OVMF vars, and drops `--inject` silently (#100 #357) | xtask/src/tasks/testing/scenarios.rs:162-170,754-792 | Boot a qcow2 overlay or use `-snapshot`; bail when inject is passed | S |
| low | xtask passes `-enable-kvm -cpu host` unconditionally (#358) | xtask/src/tasks/testing/scenarios.rs:767-775,899-908 | Use recqemu's KVM detection and OVMF lookup | S |
| low | The serial log tail prints partial lines, turns bytes into mojibake, re-reads the whole file every second and leaks temp files (#103) | xtask/src/tasks/testing/scenarios.rs:504-534,588-617,1338-1343 | Track a byte offset; use vt100; use tempfile | S |
| low | `doctor` stops at the tools-prefix check and only checks for `just` (#106) | xtask/src/tasks/tooling/doctor.rs:3-41 | Report every check; iterate the contract's tools | S |
| low | The artifact store unlinks a lock it still holds, and `gc` runs without a lock alongside `put` (legacy users only) (#6) | distro/builder/src/artifact_store/mod.rs:561-591,669-745,875-887 | Keep lock files; add a store-wide reader/writer lock | M |
| low | Blobs are hardlinked into output dirs, so an in-place write corrupts the store (#7) | distro/builder/src/artifact_store/mod.rs:217-277,975-1000 | Use reflink, or make blobs 0444 | S |
| low | The recipe runner can read a stale `.recipe-output.json` and prefers a stale `target/` recipe binary (#9) | distro/builder/src/recipe/mod.rs:130-159,317-375 | Delete the file before spawning; build via cargo | S |
| low | Disk assembly does not check that images fit, discards sfdisk stderr, uses random UUIDs and writes zeros densely (#22 #385) | distro/builder/src/artifact/disk/{assembly.rs:21-96,helpers.rs:12-41} | Size checks; capture stderr with `Cmd`; deterministic UUIDs; `conv=sparse` | M |
| low | install-split-pane can panic slicing a UTF-8 string; it has cast warnings and no license (#23) | distro/builder/install-split-pane/src/main.rs:244-250,448,461 | Cut on a char boundary | S |
| low | The hook's `json_escape` is incomplete, `MODULES_DIR` falls back to `lib/modules`, and the sed banner rewrite is unescaped (#60) | distro/variants/_shared/ring0/hooks/build-release.sh:75-77,223-245,293-299 | Escape properly or emit from Rust; remove the fallback | S |

### Recipe engine
| Severity | Finding | Location | Fix | Effort |
|---|---|---|---|---|
| medium | The HTTP 416 "already complete" branch can never run with ureq 2 (#119) | tools/recipe/src/helpers/acquire/torrent.rs:204-238 | Match `Err(Status(416, ..))` | S |
| medium | `exec()` writes child output to the JSON stdout and returns the exit code instead of throwing (#122) | tools/recipe/src/helpers/util/process.rs:201-210; tools/recipe/src/helpers/internal/cmd.rs:81-107 | Send stdout to stderr; add a throwing `exec` plus `exec_status` | S |
| medium | `download()` has no timeout, writes the final path directly and never checks the length (#127) | tools/recipe/src/helpers/acquire/download.rs:64-77; tools/recipe/src/helpers/acquire/torrent.rs:198 | Download to `.part`, check Content-Length, rename; apply timeouts | S |
| medium | `dnf_download` runs under sudo and returns only newly created files (#130) | tools/recipe/src/helpers/util/process.rs:345-351,430-457 | Run unprivileged; return every matching RPM | S |
| low | Global `unsafe set_var` runs while tokio worker threads are alive (#126) | tools/recipe/src/core/executor/private/state.rs:153-232; tools/recipe/src/helpers/util/env.rs:9-12; tools/recipe/src/llm/runner.rs:5-13 | Keep an env overlay applied per Command | M |
| low | `check_disk_space` returns Ok when it cannot measure, and calls `df` without `-P` (#128) | tools/recipe/src/helpers/install/disk.rs:34-84 | Use `statvfs`; return an error | S |
| low | Without `--build-dir`, each run leaks a temp dir and caching never works across runs (#129) | tools/recipe/src/bin/recipe/commands.rs:16-23,197-207 | Default to an XDG cache dir | S |
| low | `git_clone` reuses any checkout whatever its remote URL; it cannot pin a rev and has two near-identical implementations (#135 #390) | tools/recipe/src/helpers/acquire/git.rs:23-41,65-226 | Verify the remote; pin a rev; merge the two | M |
| low | `info` and `list` take the last `name:` line in the file (#136) | tools/recipe/src/bin/recipe/metadata.rs:12-54 | Evaluate the ctx literal | S |
| low | Read-only checks need a writable dir and leave `.rhai.lock` files behind (#137) | tools/recipe/src/core/lock.rs:13-25; tools/recipe/src/core/executor/private/actions.rs:231 | Skip the lock for checks, or lock under `XDG_RUNTIME_DIR` | S |

### Contract & variant manifests
| Severity | Finding | Location | Fix | Effort |
|---|---|---|---|---|
| medium | Empty `output_names` panics the loader before validation can report it (#51 #63) | distro/contract/src/variant.rs:1361-1380 | `.first()` plus a load error | S |
| low | Only build-host.toml's `schema_version` is checked (#50 #67) | distro/contract/src/variant.rs:572,1516-1531 | Check every manifest | S |
| medium | The derived `ArtifactIdentity` and the injected base defaults make some validator checks unreachable (#71) | distro/contract/src/variant.rs:577,1383-1497; distro/contract/src/validate.rs:193-311 | Drop the mirror; pick one policy for base defaults | M |
| medium | Ring2 tagged enums accept unknown fields; two enums are parsed by hand from strings (#72) | distro/contract/src/variant.rs:924-985,1632-1702 | `deny_unknown_fields`; serde enums | S |
| medium | The live-boot runtime check guesses the init system from the rootfs, ignores the declared overlay kind and duplicates the locale check (#74) | distro/contract/src/runtime.rs:747-1134 | Branch on `OverlayKind`; iterate services generically | M |
| medium | `required_build_tools` is inaccurate and never checked against the host; a separate REQUIRED_TOOLS list exists (#55 #359) | distro/variants/*/build-host/build-host.toml:4-17; distro/builder/src/preflight/mod.rs:29-85 | Have preflight read the contract's list | S |
| low | The kernel.release check is a prefix match, and the version token check is loose (#68) | distro/contract/src/runtime.rs:322-352; distro/contract/src/validate.rs:584-596 | Exact match after stripping localversion | S |
| low | The same defect produces duplicate violations (#69) | distro/contract/src/validate.rs:131-190,932,1169-1170 | Call each check once | S |
| low | Logical names are unvalidated strings, and product and scenario names are redefined per crate (#73 #366) | distro/contract/src/schema.rs:113-130,228-239; distro/builder/src/pipeline/planner.rs:243-280; testing/install-tests/src/scenarios/mod.rs:38-93 | Shared enum in distro-contract; validate references | M |
| low | Runtime validators invent default paths when contract data is missing (#76) | distro/contract/src/runtime.rs:81-126,192-207,385-386 | Report a violation instead | S |
| low | The recipe declaration check is a substring match; kernel.rhai never runs; the evidence script is redundant; `human:` notes are unresolved (#57 #77) | distro/contract/src/{variant.rs:486-517,build_host.rs:14-34}; distro/variants/*/build-host/{recipes/kernel.rhai,evidence/build-capability.sh} | Remove the ceremony or use a structural field | S |
| low | Path validators accept `..` and `./x`; there is a dead loop tail; iso_label and hostname are not checked (#78) | distro/contract/src/validate.rs:534-578,1029-1036,1144-1157 | Use `Path::components`; add RFC checks | S |
| low | Some scenarios report under the wrong `ContractScope` (#86) | distro/contract/src/error.rs:6-15; distro/contract/src/validate.rs:131-146,1206-1256 | Add the missing scopes | S |
| low | The Acorn seed `etc/shadow` is dead; live-docs.sh launches tmux and acorn-docs, which are absent; a missing seed_overlay is skipped (#54) | distro/variants/acorn/ring2/overlays/live/etc/{shadow,profile.d/live-docs.sh:12-30}; distro/builder/src/artifact/live_overlay.rs:73-153 | Delete the seed shadow; fix the launcher; make a missing overlay a load error | S |
| low | `inst.sshd=0` is forced onto Acorn through the base invariants (#61) | distro/contract/src/schema.rs:9; distro/contract/src/variant.rs:1474-1488 | Move it to levitate's scenarios.toml | S |
| low | Levitate's boot_baseline repeats `copy_file` for files already copied by `copy_tree` (#62) | distro/variants/levitate/ring2/products.toml:57-165 | Use an explicit `require_file` kind | S |

### Testing harness reliability
| Severity | Finding | Location | Fix | Effort |
|---|---|---|---|---|
| medium | recqemu `exec` drops every output line containing `# ` or `root@` (#185) | tools/recqemu/src/serial/exec.rs:165-166 | Treat everything between the START and DONE markers as output | S |
| medium | recqemu `exec` aborts on fatal patterns found in the command's own output, its echo, or stale lines (#186) | tools/recqemu/src/serial/exec.rs:112-126 | Apply fatal patterns only outside the marker window | S |
| medium | rootfs-tests writes into a user-supplied `--rootfs` (`--volatile=no`, useradd) (#220) | testing/rootfs-tests/src/container.rs:77-94; testing/rootfs-tests/src/tests/admin.rs:31-39 | `--volatile=overlay` for borrowed inputs; trap-based cleanup | S |
| medium | qcow2 verification mounts with sudo but reads unprivileged, hardcodes p1/p2, uses fixed sleeps and ignores cleanup errors (#224 #392) | testing/fsdbg/src/main.rs:213-399 | Require root; find partitions by label; check every status; `udevadm settle` | M |
| medium | hardware-compat is never enforced; its only callers (legacy leviso) ignore the result (#238) | leviso/src/commands/build.rs:264-322; leviso/src/artifact/iso.rs:156-213 | Add a named scenario or xtask command against `.artifacts` | M |
| medium | Vacuous `#[ignore]` recchroot tests, and 13 recstrap tests that pass as non-root without testing anything (#166) | tools/recchroot/tests/integration.rs:276-307; tools/recstrap/tests/integration.rs:110-125,363-388 | Delete the placeholders; run the root cases as an explicit VM scenario | S |
| low | recqemu `write_file` corrupts backslashes, leaves the path unquoted and can exceed the tty line limit (#187) | tools/recqemu/src/serial/utils.rs:15-31 | Chunked base64 with a quoted path | S |
| low | QEMU option strings do not escape commas, and the disk format is left to probing (#188) | tools/recqemu/src/lib.rs:299-408 | `escape_qemu_opt`; explicit `format=` | S |
| low | `find_ovmf` falls back silently when `OVMF_PATH` is wrong, and pairs CODE and VARS that may not match (#189) | tools/recqemu/src/lib.rs:441-503 | Error on a bad explicit path; return a matched pair | S |
| low | `pkill -9` kills any matching QEMU of the user; the fixed `/tmp` lock is dead; the QMP smoke always reports success (#191 #393 #209) | tools/recqemu/src/process.rs:16-80; testing/install-tests/src/bin/qmp.rs:62-134 | Kill only spawned PIDs; per-run temp dir; call it a screenshot capture | S |
| low | SSH port allocation races, and every spawn sleeps a fixed 2 s (#202) | testing/install-tests/src/qemu/session.rs:58-159 | Check `try_wait` after spawn and retry; drop the sleeps | S |
| low | Automated login uses root and ignores the contract's username and prompt (#204) | testing/install-tests/src/scenarios/mod.rs:716,755 | Use the contract values, or remove the unused fields | S |
| low | Preflight's "required" resolvers require nothing, and the first `*.iso` in read_dir order can be substituted (#207) | testing/install-tests/src/preflight.rs:462-494,622-644,702-716 | Bail with the expected path | S |
| low | The shell scenario library exits at the first failure under `set -e`, so the summary never prints; scripts use bash-isms under `#!/bin/sh` (#208) | testing/install-tests/test-scripts/lib/common.sh:99-149 | Record failures and return 0; use printf | S |
| low | Broad substring fatal patterns, copied in 5+ places; leviso treats "Reached target" as boot success (#210 #344) | testing/install-tests/src/distro/{openrc_base.rs:42,levitate.rs:37-115}; leviso/src/qemu.rs:19-266; distro/builder/src/qemu.rs:14-31 | Keep one anchored list in recqemu or the contract | S |
| low | The install scenario leaves runs stuck in "building", and a prune error turns a pass into a failure (#212) | testing/install-tests/src/scenarios/mod.rs:521-532,633-637,1166-1184 | Put all post-start work inside the closure; best-effort prune | S |
| low | Boot-injection payloads and ISOs pile up in /tmp, possibly with keys (#213) | testing/install-tests/src/boot_injection.rs:24-103; testing/install-tests/src/qemu/session.rs:139-148 | Build once per run in the run dir | S |
| low | `#[cheat_aware]` defaults every field, so empty metadata compiles (#231) | testing/cheat-test/src/lib.rs:91-176 | Required keys; validate the allowed values | S |
| low | rootfs-tests `truncate` and `to_str().unwrap()` can panic; pipelines run without pipefail (#235) | testing/rootfs-tests/src/main.rs:189-197; testing/rootfs-tests/src/container.rs:83,146 | `chars().take(n)`; `.arg()`; `bash -o pipefail` | S |
| low | The qcow2 checklist passes boot entries without `root=` and keeps its service list as a local constant (#237) | testing/fsdbg/src/checklist/qcow2.rs:62-63,236-266 | Move the policy into distro-spec | S |
| low | CONFIG_USB4 is listed twice in 7 profiles (#242) | testing/hardware-compat/src/profiles/{nuc.rs:175-185,thinkpad.rs,framework.rs,dell_xps.rs} | Remove the duplicate; reject duplicate keys | S |
| low | "is not set" is reported as "Not found"; quoted values are kept raw; the config is re-read for each profile (#248) | testing/hardware-compat/src/lib.rs:29-74 | Parse once; tell the two cases apart | S |
| low | Firmware thresholds are magic counts from one linux-firmware snapshot (#249) | testing/hardware-compat/src/profiles/*.rs | Check named blobs | M |

### Error handling & CLI contracts
| Severity | Finding | Location | Fix | Effort |
|---|---|---|---|---|
| medium | The kernel pre-install check reports every failure as "not preinstalled" and drops the error chain (#8) | distro/builder/src/pipeline/kernel.rs:23-90 | Format with `{e:#}`; give rebuild advice only on a real "not installed" | S |
| medium | A failed SSH host-key regeneration is silent under `--quiet`, and the install exits 0 (#145) | tools/recstrap/src/main.rs:436-445; tools/recstrap/src/helpers.rs:139-146 | Always warn; offer `--require-ssh-keys` | S |
| medium | recfstab falls back to `/dev/sdX` paths silently and swallows swap read errors (#156) | tools/recfstab/src/device.rs:90-124; tools/recfstab/src/lib.rs:136-144; tools/recfstab/src/swap.rs:22-25 | Warn on stderr; propagate errors other than NotFound | S |
| low | Hand-rolled CLI with undocumented aliases, still used by the hooks (#25) | distro/builder/src/bin/workflows/{commands.rs:4-115,parse.rs:36-64}; distro/variants/_shared/ring0/hooks/release-artifacts.sh:47 | Move to clap; delete the aliases | M |
| low | Error codes and exit codes mean different things in each rec* CLI; recchroot passes child exits straight through (#158) | tools/recstrap/src/error.rs:72-74; tools/recchroot/src/main.rs:14-23,369-371; tools/recfstab/src/main.rs:199-205 | Shared ToolErrorCode; 128+N for signals | M |
| low | recab exit codes come from matching message text, and status and rollback disagree on the known-good slot (#180) | tools/recab/src/lib.rs:65-105,317-359,1062 | Typed error enum with downcast | S |
| low | reciso ignores `mmd` exit status (#153) | tools/reciso/src/lib.rs:476-509 | Use the checked `Cmd` wrapper | S |
| low | recuki leaves temp files on a spawn error and writes unquoted os-release values (#165) | tools/recuki/src/lib.rs:84-158 | TempDir; escape per os-release(5) | S |
| low | The recstrap cheat-guard banner prints to end users even with `--quiet`; the check count is hardcoded to 14 (#167) | tools/recstrap/src/validation.rs:17-54; tools/recstrap/src/main.rs:401 | Show it only under a debug env var; count the checks run | S |
| low | A missing firmware dir yields dozens of per-pattern failures; `--profile` substring-matches and picks the first hit; the default path is Levitate-only and cwd-relative (#243 #244 #252) | testing/hardware-compat/src/main.rs:12-56 | Validate input; stable ids; `--distro` | S |
| low | hardware-compat has no JSON output, and High failures exit 0 (#247) | testing/hardware-compat/src/report.rs:78-139; testing/hardware-compat/src/main.rs:69-72 | `--format json`; `--fail-on` | M |
| low | No structured logging (about 1,300 println calls); recipe machine events share stderr with human narration (#131 #369) | tools/recipe/src/core/output/logging.rs:69-79; distro/builder/src/cache.rs:28-33 | tracing with env-filter; `--events-fd` | M |

### Duplication / ownership
| Severity | Finding | Location | Fix | Effort |
|---|---|---|---|---|
| medium | Four QEMU builders and three OVMF lookup rules; distro-builder's qemu.rs leaks children; recqemu's boot loop is copy-pasted (#18 #190 #343) | distro/builder/src/qemu.rs:14-560; tools/recqemu/src/lib.rs:49-466; tools/recqemu/src/serial/boot.rs; xtask/src/tasks/testing/scenarios.rs:768-792 | Make recqemu the single owner; delete distro/builder/src/qemu.rs | M |
| medium | Shell scripts embedded as Rust strings; two divergent inittab and autologin generators (#19) | distro/builder/src/pipeline/{live_tools.rs:513-842,overlay.rs:262-323}; distro/builder/src/artifact/live_overlay.rs:95-473 | Move them to `distro/variants/*/overlays` as files | M |
| medium | The release hook re-implements naming and output layout that Rust owns (#52 #353) | distro/variants/_shared/ring0/hooks/build-release.sh:19-44,79-106,240-350; distro/variants/_shared/ring0/hooks/release-artifacts.sh:103-115 | Use `${VAR:?}`; move the steps into distro-builder | L |
| medium | About 600 lines of Variant* serde types mirror schema.rs (#70) | distro/contract/src/variant.rs:520-886,1382-1852 | Derive Deserialize on the schema types | L |
| medium | The run-manifest format lives in 5 structs, with a second "latest successful run" resolver (#214 #354) | distro/builder/src/{run_history.rs:14-72,bin/run_history.rs}; testing/install-tests/src/{preflight.rs:327-329,scenarios/mod.rs:1302-1305} | One pub RunManifest with a status enum | M |
| medium | xtask scenarios.rs is a 1,446-line god module with 10–12-argument functions (#102 #364) | xtask/src/tasks/testing/scenarios.rs:103-1446 | Split into modules; options struct; reuse recqemu and recshuttle | M |
| medium | install_once is about 450 lines with three copy-pasted phase blocks (#124) | tools/recipe/src/core/executor/private/flow.rs:14-458; tools/recipe/src/core/build_deps.rs:15-23 | `run_lifecycle_phase()` | M |
| medium | hardware-compat holds 789 requirement literals for 217 options, with drifting severities (#241) | testing/hardware-compat/src/profiles/*.rs | Component table in TOML plus per-profile overrides | L |
| low | Output layout and distro-id aliases are hardcoded in about 20 places; `write_json_atomic` exists three times (#13 #355 #428) | distro/builder/src/artifact_store/mod.rs:755-772; distro/builder/src/bin/artifact_paths.rs:3-27; distro/builder/src/pipeline/io.rs:76-116; distro/contract/src/runtime.rs:16-19 | Public `paths` module in distro-builder or distro-contract | M |
| low | Four PATH lookup helpers, duplicated signal decoding, and the `which` crate barely used (#26) | distro/builder/src/process.rs:34-301; distro/builder/src/preflight/mod.rs:29-35 | One `which` | S |
| low | Rootfs cache validity depends on hand-bumped version strings; trust-marker helpers are copied across four recipes (#20) | distro/builder/recipes/{alpine-live-source-rootfs.rhai:32,fedora-dvd-source-rootfs.rhai:23-59,fedora-preseed-iso.rhai:22-54} | Stamp from a hash of the recipe file; includable module | M |
| low | AcornOS BOOT_MODULES hand-copies two shared lists (#85) | distro/spec/src/acorn/boot.rs:14-79 | Chain the shared lists | S |
| low | The justfile repeats the SSH-key injection block 5 times (#97) | justfile:108-312 | `--ssh-pubkey` on xtask | M |
| low | Three identical distro enums in xtask (#104) | xtask/src/cli/types.rs:4-37 | One validated id | S |
| low | docs inspect and tui inspect duplicate the capture pipeline, including dead `script(1)` handling (#105) | xtask/src/tasks/docs/inspect.rs:187-298; xtask/src/tasks/tui/inspect.rs:263-322 | Move into `util/pty_capture.rs` | S |
| low | Duplicated extends parsing, LLM dispatch and fence stripping; hardcoded linux-deps special case (#134) | tools/recipe/src/core/autofix/prompt.rs:19-126; tools/recipe/src/helpers/llm.rs:43-108 | Reuse the executor helpers | S |
| low | recpart encodes the layout three times, and the plan's mountpoints do not match where apply mounts (#181) | tools/recpart/src/{policy.rs:219-280,exec.rs:158-316,handoff.rs:22-23} | Generate actions from `plan.partitions` | M |
| low | Bash split-pane launchers duplicate the shipped Rust install-split-pane (#277 #338) | tui/apps/live-tools/install-docs/bin/levitate-install-docs-split; docs/tui/bin/...; justfile:420-427 | `just` runs `cargo run -p install-split-pane` | S |
| low | xtask duplicates install-tests' boot-injection code (#356) | xtask/src/tasks/testing/scenarios.rs:233-311,924-940,1229-1291 | Public constructor in install-tests | S |
| low | The hook installer hardcodes 19 submodules and misses distro-contract and rec{ab,part,shuttle} (#412) | xtask/src/tasks/tooling/hooks.rs:85-107 | Derive the list from `.gitmodules` | S |

### Dead & legacy code
| Severity | Finding | Location | Fix | Effort |
|---|---|---|---|---|
| medium | Dead modules in distro-builder: build/kernel.rs (541 LOC, every function deprecated, including an "atomic swap" that deletes staging); a shell `build_cpio`; `unimplemented!()` initramfs and ISO stubs; byte-identical alpine/timing.rs; a mock recipe (#15 #163 #349 #350 #351 #367 #379 #380 #420) | distro/builder/src/{build/kernel.rs,artifact/cpio.rs,artifact/initramfs.rs:16-98,artifact/iso.rs:86-92,alpine/timing.rs}; distro/builder/recipes/mock-build-stdout-test.rhai | Delete them | S |
| medium | install-tests' numbered phase1–6 step framework (about 2.8k LOC), the Executor trait and the `run` command that only bails (#205 #383 #421) | testing/install-tests/src/{steps/,executor.rs,bin/install-tests.rs} | Delete them | S |
| low | distro-builder's public API is mostly legacy support (alpine, component, executor, recipe::linux bound to `downloads/`) (#16) | distro/builder/src/lib.rs:1-113; distro/builder/src/recipe/{mod.rs:389-483,linux.rs:31-140} | Move into the legacy crates or a `legacy` module | L |
| low | Bugs in leviso's dead pipeline: unverified ISO renamed into place, ISO_LABEL mismatch, rpm2cpio shell strings, recipe lookup, unused common helpers, `unsafe set_var`, drifted rebuild inputs (#34 #37 #38 #41 #43 #44 #347) | leviso/src/{artifact/iso.rs,build/libdeps.rs,resolve.rs,common/,config.rs,rebuild.rs}; AcornOS/src/artifact/iso.rs:33 | Delete them with leviso | S |
| low | AcornOS writes a placeholder recstrap script and skips recfstab and recchroot silently (#381) | AcornOS/src/component/custom/live.rs:98-188 | Delete with AcornOS | S |
| low | Hardcoded `/home/vince` paths persisted into tracked recipes; `persist_ctx` defaults to true (#40 #426) | leviso/deps/{rocky,recstrap,recchroot}.rhai; tools/recipe/src/lib.rs:167 | Delete `leviso/deps`; default persist_ctx to false | S |
| low | leviso stale docs and tracked junk (#42) | leviso/{CLAUDE.md,README.md,SUBSYSTEM_AUDIT.md,build.log} | Delete them | S |
| low | The leviso docs-tui recipe builds a missing `src/index.tsx` (#284) | leviso/deps/docs-tui.rhai:17,117 | Delete it | S |
| low | Dead flat-layout and legacy-filename handling in the loader (#56 #75) | distro/contract/src/variant.rs:33-75,599-815 | Delete it | S |
| low | Unused distro-spec shell-string helpers (useradd, mount, services) and an unused CHROOT_BIND_MOUNTS "source of truth" (#80 #391) | distro/spec/src/shared/{users.rs:61-77,chroot.rs:13-75,services.rs:25-43} | Delete them | S |
| low | Dead recipe helpers (ShellCmd, url_utils, most of fs_utils, check_throws) (#123) | tools/recipe/src/helpers/internal/{cmd.rs,url_utils.rs} | Delete them; make `internal` `pub(crate)` | S |
| low | Dead cheat API (`cheat_check!`, CheckResult, canary "detection") (#230) | testing/cheat-guard/src/lib.rs:172-231; testing/cheat-test/src/lib.rs:343-473 | Delete it | S |
| low | Dead hardware-compat API (categories, recommended_*, Value, Serialize) (#245) | testing/hardware-compat/src/{profiles/mod.rs:10-100,kernel.rs:7,27} | Delete it | S |
| low | Dead kit API (about 60% of exports), a parallel ThemeProvider, test-era aliases and a no-op dispose (#272 #289) | tui/kit/core/src/{theme/theme-provider.tsx,patterns/wizard-flow.tsx:26-28,app/create-app.tsx:38}; shared/tui-kit/src/index.ts | Prune them | M |
| low | Seven stub packages plus the kit/presets-install and kit/testing stubs, whose start scripts exit 0 (#273) | tui/apps/install/{bootloader,bootstrap,chroot-config,filesystem,fstab}; tui/apps/login-gate/firstboot-check; tui/kit/{presets-install,testing} | Delete them, or make start fail | S |
| low | Dead distro-profile layer in install-docs (Acorn uses plain_text, not removed) (#274 #288) | tui/apps/live-tools/install-docs/src/domain/distro/*.ts | Shrink to `{ title, defaultSlug }` | S |
| low | About 48 `allow(dead_code)` sites, three of them crate- or module-wide (#429) | leviso/src/lib.rs:11; leviso/src/component/mod.rs:35; QMP client | Remove the allows | S |
| low | recguard (spec only) and llm-toolkit are submodules no build uses; recinit is an implicit workspace member (#339 #363 #411 #432) | .gitmodules:105-108; tools/recguard; llm-toolkit; Cargo.toml:3-27 | Unpin or justify them; list recinit in members | S |
| low | Tracked editor caches, 226 `.teams` logs, build.log, `.checkpoints`, stale `.gitignore` rules, unreferenced dnf-guard.sh (#112 #340 #410 #430) | .lsp/.cache/; .clj-kondo/.cache/; .teams/; .gitignore:1-92; tools/dnf-guard.sh | `git rm --cached`; prune | S |

### Policy compliance (unit tests & process)
| Severity | Finding | Location | Fix | Effort |
|---|---|---|---|---|
| medium | leviso has 25 `#[test] #[ignore]` artifact tests aimed at an `output/initramfs-root` that nothing produces (#33 #365 #413 #422) | leviso/tests/{validation_tests.rs,helpers.rs,README.md} | Delete them (fsdbg checklists already cover this) | S |
| low | Runnable ```rust doctests remain in distro-spec (#84) | distro/spec/src/lib.rs:23-36; distro/spec/src/acorn/packages.rs:14-20 | Change to ```text, or delete | S |
| low | The recguard spec prescribes minimum coverage and a traceability matrix (#169) | tools/recguard/REQUIREMENTS.md:278-306 | Replace with a list of scenarios | S |
| low | Empty leftover unit-test dirs (#253 #298) | testing/hardware-compat/tests/; shared/tui-kit/tests/unit/; tui/kit/core/tests/unit/ | Delete them | S |

The gates (#3 #95 #206 #382 #419) are in Top priority 6. Process rules in the public FAQ (#307) are under Docs drift.

### Dependencies & workspace
| Severity | Finding | Location | Fix | Effort |
|---|---|---|---|---|
| medium | leviso-cheat-guard and leviso-cheat-test are built twice, from the path and from crates.io (#107 #229 #323 #361 #423) | testing/cheat-guard/Cargo.toml:16; testing/rootfs-tests/Cargo.toml:10; Cargo.lock:1963-1995 | Path deps with a version, or `[patch.crates-io]` | S |
| medium | Missing or inconsistent licenses: MIT-only crates under a dual-license root; no license on install-split-pane, which ships in the ISO; missing LICENSE files; stale root LICENSE text (#195 #326 #405 #431) | distro/builder/install-split-pane/Cargo.toml; testing/{hardware-compat,rootfs-tests,cheat-*}/Cargo.toml; tools/recfstab; tools/{recpart,recshuttle}; LICENSE:9-65 | Inherit `MIT OR Apache-2.0` from the workspace; add LICENSE files; fix the root text | S |
| medium | librqbit pulls in about 290 of recipe's 391 deps, including a second HTTP/TLS stack and aws-lc (#329) | tools/recipe/Cargo.toml:33,40 | Put torrent behind an optional feature | M |
| low | No `[workspace.dependencies]`; duplicate majors (which 5/7, thiserror 1/2, nix, bitflags) (#324 #362) | Cargo.toml:30-31; distro/builder/Cargo.toml:26; leviso/Cargo.toml:35 | Workspace deps; unify versions | M |
| low | `[workspace.package]` is never inherited; editions are mixed (2021 and 2024); MSRV is set on only about 10 crates; the README says Rust 1.75+ (#325) | Cargo.toml:30; xtask/Cargo.toml:4; tools/recipe/Cargo.toml:4; README.md:81 | Inherit edition, MSRV and license; add rust-toolchain.toml | M |
| low | Wrong repository URLs in the recipe and cheat crate manifests (#133 #327) | tools/recipe/Cargo.toml:7-9; tools/recipe/scripts/build-apk.sh:50; testing/cheat-{guard,test}/Cargo.toml:8 | Match `.gitmodules` | S |
| low | Confirmed unused deps (recipe sha3 and semver; leviso dirs, walkdir and sha2; fsdbg colored, humansize and cheat-guard; hardware-compat serde_json, thiserror, walkdir and regex; install-tests libc) (#234 #246 #328) | tools/recipe/Cargo.toml:24,36; testing/fsdbg/Cargo.toml:18-20; testing/hardware-compat/Cargo.toml:9-16 | Remove them; confirm with cargo-machete | S |
| low | recinit is an implicit member with no MSRV, stale doc example and unused public API (#168) | tools/recinit/Cargo.toml; tools/recinit/src/lib.rs:228-343 | List it in members; prune | S |
| low | Two JS lint toolchains (biome and oxlint), unpinned Bun in CI, split TypeScript ranges (#335) | package.json:20,26; docs/content/biome.json; .github/workflows/bun-ci.yml:34 | Standardize on oxlint and oxfmt; `bun-version-file` | S |

### CI / release maturity
| Severity | Finding | Location | Fix | Effort |
|---|---|---|---|---|
| medium | Standalone submodule CI cannot resolve its `../../` path deps (recstrap, recchroot, recpart, recipe, leviso, install-tests, docs/website `workspace:*`), so release jobs never fire; reusable workflows are pinned to `@master` (#121 #157 #310 #332 #395 #418) | tools/recstrap/{Cargo.toml:21-33,.github/workflows/ci.yml:29-116}; tools/recipe/{Cargo.toml:52-53,.github/workflows/ci.yml}; docs/website/{package.json:18,.github/workflows/ci.yml:12-23} | Build only from the superproject (delete the standalone CI), or check out the superproject; pin to a tag or SHA | M |
| medium | `contents: write` granted to every job, including PR jobs; actions and toolchains are unpinned (#403) | distro/elf/.github/workflows/ci.yml:17-18; tools/recipe/.github/workflows/ci.yml:17-18; .github/workflows/bun-ci.yml:32-34 | Read-only at the top level; write only on release; SHA pins | S |
| low | Tags are inconsistent (`v.0.1.0-alpha`, `0.0.0-pre-alpha`, mixed `v` prefixes), and the recipe CHANGELOG has gaps (#404) | git tags; tools/recipe/CHANGELOG.md:8-23 | One format, `vX.Y.Z` | S |
| low | Submodule URLs mix https and ssh; repo names differ from paths; `push.recurseSubmodules` is unset (#406) | .gitmodules:1-123 | https or relative URLs; set `push.recurseSubmodules check` | S |
| low | Dead CI env vars (`REC_BUILD_RELEASE`, `RECIPE_BINARY`) (#409) | .github/workflows/ci.yml:11-12,31 | Remove them | S |

### Docs drift
| Severity | Finding | Location | Fix | Effort |
|---|---|---|---|---|
| high | The "Complete" recipe CLI reference documents `update`, `upgrade`, `search`, `deps` and `install --deps`, none of which exist (#302) | docs/content/src/content/02-package-manager/01-cli-reference.ts:5-142 | Generate it from clap `--help` output | M |
| medium | CLI reference examples use the retired recipe format and unregistered helpers (`cd`, `run`, `install_bin`, `PREFIX`) (#303) | docs/content/src/content/02-package-manager/01-cli-reference.ts:357-430 | Use ctx-style examples; parse them at docs build time | S |
| medium | llms.txt and llms-full.txt are hand-written, name 17 of 26 helpers that are not registered, and miss 11 pages (#304) | docs/website/public/llms.txt; docs/website/public/llms-full.txt | Generate them at build time | S |
| medium | The homepage claims a local SmolLM3 model powers the recipe assistant; recipe only shells out to Codex or Claude (#305) | docs/website/src/pages/index.astro:165-167,457 | Correct the claim, or mark it as planned | S |
| medium | Retired appliance variants and past release dates are still published (#306) | docs/content/src/content/04-architecture/03-atomic-updates.ts:63-69; docs/content/src/content/01-getting-started/01-getting-started.ts:34-41; docs/website/src/pages/download.astro:25-27 | Remove or update them | S |
| medium | The public FAQ publishes agent-process rules that contradict AGENTS.md, "every commit is tested", and a link to a missing SUPPLY_CHAIN.md (#307) | docs/content/src/content/04-architecture/01-faq.ts:113,161,493-560 | Remove them | S |
| medium | README and distro-spec say the base is Rocky; the product builds from Fedora 43; the Quick Start is leviso-era with broken `\\` continuations (#111 #427) | README.md:25-28,73-99,148-202; distro/spec/src/lib.rs:40 | Rewrite around `just release-build` | S |
| medium | hardware-compat docs claim CI verification, CPU, UEFI and Secure Boot checks that do not exist (#250) | testing/hardware-compat/{README.md:7-21,CLAUDE.md:9-11,PLAN.md} | Rewrite; delete PLAN.md | S |
| low | The recipe helper reference leaves out about 30 registered helpers, including the dnf, apk and rpm family (#318) | docs/content/src/content/03-helpers/01-overview.ts | Add a package-source helpers page | M |
| low | distro-builder's README, lib.rs header and migration index describe an architecture that no longer exists (#24) | distro/builder/{README.md:9-77,src/lib.rs:1-56,docs/00_MIGRATION_INDEX.md:15-29} | Rewrite around the ring pipeline | S |
| low | Variant READMEs and AcornOS/CLAUDE.md are wrong (#58 #368) | distro/variants/{levitate,acorn}/README.md; AcornOS/CLAUDE.md; leviso/CLAUDE.md:30-44; testing/install-tests/CLAUDE.md:45-49 | One distro/variants README; point at AGENTS.md | S |
| low | distro-spec's README and CLAUDE.md misdescribe the crate (no_std, "Acorn stubbed"); the `std` feature does nothing (#83) | distro/spec/README.md:24-85 | Rewrite; drop the feature | S |
| low | Recipe lib.rs helper list, `--autofix-cwd` help text and APOLOGY.md are stale (#138) | tools/recipe/src/lib.rs:44-92; tools/recipe/src/bin/recipe/cli.rs:64-65; tools/recipe/APOLOGY.md | Fix or delete | S |
| low | recpart docs describe a removed TUI and cargo-test baselines; unused tool requirements and error codes (#183) | tools/recpart/docs/installer-production-readiness-gaps.md; tools/recpart/REQUIREMENTS.md:118-142; tools/recpart/src/preflight.rs:52-61 | Delete or move them | S |
| low | install-tests CLAUDE.md, FAIL_FAST_POLICY.md and APOLOGY.md describe a harness that no longer exists (#211) | testing/install-tests/{CLAUDE.md:28-53,docs/FAIL_FAST_POLICY.md:73-80,APOLOGY.md} | Rewrite or delete | S |
| low | rootfs-tests and fsdbg document flags, categories, methods and tarball paths that do not exist (#232 #233) | testing/rootfs-tests/{README.md,src/main.rs:113-114}; testing/fsdbg/README.md:32-92 | Sync with the CLI | S |
| low | The cheat crate READMEs give wrong crate names, and one promotes a unit-test example (#236) | testing/cheat-{guard,test}/README.md | Fix names; use a CLI example | S |
| low | tui MIGRATION_PLAN.md and README misstate what is done (#265) | tui/MIGRATION_PLAN.md:23-154; tui/README.md:16-18 | Rewrite to match the current state | S |
| low | Keybinding docs disagree between README, `--help` and the status bar (#297) | tui/apps/live-tools/install-docs/README.md; tui/apps/live-tools/install-docs/src/cli/help.ts | Generate both from one keymap table | S |
| low | docs-content README, license and tracked `.turbo` logs are stale (#317) | docs/content/{README.md:44-113,LICENSE,package.json:4} | Fix; `git rm --cached` | S |

### TUI / TS
| Severity | Finding | Location | Fix | Effort |
|---|---|---|---|---|
| high | ansi256 mode returns bare numbers, which Ink ignores, so every theme color disappears (#280) | tui/kit/core/src/theme/colors.ts; shared/tui-kit/src/theme/colors.ts:260-336 | Return `ansi256(n)` | S |
| high | `minColumns: 88` blanks the viewer on 80-column VTs and on split panes narrower than about 177 columns (#291) | tui/apps/live-tools/install-docs/src/presentation/ink/theme/index.ts:76; tui/kit/core surface-frame | Lower the minimum, or collapse the sidebar | S |
| medium | The ErrorBoundary swallows fail-fast render errors with no stack and hangs instead of exiting non-zero (#271 #285) | tui/kit/core/src/app/{error-boundary.tsx:29-38,render-app.tsx:28} | onError: log the stack, exit(error), non-zero status | S |
| medium | recab and postinstall-tools are byte-identical apps wired nowhere; recab-runner.ts is duplicated (#258 #394) | tui/apps/runtime/{recab,postinstall-tools} | Delete one | S |
| medium | Runtime apps typecheck against a hand-written kit shim and sit outside the workspaces (#259 #336) | tui/apps/runtime/*/tsconfig.json:15-17; package.json:4 | Drop the shim; add them to workspaces | S |
| medium | lint, format and check scripts point at a `.oxlintrc.json` and `.oxfmtrc.json` that do not exist; `check` runs twice (#110 #262 #283 #334) | package.json:4-23; turbo.json:18-20; tui/kit/core/package.json:16-20; tui/apps/live-tools/install-docs/package.json:13-18; docs/content/package.json:22-23 | Commit the root configs (with react-hooks rules); pin oxfmt | S |
| medium | Line measurement and rendering use separate implementations that have drifted (nested code widths, table cells) (#286) | tui/apps/live-tools/install-docs/src/presentation/ink/blocks/{code-block.tsx:97-182,qa-block.tsx,table-block.tsx:114-127} | Derive measure from render's rows | M |
| medium | The "default" color sentinel falls back to substitute colors in ansi256 and ansi16 (#281) | tui/kit/core/src/theme/colors.ts:369-384 | First-class inherit value | S |
| low | Rules-of-hooks violations (27 oxlint errors); practically safe today because they are useContext calls (#257 #287) | tui/apps/live-tools/install-docs/src/presentation/ink/{screens/install-viewer.tsx:224-467,blocks/shared/intent-color.ts:16-23} | Call hooks unconditionally | S |
| low | The recab TUI commits or rolls back on a single keypress, with a busy guard read from a stale closure (#266) | tui/apps/runtime/postinstall-tools/src/presentation/runtime-recab-screen.tsx:109-168 | Ref guard; Confirm step; timeout | S |
| low | disk-plan rebuilds a weaker preflight (`bash -lc`, `statSync`) than recpart's (#267) | tui/apps/install/disk-plan/src/app/preflight.ts:12-65 | Render recpart's own preflight | M |
| low | `listHostDisks` swallows lsblk failures (#268) | tui/apps/install/disk-plan/src/app/plan-runner.ts:162-188 | Return the error and show it | S |
| low | Duplicated JSON guards and spawn runners; `REPO_ROOT` used as cwd; dead exports (#276) | tui/apps/install/disk-plan/src/{domain/plan-model.ts,domain/apply-model.ts,app/plan-runner.ts:4-115} | One `runJsonCommand()` | S |
| low | The docs viewer reports "copied" even when OSC 52 is ignored (#269) | tui/apps/live-tools/install-docs/src/presentation/ink/screens/install-viewer.tsx:188-196,376-383 | Say "sent (OSC 52)" | S |
| low | Hotkeys case-fold, so `G` fires both handlers; a space binding is trimmed to nothing and never fires (#270 #296) | tui/kit/core/src/runtime/ink/use-input.ts:60-74,171-189; tui/kit/core/src/utils/arrays.ts:1-9 | Literal key plus `S-x`; never trim | S |
| low | The rendering layer imports from presentation (#275) | tui/apps/live-tools/install-docs/src/rendering/plan/line-metrics.ts:4 | Move the measure functions | M |
| low | The inspect script assigns bash's `SECONDS`; the PTY e2e depends on cwd and builds xtask at import (#278 #292) | tui/apps/live-tools/install-docs/scripts/verify-inspect-snapshots.sh:10,30; tui/apps/live-tools/install-docs/src/install-viewer.chrome.pty.test.ts:36-37 | Rename the variable; use `import.meta.dir` | S |
| low | The whole document is rebuilt and laid out on every keypress (#290) | tui/apps/live-tools/install-docs/src/{rendering/pipeline/viewport.ts:68-79,presentation/ink/components/content-pane.tsx} | Memoize; render only the window | M |
| low | Literal colors bypass the color runtime; `enabled:true` downgrades the color depth (#293) | tui/kit/core/src/primitives/display/rich-text.tsx:27-29; tui/kit/core/src/theme/colors.ts:233-238 | Resolve literal colors | S |
| low | Width is measured in UTF-16 code units, not terminal cells (#294) | tui/kit/core/src/utils/strings.ts:15-43 | string-width; grapheme slicing | M |
| low | Package metadata: exports point at raw `.ts`, an unused dist build, a fake `deprecated` field, SPDX-only LICENSE (#295) | tui/kit/core/{package.json,LICENSE}; shared/tui-kit/package.json | Drop the build or point exports at dist; ship license texts | S |

### Docs build & llm-toolkit
| Severity | Finding | Location | Fix | Effort |
|---|---|---|---|---|
| high | `DataCollatorForLanguageModeling` discards the loss mask, so training covers prompts and never learns EOS (#299) | llm-toolkit/train_lora.py:5-6,185-191,277-279,400 | `default_data_collator`; build labels from the attention mask | S |
| medium | `sweep_hyperparams.py` passes flags the other scripts do not have and runs rmtree on adapters first (#300) | llm-toolkit/sweep_hyperparams.py:49,104-134,224-226 | Rewrite with direct imports, or delete | M |
| medium | evaluate.py and llm_server.py fall back silently to the base model when the adapter path is wrong (#301) | llm-toolkit/evaluate.py:134-138,353; llm-toolkit/llm_server.py:91-97 | Fail loudly | S |
| medium | generate_data mixes index spaces in a shared checkpoint; `validate` says "all valid" when lines were dropped; `--substitutions` is ignored (#311 #312) | llm-toolkit/generate_data.py:53-177,302-436 | Key by original index; count parse failures | M |
| medium | llm-toolkit packaging: dead `__init__.py`, version floors too low, unpinned heavy CI installs, `trust_remote_code` everywhere (#314 #408) | llm-toolkit/{__init__.py,requirements.txt,.github/workflows/ci.yml}; .github/workflows/python-ci.yml:36-46 | Decide whether it stays a product; pin; opt-in flag | M |
| medium | Two content-discovery implementations; the production site uses the unvalidated one (#308) | docs/content/{src/discovery.ts:28-63,scripts/build.ts:84-270}; docs/website/src/pages/docs/[slug].astro:4 | One shared module | M |
| medium | The generated index is gitignored while its check says "commit it"; turbo does not cache the output (#309) | docs/content/{.gitignore:4,scripts/build.ts:52-62,package.json:5-16} | Pick committed or built; declare the outputs | S |
| low | llm_server returns HTTP 200 on failure and accepts unbounded size and `max_tokens` (#313) | llm-toolkit/llm_server.py:198-297 | 500 or 422; caps | S |
| low | The InlineContent link regex captures `)` into the href (#315) | docs/website/src/components/docs/InlineContent.astro:43 | Fix the regex, or remove the legacy parser | S |
| low | Invalid meta is silently replaced with defaults; a page with no DocsContent export only warns (#316) | docs/content/scripts/build.ts:131-249 | Throw | S |

## Per-repo health
- **superproject (CI, justfile, root config): D.** CI does not run on submodule bumps, lockfiles are gitignored, and the policy gates are forced.
- **distro-builder: C.** The active pipeline is sound, but supply chain in the payload, the forced policy gate and legacy library bloat drag it down.
- **distro-contract: B.** It is the right owner, but validation is opt-in, parts of the schema are never validated, and it carries about 600 lines of serde mirror types.
- **distro/variants: B-.** The manifests are clean but declare outputs nothing builds, and the shared hook re-implements logic Rust owns.
- **distro-spec: C.** Marked deprecated but load-bearing, it duplicates kernel and identity facts and defaults the boot editor to on.
- **xtask: C.** A 1,446-line scenarios module, process gates, and VNC on 0.0.0.0.
- **leviso: D.** Dead, uncompiled code with ignored tests, kept alive only by recart.
- **leviso-elf: C.** Copy follows symlinks, readelf parsing depends on locale, and it auto-publishes on every push.
- **AcornOS: D.** Broken at runtime, duplicates distro/variants, and is still a workspace member.
- **tools/recipe: C.** ctx persistence and scope drift, tar hard links, and standalone CI that cannot build.
- **tools/recstrap: C.** Plaintext password script, and `--force` overrides every guard.
- **tools/recfstab: B.** Drops `ro` and `noatime`, and falls back to `/dev/sdX` silently.
- **tools/recchroot: C.** Writes into the host's resolv.conf, mounts propagate to the host, and Ctrl-C is ignored.
- **tools/recinit: C.** Drops gzip finish errors, its busybox cache can be poisoned, and output is not reproducible.
- **tools/reciso: C.** Ships the host's systemd-boot, the legacy alias changed meaning, and it deletes fixed scratch dirs.
- **tools/recuki: B.** Temp file cleanup and os-release quoting only.
- **tools/recab: C.** Rollback leaves the one-shot armed and failures leave boot config half-changed.
- **tools/recpart: C.** Cannot partition a blank disk and wipes held disks.
- **tools/recqemu: C.** `exec` corrupts output, and the fatal-pattern check fires on the command's own output.
- **tools/recshuttle: C.** Has its own A/B layout and a fixed-index ordering bug.
- **tools/recart: D.** Built entirely around the legacy crates, stores outputs under the wrong keys, and exits 0 on failure.
- **tools/recguard: D.** No code; its spec conflicts with project policy.
- **testing/install-tests: C.** False passes, stale cache, leaked QEMU processes and about 2.8k dead LOC.
- **testing/fsdbg: C.** Unconditional passes, broken EROFS reader, CPIO-only checklists.
- **testing/rootfs-tests: C.** Mutates the rootfs it is given, and its checks pass on bad input.
- **testing/cheat-guard and cheat-test: C.** The macro drops signatures, the crates are built twice, and the API is dead.
- **testing/hardware-compat: D.** Never enforced, firmware globs count directories, and the data is mostly copy-paste.
- **tui (kit/core, install-docs, disk-plan): C.** The disk-plan apply is critical; colors break in ansi256 mode.
- **shared/tui-kit: D.** A deprecated fork that has diverged and that CI still targets.
- **docs/tui: D.** A deprecated fork, and its inspect check renders a different app.
- **docs/content: C.** The public CLI reference is false, and there are two discovery implementations.
- **docs/website: C.** False SmolLM3 claim, hand-written llms.txt, and standalone CI that cannot build.
- **llm-toolkit: D.** Loss masking is undone, the sweep script is broken, and nothing in the repo uses it.

## Suggested sequencing

None of these waves adds unit tests or mandatory process. Check each fix with compilation plus the named CLI, scenario or PTY runs. Keep every check explicit, never a forced gate.

**Wave 1: quick wins, mostly S effort (1–2 weeks)**
- Delete the policy gates and the build-hours window (TP6).
- Commit the lockfiles and add `--locked` (TP5).
- Fix the CI filter, submodules, deny.toml, tui-matrix and workspace clippy (TP7).
- recchroot: resolv.conf, `MS_SLAVE` and `pre_exec` signal reset. recstrap: password over stdin (TP3).
- recpart: wait after sfdisk, drop `--force`, roll back mounts (TP2).
- disk-plan: one-shot apply and full-stdout `JSON.parse` (TP1).
- Drop `--no-check-certificate` and `--allow-untrusted`, restore resolv.conf, pass apk packages as argv (TP4, part).
- Tool probes with `command -v`, the QEMU `Child` guard, and recinit gzip `finish()` (TP8, TP13, part).
- VNC on loopback by default, safe sync script, ansi256 fix, editor off by default.
- Delete the obvious dead code: build/kernel.rs, cpio.rs, the unimplemented stubs, the leviso orphans and ignored tests, install-tests `steps/`, the TUI stubs, and the tracked caches.

**Wave 2: stop false passes and protect artifact integrity (2–4 weeks)**
- Evidence helper with marker checks, a fingerprint that covers the contract and scripts, overlay disks, a boot deadline (TP8).
- Fix the fsdbg, cheat-test, rootfs-tests and hardware-compat checks (TP14).
- Fix the recab one-shot clear, loader.conf snapshot, efivar reconciliation and fsync (TP11, part).
- Fix the recipe engine: hard links, ctx serializer, `build_scope`, check errors and dep lifecycle (TP12).
- Add checksums to every recipe download, acquire bun, systemd-boot and busybox through recipes, and build `--release` binaries (TP4).
- Finish recinit hardening (TP13).

**Wave 3: consolidate ownership (1–2 months)**
- Retire leviso, AcornOS, recart's legacy coupling, and the docs/tui and shared/tui-kit forks (TP9).
- Make the contract the only source of facts: `ValidatedContract`, kernel facts from build-host.toml, contract-driven tool, pattern and user lists, distro-spec reduced to its live parts (TP10).
- Single A/B layout contract, with recshuttle calling recpart (TP11).
- recqemu as the only QEMU owner, one run-manifest type, a public `paths` module, workspace deps and package inheritance.
- Move the hook's work into Rust.

**Wave 4: release maturity (ongoing)**
- Provenance in the run manifest, `SOURCE_DATE_EPOCH` through xorriso, EROFS and FAT, ISO upload with its `.sha512`, releases on tags with concurrency and SHA-pinned actions (TP15).
- Settle licensing and repository URLs.
- Regenerate the CLI reference and llms.txt from the real code, and rewrite the README and stale CLAUDE.md files.
- Decide whether llm-toolkit and recguard stay.