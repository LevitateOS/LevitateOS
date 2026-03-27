# Repository Guidelines (Policy-First)

This file is intentionally compact. Priority is preventing policy violations and reward-hacking behavior.

## 0) Prime Directive
- Do not optimize for green checks. Optimize for true checkpoint correctness and reproducibility.
- Never hide failures. Fix root causes.
- If a command passes only because of suppression/masking/fallback tricks, treat it as a failure.

## 1) Workspace and Ownership
- New work belongs in: `distro-variants/*`, `distro-builder`, `distro-contract`, `distro-spec`, `testing/*`, `xtask`.
- Legacy crates are read-only unless explicitly requested for scoped compatibility:
  - `leviso/`, `AcornOS/`, `IuppiterOS/`, `RalphOS/`.

## 1.1) Recipe vs Orchestrator Boundary (Strict)
- `recipe` is the canonical owner for package/source knowledge.
- Rust/TOML orchestration is the canonical owner for sequencing, policy, contract checks, and artifact topology.
- Keep the boundary hard:
  - Recipes own versions, URLs, checksums, torrent links, package lists, trust-marker semantics, default paths, and acquire/build/install/remove logic.
  - Rust owns recipe selection, build/output roots, product/release/scenario sequencing, checkpoint policy enforcement, and contract validation.
- Rust must stay dumb about recipe contents.
- Forbidden in Rust/TOML default paths:
  - duplicating release URLs, checksums, torrent URLs, or package lists solely to inject them into recipes
  - constructing large required `--define` maps for canonical recipe execution
  - hardcoding recipe-specific trust-marker names, remote layout rules, or package expectations that belong in the recipe
  - treating a user-facing recipe as an internal worker script while still exposing it like a normal recipe
- Required model for user-facing recipes:
  - the canonical recipe must run with zero required defines for the normal case
  - overrides may exist, but they must be optional and additive
  - a user must not have to look up upstream metadata manually just to run the recipe
- If a script truly requires injected defines, then it is an internal worker, not a normal recipe.
- Internal workers must be clearly marked as such and must not be the default public UX when a canonical recipe should exist.
- When Rust and a recipe both know the same source/package fact, that is an ownership bug.
- Fix ownership bugs by moving factual knowledge into the recipe and reducing Rust to orchestration only.
- Desired mental model:
  - recipe = smart, self-contained source/package knowledge
  - Rust = dumb orchestrator that runs the right recipe at the right time

## 2) Hard Ban: Legacy Binding
- Never wire checkpoint/rootfs/tooling paths to legacy crate downloads outputs.
- Forbidden examples (non-exhaustive):
  - `*/downloads/rootfs` from legacy crates
  - `leviso/downloads/.tools` (or any legacy crate equivalent)
  - dynamic fallback/autodiscovery to `*/downloads/.tools`
- Required guard before build/commit:
  - `cargo xtask policy audit-legacy-bindings`
  - alias: `just policy-legacy`
- Any violation is a hard failure.

## 2.1) Guard Placement (Authoritative Boundary)
- Guard placement is valid only at executable entrypoints that perform work (`distro-builder`, `testing/install-tests`, `xtask` commands that build/test/scenario-run).
- `just` checks are convenience only and must never be the sole enforcement layer.
- A command path that can build/test artifacts without running policy guards is a policy bug and must be fixed immediately.
- Required model:
  - preflight guard runs inside the executable command path, before any artifact mutation or QEMU/test launch
  - fail-fast on guard violation with cheat-guard diagnostics
  - no "best effort continue" after guard failure
- Treat wrapper-only guard wiring as insufficient even if current developers usually use wrappers.

## 3) Checkpoint Vocabulary (Canonical)
- Use canonical checkpoint names only where the conformance ladder is the real owner:
  - `00Build`, `01Boot`, `02LiveTools`, `03Install`, `04LoginGate`, `05Harness`, `06Runtime`, `07Update`, `08Package`
- Rings, products, releases, and scenarios own manifests, filetree layout, and orchestration.
- Do not introduce new legacy alias families, `sNN_*`, or checkpoint-numbered owners outside explicit compatibility or reporting surfaces.

## 4) Checkpoint Artifact Split (Current Output Family)
- Every checkpoint writes non-kernel outputs only into its own directory family:
  - `.artifacts/out/<distro>/sNN-<checkpoint-name>/...`
- Cross-checkpoint reuse of non-kernel artifacts is forbidden in release/hardening mode.
- Kernel artifacts are the only shared artifacts across checkpoints in release/hardening mode.
- While `9.2` bypass mode is active, `02LiveTools` may intentionally carry forward-looking non-kernel payload for filesystem design convenience.
- No cross-checkpoint symlinks/copies/manual post-build surgery to fake checkpoint outputs.

## 5) Checkpoint Envelope (Mode-Aware)
- A checkpoint artifact must contain exactly what that checkpoint needs in release/hardening mode.
- Missing required payload = fail.
- Carrying later-checkpoint payload = fail in release/hardening mode.
- While `9.2` bypass mode is active, `02LiveTools` may include later-checkpoint-intent payload, provided it is produced via canonical builder/producers.
- Checkpoint scope must be produced by composition in builder/contract, not by runtime suppression.

