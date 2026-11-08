# PRD04 — storage and descriptor findings (in progress)

**PRD04 remains open after a failed authorized Daybreak item 5.** Returned
results are reconciled below; this chat has not executed the security probes.
The [pinned request](prd04-daybreak-request.md) supplies the exact candidate,
comparison source, all recorded failure criteria and required receipts. No
publication or runtime/gitlink replacement is authorized by this preparation.

## Returned Daybreak result — 2026-11-08

Daybreak completed the [six-item verification](/home/vince/levitate-development/ginux-prd04/.artifacts/prd04/daybreak-2026-11-08/receipts.md)
on unchanged `5449e2a1`: items 1–4 and selected item 6 controls pass, including
the ABI adjudication, policy red/green, original inherited-budget cases,
assignment prefixes and valid small-stack controls. **Item 5 remains failed:**
a final backing-close error returns structured supervisor-error but leaves a
complete 2,560-byte export at its requested final path. Export runs before
backing teardown discovers that error. The user requested a separate correction
without changing this pinned candidate, then a new revision/binary hash and
focused authorized Daybreak retest. No correction or retest has occurred yet;
PRD04 remains open. The latest user task is PRD06 reconciliation, which must
retain this defect explicitly rather than hiding it behind these selected passes.

## Exact source and local corrections

The clean isolated candidate is `5449e2a184bb1147cf618b140a29196b74206b13` in
`/home/vince/levitate-development/ginux-prd04`, based on the PRD02 integration
`5300f5ef9a8281f4f10f5519d5eb761654e04829`. This preserves the original integration
and the separate PRD03 foundation `9802ef03`. Its full delta is eight files,
208 insertions and 13 deletions, retained in `.artifacts/prd04/candidate.patch`.
This is a local integration correction, not the future PRD05 merged-source join.

| Commit | Correction and owner |
| --- | --- |
| `75d5dcc3` | [Configured startup](/home/vince/levitate-development/ginux-prd04/crates/ginux-init/src/launch.rs:539) closes undeclared inherited FDs before private reservation; keeps stdio, explicit coordinator channel and result. Captured public limits, storage/object capacities and host hard limits are unchanged. |
| `e91bba31` | [Semantic classification](/home/vince/levitate-development/ginux-prd04/crates/ginux-init/src/launch.rs:1400) includes regular read/readv after console/TCP identity. The actual table lookup still narrows Linux FD arguments to 32 bits. Returned Daybreak low/high-bit red/green and private-handle controls pass. |
| `eb4a342e` | [Assignment check](/home/vince/levitate-development/ginux-prd04/crates/ginux-init/src/fs_hooks.rs:202) ports PRD03's no-symlink/no-file prefix precondition to the integration before restore/init, preserving the existing namespace, argument-hold, mappings and exec owners. |
| `5449e2a1` | [Memory backing cleanup](/home/vince/levitate-development/ginux-prd04/crates/ginux-init/src/storage_memory.rs:103) latches a failed close instead of panicking. [Resume](/home/vince/levitate-development/ginux-prd04/crates/ginux-init/src/launch_tasks.rs:682) stops before guest continuation; teardown propagates the error through the existing structured completion path. No numeric close retry. |

## ABI versus authority adjudication

