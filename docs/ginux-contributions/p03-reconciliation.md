# Remaining-change reconciliation — P03

P03 complete as inventory/classification, against upstream `3f2fe7df` and local
integration `7cf2638a`. Full candidate source review and acceptance remain pending.
[Catalog](p03-catalog.json) assigns all66 integration commits,49 additional commits
from preserved worktrees and59 changed paths. The261 changed hunks are assigned to
path owners in `.artifacts/ginux-contribution-preparation/p03/hunks.json`; mixed
files require the symbol-level extraction boundaries in P04/P05. Assignment does
not claim that a whole file or whole historical commit should be submitted.

## Confirmed missing VFS correction

The direct VFS regression `review_duplicated_reads_share_one_offset` opens `/base`,
duplicates its descriptor, reads `B` through the original and expects `A` through
the duplicate. Both upstream3f2fe7df and integration7cf2638a instead return `B`
again. The same regression passes on preserved review fixb500f859. Each was built
from its exact committed VFS source using `rustc --edition=2021 --test`, then only
the named regression was run. No original source/index/branch was modified.

| Exact source | Regression | Result |
| --- | --- | --- |
| `3f2fe7df` | shared duplicate offset | fails, expectedA/gotB, exit101 |
| `7cf2638a` | same | fails, expectedA/gotB, exit101 |
| `b500f859` | same | passes, exit0 |

Logs and source hashes: `.artifacts/ginux-contribution-preparation/p03/` and
`/tmp/ginux-contribution-p03/`. This is a confirmed independent VFS bug candidate
within G05, not permission to copy old feature APIs. Its extraction must also
review slot reuse, independent opens and CLOEXEC. Old7bc0ee04 mixes this with
copy-up identity and unlink fixes; retain those separate semantics for review.
Current copy_to_overlay does not redirect an already-open image descriptor to its
new overlay. Whether that older intended behavior is required must be reproduced
and reviewed against Ginux's current VFS contract, not silently added.

## Other four-review-commit outcomes

- `acbde526`: equivalent lost-stop handling is present in `d693ae63` and current
  launch_tasks GETREGS/GET_SYSCALL_INFO ESRCH paths; no additional code import.
- `17050a20`: its lexical normalization hook is superseded by joined_path plus
  component-wise resolve_namespace. Keep its three intermediate-directory negative
  cases in G09/G13/G16 fixtures; do not reintroduce lexical cancellation.
- `3fac8e05`: c-string backing spelling and actual public-owned-alias mapping are
  present; keep its offset-unchanged assertion when reviewing G12.

## Older branches and unfinished edits

The49 commit entries have individual dispositions. Native process experiments are
superseded by merged #154 and newer native changes; configured process support is
still a different packet. The old loader's manual basename/staging/memory ledger
is superseded by private owned backing/native mappings; its partial-page, alias,
zero-length, failed-copy and lifetime cases remain review inputs for G12/G26.
The old binary retained-image library is superseded by the current configured
archive/control owners; consistency, safe restore and export-failure requirements
remain G19–G22. Do not add a second persistence format or old demonstration path.

Uncommitted typed-handle/explicit-close/dispatch simplification experiments remain
preserved. They are optional owner-local simplifications to assess with G05/G08,
not an extra refactor contribution. Staged combined VFS edits correspond to the
already-merged three PRs. Interrupted merges in limits/upcoming trees contain
restored upstream unit modules and unresolved conflicts: preserve them and use
current upstream tests as authoritative instead of copying those trees wholesale.
Both interpreter stashes are identical patch bytes (SHA256
`79406623a8fa6e07256ffb56784c11b1576d0b7546f7d52c011e150973edffe2`);
current Script::resolve and held-pin release supersede them. Both remain intact.

## Incoming upstream preservation

A merge preview using `git merge-tree --write-tree --messages --name-only`
produced tree473b84ee with conflicts only in `supervisor.rs` and VFS`lib.rs`.
It changed no source, index, branch or remote. P04/P05 must preserve the actual
#217/#218 path/unlink corrections and #221 protected-copyout/fault fixtures.
An integration-minus-main diff contains deletions of those newer helpers/tests;
these are incoming reconciliation work, never proposed test removals.

Next task: P04 — verify exact capability dependencies, split the standalone
duplicate-offset fix from configured descriptors, and map upstream policy holds.
