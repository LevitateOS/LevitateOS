# G31 — checkpoint/restore CI coverage reconciliation

Source5e20a65f29b2d45e2840a69ef0e24613dc40e278 changes exactly one workflow
line: sudo make test-launch-checkpoints. Its useful coverage is already included
in the introducing feature owners. Do not create a separate wiring-only patch,
commit candidate or PR.

- [G20](g20-review.md),7afa833d1babcc5dd62ef326f81c52835f9aa349: workflow invokes
  test-launch-checkpoints; its Make target invokes the checkpoint runner with the
  storage fixture. Workflow builds that fixture first.
- [G21](g21-review.md),33bf08a746bd64306e0375f8d419af4744d31064: the independently
  extracted startup restore owner has its own test-launch-restore workflow/Make/
  runner target, also built against the storage fixture. It does not import G20.

Exact workflow, Makefile, runner and fixture bytes match each retained head in
both source and strict clean exports. Runner syntax parses, make -n of fixture
build, corresponding target and test-unit passes in all4 exports. All7 original
unit package commands and required make test-unit remain. No tests are removed
or disabled. Existing recorded source/clean runtime receipts pass their selected
ordinary checkpoint or restore modes on those exact heads; this read-only wiring
review does not rerun their unchanged scenarios or imply default security runs.

G31's source disposition is fully accounted as folded into G20/G21. Full/default
checkpoint/restore security fixtures, combined control+restore owners, thread
joins and remote Linux/macOS CI remain pending under those packets and F02/F03.
The final combined workflow must include each introducing owner's invocation
exactly once, including all newly added storage/network/poll coverage and all
original units. No claim that CI is green or that consumer closure is complete.

This is a separate source/wiring self-review, not independent review. Evidence,
dry-run output and the original one-line donor diff are retained under
.artifacts/ginux-contribution-preparation/G31/. No Ginux source/ref/branch/index,
remote body/comment or publication changes were needed.
