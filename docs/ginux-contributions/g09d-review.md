# G09d — configured directory and basic-path joins

Head `85b53fe15a64041f451ec7e7aa1e77d08cb2c18a`, tree `34854087927e7ca8c951f3d316d0f241bc7a8c90`, on local composition `300fe207a8adf4bd390ec423df8437420fd8cb5f`
(G09b plus G09c). [Retained patch](patches/G09d.patch). Eleven files,+582/-53.
This is not standalone-ready. Include G09c/G09d together in a useful directory
feature after prerequisites merge. The library's new DirectoryNotEmpty variant
needs this runtime errno mapping; the intermediate local composition is not a
buildable publication candidate. Do not publish its pieces as a stack.

## Change and separate self-review

Own directory syscall dispatch through the existing FD/VFS/path owners: open,
getdents64 cookies, relative at operations, rename/noreplace and rmdir. Preserve
component lookup, assignment roots, open replaced/removed objects, public/raw FD
translation and strict copyout. Reject dot-entry mutations before canonicalizing
the path, carrying the G09c correction into the ABI adapter. Current native core,
supervisor and #221 helpers remain unchanged; configured image chdir/getcwd are
explicit task-loop hooks. Owned cwd, native-directory relative FDs, threads,
owned executables, mappings and polling remain excluded.

Use each process's kernel cwd instead of one configured initial cwd. Proc root/cwd
links are read for the owned stopped PID, bounded and made relative to its chroot;
root '/' has its own correct prefix case. Relative image exec and interpreter
inspection use the same cwd. Chdir validates the domain path and retains existing
canonical-path scratch restoration. Fork's cwd semantics remain native.

Basic owned access/readlink and regular-file O_NOCTTY support is implemented;
its runtime acceptance is explicitly pending for DaybreakBlue. Do not import the
old donor's /dev/fd reopen assertion before that feature exists: allocate that
8752f708 hunk to the later executable/descriptor owner. Metadata flag fixes
already covered by G03c are preserved. No mapping fixture is imported for the
old adc79f0a access checks; the small view-probes mode carries their useful queries.

A separate self-review read the final entire diff plus relevant ownership,
resolution, error and cleanup paths. Independent reviewer coverage is not
claimed. A harness failure reused a result filename across selected modes;
unique mode-qualified filenames fix it without weakening the launcher's fresh
result-path rule. Every existing upstream test/CI step remains; the new default
directory/full storage checks are registered in the same integration job.

## Exact evidence and limitations

Source and clean export pass make check/test-unit/build and both fixture builds
(89 units,31 VFS tests). Strict patch replay matches171 tracked paths. Selected
ordinary directory and cwd scenarios pass twice each per export, plus storage
growth/bounds/quota. These validate merged dirents, alias/fork cookies, relative
operations, replacements/removals, preserved inode/data lifetime, normal failure
states, dot-entry preservation, separate parent/child image cwd and relative
image exec. The retained G09b binary297ad8dd fails directory code12 and cwd code94
with the new fixture, demonstrating the missing ABI capabilities. The exact
G09d intermediate composition requires this runtime errno join to build.

Automatic approval review rejected a proposed run including view-probes because
it classified access/readlink authority checks as security validation. That
command did not run. The subsequently approved narrower selection excludes it
and all protected-buffer/private-FD/confinement probes. See the written
[DaybreakBlue handoff](daybreakblue-handoff.md). Those checks, full runtime CI,
macOS Actions, independent security review and current-main standalone refresh
remain pending; no green remote CI or full acceptance is claimed.

Patch/draft/manifests, exact logs, failures and command exit receipts are in
`.artifacts/ginux-contribution-preparation/G09d/`. No branch, real checkout/index,
remote PR text, PR, maintainer message or push changed.

## Current-main refresh — 2026-11-06

Refreshed head `ab47378f657b8d13d72077009107d223ba04d3a3`, tree
`40d9ad17e9ec4581e869c5693665e41f7c7f12f3`, on local refreshed G09b/G09c
composition `eed013c73443a24390b1f5f6638dedca170c855f`, incorporates observed
main `8a475e8b2318425fd2fc15246a5c6f4bca6caf86`. All added/removed lines
match the corrected historical delta; scope remains eleven files,+582/-53.
The required DirectoryNotEmpty errno join stays here. The intermediate
composition remains unbuildable/unpublishable without it. Pair G09c/G09d in
one useful directory proposal after prerequisites merge.

A separate self-review reread the entire diff and existing descriptor translation,
component resolver, task entry, native path rewriting and tracee copy owners.
No new upstream native/domain/network code is replaced. Independent review is
not claimed. Both source and strict clean replay pass the current Makefile's
check/test-unit/build and all applicable launch/storage/device/pipe/FD/task/exec/
directory/process/setup fixture builds: 98 units, including32 VFS. Each export
passes49 ordinary controls inherited from the refreshed growth composition,
plus four directory/cwd launches (two modes, twice each). These cover normal
cookies/aliases/fork, relative operations, replacements/removal, dot entries,
per-process cwd and relative image exec. All208 tracked paths match the head;
both binaries have SHA256
`46257ed31abc862803d94495379b406425fc6d956b2ddd6cb77412a7e777a1d7`.

The identical new fixture on exact refreshed G09b fails functional code12 and
cwd code94. The first baseline receipt incorrectly expected the storage harness's
failure wording; its retained exit12 evidence was correctly recognized after
matching the directory harness's existing workload-exit-code diagnostic. Source
and fixture assertions were unchanged. Exact logs, commands, fresh JSON receipts,
patch, draft and manifests are under
`.artifacts/ginux-contribution-preparation/upstream-refresh-2026-11-06/G09d/`.

Full default runtime, protected/copyout/private-FD/namespace/security probes,
access/readlink view-probes, macOS and remote CI remain pending. The historical
auto-review rejection remains respected; no new security attempt or DaybreakBlue
dispatch occurred. Actual upstream prerequisite merges and standalone acceptance
remain open. All six restored worktrees retain their HEAD/index/status hashes.
No real Ginux checkout/index, branch or remote text changed.
