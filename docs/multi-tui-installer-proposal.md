# Multi-TUI Installer Proposal (Stage-Native, Non-Monolithic)

## Status

Draft proposal for implementation planning.

## Goal

Build a multi-TUI installation system using `tui-kit` where each TUI owns a single installation concern and stage boundary, instead of creating one large "do everything" installer.

This proposal intentionally avoids a monolithic, highly-branching installer flow. The design is explicit, stage-native, and reproducible.

## Scope Gate (Stage 02 Policy)

This proposal applies only to distro variants that declare Stage 02 `install_experience = "ux"` in `distro-variants/*/02LiveTools.toml`.

- In-scope UX variants: `levitate`, `acorn`
- Out-of-scope (default path): `ralph`, `iuppiter` (`install_experience = "automated_ssh"`)

For `automated_ssh` variants:

1. Stage 02 remains SSH/headless automation oriented and must not route default ISO flow through local TUI install surfaces.
2. Stage 03 install flow also remains automation-first; local installer TUIs are disabled by default.

## Stage UX Authority Split

1. Stage 02 (`02LiveTools`) owns **session UX**: shell profile, tmux/session layout, docs visibility defaults, live overlay UX, and install entrypoint launch behavior.
2. Stage 03 (`03Install`) owns **task UX**: disk/filesystem/bootstrap/fstab/chroot/bootloader mutation workflows and their handoff artifacts.

## Design Intent

1. Keep installation as multiple focused TUIs, each with narrow scope.
2. Avoid a single "smart" installer that guesses user intent.
3. Require explicit artifact/state handoff between TUIs.
4. Make each TUI runnable independently for debugging and testing.
5. Keep commands transparent: users can see exactly what each TUI will execute.

## Stage Mapping and TUI Set

### 02LiveTools (session UX platform, `ux` variants only)

- `tui-install-docs`
  - Canonical install docs/navigation surface in live environment.
  - Intended to run as part of Stage 02 session layout (for example, tmux pane integration).

### 03Install (primary interactive stage)

- `tui-disk-plan`
  - Captures disk target selection, partition strategy, and encryption options.
  - Produces a disk plan artifact.

- `tui-filesystem`
  - Confirms format choices and mount layout.
  - Consumes disk plan; produces mount plan artifact.

- `tui-bootstrap`
  - Runs rootfs extraction/bootstrap (`recstrap`) with progress and real-time diagnostics.
  - Consumes mount plan; produces bootstrap completion artifact.

- `tui-fstab`
  - Generates and previews fstab (`recfstab`) before apply.
  - Consumes mount/bootstrapped root context; produces fstab artifact.

- `tui-chroot-config`
  - Applies core installed-system configuration through `recchroot` actions:
  - hostname, users, auth defaults, timezone, network/service toggles.
  - Produces configuration artifact and command log.

- `tui-bootloader`
  - Installs and validates bootloader setup with explicit checks.
  - Produces bootloader verification artifact.

### 04LoginGate (support/debug UI)

- `tui-firstboot-check`
  - Diagnostics for login prompt availability and first-boot service readiness.
  - Primarily a verification/recovery UI, not a primary install flow UI.

#### 04LoginGate Auth Boundary Policy

1. Default S04+ auth boundary remains native `getty` + PAM login prompt.
2. First TUI step is post-login (`tui-firstboot-check`) as the default UX gate.
3. A full TUI login replacement is allowed only as an explicit variant policy change, with matching updates in:
   - `distro-contract` auth declarations
   - stage test expectations (`04LoginGate` + `05Harness`)
   - conformance/runtime checks for auth/session correctness
4. No silent fallback between auth models; the selected model must be explicit and contract-validated.

### 06Runtime (support/debug UI)

- `tui-postinstall-tools`
  - Verifies expected runtime tools/services.
  - Provides concrete remediation commands for failed checks.

## Execution Model

1. Each TUI is a separate executable (or explicit subcommand), not a hidden internal mode.
2. Shared rendering/components come from `tui-kit`:
   - forms
   - progress views
   - command preview/runner widgets
   - structured error panels
   - confirmation/commit dialogs
3. Shared state is persisted per installation run:
   - live path suggestion: `/run/levitate/install-state/*.json`
   - optionally copied into target root for auditability.
