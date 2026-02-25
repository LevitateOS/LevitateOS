# `tui/kit/core` Agent Boundary Guide

`tui-kit` is a React + Ink toolkit, not a docs-domain or product-flow engine.

## Canonical ownership

`tui-kit` owns:

- `src/app/*`: app creation and mount lifecycle wrappers
- `src/runtime/*`: terminal/input/focus/exit primitives
- `src/primitives/*`: low-level layout/display blocks
- `src/components/*`: generic widgets (forms/navigation/feedback/data)
- `src/patterns/*`: reusable composition patterns
- `src/theme/*`: tokens/colors/typography/spacing
- `src/hooks/*`: generic hooks

`tui-kit` does **not** own:

- docs-specific rendering/navigation models
- product-specific state machines or business workflows
- backend command orchestration

## Public API rules

- Stable exports are `@levitate/tui-kit` and `@levitate/tui-kit/theme`.
- Do not add a docs-specific subpath export.
- Keep default runtime/module loading ESM-only.

## Design rules

- React hooks are the lifecycle mechanism; do not add a parallel lifecycle subsystem.
- Prefer composable TSX primitives over imperative controller abstractions.
- Fail fast with explicit errors; no silent fallback behavior.

## Architecture Contract

### Provider Architecture (`tui-kit`)

`tui-kit` uses a **component-owned render architecture**:

- primitives own their own visual invariants (line width, seams, borders, padding)
- hooks own lifecycle/input state transitions (scroll, hotkeys, focus, viewport)
- patterns/surfaces compose primitives; they do not run global post-processing
- no endpoint/global smart-line processor is allowed

### Provider Abstraction Rules (`tui-kit`)

`tui-kit` abstractions must be:

- prop-driven TSX components and hooks
- deterministic (same props/state => same terminal output)
- composable in a tree like React for web
- domain-agnostic (no docs-content, distro, or product flow coupling)

Priority migrations from `docs/tui` that define this contract:

- `src/presentation/ink/blocks/shared/intent-color.ts`
- `src/presentation/ink/blocks/shared/syntax-line.tsx`
- `src/presentation/ink/primitives/command-line-row.tsx`
- `src/presentation/ink/primitives/command-line-series.tsx`
- `src/presentation/ink/primitives/admonition-frame.tsx`
- `src/presentation/ink/primitives/rich-text-runs.ts`

Secondary generic migrations (only after they are domain-free):

- render-plan measurement helpers
- viewport + scroll-window math
- section-aware list navigation helpers

### Consumer Integration Rules (apps using `tui-kit`)

Consumers should behave like HTML React apps:

- build UI by composing leaf components
- style/update one component without touching unrelated endpoints
- replace components incrementally without rewriting central processors
- keep business/domain logic outside `tui-kit`

Never move docs schemas, distro scope rules, or product/session orchestration into `tui-kit`.

### Boundary Summary

Provider (`tui-kit`) owns:

- reusable rendering primitives and composition patterns
- generic input/viewport/scroll/focus hooks
- deterministic styling/layout contracts

Consumer apps own:

- domain schemas and content contracts
- product workflows, session state, and business rules
- app-specific navigation semantics beyond generic hooks

## Abstraction-Loop Guard (Mandatory)

- Complexity must track bug size. If a fix starts exceeding the defect scope, stop and reclassify before further edits.
- Do not fix local visual/layout defects by defaulting to cross-layer plumbing, measurement guessing, or global contracts.
- Enforce invariants only at the layer that owns required state (ownership rule).
- If tests and user-observed output disagree, user-observed output is authoritative.
- For strict TUI chrome invariants (borders/seams/splits), prefer structural construction over timing-dependent behavior.
- If trigger conditions appear (multi-round patch churn, proof mismatch, disproportionate LOC), run a mandatory `MODEL RECLASSIFICATION GATE` before continuing:
  - invariant sentence
  - current enforcement layer
  - at least two alternate models
  - short decision matrix (determinism, complexity, ownership, testability, compatibility)
  - selected model and explicit tradeoff callout
