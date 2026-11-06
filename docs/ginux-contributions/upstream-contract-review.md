# Upstream PR231 reconciliation in progress

Fresh read-only GitHub checks identify Greg's upstream main as
[`ff926d0c228b71d55d0f2f0909dd86ec62197c8a`](https://github.com/gmkurtzer/ginux/commit/ff926d0c228b71d55d0f2f0909dd86ec62197c8a),
merging [PR231](https://github.com/gmkurtzer/ginux/pull/231): configurable contracts
and audit. Eight commits/43 paths changed since8a475e8b. Ginux leads; proposed
contributions and Recipe must adapt to this upstream owner. The inspected main
checkout still pins tools/ginux at2eac10bf; that checkout/pin was not modified.

Exact upstream export, full delta, provenance, read position and baseline evidence
are in `.artifacts/ginux-contribution-preparation/upstream-contract-refresh-2026-11-06/`.
The object was fetched without refs/FETCH_HEAD updates. Source paths match the
upstream tree. `make check test-unit build build-launch-fixture
build-process-fixtures build-gate1-fixture` passes:83 units, with the new upstream
contract module retained. The generated contract-document check and eight ordinary
configured-launch/valid wait-status controls pass. Separate target directories
preserve existing artifacts. These are baseline results, not candidate acceptance,
new contract runtime enforcement evidence or remote CI.

Read so far: complete configurable-contract documentation, main/domain/launch/
Makefile diffs. Domain normalization now resolves versioned contract inputs,
derives the digest and validates overrides transactionally. Upstream native_main/
supervise_native use EffectiveContract, semantic operation selection and
AuditBudget. Configured launch remains a separate supervisor; the retained task
composition calls dispatch::decide_launch/decide_balanced. Trace these actual
callers before deciding the smallest required join. Do not infer enforcement
from a clean patch apply, configuration normalization or old ordinary passes.
No new adversarial probes or cloud dispatch occurred.

Remaining: read the complete ginux-contract library/tool and remaining config/
script/document deltas; trace Recipe's emitted config and configured launch
callers; reconcile source through existing owners in dependency order; verify
fresh exact candidates. Thirteen candidates have source/ordinary evidence on
compositions of earlier8a475e8b;24 have not reached that base. None of the37
retained patches has latest-main acceptance yet. Keep G07/security/full CI and
actual upstream prerequisite holds; do not continue G11 on stale assumptions.
No branches, remote text, PRs or publication changed.


## Source trace and first adjustment — 2026-11-06

The complete2120-line contract library and102-line contract-doc tool have now
been read, along with Cargo/config/script deltas and Recipe's actual config
renderer/preparation caller. The upstream catalog/versioned profiles/selectors/
digest/AuditBudget remain the owners. Configured launch still calls the fixed
balanced dispatch, while configured task entry handles fork/exec/wait and owned
hooks before a new contract guard. A clean replay would not establish the new
contract semantics. At G03c, integrate configured effects through the upstream
contract/catalog/audit owners before proceeding with dependent runtime packets;
review later task hooks at G06b. Security acceptance stays pending downstream.
No contract-denial or adversarial runtime probe was run.

Recipe emitted `[contract] profile = "balanced"`, which upstream now rejects.
Recipe commit `d0fafc199640b6c9e71b9589c75378e2a7345b32` changes only that
renderer value to upstream's `gate5-balanced-v1`. `cargo check --offline --locked
-p levitate-recipe --lib --bins` passes with a separate target directory. On exact
upstreamff926d0c, the same ordinary gate5 configuration with old profile exits2
with invalid-contract; the versioned profile's contract-plan exits0. Initial
receipt matching expected the diagnostic on stderr; upstream prints it on stdout,
and the corrected receipt records the observed rejection. No parser or assertion
was relaxed. This is profile acceptance, not acceptance of Recipe's full rendered
configuration or end-to-end execution: storage quota/network extensions and
remaining configured capabilities still require their actual contribution owners.
The original paused Recipe source and destination binaries remain unchanged.

[G05a](g05a-review.md) is the first refreshed candidate directly onff926d0c:
head25d4f05a,86 units/static/build and8 ordinary controls pass per source/clean,
with three exact-base regressions failing. The other36 retained patches still
need latest-base reconciliation; all full default/security/macOS/remote CI holds
remain. The latest retained-patch integrity inventory is now
`upstream-contract-refresh-2026-11-06/current-inventory.json`. Next: G03a, G06a,
G03b serially, then the configured contract join at G03c.
