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


Configured launch already owns a64 KiB byte audit cap in `launch.rs`
(`AUDIT_BYTE_BUDGET`), distinct from upstream native's8192-record AuditBudget.
Preserve the existing configured writer and its bound when joining contract
metadata/disabled mode; do not add a parallel audit system or assume native's
record budget is an instruction to replace this public configured behavior.


G03a is now also refreshed directly onff926d0c at
`feafbad2a374e80c18943370e7efd1263554a245`:83 units/static/build and9 ordinary
controls pass per source/clean; exact upstream fails the same workdir case.
G05a/G03a are the two latest-base preparations;35 retained packets remain.
Next is G06a, then G03b and the configured-owner join at G03c. Complete raw
upstream delta is retained; upstream HDAE records describe its own project and
are not authorization to change this downstream task's branch/publication rules.


G06a is now also directly onff926d0c at
`4a62d81e3d81ede3b987d679333c39b62212c8bf`:86 units/static/build and eight
ordinary controls pass per source/clean; three exact-base adoption regressions
fail. Three latest-base preparations exist;34 retained packets remain. Next is
G03b on refreshed G05a, then the configured-owner join at G03c.


G03b is now refreshed on latest G05a at
`095f89c5bfb2f8d07404f2304a5088f709ef7652`:95 units/static/build and eight
ordinary controls pass per source/clean. Four preparations are rooted atff926d0c;
33 retained packets remain. Next is G03c on refreshed G03b/G03a. Review actual
catalog membership for newly implemented configured syscalls as well as guarding
hook dispatch before effects; preserve all existing catalog/core consistency
coverage and configured audit bounds. Actual upstream prerequisite merges and
security/full CI remain required.


The unverified G03c source preview is prepared on latest G03b/G03a composition
85ee433a. Its actual adapter implements lstat6/fchmodat268, absent from upstream's
catalog. Its filesystem claims and path rewriting also precede any resolved
contract check. Reconcile these through existing contract/catalog and launch/
audit owners, preserving native denials and all existing consistency assertions
for previously cataloged operations. No preview build/runtime check has run;
no G03c readiness claim or retained-head replacement is made. Exact source/read
position is in the preview's state.json.


## Configured storage join verified ordinarily — 2026-11-07

G03c's preview is superseded by final
`3801fd45bd6280a5b320f2d464ada0bd04539326` on composition85ee433a.
[Exact review](g03c-review.md) records the real shared rule-selection/catalog/
pre-effect configured guard and existing audit integration, not a clean-apply
inference. New catalog entries retain native denials; all previous consistency
assertions/CI coverage remain. Source/clean pass96 units/static/build, generated
docs and15 ordinary controls each; all199 source paths and both binaries match.
The earlier proposed contract gap is corrected in this source revision; monitored
runtime denial/authority/shared-argument acceptance remains pending. The default
Dpolicy fixtures are compiled/preserved and unrun. No security agent was dispatched.

Five latest-base preparations exist;32 retained patches remain. Next is G04 on
G03c, ensuring its device claims stay under the resolved guard; then descriptor/
task joins at G05/G06b must preserve the same ordering. Actual upstream prerequisite
merges, catalog/contract scope review, full CI/security and F02/F03 remain open.