4. Strict handoff contract:
   - each TUI consumes explicit prior-state files
   - each TUI emits explicit output-state files
   - no implicit hidden state.

## Guardrails (Explicitly Avoiding Monolithic Installer Drift)

1. No giant wizard with deeply nested branching.
2. No silent defaults applied without clear user confirmation.
3. No fallback logic that masks wiring or stage errors.
4. Every "Apply" screen must show exact commands and targets.
5. Fail fast with actionable diagnostics:
   - component
   - stage
   - expectation
   - remediation command/path.

## State and Contract Shape

Each TUI should own a minimal schema and artifact:

- `disk-plan.json`
- `mount-plan.json`
- `bootstrap-state.json`
- `fstab-state.json`
- `chroot-config-state.json`
- `bootloader-state.json`
- `firstboot-check.json` (optional diagnostics)
- `runtime-check.json` (optional diagnostics)

Contract rules:

1. Producer writes exactly one canonical artifact per intent.
2. Consumer validates required upstream artifact before any mutation.
3. Contract violations fail immediately.
4. Artifacts are append-only by stage progression; no subtractive "shape later" logic.

## Repository Ownership and Placement

Policy-aligned placement for new implementation work:

- `distro-builder`
  - stage wiring, execution orchestration, artifact integration.
- `distro-contract`
  - schema and validation for TUI handoff artifacts.
- `distro-spec`
  - stage/tool expectations and conformance updates.
- `testing/*`
  - stage-level TUI validation and integration scenarios.
- `xtask`
  - developer and CI orchestration commands.

Do not introduce new default behavior in legacy read-only crates unless explicitly requested for scoped compatibility.

## Testing Strategy

### Stage-level checks

Each TUI must be validated for:

1. launch success in live environment.
2. input validation correctness.
3. artifact emission correctness.
4. deterministic failure behavior on invalid input or failed command.

### Integration checks

1. Add install-tests flow for scripted/non-interactive TUI mode.
2. Assert strict handoff between TUIs (artifact contract checks).
3. Ensure stage scripts verify TUI-produced state where applicable.

### Failure policy

1. No downgrade from `FAIL` to `SKIP/PASS`.
2. No suppression wrappers to keep pipeline green.
3. Failures are surfaced with concrete remediation.

## Rollout Plan

### Milestone 1

- `tui-disk-plan`
- `tui-filesystem`

Reason: highest-risk install steps and largest immediate UX gain.

### Milestone 2

- `tui-bootstrap`
- `tui-fstab`

### Milestone 3

- `tui-chroot-config`
- `tui-bootloader`

### Milestone 4

- `tui-firstboot-check`
- `tui-postinstall-tools`

## Non-Goals

1. Building a one-shot "automatic installer" that hides command execution.
2. Replacing stage boundaries with runtime suppression logic.
3. Introducing implicit build side effects into boot/test wrappers.

## Next-Step Implementation Spec (Follow-on Doc)

When ready, create a follow-on spec that defines:

1. exact command names and CLI surfaces.
2. JSON schema for each handoff artifact.
3. crate/module ownership map by file path.
4. CI and install-tests execution matrix for each TUI.
5. migration plan from current shell-driven flow to multi-TUI flow.

## Superproject TUI Centralization Proposal

This section defines the concrete repository layout to centralize all TUI work currently spread across:

- `docs/tui` (canonical implementation)
- `shared/tui-kit` (shared package)
- `tools/recpart/frontend` (deprecated legacy path; removed)

### Invariant

All interactive TUIs for installer and rec tools must live under one superproject TUI tree, with exactly one canonical UI owner and one shared kit owner.

### Ownership Layer

Monorepo `tui/*` workspace with explicit package boundaries:

- `tui/apps/*` owns app/domain behavior.
- `tui/kit/*` owns reusable rendering/runtime/pattern primitives.
- `distro-contract` owns handoff artifact schemas and validation.
- `distro-builder` owns stage wiring/entrypoint routing.

### Why This Layer Can Enforce It

The workspace root can enforce dependency direction (`apps -> kit`, never `kit -> apps`) and prevents drift caused by keeping canonical app code in one tree and reusable code in another disconnected tree.

