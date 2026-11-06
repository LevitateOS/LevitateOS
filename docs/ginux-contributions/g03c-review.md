# G03c — configured ephemeral memory storage

Current retained local commit `3801fd45bd6280a5b320f2d464ada0bd04539326`, based on
composition `85ee433afd71c9985cb714246c343145c2cdfb01` of refreshed G03b and G03a,
rooted at observed main `ff926d0c`. The current-main refresh below owns fresh
evidence and pending checks; earlier results remain historical.

Earlier local feature commit `b36e0752caa1c91e4bbbe112c589c85915a6ce6d`.
Local base `603391c66d37915f3db2c9cec63d1c3e832216a6` composes reviewed
G03b785bfb9d (including G05a) and G03a0b563cf8 on upstream3f2fe7df.
[Retained source patch](patches/G03c.patch). Ten files,+1140/-28.

This joins the existing configured-launch/VFS owners: explicit memory assignments,
bounded supervisor-owned files/descriptors, virtual metadata and strict tracee
copies. It adds storage fixtures and CI commands while retaining every existing
unit module, target and native-supervision command. No downstream policy or
Recipe implementation enters the source patch.

## Review corrections

A separate full-diff self-review covered configuration, descriptor authority,
path classification, offsets, copy faults/partial effects, bootstrap and task
teardown, host/native fallback, documentation and CI. Independent reviewer
coverage is not claimed.

Four real cases fail on the preserved initial adapter binary (exit81/82/83/84):
file/../created wrongly creates a file; missing/../existing erases a missing
component; an image regular-file parent can be erased before a memory create;
and an image link into assigned memory exposes a physically hidden image file.
The final fixture passes all four with component-wise resolution using the
existing VFS walker. The small image lookup/canonical rewrite from G16 moves
forward because assignment isolation already requires it. G16 still owns later
owned-link/cwd/device/exec joins; none of those features is imported here.

Canonical image fallback is checked through an assignment hiding an image
symlink: /virtual/../image reads the correct image file. Native AT_EMPTY_PATH
metadata remains functional. Owned access/readlink operations never leak hidden
image objects; unsupported behavior returns an error.

Temporary native paths restore scratch bytes and original argument registers at
the syscall exit before userspace resumes. Controlled mapped pages cover full
success, overlapping output, read-only scratch and a partially writable buffer.
Partial setup failure restores exactly the changed prefix; failed restoration
terminates supervision rather than resuming with altered memory. Stat/fstatat/
statx validate supported flags, reserved mask and conflicting sync bits.

Existing storage checks additionally cover immutable host sentinels, repeated
fresh launches, unknown/duplicate/noncanonical assignments, regular data and
metadata, unlinked-file lifetime, failed truncating open at FD exhaustion, object
quota, per-file capacity, bad pointers, read-only pages and partial read copyout.
The newer upstream supervisor/core files remain byte-for-byte unchanged, including
#221's protected-copyout/fallback helpers and fixtures.

## Validation and delivery boundary

Both the candidate source and a second clean prerequisite export with only this
patch applied pass77 unit tests and:

- `make check test-unit`
- `make build build-launch-fixture build-launch-storage-fixture build-process-fixtures build-gate1-fixture`
- `sudo -n make test-launch test-launch-storage test-native-supervision test-gate1-setup`
- `make test-process-native` outside the ptrace-restricted execution sandbox

The initial sandboxed process attempt could not stop the fixture after exec;
the complete unrestricted rerun passes. An intermediate harness mode name collided
with the scratch mode and was corrected before final validation. Logs retain both
failures and corrected results. Clean replay matches all153 tracked source paths.
The source patch passes strict whitespace/application checks; existing modes and
symlinks are preserved. Complete logs, exact hashes, manifest and short draft are
in `.artifacts/ginux-contribution-preparation/G03c/`.

