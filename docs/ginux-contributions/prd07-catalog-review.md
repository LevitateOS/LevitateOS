# PRD07 local catalog prerequisite extraction

**Local only; no upstream publication.** Upstream Ginux takes precedence. This
extracts the existing user-authorized configured contract versioning for review;
it is not maintainer approval of the proposed catalog or a requirement that
upstream adopt this design. Reassess it against future upstream foundation work.

## Exact source and independence

Clean `/home/vince/levitate-development/ginux-prd07-catalog` at
`2ffd8884d124d62c2a29195e57b2e1a906383901`, tree
`01977fdf152d99b6254d0c2b58650c71e0cd91aa`, directly on main
`53a54addea67969abaa240ea478def1b94130eee`.
[Complete five-file diff](patches/PRD07-catalog.patch) contains 344 additions and
34 deletions. Commits: `b27093d0` extraction of `f1c3d3a7`, then `2ffd8884`
retained authority/parser tests and runtime-boundary documentation.

The donor's context included #243's runtime-rule APIs/tests. Resolving its two
cherry-pick conflicts preserved current main at those locations and retained
only the new catalog test. No #243 implementation or test was silently folded
into this contribution. The catalog and inspector compile independently;
actual configured enforcement remains a separate required owner (#243 plus
the later configured backend). Catalog 3 / typed storage still follows this
mechanism in the local PRD03 stack, pending a real upstream/equivalent decision.

## Complete diff and failure paths

- `ginux-contract`: separate ten-operation catalog extension; catalog 1 stays
  intact. Validate combined capacity/duplicate IDs/numbers. Select version 2
  only by its named legacy profile or explicit recipe catalog; reject unknown
  recipe versions. Resolve wildcards against the selected set, preserve deny
  ordering/provenance and include the selected catalog in stable identity.
- `ginux-instance`: validate the full u64 catalog before narrowing it; use the
  selected catalog during effective resolution. Normalization already writes
  the selected version. A version such as 4,294,967,298 cannot wrap into 2.
- Init inspection: print the actual catalog. Diff matches by operation ID,
  reporting added/removed operations rather than silently truncating a zip of
  different-sized catalogs. Existing same-catalog changed actions remain visible.
- Example/docs: explicit opt-in and all ten additions, unaltered native policy,
  backend-dependent resource enforcement and separate runtime implementation.
  New operation fixture labels identify later owners, not tests available or
  implemented calls on this standalone main source.

The initial extraction had only a legacy-profile unit check. Review added
meaningful self-contained wildcard/deny/version and parser round-trip/width
controls. The existing 89 main units remain; three new cases make 92. It also
clarified that an authority catalog does not provide clock/storage/network/task
implementation. No native/core/dispatch policy or catalog-1 operation changes.

## Exact validation and reproduction

All existing Linux CI command groups pass on the clean revision: `make check`,
`make test-unit` (92 cases), release/launch/process/setup fixture builds,
`sudo make test-launch`, `sudo make test-native-supervision`,
`make test-process-native`, `sudo make test-gate1-setup`. macOS is unrun locally.

The exact original main contract source and candidate source were separately
compiled with an identical driver. All nine catalog-1 legacy profile, override
and self-contained recipe rule tables **and digests are byte-identical**.
Original main rejects the new configured profile (exit 2); the candidate accepts
it (exit 0). Eight actual internal CLI controls pass: old/new validation,
inspection, explanation, ten added and ten removed operations in both diff
directions, exact-deny precedence and rejection of a wrapping version number.

An initial manual CLI attempt used the public host wrapper syntax on the inner
launcher binary and got `invalid-value`; absolute paths did not change that.
The internal binary takes `instance-validate`, `instance-inspect`,
`instance-explain` and `instance-diff`. Using its actual ABI passes. This was a
command-layer mismatch, not evidence of a parser or profile defect.

From `/home/vince/Projects/LevitateOS`, reproduce retained controls with:

```sh
python3 .artifacts/ginux-contribution-preparation/prd07/check-catalog-compat.py
python3 .artifacts/ginux-contribution-preparation/prd07/check-catalog-cli.py
```

Full commands/source/exit/log hashes are in `catalog/receipts.json`,
`catalog-compat/receipts.json`, `catalog-cli/receipts.json` under that artifact
directory; `catalog-source.json` binds patch, tree, static log and launcher.
For CI use the commands in [candidate review](prd07-candidate-review.md), with
this worktree and exact revision. Recheck actual source and affected tests when
upstream changes or the enforcement/backend join is made.

## Retained PR draft; do not submit

Title: **feat(contract): select an explicit configured operation catalog**

Keep catalog-1 recipes and wildcard authority unchanged while permitting an
explicit catalog-2 opt-in for configured operations. Include the selected version
in resolution/identity and inspection, reject unknown versions, and compare
different operation sets by ID. Runtime implementations and enforcement remain
separate; this proposal does not expand native policy.

Validated directly on main `53a54add`: full local Linux CI, 92 units,
byte-identical original catalog-1 tables/digests for nine inputs, and eight
internal CLI positive/deny/version/diff controls. Upstream adoption of this
catalog shape remains unresolved; the user directs keeping all work local.
