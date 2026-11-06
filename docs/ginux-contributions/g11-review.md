# G11 — owned scripts and interpreter chains; security held

Head `0798d991ab2d652e54682750acb4fdd9f1e6d9ac`, tree `d3fd5defc49cd7194aac82e3e6d6e4565111c19b`, sole local prerequisite
G10 `14866b1de07bc8474557396c09f74275e8fb6bf5`. [Retained patch](patches/G11.patch). Fifteen files,+1162/-57.
Source and ordinary functionality are prepared; security acceptance, full CI and
current-main independence remain pending. Do not publish the local dependency stack.

## Change and review

Reuse the current per-process exec transaction for owned scripts: pin the initial
script and up to three more owned script segments, resolve to an owned ELF or
permitted image chain, and populate the chosen sealed backing. Restore virtual
script names and single optional arguments on the newly committed exec stack,
including AT_EXECFN. Transient image-interpreter script backing is closed at the
first syscall entry, then that original syscall is replayed. Owned ELF leaves
retain their executable pin; script-only references release on successful commit,
failure or death. Native path scratch and typed wait/copyout semantics remain.
No G07/G08 threads/readiness or G12/G17 mapping/backing-budget code was imported.

The separate final self-review read the entire candidate diff and existing
metadata, path, VFS positions/whiteout, descriptor, exec/signal/death and teardown
callers. It is self-review, not independent reviewer or security acceptance.

- **High, port correction:** donor reopening copied an obsolete offset field.
  lib.rs:636 allocates an unreferenced position with initial offset zero, keeping
  G05a's shared-dup-offset model while giving reopens independent descriptions.
  lib.rs:2472 verifies deleted-object retention, replacement identity, independent
  file/directory positions and final-close behavior. It also requires a readable
  source description rather than silently granting read access from write-only.
- **High, source correction:** the late donor used lexical interpreter selection.
  fs_hooks.rs:501 uses current component resolution and assignment precedence;
  native interpretation must match physical image traversal, and ELF leaves keep
  G10's PT_INTERP authority check. Runtime authority/adversarial cases remain
  pending; ordinary owned/image/mixed chains pass.
- **Moderate, source correction:** avoid a second shebang parser. Both native
  metadata inspection and owned-chain planning use path_hooks.rs:198. It honors
  the actual read length, one optional argument, newline/NUL boundaries and the
  existing bounded path contract. Malformed/truncated-line security acceptance
  remains unrun.
- **Moderate, retained cleanup:** launch_tasks.rs:705/934/1344 consume intermediate
  pins exactly through the existing failure/commit/death owner, preserve native
  rewrite restoration and delay CLOEXEC retirement until the exec event. A
  descriptor-named script rejects CLOEXEC before mutation because its interpreter
  must reopen it. The current per-task model replaces donor thread-group calls.
- **Moderate, fixture/docs correction:** old owned-ELF coverage expected scripts
  to fail with ENOTSUP. Keep its failure scenario by using a missing interpreter
  and expecting ENOENT instead. Retain full malformed/loop/large-argument/private-FD
  tests, add their default CI wiring and correct stale README unsupported claims.
- **Blocking for publication:** inherited shared-memory kernel-argument capture
  remains unresolved. New exec stack reconstruction and private script descriptor
  retirement require monitored low-level verification. The
  [DaybreakBlue handoff](daybreakblue-handoff.md) names the exact revision and
  unperformed cases. No exploit-like validation was executed here.

## Exact evidence and limits

Both source and strict clean base-plus-patch export pass `make check test-unit
build build-launch-script-fixture build-launch-owned-exec-fixture
build-launch-directory-fixture build-launch-storage-fixture`:92 units,34 VFS tests.
All177 tracked paths match, including symlinks. Core/native supervisor/process-table
source remains byte-for-byte identical to G10. Existing tests and CI steps remain;
new script/default chain coverage is registered in the same integration job.

Fourteen selected ordinary script scenarios pass in both exports: plain/optional
arguments, nested image interpreter, relative path, absolute execveat, public read
FD, unlinked FD, directory-relative and renamed directory names, fork/vfork,
owned ELF leaf, four-script owned chain and mixed owned-to-image chain. Checks
cover argument/environment order, normal reopen offsets, successful CLOEXEC state,
script bytes becoming writable after commit, result records and complete reported
teardown. They do not inspect proc/private FDs, seals or auxiliary-vector security.

The same compiled ordinary-plain fixture fails on exact G10 with exec ENOTSUP
and workload code94, then passes on G11. Six ordinary owned-ELF controls plus image
cwd and storage growth/bounds/quota also pass on source and clean replay. Commands,
exit receipts, logs, hashes and reconstruction helpers are retained under
`.artifacts/ginux-contribution-preparation/G11/`; binaries stay out of commits.

Default full script/chain security fixtures were compiled and preserved, not run.
Adversarial argv/env/auxv, malformed/cyclic/excessive chains, protected buffers,
private backing/FD retirement and namespace authority validation remain with
DaybreakBlue. macOS Actions, full remote CI and independent review are unrun.
No aggregate RAM quota or complete Linux ABI compatibility is claimed; signals
interrupting private setup remain fail-closed. G11 inherits G10's security hold.

No real Ginux branch, index/checkout, remote, PR text or maintainer message changed.
This is local preparation only; F04 user inspection precedes publication.