R01–R03 and clean composition replay pass locally. **Standalone delivery waits
for G05a/G03a/G03b upstream**, then refresh/extract/revalidate against new main;
this local composition must not be published as a stack. macOS Actions/full
Rocky VM checks are unrun. Bounds are32 objects,64KiB per file,64 descriptors and
4096-byte I/O. Owned symlinks, directory FDs, generated exec, mappings and retention
remain later work. Canonical native paths require writable scratch below the
stack; overlapping output returns EFAULT. Strict process_vm copies do not force
writes into protected pages. No real branch/index/worktree/remote was changed.

A final introductory README correction removes the obsolete blanket rejection of
storage assignments and clarifies the owned-symlink exclusion. The final object
differs from the runtime-tested5ee45634 only in README; all executable source
bytes are identical. A fresh final patch application matches all153 paths.

## Current-main refresh — 2026-11-06

The exact local prerequisite combines refreshed G03b `5fb12ab4` (including G05a)
and G03a `c28dd132`; both parent identities are recorded. Only this storage patch
is then applied. It remains ten files, +1140/-28; all added/removed source lines
match the previous corrected extraction. Current upstream native/systemd/domain/
network additions remain intact. Configuration storage still uses the same
eight bounded entries, and configured `launch` still has its own bootstrap and
supervisor; the newer native storage support does not replace this owner.

A separate self-review reread the full proposed diff and current configuration,
VFS, dispatch, bootstrap, syscall completion and failure/teardown callers.
The one-time root index is immutable after preparation; descriptor reservation
remains confined to enabled assignments, owned operations reuse the bounded VFS,
and unclaimed native operations retain the upstream contract. The shared walker
is exported without changing its implementation. Path scratch and kernel-consumed
arguments retain their existing security review requirements; ordinary success
does not prove authority or race safety. No new blocking source correction was
identified, and independent reviewer coverage is not claimed.

Both the candidate and independent clean prerequisite-plus-only-G03c export pass:

- `make check test-unit build build-launch-fixture build-launch-storage-fixture build-process-fixtures build-gate1-fixture`, including all 85 current unit tests and 24 VFS tests.
- Eight ordinary configured-launch/valid-wait controls and three ordinary storage
  controls per export. The latter exercise valid 64 KiB byte-checked I/O, relative
  creation, pread/seek, sparse/append behavior, metadata/modes/CLOEXEC, open-unlinked
  lifetime, truncation, fresh-launch repetition and bounded object exhaustion.

The exact prerequisite is separately built and rejects the same trusted valid
storage fixture with exit 2 and `launch does not support storage or device
assignments`. Candidate and clean replay execute those identical fixture bytes
successfully. The ordinary fixture is a downstream verification artifact;
existing complete Ginux fixtures and default CI remain unchanged and enabled.

All 190 tracked source paths match the candidate in both exports. Both binaries
hash `932c0cc094cc8d331a39311d0cd50c8bfc9d270ada41e29f77b8d3915af5d20b`.
Source/clean/base builds use separate target directories. Exact commands, logs,
receipts, source exports, previous/current patches, manifest and concise draft
are retained under
`.artifacts/ginux-contribution-preparation/upstream-refresh-2026-11-06/G03c/`.
No real Ginux branch, shared index, checkout or remote text was changed.

Full default storage/launch/native/setup suites include authority, protected,
fault and scratch probes and remain unrun under the written security boundary.
Their required validation remains assigned downstream to DaybreakBlue; no agent
has been dispatched. macOS/remote CI is also unrun. R03 retains those checks;
R04 standalone delivery waits for actual G05a/G03a/G03b upstream merges, followed
by extraction/revalidation. Do not publish this local composition as a stack.
The broader combined F02/F03 closure remains pending at its final revisions.

## PR231 contract integration — 2026-11-07

