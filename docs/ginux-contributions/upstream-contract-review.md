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
