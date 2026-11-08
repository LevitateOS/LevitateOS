# PRD07 local storage prerequisite rehearsal

This continues local extraction/integration under the user's instruction.
**No push, PR or upstream comment.** These compositions are not actual merges
or standalone upstream candidates; upstream Ginux takes precedence when its
foundation changes land. Preserve the independent contributions and reassess
their necessity and shape against that source.

## Reviewed prerequisite composition

Clean `/home/vince/levitate-development/ginux-prd07-foundation` extends the
original `f1c3d3a7727f5a8027b448b4dcad0ad52c896c4c` composition, whose main
ancestor is current `53a54addea67969abaa240ea478def1b94130eee`:

- `1edff1a7`: preserve the retained writer offset and empty/rejected-write oracle
  when composed with #232's shared descriptions. The old composition plus that
  regression fails (exit 101); the correction treats empty writes as no-ops
  after checking rights. The original stale-gap test remains.
- `04857f38`: carry the exact reviewed catalog tests/docs from `2ffd8884`.
  Resolve the test-location conflict by retaining #243's two original runtime
  rule/family tests and adding the new recipe catalog test. No test is dropped.

Exact head `04857f38` passes `make check` and all original Linux unit/build/
launch/native/process/setup commands (101 unit cases). This is a rehearsal of
#232/#233/#236/#238/#243, zero-fill and the local versioned catalog, not evidence
that upstream has merged any of them. #234 remains separate and independently
validated; its six orphan oracles still require the later configured-task join.

## Typed-storage composition and conflict disposition

Clean `/home/vince/levitate-development/ginux-prd07-storage` at
`16dd447e0fbdc12f702d7acaedf04f7249a27078`, tree
`c7f5fd324ea57f6bc0299b35f1e2253ee547dbfe`, composes all five PRD03 feature
commits on that foundation, then explicitly adapts the new catalog tests:

| Original PRD03 source | Rehearsal commit | Disposition |
| --- | --- | --- |
| `9d487303` | `fc53f98b` | Slot enum, unchanged behavior. |
| `d37688c3` | `7fefaaa5` | Full VFS feature retained. Resolve empty-write helper conflict with `Ok(offset)` because this helper returns the end position; public write still returns 0 and does not update offset on empty input. Retain both the new empty/rejected-write test and original forty-link namespace test at their conflicting insertion location. |
| `72cf37a7` | `b84f48fa` | Assigned memory-storage launcher and catalog 3 retained. |
| `4cb12f51` | `231c9769` | All PRD03 spec corrections retained. |
| `9802ef03` | `1344daa5` | Read permission before truncating preserved. |
| New catalog-test join | `16dd447e` | Catalog 3 is intentionally supported here. Original catalog-2-only unknown-version assertions fail before adaptation; retain their authority/width oracles, positively check recipe catalog-3 admission and round-trip, and reject unsupported 4 instead. This does not change the independently tested catalog-2 candidate. |

The complete reviewed delta against original `9802ef03` is only new tests and
catalog documentation/comments; its runtime implementation remains identical.
All original 138 unit cases remain, plus three retained cases for 141 total.
`make check`, every original Linux CI group, storage fixture build and the full
40-case assigned-storage runner pass on this exact clean rehearsal. These
functional results do not replace final-source Daybreak or Recipe acceptance.

## Evidence, full diffs and next owner

`.artifacts/ginux-contribution-preparation/prd07/` retains:

- `foundation-source.json` / `storage-source.json`: full source/base/tree,
  launcher hash, full feature-diff hash and unit count;
- `foundation-feature.diff`, `storage-feature.diff`: complete candidate deltas;
- `storage-versus-prd03.diff`: complete preservation/composition review;
- `foundation/receipts.json`, `storage/receipts.json`: exact commands and log
  hashes, exits and source pins; `foundation-check.log`, `storage-check.log`;
- `foundation-controls/`: original-base red empty-write source/binary/log/receipt;
- `storage-catalog-test-pre-adaptation.log` and
  `storage-parser-test-pre-adaptation.log`: the expected version-3 test join
  failures, retained rather than silently omitted.

Reproduce with the commands in [candidate review](prd07-candidate-review.md),
substituting the appropriate worktree/revision, plus
`make build-launch-storage-fixture` and `sudo make test-launch-storage` for the
storage composition. The existing receipt runner accepts `foundation` and
`storage` labels and records those exact runs. Original PRD03 evidence stays at
`9802ef03`; no pin or published head is rewritten.

The G09a library can be ported independently onto this typed VFS, preserving
slots, modes, search checks, live copy-up identity and directory/data-slot
separation. Preserve T-02's full 9,000-byte copy and mid-4,096 failure/reclaim
oracle. Inspecting the complete G09b donor confirms its growth fixture also
requires fork/wait/shared public offsets; its full runtime delivery must follow
G04/G05/G06, as the original coverage map requires. Do not drop that fork oracle
to call the pair standalone earlier. Next local integration follows those
device/descriptor/task prerequisites, then the complete growable owner. This
work is independent of the pending post-publication export commit-point
decision; export acceptance stays open.