Head `3801fd45bd6280a5b320f2d464ada0bd04539326`, tree
`f85bd3524a30f7f794847516ffd3453d3f49423d`, is on refreshed G03b/G03a composition
`85ee433afd71c9985cb714246c343145c2cdfb01`, rooted atff926d0c. Source preview
e72f1f95 alone lacked the required contract join and was never accepted. The
final scope is thirteen files,+1351/-39 (previously ten,+1140/-28).

Configured launch now resolves the upstream EffectiveContract once and checks
raw/semantic rules before filesystem claims or canonical scratch writes. Shared
rule selection moved into the existing EffectiveContract owner and is reused by
native supervision with identical precedence: semantic denial wins, semantic
allowance cannot grant an unknown/denied raw syscall, and implementation/resource
constraints still apply after policy admission. The filesystem owner classifies
path/owned-file operations without reading guest memory or mutating objects.
Native memory/signal/time/credential families use upstream's canonical IDs.

Configured lstat6/fchmodat268 are registered in the same catalog and generated
coverage document (184 operations). Existing systemd/core consistency assertions
remain for every prior ABI; the two newly cataloged configured-only operations
explicitly assert native Deny38 and catalog Mediate. No core/native syscall
implementation was enabled. Public contract resolution follows the existing
versioned profile owner; catalog/digest additions remain part of the proposed
feature's upstream review, with no compatibility fallback or parallel registry.

The existing configured audit writer retains its64KiB byte cap and whole-record
truncation. Its single-record buffer grows256→512 for bounded domain/digest/policy
metadata. Disabled mode suppresses decision records. The native8192-event audit
budget remains its own upstream implementation. New default fixtures cover valid
owned metadata, raw/semantic denied create without a partial file, disabled audit
and complete contract records at budget exhaustion; all original fixtures and CI
steps remain present/enabled. Denial/security fixture modes were not run locally.

A separate final self-review reread the full original storage delta and complete
PR231 integration delta, existing config/dispatch/bootstrap, VFS namespace/storage,
copy/rewrite, native rule selection and audit/teardown paths. Core, process table,
runtime, supervisor and domain source match the prerequisite exactly. Independent
reviewer/security coverage is not claimed; shared/indirect arguments and all
existing authority/protected/scratch probes remain pending downstream.

Both source and strict prerequisite-plus-only-G03c replay pass:

- `make check test-unit build build-launch-fixture build-launch-storage-fixture build-process-fixtures build-gate1-fixture`:96 units, including24 VFS and10 contract tests.
- Generated contract-doc consistency and Python harness compilation.
- Fifteen ordinary controls per export:8 launch/valid wait,3 data/metadata/position/lifetime/quota,1 valid lstat/fchmodat mode and3 audit metadata/disabled/budget cases. The budget case uses the existing4000-getpid workload and preserves the whole-record/64KiB assertions.

Exact prerequisite85ee433a rejects the identical trusted storage fixture at exit2.
It also reproduces ignored disabled audit with an exit-zero workload; the candidate
suppresses the records. Runtime policy-denial validation remains unrun. The audit
receipt helper initially shadowed its result dict with a regex and failed JSON
serialization; the corrected variable name preserves assertions, and the rerun
passes. Initial evidence and correction record remain retained.

All199 tracked paths match the head in source and clean. Both binaries hash
`43f1ad0ed0a1841fed70665af0e07d1f6754a67d613360d09de1d5777bde06d7`.
Exact sources, complete integration delta, commands/receipts/logs, old/final patch,
manifest and local draft are under
`.artifacts/ginux-contribution-preparation/upstream-contract-refresh-2026-11-06/G03c/`.

This is source/ordinary preparation only. Actual prerequisite merges, default
runtime/policy-denial/security suites, macOS/remote CI and final F02/F03 remain
pending. Do not publish the local composition. Later device/descriptor/task
adapters must preserve policy admission before their effects; G06b's early task
branches need the same join. G07 remains unready. No real checkout/index, branch,
remote text, PR, maintainer message or DaybreakBlue dispatch changed.
