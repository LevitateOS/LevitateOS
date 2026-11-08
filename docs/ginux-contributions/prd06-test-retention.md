# PRD06 — useful test retention and replacement

These are source dispositions, not new runtime passes. The complete
[path/name/version ledger](prd06-source-inventory.json) records every Rust test,
fixture version, source ID, line/hash, owner and replacement. Exact source objects
and the tracked historical patches remain available. PRD05/PRD07 must carry the
following oracles into the actual integrated candidate.

## Required ports that current source does not contain

**T-01, G12 public-alias mapping offset.** Historical
`3fac8e0598193218d6bcda4d7e0813ee60d96238`,
`tests/fixtures/launch-fd-fixture.c`, changes the obsolete “owned mmap unsupported”
expectation into a real owned-public-alias mapping. After the first three bytes
have been read through shared aliases and a native backend fd number has been
reused for an image, the assertion is:

```c
mapped=call(9,0,4096,1,2,memory,0);
CHECK(mapped>0 && *(unsigned char *)mapped=='A',24);
CHECK(DO(8,memory,0,1)==3 && DO(11,mapped,4096,0)==0,49);
```

Current `5300f5ef` and `5449e2a1` fixtures test byte A/unmap but omit that additional
SEEK_CUR check. Retain it with G12; port the assertion without restoring the
obsolete EOPNOTSUPP expectation. This is a real test omission, not a demonstrated
current offset bug. Other ordinary-mapping offset tests do not account for this
specific public-alias/native-number-reuse condition.

