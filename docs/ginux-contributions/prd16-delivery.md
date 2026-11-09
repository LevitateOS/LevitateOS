# PRD16 — final source pins and artifact provenance

**In progress: exact source publication requires user permission.** The owning
repositories already contain coherent, clean commits. Nine selected descendants
are absent from their public delivery branches. Final parent gitlinks and the
published-baseline `delivery-sources.json` are deliberately unchanged until the
required source commits are durably available. Local recovery bundles and green
builds do not complete PRD16.

Active skill: `hdae-development`, discovered at
`.agents/skills/hdae-development/SKILL.md`; reload it, its required reference,
root instructions and this checkpoint after context recovery. The root rules
prohibit mandatory process gates and Recipe unit tests. This task uses existing
source/product owners and preserves the shared worktree and staging.

## Exact delivery to authorize

The [candidate manifest](prd16-delivery-candidate.json) records all sixteen source
repositories, source trees, original parent pins, observed public heads, complete
patch hashes, actual products and their inputs. GitHub read-only branch/commit
and repository metadata confirmed all sixteen repositories are public. Seven
selected heads already match their delivery branches; nine need fast-forward
publication. No source branch, commit, public discussion or remote text was
rewritten.

| Owning repository | Existing branch | Exact selected head | Commits since observed public head |
| --- | --- | --- | ---: |
| LevitateOS/ginux | main | `e4d95de10520aa8e91bc2ec69e64da004443e2da` | 9 |
| LevitateOS/recipe | master | `b185e0363ceba1290201ba84593aefb313a85cc4` | 9 |
| LevitateOS/distro-builder | main | `9468d26e4782f3dc556e609432f62292431c3c0c` | 3 |
| LevitateOS/docs-content | master | `d9384939cfcecf6af37d45040c0404f50dbf69f7` | 1 |
| LevitateOS/recab | main | `61b859c4b0bec5337b6dced96d033a8847d966ce` | 1 |
| LevitateOS/recshuttle | master | `dfe9ef12ea7ba3956738a4e05df6a16630808f4e` | 1 |
| LevitateOS/recqemu | main | `f0364bbf8c3a6093132729f73420bde848466f7b` | 1 |
| LevitateOS/install-tests | master | `056a46b008c7be433b6493676c44aed6603e52ba` | 2 |
| LevitateOS/tui-root | master | `ade4d9aa649cc7806f899a826723881ed4263296` | 1 |

The full diffs are under
`.artifacts/ginux-contribution-preparation/prd16/<repository-path>.patch`, with
exact paths and hashes in the candidate. Each selected head descends from the
read-only observed public branch; recheck that relationship immediately before
an authorized push. Use the exact SHA as the push source. Do not move Ginux's
older local main ref, switch its recovery worktrees, force a ref, or replace a
head if another actor has advanced it. Stop and reconcile actual incoming work.
Upstream Ginux remains authoritative; its current main and #249 ancestry are
retained. No upstream PR or comment is included.

Pushes can trigger existing CI: Ginux `check`, Recipe `CI`, docs/content `CI`.
Recipe release publication requires explicit workflow-dispatch opt-in and is
not part of a source push. Ginux retains its upstream macOS job; macOS acceptance
is indefinitely deferred, and no separate macOS dispatch, investigation or
acceptance claim is required here. No separate hosted workflow dispatch,
repository-variable mutation, release, package, tag or ISO upload is authorized.
Existing automatic CI effects must be included in any push permission.

## Completed local verification

- Commit `46c7e25a7c01076bc8128bfbdf487095e4f039b6` changes fourteen public
  submodule URLs from SSH to HTTPS. Names, branches and gitlinks are unchanged.
  Existing checkout origins and user Git configuration were not edited or synced.
- Sixteen HEAD-only bundles retain exactly the selected reachable histories;
  unrelated branches, stashes and worktrees were not bundled, moved or deleted.
  Each verifies and restores into an independent detached repository without
  object alternates. The isolated root index contains the exact proposed pins
  and tested workspace lock; no shared index or root gitlink was changed.
- The isolated selected checkout reproduces the actual Ginux launcher with
  ordinary `make build`, Rust 1.99.0 and a fresh target: **21a54e60…**.
