# G13 — owned symlink source and ordinary checks; security held

Head `5c3ae8e5068b5da21d2c37f9545c0f7b10eba147`, tree `ac64b27027ee6f396e844531557846e3a0073bf6`, sole local prerequisite
G11 `0798d991ab2d652e54682750acb4fdd9f1e6d9ac`. [Retained patch](patches/G13.patch). Eight files,+516/-58.
Current-main independence, full CI and security acceptance remain pending; this
is not publication-ready. G12 mappings are not a capability prerequisite and
were excluded from this candidate's ancestry.

## Change and separate self-review

Add owned link target storage, symlink/symlinkat, readlink/readlinkat, nofollow
metadata and correct symlink type/length/directory entries. Keep current shared
VFS positions and exec pins. The existing upstream-derived component walker is
byte-for-byte unchanged: it expands links before later parent components and
checks every directory. Its lookup callback now reads owned target bytes.

Namespace mutation strips trailing separators for final-entry lookup, then
requires that entry itself to be a directory where needed; it must not mutate
a symlink's target directory. Existing final-dot guards stay intact. Owned link
traversal remains within assigned storage; cross-image traversal is still G16.
Image aliases into assigned storage retain current precedence. Current per-task
FD translation handles symlinkat's second descriptor argument and restores it.
Stat/access consume the already-resolved entry without accidentally following
it again. Readlink checks the actual copied byte count rather than reporting an
unperformed complete copy. Runtime readlink/protection acceptance is pending.

The final separate self-review read the complete diff plus existing VFS
lookup/whiteout/position/pin, path request/native rewrite, FD transaction, metadata,
exec/interpreter and teardown callers. It is self-review, not independent reviewer
or security validation. Core/process/native supervisor, script owner, task owner
and storage backing source remain byte-for-byte unchanged from G11.

- **High, reproduced whiteout regression fixed:** allocate_overlay at lib.rs:1475
  must test the existence of the image entry with lookup_exact, not resolve its
  target. A dangling image symlink's whiteout remains reserved. Restoring the old
  followed-target test makes the focused regression fail with Ok(fd3) instead of
  OverlayLimit (exit101); the final implementation passes. Symlink creation may
  reuse its own unreferenced/unpinned whiteout, and a failed link-byte allocation
  preserves that whiteout. The same-name replacement is tested at one-object capacity.
- **High, reproduced donor port regression fixed:** importing donor mkdir also
  replaced the current create_overlay owner with a generic allocation path.
  Removed image names could then fail same-name directory creation at full capacity.
  The expanded regression fails on that donor port (exit101), then passes after
  restoring the existing helper. The final mkdir delta is only namespace resolution.
- **High, retained final-entry semantics:** namespace_path at lib.rs:1087 and
  mutation_path at fs_hooks.rs:216 preserve trailing slash/type requirements;
  rmdir/rename keep final-dot guards. Reusing generic follow-final resolution here
  would act on a directory-link target. Library functional coverage verifies
  failures leave the target directory intact. Runtime adversarial checks are unrun.
- **Moderate, fixture routing corrected:** the donor's first-character 's' mode
  would shadow current scratch-restoration coverage. Match the full 'symlinks'
  name for the new full fixture; preserve the old scratch path and all default
  tests. Ordinary modes exclude runtime readlink, confinement/loop and protected
  buffer probes. No test coverage was removed or disabled.
- **Moderate, packet boundary:** archive/restore/checkpoint source and fixture
  hunks from aa8becf2/d7f628cc remain with G19/G21/G22, not this runtime capability.
  Selected deferred patches and original donor IDs are retained in artifacts.
  Current common resolution replaces the old routed_path lexical implementation;
  no duplicate walker or unused canonical-namespace wrapper is added.
- **Blocking for publication:** inherited shared-argument races and runtime
  traversal/readlink authority verification remain unresolved. The
  [DaybreakBlue handoff](daybreakblue-handoff.md) identifies the exact final head
  and excluded low-level cases. Ordinary success is not confinement acceptance.

## Exact evidence

Source and strict clean prerequisite-plus-patch export both pass `make check
 test-unit build build-launch-script-fixture build-launch-owned-exec-fixture
 build-launch-directory-fixture build-launch-storage-fixture`:95 units,37 VFS
 tests and177 matching tracked paths. This includes ordinary library API cycle,
 target-byte, nofollow, namespace rollback and capacity/whiteout checks; it does
 not include a runtime adversarial chain or security probe. Current unit modules
 and existing CI commands remain intact; new full and ordinary fixture modes run
 from the existing storage/script default targets.

The ordinary link scenario passes in both exports: relative/absolute/dangling
links within owned storage, link-before-parent lookup, symlinkat relative to an
owned directory FD, file I/O through aliases, nofollow stat size/type, symlink
getdents type, and unlink/rename preserving target data. Two new ordinary script
modes preserve the invoked script name and owned-interpreter alias; all16 ordinary
script modes, six owned-ELF controls, storage growth/bounds/quota and directory
functional/cwd controls pass in both exports. Explicit command/exit receipts are
retained. The exact G11 binary rejects syscall88 with ENOSYS and exits code200
on the same valid-link fixture; the final candidate passes.

The first reviewed head881282db and its successful pre-correction clean logs are
retained explicitly as pre-mkdir-fix artifacts. Final clean evidence belongs to
5c3ae8e5 at /tmp/ginux-contribution-preparation/G13/verified-head-final;
do not substitute the earlier export. The final delta from that reviewed head
was inspected separately and consists only of the mkdir helper restoration and
expanded regression. Binary hashes match source/clean builds.

Full runtime readlink/protected-buffer/confinement/loop/host-namespace probes,
macOS Actions, remote CI, independent review and standalone refresh are unrun.
Link target bytes use the same bounded file capacity; no larger namespace,
retained storage, mapping or broad Linux ABI promise is made. Patch, draft,
manifests, helper scripts, failures and logs are retained under
.artifacts/ginux-contribution-preparation/G13/. No branch/checkout/index, remote,
human PR text or maintainer communication changed. F04 remains the publication stop.