### Proof Artifact Path

`docs/multi-tui-installer-proposal.md` (this section) is the canonical target-tree contract.

### Complexity Budget

- Added: one structural proposal section (this doc).
- Removed: none.
- Net LOC: documentation-only delta.

## MODEL RECLASSIFICATION GATE

Current pain is architectural drift (multiple partial owners), not missing widgets. The model must change from "scattered app+kit implementations" to "single TUI workspace with strict ownership and stage-native app packages."

Decision matrix:

| Option | Determinism | Complexity | Ownership Fit | Testability | Compatibility Impact |
|---|---|---|---|---|---|
| A. Keep current scattered paths and patch imports | Low | Medium-High (ongoing) | Poor | Medium | Low short-term, high long-term drift |
| B. Centralize under `tui/*` with app/kit split | High | Medium (one migration) | Strong | High | Medium (path + workspace updates) |
| C. Fold everything into one `docs/tui` package | Medium | Medium | Weak (kit and apps coupled) | Medium | Low-Medium |

Chosen: **Option B** because it gives one canonical app lineage plus a reusable kit with explicit dependency direction, minimizing future refactor loops.

## Target File Tree (Proposed)

```text
/tui
  /README.md
  /AGENTS.md
  /apps
    /s02-live-tools
      /install-docs                    # canonical S02 docs/session UX surface
        /src
        /bin
        /scripts
        /snapshots
        /package.json
        /tsconfig.json
        /README.md
    /s03-install
      /disk-plan                       # tui-disk-plan (canonical recpart UI surface)
      /filesystem                      # tui-filesystem
      /bootstrap                       # tui-bootstrap
      /fstab                           # tui-fstab
      /chroot-config                   # tui-chroot-config
      /bootloader                      # tui-bootloader
    /s04-login-gate
      /firstboot-check                 # tui-firstboot-check
    /s06-runtime
      /postinstall-tools               # tui-postinstall-tools
  /kit
    /core                              # re-architected tui-kit owner
      /src
      /tests
      /package.json
      /tsconfig.json
      /README.md
```

## Migration Map (Current -> Target)

1. `docs/tui` -> `tui/apps/s02-live-tools/install-docs` (canonical S02 session UX/docs app).
2. `shared/tui-kit` -> `tui/kit/core` (re-architecture happens here; no docs-domain logic).
3. legacy `tools/recpart/frontend` -> `tui/apps/s03-install/disk-plan` (stage-native UI owner).

Compatibility shims during migration:

1. Keep legacy paths as thin wrappers/symlinks only for one transition window.
2. `just docs-tui` routes to `tui/apps/s02-live-tools/install-docs`.
3. `just tui-s03-disk-plan` routes to `tui/apps/s03-install/disk-plan`.

## Dependency Rules (Must Hold)

1. `tui/kit/core` cannot import from any `tui/apps/*`.
2. `tui/apps/*` may import from `tui/kit/core`.
3. Handoff schemas stay in Rust-side `distro-contract`; TUI apps consume generated TS bindings/schemas only.
4. Stage routing remains owned by `distro-builder`; no app-side fallback routing.

## Workspace Wiring Changes

Update root `package.json` workspaces from:

- `docs/tui`
- `shared/tui-kit`
- `tools/recpart/frontend` (removed)

to:

- `tui/apps/s02-live-tools/*`
- `tui/kit/core`
- `tui/apps/s03-install/*`
- `tui/apps/s04-login-gate/*`
- `tui/apps/s06-runtime/*`

## Phase Plan

1. **Phase 0 (no behavior change):** create `tui/*` tree, move packages, keep CLI wrappers.
2. **Phase 1:** re-architect `tui/kit/core` API surface (runtime/primitives/components/patterns).
3. **Phase 2:** repoint S02 install-docs app imports to new kit API and lock snapshots.
4. **Phase 3:** reintroduce recpart-backed S03 disk-plan UI at `tui/apps/s03-install/disk-plan` on top of new kit API.
5. **Phase 4:** implement stage-native installer TUIs (`s03/s04/s06`) with contract-gated handoff artifacts.
6. **Phase 5 (optional auth-model promotion):** if desired, add a true TUI login for S04+ as an explicit model change after contract/test parity is complete.
