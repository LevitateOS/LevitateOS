# PRD03 — configured-storage candidate review

PRD03 is complete as local preparation and pre-review. Candidate
`9802ef036d53ae1ee5276fe59cc719389dda5879` is clean and committed in
`/home/vince/levitate-development/ginux-storage-prd03`. Its complete storage
feature diff is **15 files, 4,004 insertions and 129 deletions** against
`f1c3d3a7727f5a8027b448b4dcad0ad52c896c4c`. This is a tested composition with
unmerged prerequisites, not a standalone patch against current main or release
acceptance. Nothing was pushed, published or commented upstream. The existing
Recipe integration, gitlinks, donor worktrees and unrelated staging remain intact.

## Exact sources and dependencies

Read-only GitHub refresh at **2026-11-08 01:44:19 UTC** found main still at
[`53a54addea67969abaa240ea478def1b94130eee`](https://github.com/gmkurtzer/ginux/commit/53a54addea67969abaa240ea478def1b94130eee).
Every PR below remains open/unmerged. Maintainer comments request no changes
and say review follows the foundation work; they do not approve this candidate.
The saved metadata is `upstream-final.json` in the evidence directory below.

| Prerequisite | Exact upstream/retained source | Composed commit | Disposition and verification |
| --- | --- | --- | --- |
| Main / merged #237 | `53a54addea67969abaa240ea478def1b94130eee` | base | Preserve identity reduction, contract UX, tests, CI and HDAE records. All 89 current-main unit test names remain. |
| [#232 shared offsets](https://github.com/gmkurtzer/ginux/pull/232) | `5e79a17286f793b3e7c9301b7653d43abe3a75eb` | `44f7780d4a53960721142ae1b98f2eed219dea5e` | Required for shared open descriptions. Dup, close/reuse, independent-open and append controls pass. |
| [#233 workdir creation](https://github.com/gmkurtzer/ginux/pull/233) | `f751cd6f04f7d95487b0fba5e1c01b6540ab43a7` | `d63e63f1b46e2dbe79a6e68edff1d8ad2d8736b2` | Use the latest head, replacing the spec's historical `44b3f89c`. Launch workdir checks pass. |
| [#238 signal-driven I/O](https://github.com/gmkurtzer/ginux/pull/238) | `efaeeca84829e0c232709da8fcf91d5248952e32` | `913b0acb29e00c72ac8b18b288947d353e50799c` | Prerequisite of #243. Native/launch fcntl and ioctl refusal checks remain and pass; memory fcntl retains the same guard. |
| [#243 contract and audit](https://github.com/gmkurtzer/ginux/pull/243) | `b9b2435528400065b0620bd9f6532c87d36e9c63`, `067d992f29d11f7b79e78753258721fc94a2f2c3` | `88fd1a92999275f7c3dec4a7e6130dcd01416876`, `8e92edafade6b2380abbac62aa04623120520554` | Required before storage effects. Launch denial, provenance, unenforceable-deny startup refusal and disabled-audit tests pass. |
| Zero-fill | unpublished `6f11d011c70b766662835852968c0a40e18fe952` | `7a7e8b5c109b3f4d12faf37a3627abe5285ee735` | Compose with #232's shared offset; retain equivalent `1f34a48c` implementation and regression. Gap, truncate/grow, zero-length and past-capacity read tests pass. Publication/delivery remains separate. |
| [#236 inherited directories](https://github.com/gmkurtzer/ginux/pull/236) | `1f43d7fa1d2ea30975629356492e640e095b4d4e` | `6398b8ea3ca0b80a8f830b12ed922b7b567ac653` | Newly explicit confinement prerequisite: reject directory standard streams, close other inherited FDs. Retain both this guard and #243 startup checks. Launch/native controls pass. |
| Explicit configured catalog 2 | extraction from PRD02 `5300f5ef9a8281f4f10f5519d5eb761654e04829` | `f1c3d3a7727f5a8027b448b4dcad0ad52c896c4c` | Separate unpublished prerequisite. Includes version selection, inspection/diff, config and contract/instance tests, not the whole Recipe integration. Catalog 1 remains original; catalog 2 adds ten existing configured calls. Needs its own upstream disposition before storage can be standalone. |

[#234 orphan adoption](https://github.com/gmkurtzer/ginux/pull/234), head
`78a534c1dcff70207900013381ff857b5687a66e`, was also inspected. It changes
process-parent ownership, not the single-task storage launcher, so it is not
included as a storage prerequisite. The existing process-native suite passes
without it. Its wider product disposition remains with the release/runtime owners.

The retained donor is `prep/configured-storage` at
`545380dd49ea432d9e84e6b956f129f48c72822e`, based on older main `59e2f7aa`.
Its raw diff against current main would remove #237 changes, tests and HDAE
records. It was therefore inspected as a source, then its three feature commits
were composed onto current main plus the prerequisites above. The original
worktree/ref was preserved. Conflict resolutions retain the shared description,
zero-fill, contract/audit, host identity and inherited-descriptor owners.

Feature commits, in order:

1. `9d487303ac2b9960f2e5f5d60233b13f734e839f` — behavior-neutral slot enum.
2. `d37688c3e710f1396e654ed8b01fe2935b58ea61` — retained VFS feature.
3. `72cf37a7b94f2716ea650cb5bd65b46d9df073c4` — retained configured-storage consumer.
4. `4cb12f51aa722c0a2d7d32b34fd9a8c0a2fb8a03` — spec corrections and boundary regressions.
5. `9802ef036d53ae1ee5276fe59cc719389dda5879` — check read permission before read-only truncation.

The final diff has no removed files or upstream unit tests. Existing CI jobs and
steps remain; storage fixture compilation and execution are added. Only the
first preparatory commit claims behavior neutrality; later commits are a coupled
VFS/launcher feature and the final candidate is the acceptance subject.

## Findings and storage-spec disposition

| Finding / spec item | Final behavior and concrete source | Evidence / disposition |
| --- | --- | --- |
| 1. Linux flags leaked into VFS | [VFS status flags](/home/vince/levitate-development/ginux-storage-prd03/crates/ginux-vfs/src/lib.rs:992) return crate values; [launcher fcntl](/home/vince/levitate-development/ginux-storage-prd03/crates/ginux-init/src/launch_storage.rs:857) translates Linux access/append flags. FD_CLOEXEC is separate and per descriptor. | Invalid-FD, status, duplicate and close-on-exec units; real `Status` fixture. Resolved. |
| 2. Raw seek and wrong errors | [Typed Whence and checked seek](/home/vince/levitate-development/ginux-storage-prd03/crates/ginux-vfs/src/lib.rs:973); negative/overflow/unsupported whence give EINVAL without moving the offset. | Base SEEK_END, shared offset, invalid seek, pread/pwrite, past-capacity read controls. Resolved. |
| 3. Copy-up had stable inode but stale readers | [Copy-up](/home/vince/levitate-development/ginux-storage-prd03/crates/ginux-vfs/src/lib.rs:1253) keeps base inode and retargets existing base descriptors only after a successful copy, preserving their access/offsets. Every open sees the same live bytes/metadata. New overlay inodes are monotonic. | Retained-code control fails on stale fstat/read; final control passes. Existing rollback and unlinked/recreated identity tests remain. Resolved. |
| 4. Inconsistent/advisory modes | [Open](/home/vince/levitate-development/ginux-storage-prd03/crates/ginux-vfs/src/lib.rs:523), mkdir/chmod/fchmod preserve 07777; new opens enforce read/write, traversal checks search, parent mutation checks write. Image chmod/fchmod returns EROFS. Existing open rights survive chmod. | Two retained-code controls fail on mkdir bits and advisory modes; final units and real `Modes` fixture pass, including empty-path access and whiteout recreation. Resolved with the explicit common-class model below. |
| 5. Magic slot states | [Slot enum](/home/vince/levitate-development/ginux-storage-prd03/crates/ginux-vfs/src/lib.rs:428), introduced in its own refactor commit. | Complete preparatory diff replaces state numbers without changing decisions. Resolved. |
| 6. Directories lacked operations/accounting | [Directory enumeration/removal](/home/vince/levitate-development/ginux-storage-prd03/crates/ginux-vfs/src/lib.rs:1070), [object/data accounting](/home/vince/levitate-development/ginux-storage-prd03/crates/ginux-vfs/src/lib.rs:1024), launcher directory FDs, openat, chdir/fchdir/getcwd and [getdents64](/home/vince/levitate-development/ginux-storage-prd03/crates/ginux-init/src/launch_storage.rs:957). Directories reserve metadata, not a regular-file data slot. Removed open directories retain inode/fstat but cannot rebind a path. | Merged child enumeration, empty/nonempty removal, dirfd traversal, native/memory cwd transitions, short/faulted output and no cursor advance on first-record failure. Resolved. |
| 7. O_APPEND lived on descriptor | Shared Description owns append and offset; dup/F_DUPFD share it, F_SETFL updates all aliases, independent opens remain independent. | Independent/interleaved append, shared toggling and real `Status` fixture; zero-length append leaves offset unchanged. Resolved. |
| 8. Trailing slash and exclusive creation | Shared component walker preserves traversal order and trailing-directory requirements; O_CREAT with a trailing slash returns EISDIR without creating a file. Exclusive create checks existing final names without following a final link. | Missing/file parents, root/empty path, existing file/directory/link, O_EXCL without O_CREAT, and no-side-effect controls. Resolved. |
| Assignment/confinement composition | [prepare/check_image](/home/vince/levitate-development/ginux-storage-prd03/crates/ginux-init/src/launch_storage.rs:199) reject invalid/reserved/duplicate targets and image-link/file prefixes. [Root lookup/search](/home/vince/levitate-development/ginux-storage-prd03/crates/ginux-init/src/launch_paths.rs:89) uses confined openat2 and actual managed host identity; `.`/`..` search is checked. #236 keeps host directory streams out. | Existing invalid-assignment, hidden image, cross-boundary, host-link, native parity, scratch restoration, inherited-stream and setup-failure controls pass. Resolved for this foundation; separate PRD04/PRD09 reports remain open. |
| Donor broadened catalog 1 | [Explicit storage catalog 3](/home/vince/levitate-development/ginux-storage-prd03/crates/ginux-contract/src/lib.rs:255) adds fchmodat(268) to catalog 2's 192 operations. Catalogs 1/2 remain byte-identical. Nonempty assignments with an older catalog fail before launch. | Old-catalog startup refusal, fchmodat allowance/explicit denial and family denial pass. Five old profiles × three deny variants produce identical full tables/provenance/digests before/after. Resolved without silent wildcard growth. |
| #237 identity mismatch in donor | Removed donor imports of deleted UID/GID constants; image checks use `managed_host_identity`, matching actual tracee identity. | Strict build and actual launch suite under the root-origin UID/GID 65534 backstop pass. Resolved. |
| Read-only truncation could mutate before EACCES | [Launcher open](/home/vince/levitate-development/ginux-storage-prd03/crates/ginux-init/src/launch_storage.rs:635) now checks eventual read access before opening its temporary truncating writer. | New real fixture fails on preceding `4cb12f51` with exit 233 (size changed), and passes on `9802ef03`, preserving all four bytes. Resolved. |

## Contract, limits and remaining integration

The selected contract's syscall and semantic rules are resolved by the existing
`runtime_rule` owner in [launch](/home/vince/levitate-development/ginux-storage-prd03/crates/ginux-init/src/launch.rs:1142)
**before** storage dispatch or scratch rewriting. Explicit deny wins; storage
cannot bypass it. Assignments, modes, descriptor ranges and capacity then narrow
the allowed operation. Entry audit reports syscall, semantic operation/source
rule, action and result; disabled audit stays disabled. Catalog selection does
not change the native syscall policy. The internal operation-table capacity is
256 to fit catalog 3; public storage, FD and resource limits were not raised.

Memory mode evaluation uses any corresponding read/write/search bit as a common
class, independent of virtual UID/GID. This enforces the declared bounded VFS
model; it does not claim full Linux DAC/ACL/sticky/set-ID behavior. Creation uses
fixed umask 022; special mode bits are retained as metadata. Image search checks
actual host UID/GID mode classes, without ACL evaluation. Image kernel calls
still face the kernel's access checks. Directory cookies are valid only while
the namespace is unchanged. A removed memory cwd fails relative operations and
getcwd with ENOENT; recreating its path does not rebind it.

Bounds remain eight assignments, 32 namespace objects (including directories
and retained whiteouts), 64 KiB per regular file, 64 memory descriptors starting
at 1048576, 255-byte paths/link targets and 40 links in composed resolution.
The data pool is statically preallocated; directories do not reserve its slots.
Storage fits the retained 2 MiB stack control. O_PATH/O_TMPFILE, memory links and
rename, target-number dup2/dup3, readv on memory files, memory mappings/execution,
full Linux flag/metadata fidelity, persistence, generations and export are not
implemented here. F_GETFL reports access/append plus O_LARGEFILE; it does not
promise round-tripping every accepted open flag. Proc magic links fail deliberately.
Other unsupported descriptor operations fail without gaining host backing FDs.

The existing Recipe renderer still selects catalog 2. **This foundation cannot
replace the current Recipe integration unchanged:** PRD05 must migrate storage
launches to explicit catalog 3 and reconcile all configured-handler, descriptor,
thread/exec, mapping, export/archive, checkpoint and clock owners from PRD02's
integration before fresh consumer acceptance. No Recipe source was changed.
PRD01/PRD06/PRD07 own maintained/pinned source delivery. The public-FD authority
choice, inherited-FD budget report and private backing/transfer findings remain
PRD04/PRD09 work; the retained high-bit denial control passing here does not
close those reports. macOS CI execution and full product/release acceptance
remain their queue owners. NEEDS is not implemented in this task.

No required product decision remains missing for local PRD03 preparation.
Standalone publication still requires all prerequisite dispositions/merges,
refresh against their actual merged sources, affected revalidation and explicit
user authorization. An upstream catalog version collision would also require
explicit reconciliation before publication, never silent renumbering or growth.

## Exact-candidate evidence and reproduction

Evidence directory:
`/home/vince/levitate-development/ginux-storage-prd03/.artifacts/prd03-storage/`.

- [Full feature diff](/home/vince/levitate-development/ginux-storage-prd03/.artifacts/prd03-storage/storage-feature.patch), [prerequisite diff](/home/vince/levitate-development/ginux-storage-prd03/.artifacts/prd03-storage/prerequisites.patch), [complete main-to-candidate diff](/home/vince/levitate-development/ginux-storage-prd03/.artifacts/prd03-storage/full-main-to-candidate.patch), and individual signed-off patches in `feature-commits/` and `prerequisite-commits/`.
- `candidate-receipt.json`, `commit-stack.txt`, `upstream-final.json`, `toolchain.txt`, and verified `SHA256SUMS` identify sources, tools, binaries and evidence. The
  77-file manifest SHA-256 is `128bdbf4716deee468bf5da8f4215e3b7d7357e67d173b5b0f5e02d7364d05ae`. The exported feature patch was applied to an archive of the declared base and reproduced the final tree exactly (`patch-check.log`).
- `reproduction.log`: final `9802ef03` passed the exact script below, with no tracked source delta. `make check`, all eight `make test-unit` packages (**138 tests**, including 56 VFS), catalog-doc check and the complete CI build command pass.
- Runtime logs: **40 storage launches**, **65 ordinary launch checks**, native-supervision, process-native (including repeated schedules and process_vm_writev fallback variants) and Gate 1 setup-failure suites pass. These are local Linux/x86-64 results with Rust 1.99.0, not a claimed remote macOS CI run or VM/product acceptance.
- `controls.log` plus `control-*.log`: three focused tests transplanted unchanged onto retained `72cf37a7` each fail as expected; all pass in the final unit suite. `control-truncate-mode.log` records the additional real-launch red control on `4cb12f51`.
- `catalog-comparison.log` and `catalog-comparison/`: old-profile tables, provenance and digests compare equal. Catalog arrays are separately compared byte-for-byte in the source receipt. All 89 main unit-test names remain; final count is 138. Existing CI steps and feature regressions were preserved rather than weakened.

```sh
cd /home/vince/levitate-development/ginux-storage-prd03
git rev-parse HEAD                 # 9802ef036d53ae1ee5276fe59cc719389dda5879
git status --porcelain            # empty
(cd .artifacts/prd03-storage && sha256sum -c SHA256SUMS) # verify retained receipts first
bash .artifacts/prd03-storage/reproduce-candidate.sh
python3 .artifacts/prd03-storage/reproduce-controls.py
python3 .artifacts/prd03-storage/reproduce-truncate-control.py
python3 .artifacts/prd03-storage/compare-catalogs.py
python3 .artifacts/prd03-storage/reproduce-patch.py
python3 .artifacts/prd03-storage/verify-source.py

git diff --binary f1c3d3a7..9802ef03    # feature only
git diff --binary 53a54add..f1c3d3a7    # prerequisites only
git diff --binary 53a54add..9802ef03    # complete local composition
```

The scripts regenerate logs; verify the retained manifest before rerunning them.
The final launcher SHA-256 is
`91e97e66181d88638f90914f2a0bd2b35ebfb4e18a2a525a61c2fc4b546a9b0c`;
the storage fixture is
`4f0ccabb5aa7c9ef7c8beb2f12c9c8f61eb07e0e56578f46c361ea1a92022cb2`.
Copies of these and the preceding-revision control launcher are in `binaries/`.

The reproduction script contains `make check`, `make test-unit`, the host
catalog-doc check, `make build build-launch-fixture build-launch-storage-fixture
build-process-fixtures build-gate1-fixture`, `sudo make test-launch`,
`sudo make test-launch-storage`, `sudo make test-native-supervision`,
`make test-process-native`, and `sudo make test-gate1-setup`. Privileged tests
need disposable chroots, credential changes and ptrace. The restricted sandbox
requires the managed privileged execution environment; a ptrace setup refusal is
not a passing functional result. No VM or network download is needed.

Intermediate failures remain classified: the donor initially failed compilation
against #237's removed identity constants; initial red spec controls failed;
a new unit fixture first named a nonexistent `/tmp` parent and was corrected;
strict lint identified pattern/loop/unwrap changes that were corrected; an
initial patch-replay command used an absolute Git apply prefix and was corrected
to a repository-relative prefix. None
was counted as acceptance. The final exact-candidate script exits zero.

[Prepared PR draft](prd03-storage-pr-draft.md) is local text only. The next
queue work is PRD04 and the explicit upstream/integration dependencies above,
not publication or replacing Recipe's runtime with this foundation.

Active skill: `hdae-development`, discovered at
`.agents/skills/hdae-development/SKILL.md`. Reload its complete contents,
required reference, applicable instructions and the current queue after context
shift. PRD03 local preparation is complete; no human answer is pending for it.
