# PRD17 — production acceptance is incomplete

Recipe is **not production ready**. Audit in progress against selected Ginux
e4d95de1 / launcher 21a54e60, Recipe b185e036 / CLI-worker e7073fc1,
image producer aa13ac9b and acquisition image ffe04d86. Current source selection
and actual products are in delivery-sources.json and delivery-provenance.json.

Active skill: hdae-development, project discovery path
.agents/skills/hdae-development/SKILL.md. Reload the complete skill, required
reference, root instructions and this checkpoint after context recovery. Root
rules prohibit mandatory process gates and Recipe unit tests. Existing managed
CLI, producer and product owners provide verification. No subagents requested.

## Current findings and decisions

- Nine authorized owning-source pushes are complete; anonymous public acquisition
  passes 16/16. Root pin commit 89863d5 recursively obtains those sources and
  passes locked metadata. Exact parent publication request 1b2d96f remains
  unanswered; PRD17 is not that permission. Audit corrections create a later
  candidate, which needs its own exact source/delivery reconciliation.
- Ginux e4d95de1 hosted run 42609102747 succeeds, including the required Linux
  job and compiler FD/stack/O_PATH/event controls nested in its existing make
  targets. macOS remains deferred; no macOS investigation.
- Recipe b185e036 run 42609021436 fails managed CI before execution: all four
  runtime variables are empty. Hosted build/fmt/Clippy/MSRV and independent
  stable/MSRV unlocked resolution pass; release is skipped. No managed pass.
- docs-content d938493 run 42609031135 fails frozen installation: standalone
  bun.lock lists removed Biome/React/Vite dependencies and misses @types/bun.
  packageManager is absent, so setup selected Bun 1.4.4 instead of pinned 1.4.2.
  Correct in its owner, retain frozen-lock enforcement and verify independent
  install/typecheck/build with unchanged authored content.
- Upstream main remains 53a54adde. #232/#233/#234/#236/#238/#243 and #249 remain
  open/unmerged. PRD05 must wait for actual merges/replacement foundation; the
  maintained-source choice does not waive this explicit completion criterion.
- Final-source Daybreak results/authentication remain missing. The authorized
  gpt-6-sol/high/daybreak_blue CLI was rejected for missing authorized hardware-key
  authentication. Do not retry unchanged, use an ordinary-model fallback or run
  the adversarial matrix in the parent. Prior partial receipts remain partial.

The user resolved two additional decisions during this audit:
1. Hosted managed CI must build its image with tracked public bootstrap producers
   and pinned inputs, checking the expected digest. Do not upload a substitute
   image release or weaken input/trust validation. New source publication and
   hosted dispatch still need exact permission.
2. Explicit dependent-recipe remove and cleanup must require matching cached
   dependency products, validated with installed checks, and never run dependency
   install hooks. This supersedes the former held decision. Implement in the
   existing backend, migrate documentation and retain real CLI controls covering
   nested dependencies, cache/input misses and prior-state preservation.

## Exact next actions

Implement those bounded corrections and docs-content standalone CI correction;
compile and run meaningful managed regressions. Pin new CLI/worker/harness bytes
and source joins, then refresh every affected required outcome on the resulting
combination. Preserve all original successful and failed receipts. Assemble the
criterion-by-criterion PRD01–PRD19 evidence/disposition and concrete artifact/PR
publication package before new permission requests. No production acceptance,
new push, PR/comment, tag, release, repository-variable write or hosted dispatch
is authorized by the audit. Wait for actual security authentication/results,
required upstream merges and final user acceptance; do not waive them.

Read-only source/CI metadata and selected logs are retained under
.artifacts/ginux-contribution-preparation/prd17/initial-evidence.json and
{recipe-failed,docs-failed,ginux-linux}.log. The first attempt to save excessive
raw GitHub content exceeded the OS argument limit and wrote nothing; trimmed
exact metadata and complete selected logs were then saved in bounded chunks.
Original shared staged patch f4d8c81b remains protected. No existing human-authored
public text is edited.