## 6) Incremental Producer Model (No Subtractive Shaping)
- Build checkpoint rootfs incrementally by producers.
- Forbidden: prune/exclude/post-copy deletion strategies to shape checkpoint payload.
- Checkpoint progression is additive:
  - `s00` minimal build payload
  - `s01 = s00 + boot additions`
  - same model for later stages
- In `9.2` bypass mode, prefer implementing candidate payload in `02LiveTools` producers first, then formalize checkpoint-specific envelopes during hardening.
- Do not reintroduce `NNStage.toml` or checkpoint-scoped file allow/deny manifests.

## 7) No-Mask / No-Suppression Policy
- Do not mask services/checks to make checkpoints pass.
- Do not downgrade `FAIL` to `SKIP/PASS` to keep pipelines green.
- Do not add fallback paths that hide wiring errors.
- Any intentional disablement must be explicit checkpoint policy intent and validated in `distro-contract`.

## 8) Kernel Boundary Policy
- Kernel artifacts are only:
  - `.artifacts/kernel/<distro>/current/kernel-build/**`
  - `.artifacts/kernel/<distro>/current/staging/boot/vmlinuz`
  - `.artifacts/kernel/<distro>/current/staging/{lib,usr/lib}/modules/<kernel.release>/...`
- Kernel rebuilds only through:
  - `cargo xtask kernels build <distro>`
  - `cargo xtask kernels build-all`
- Everything else is non-kernel and may be rebuilt without rebuilding kernel.

## 9) Reproducibility-First
- No ad-hoc artifact surgery (`cp`, `mv`, `ln -s`, manual edits under `.artifacts/out/*`) to force success.
- If a checkpoint build fails, fix code/contracts/wiring; do not patch outputs manually.
- A checkpoint is considered working only if reproducible from repository commands with existing kernel artifacts.

## 9.1) Build/Boot Boundary (Strict)
- `build` and `release-build` commands produce artifacts; `scenario*` commands consume existing artifacts.
- Do not add implicit build side effects to scenario boot/test wrappers.
- Fixing wrapper parity/routing must not change artifact freshness policy.
- If fresh artifacts are required, use explicit build commands (`just build ...`, `just build-up-to ...`, or `just release-build ...`) before scenario boot/test.

## 9.2) Checkpoint Ladder Bypass (Current Workflow Override)
- Strict adherence to checkpoint test ladders is an anti-pattern for the current development mode.
- The owner intentionally bypasses ordered checkpoint gates while iterating on filesystem design.
- `02LiveTools` is the canonical convenience surface for filesystem composition, including payload that may later be required by installed checkpoints.
- Prefer implementing payload at producer/rootfs level (not ad-hoc artifact edits) so `02LiveTools` design work naturally carries forward.
- Before release/hardening, run the full checkpoint ladder and fix any regressions discovered outside the bypass workflow.

## 10) Error Messaging Policy
- Fail fast with explicit diagnostics.
- Errors must name: component, checkpoint or product, expectation, and concrete remediation command/path.
- Silent fallback is prohibited.

## 11) Live Environment Completeness
- Do not ship warning-prone checkpoint environments by ignoring missing runtime payload.
- Fix missing payload at producer level, wire canonical config, enforce in contract checks.

## 12) Required Commands (Day-to-day)
- Legacy policy guard:
  - `cargo xtask policy audit-legacy-bindings`
- Build release products:
  - `cargo run -p distro-builder --bin distro-builder -- release build iso <distro> <product>`
  - `just release-build <distro> <product>`
- Scenario tests:
  - `cargo xtask scenarios test <scenario> <distro>`
  - `cargo xtask scenarios test-up-to <scenario> <distro>`
  - `just scenario-test <scenario> <distro>`
  - `just scenario-test-up-to <scenario> <distro>`
- In `9.2` bypass mode, scenario tests may be run out of ladder order during design iteration; full ordered checkpoint coverage remains required before release/hardening.

## 13) Dirty Tree Rule
- Assume existing diffs are intentional.
- Do not revert unrelated changes unless explicitly asked.

## 14) Commit Rule (when user asks “commit ALL”)
- Commit dirty submodules first.
- Commit superproject after submodule pointers update.
- Do not commit generated junk/secrets/caches.

## 15) Practical Build Notes
- Prefer `just` wrappers for env/tooling consistency.
- Keep CLIs quiet on success, loud on failure.
- Keep Rust code `fmt`/`clippy` clean.

## 16) Single-Intent Ownership (Agent Workflow)
- Treat every behavior request as an ownership update: **locate the existing codepiece first**, then edit that piece directly.
- If an older behavior already exists, refinement must be additive/replace-in-place on that owner; do not create a parallel implementation by default.
- Do not discard prior refinements during a migration; preserve and migrate existing behavior into the canonical codepath unless explicitly asked to archive it.
- Before editing, do a quick duplicate-intent scan for the same user-visible behavior:
  - if one canonical owner exists, use that;
  - if no owner exists, create one implementation only.
- If competing implementations are found, you must consolidate or remove/retire all but one in the same change set.
- Preserve a single default output/logic path:
  - no shadow/compatibility branch in the default path.
  - if a legacy/compat path is needed, it must be a separate explicit command/flag and not the default behavior.

## 17) Module System Policy (Strict)
- CommonJS is strictly forbidden.
- Do not introduce `require`, `createRequire`, `.cjs`, or any CJS fallback branch.
- Runtime/module loading must be ESM-only.