- Its exact Recipe b185e036 archive builds outside the superproject with its own
  tracked lock and release profile, Rust 1.99.0, an empty target and
  `cargo build --locked --offline --release --bins`. CLI and worker are identical
  **e7073fc1…**; image producer is **aa13ac9b…**. These equal the verified runtime
  bytes. Source archive **7d8098e4…** also matches the earlier independent rebuild.
  Only the ordinary acquired crate cache was reused; no target cache, private
  worktree or migration binary supplied source or executable output.
- `cargo metadata --locked --offline --no-deps --format-version 1` passes on the
  isolated selected workspace. The selected lock **36066e42…** changes local
  package edges only; registry/git package records are unchanged.
- Fresh integrity verification rehashes all six ISO products, eighteen scenario
  manifests and both real installer PTY transcripts. Every selected source and
  checked build input matches the successful product wave. Original run/build
  identities remain in provenance; historical results were not relabeled.

Exact commands, logs, bundle hashes, preserved staged patch and before/after
checks are under `.artifacts/ginux-contribution-preparation/prd16/`:
`preparation.json`, `fresh-builds.json`, `provenance-validation.json`,
`all-remote-heads.json`, and `public-source-urls.json`. The proposed parent pin/
lock patch is **02178705…**; full SHA-256 values are in the candidate manifest.
The source-only restoration used local bundles because the nine final heads
are not yet public. Anonymous public obtainability of those heads is **not run**
and cannot be inferred from local refs or a bundle restoration.

The selected product wave is Acorn base/live-boot/live-tools
`A1My9Q1kl3g1ooze` / `A1MzccgZQTOQDJVy` / `A1Mzhthmceb3c7Vy`, and Levitate
`A1N0nAIeA6xJQl5D` / `A1N3koWnCN7n6CCL` / `A1N3t5Bs0nxI0K6X`.
The candidate records full ISO hashes and links each installer/A/B result to
those exact products. This establishes correspondence to the tested products;
it does not establish byte-for-byte ISO rebuild determinism. The actual runtime
catalog remains 154/154 on e4d95de1 / e7073fc1 / ffe04d86. Changed external tool
recipes and embedded guides have their own complete producer/product/example
receipts, as recorded in the [product checkpoint](prd15-product-contract.md).

## Reproduction and remaining work

1. Obtain explicit permission for the nine exact source pushes above, including
   normal automatic CI effects. Until then, wait; do not substitute another
   delivery approach or silently publish the old baseline as current.
2. After permitted publication, fetch each exact commit anonymously over its
   HTTPS URL into empty repositories with no local alternates or private source
   paths. Verify revisions/trees and final source contract before adopting pins.
3. Commit the selected root gitlinks, tested lock and final provenance together.
   The existing `delivery-sources.json` identifies the historical public
   28f4368f/e7bfc166 baseline until this step; preserve that old identity in
   history. Reconcile its current pair with the candidate and source joins.
4. Verify a fresh selected parent checkout obtains the exact source combination.
   For a published root commit, initialize exact submodules with
   `git submodule update --init --recursive`, never `--remote`. Build Ginux with
   `make build` in `tools/ginux`. Build Recipe from an exact standalone archive
   or independent clone, using its own Cargo.lock/profile, as above; the parent's
   release profile intentionally differs. Reproduce the pinned acquisition image
   through Recipe's tracked bootstrap manifests and ordinary public signed
   inputs; retained PRD14 producer/fresh-host receipts remain source-backed.
   Use the actual equal CLI as worker and verify the image digest ffe04d86.
5. Preserve actual product receipts and their exact source/byte identities.
   Revalidate affected behavior if source changes during reconciliation; moving
   delivery refs alone does not turn old execution into a new run. Final parent
   publication, if needed, requires a separate exact permission after its commit
   and complete remote-relative diff are reviewable.

The original shared staged patch is **f4d8c81b…** and remains unchanged. Root
review notes, mixed interface edits, deny.toml and other pending documentation
are outside this isolated delivery change unless specifically reconciled; none
is silently committed. Human-authored public text, original pinned PRD04 sources,
all recovery worktrees and historical evidence remain intact.

Security authentication/results, the user-held dependent-recipe remove/cleanup
decision, PRD05's actual upstream merge requirement and hosted Linux acceptance
remain open in their owning tasks. PRD16 source delivery does not waive them or
claim production acceptance. **Next action: request the bounded nine-source push
permission and wait; final parent pins remain open until durable availability.**