Linux declares read's FD as `unsigned int`, and x86 wrappers cast decoded
register arguments to the syscall's declared types. A high register bit therefore
does not create a distinct FD identity. This is supported by the
[Linux read implementation](https://github.com/torvalds/linux/blob/master/fs/read_write.c),
[pinned x86 wrappers](https://github.com/torvalds/linux/blob/v6.12/arch/x86/include/asm/syscall_wrapper.h#L202)
and existing Ginux FD fixture: it already requires a high-bit close alias to
close the same public slot. PRD03/#243 likewise compare FD values as 32-bit values.
No existing user decision establishes a stricter 64-bit canonical-FD ABI.

Thus the G12 observation `read(0x100000004)` returning public FD 4's byte is a
valid observation, but by itself does **not** prove an authority bypass. The
contract is bounded public-table lookup and consistent policy for the narrowed
identity. Internal backend tags and private native FDs must remain inaccessible
through that table. Tightening the API to reject all high-bit aliases would
change this ABI rather than repair the recorded observation.

A genuine independent gap exists in the current source: `read` and `readv` on
non-console/non-TCP public FDs had no filesystem-read semantic classification,
so the syscall grant could survive a filesystem-read denial. This is fixed in
source. The new `policy-read` fixture obtains an owned read/write descriptor
through permitted write authority, then requires EPERM on both normalized and
high-bit read/readv calls, zero offset movement, continued allowed writing and
identical denial audit provenance. It must fail on the base and pass on the
candidate before closure. This is not a claim that every file observation, such
as a separately granted mapping/exec operation, is controlled by this one family.

Linux also clears a valid FD table entry before reporting late close errors;
blindly retrying its numeric FD can affect a reused descriptor. The
[pinned close implementation](https://github.com/torvalds/linux/blob/v6.12/fs/open.c#L1463)
supports retiring the owner once, recording failure and stopping the instance.
A strace injection that skips close is a useful completion-path test, not proof
that a real Linux late-close error leaves that same FD live.

## Recorded failures and required closure

| Record | Current disposition | Required fresh result |
| --- | --- | --- |
| G03c B2 / G12 high-bit alias | Strict 64-bit canonicality was not the actual Linux/Ginux ABI. Actual low/high-bit policy consistency and private-handle isolation remain security acceptance criteria. Read/readv semantic gap corrected. | Daybreak ABI-positive, denial-negative, private/backend and base-red/candidate-green controls. |
| G17b 190 inherited FDs / hard 256 | Confirmed availability failure: fixed private reserve ignored undeclared inherited occupancy. Candidate retires those unused inherited descriptors, rather than raising hard/public limits or adding a larger allowance. | All original 0/120/190 × 256 and 190 × 400 controls; all 80 files; declared control survives, parent FDs preserved, lower public bound/insufficient hard limit unchanged. |
| B1 image-link assignment prefix | Prior current-source result proved config acceptance, not a host oracle. PRD03 already rejects the prefix; integration now has the same precondition. | Fresh startup refusal with unchanged sentinels; valid missing/real prefixes and final-component replacements remain usable. |
| B3 old inline storage stack claim | Current integration uses mmap-backed namespace and storage metadata; PRD03 uses static storage. The old 2 MiB memory config was invalid. No automatic “three live defects” conclusion. | Fresh valid 16 MiB instance / 3 MiB stack controls on this candidate; explicit narrow limitation. |
| G17b close-error panic / empty result | Confirmed completion-protocol gap in `MemoryStorage::clear`; replaced by sticky error and structured failure, with no resume after workload-time release failure. | Retained close injection at final teardown and workload-time final close: exit 1, valid supervisor-error, no panic or accepted success, settled tasks/references. |
| Private backing / G12 transfer lifetime | Prior selected alias/unlinked/CLOEXEC, Pair/map/receipt/death checks are historical passes. They do not establish all close/failure/transfer lifetime criteria on the new source. Owners and argument guards are retained. | Fresh retained applicable lifetime/private visibility/rollback checks and an explicit disposition of any unrun PRD09-only schedules. |

## Local evidence (not security closure)

In `.artifacts/prd04/` under the candidate worktree:

- `candidate.json` pins source, binaries, runners and the pending security status.
- `check.log`, `build.log`, `unit.log`: strict static checks, release/fixture builds
  and all eight unit-test packages pass (138 tests). The initial sandbox denied
  loopback sockets; `unit-sandbox-failure.log` records it, and unchanged tests
  passed with normal host loopback access.
- `fd-limits.log`: lower public/native limit 8, private file capacity and impossible
  hard-limit refusal pass.
- `storage-ordinary.log`: growth, bounds, quota, ordinary mapping and unlinked
  mapping pass; `checkpoints.log` verifies the declared control channel and valid
  snapshots/exports still work after inherited-FD cleanup.

No high-bit/private-FD probe, assignment-prefix security runner, inherited-pressure
probe or close-error injection was executed in this chat. The user selected
Daybreak rather than authorizing a verifier here. No required product decision
is currently missing. Returned verification found F-01; its separate correction
and focused authorized retest are outstanding. The queue stays unchecked.

Active skill: `hdae-development` at `.agents/skills/hdae-development/SKILL.md`.
Reload the complete skill, required reference, applicable instructions and current
queue after a context shift. Exact next action: correct final export publication
in a separate revision, preserve frozen `5449e2a1`, provide source/binary hashes
and request focused authorized Daybreak item-5/affected-export retest before closure.