**T-02, G09a/b large copy-up and mid-copy failure.** The exact test
`chunked_copy_up_preserves_all_bytes_and_reclaims_partial_failure` is retained
at [G09a.patch:383](patches/G09a.patch#L383). It creates a 9,000-byte image file,
returns `(offset+i)%251` through a BaseReader, and injects Io when a subsequent
read starts at or beyond byte 4,096. Success must read and verify all 9,000 bytes;
failure must leave zero provisional overlay files and allow a new file to reuse
the sole storage slot, write/read `ok`. Neither current integration nor PRD03
contains this exact case. Port it to the combined typed-VFS/generic-backend owner,
including unchanged preexisting image descriptions and metadata on failure.
Small-file rollback and a 4,096-byte production copy loop are not substitute
coverage. The tracked patch preserves the full test, not just its name.

**T-03, G06a incoming orphan suite.** Pending upstream #234 at
`78a534c1dcff70207900013381ff857b5687a66e` retains all six cases:

- `exited_parent_hands_children_and_their_threads_to_init`
- `signaled_parent_hands_children_to_init`
- `init_reaps_a_child_that_exited_before_its_parent`
- `children_stay_with_their_process_while_a_thread_runs`
- `grandchildren_keep_their_living_parent`
- `an_init_thread_reaps_orphans_after_the_init_leader_exits`

Current aggregate tests cover several of the same behaviors but are not a
license to drop the already-exited child, living grandparent/intermediate parent,
or surviving init-thread variants. Keep the incoming suite and current fork/
thread reservation and typed wait-result tests when integrating that PR.

## Explicitly replaced historical Rust cases

| Historical case / source family | Replacement and retained assertion intent |
| --- | --- |
| `crates/ginux-domain` tests | Same-name cases in `crates/ginux-instance`; schema/terminology changed upstream. The ledger maps each exact path/name, not just the crate. |
| Kernel-view `identity_and_namespaces_are_domain_owned_and_consistent` | `identity_and_namespaces_are_instance_owned_and_consistent`; same per-instance identity/namespace isolation. |
| `overlays_are_isolated_between_domain_instances` | `overlays_are_isolated_between_instances`; distinct overlay bytes/identity remain. |
| `duplicated_descriptors_share_offsets_without_aliasing_reused_slots` | #232's `duplicated_reads_share_offset_while_separate_opens_do_not`, `duplicated_offset_survives_close_slot_reuse_and_cloexec`, and `duplicated_writes_and_reads_advance_the_same_offset`. Preserve shared description, separate opens, close/reuse and CLOEXEC. |
| `supervisor_reservation_wait_peek_and_orphan_adoption` | Current `fork_reservation_cancellation_preserves_published_state`, `wait_inspection_preserves_typed_outcomes_until_consumed`, `exited_and_signaled_parents_reparent_children`. |
| `threads_share_child_waits_and_reap_without_consuming_processes` | `thread_reservation_rollback_retains_process_identity`, `orphan_adoption_waits_for_the_last_group_member`; keep current wait/thread cases too. |
| `signaled_child_peek_preserves_status_and_adopts_orphans` | `wait_inspection_preserves_typed_outcomes_until_consumed`, `exited_and_signaled_parents_reparent_children`. |
| `thread_reservations_cancel_only_in_the_running_parent_group` | Exact renamed current `thread_reservation_rollback_retains_process_identity`: wrong parent, leader-vs-thread, running/exited group and identity rollback assertions remain. |
| `owned_storage_syscalls_follow_their_semantic_rules` | PRD03 `storage_catalog_preserves_older_authority` plus `syscall_semantic_operations_are_catalogued`; catalog 3 is explicit. Do not retain a test that silently grows catalog 1. |
| `mkdir_creates_directories_that_cannot_be_opened_or_unlinked` | PRD03 `mkdir_creates_openable_directories_but_unlink_requires_rmdir`. Directories are openable and removable with rmdir, not unlink. |
| `mkdir_keeps_the_sticky_bit_and_drops_setuid` | PRD03 `mkdir_keeps_all_supported_mode_bits` and `directory_creation_keeps_the_same_mode_bits_as_chmod`. Preserve supported 07777 metadata. |
| `files_without_write_bits_refuse_writable_opens` | PRD03 `existing_overlay_modes_constrain_new_opens` and launcher Modes fixture; access failure must preserve bytes/rights. Retain the corrected read-only-truncate precheck regression. |

The source ledger marks the six #234 cases **retained**, not “replaced by aggregate
coverage,” and marks T-02 retained in its tracked patch, not present in integration.
The old aggregate donor's removals of native read-only/unmapped/straddling wait
buffer cases and process_vm_writev ENOSYS/EPERM fallback controls are explicitly
replaced by current upstream: `scripts/test-process-native.py` and
`tests/fixtures/process-review-fixture.c` are byte-identical between main and
current integration, and the seven native wait/copyout function blocks also
match. Preserve the incoming controls rather than the old deletions.

All main cases remain; the only changed main test body is the unlink alias case
adapted to #232 shared offsets. Other historical names have the explicit
replacement keys above or the identical renamed instance case.

## Older runtime prototypes and dirty memory fixture

The 49 historical commit entries preserve individual source and rationale.
Replacing an old implementation does not discard its failure oracle:

| Preserved implementation/test | Disposition and concrete destination |
| --- | --- |
| `17050a20` path normalization | Replace lexical cancellation with component walker. Retain `file/../file`, `missing/../file`, `file/.` intermediate-directory errors in G09/G13/G16 fixtures; PRD03 traversal controls exercise the new owner. |
| `acbde526` stop/death race | Replace with `d693ae63` and current launch-task GETREGS/GET_SYSCALL_INFO ESRCH/death handling. Keep lost-stop/child-death fixtures with G06; do not revive the old supervisor. |
| `7bc0ee04` mixed copy-up/unlink/offset | Failed-open/unlinked lifetime belongs to merged #216/#217; shared offset to pending #232; stale-reader copy-up identity to PRD03 live-descriptor retargeting. Keep all three distinct oracles. |
| `b500f859` duplicate read | Replace old patch with #232 and its explicit alias/separate-open/reuse tests, included in PRD03. |
| Native/demo process/exec/cancel prototypes | Current upstream native ProcessTable/supervisors and configured G06/G07/G10/G11 replace these APIs. Keep typed wait, thread/group, exec rollback, signal and kill/reap intent in their fixtures. Old #154-native code is not an extra configured capability. |
| Old raw ChannelSpace handle tests | Public DescriptorTable/channel owner replaces exposed raw backend IDs. Preserve stale object isolation and close/reuse behavior through actual public FD/held-object identity; Linux may reuse an integer FD, so do not assert that every reused integer remains invalid forever. |
| Manual loader/basename/staging/memory ledger | G10 pinned owned ELF, G12 private backing/native mappings, G26 resource bounds replace the loader. Preserve actual byte, partial-page, stale-reference, offset, rollback and teardown oracles. No manual map ledger is revived. |
| Old binary retained-image library | G19/G20/G21/G22 archive/control format replaces it. Keep generation consistency, cancellation, atomic publication, corruption/deletion and verified restore oracles. F-01 shows publication closure is still needed; this is not a second format proposal. |
| Interrupted limits/upcoming merges | Preserve staged/unmerged originals and restored unit modules. Current main tests are authoritative; do not copy conflict markers or replace current tests with older modules. |
| Typed-handle/explicit-close dirty edits | Preserve optional local simplification downstream; retain actual FD/pipe/device behavior with current G05/G08 tests. |

The recovered untracked files at
`/home/vince/Projects/LevitateOS/.artifacts/development-migration/2026-11-06/recovery/worktrees/8-ginux-i04m5-c821b4a4/files/`
remain exact source inputs:

- `scripts/test-memory-native.py`, SHA256 `d0606ee1f44b84f9199330fb1691146f7c5f9f4571fe59042ac78802c9f60f27`.
- `tests/fixtures/memory-review-fixture.c`, SHA256 `31bb6a41c2a96ef2c3c37e342e1cdf953dea1f59b275fe68774d4cf8ddbc1e9a`.

All seven modes have a destination: `basic` → owned ELF/ordinary mapping;
`partial-page` → final-page bytes/zero fill and SIGBUS boundary; `unmap-prefix` →
partial unmap and retained-range lifecycle; `stale-fd` → closed/reused public FD vs
held backing identity; `zero-length` → mmap zero-length rejection without state
change; `offset` → correct mapped bytes and unchanged shared position; `fault` →
failed/protected mapping rollback and teardown. G12/G26 own the translations;
PRD09 owns adversarial schedules. Source retention is explicit; this record does
not assert these obsolete runners execute successfully against the new ABI.
Current ordinary mapping, protection, fixed-region, alias, private-transfer and
nonleader fixtures remain alongside them. Their source versions/functions are
indexed, including tests that were later rewritten.

Every C/assembly fixture version is retained exactly when current, or replaced
with its named feature owner while preserving the old source/hash as a regression
input. A changed fixture name or old raw API does not prove assertion equivalence.
PRD07's actual candidate review must check these source-bound ports; PRD06 records
their full disposition and does not preclaim that review.
