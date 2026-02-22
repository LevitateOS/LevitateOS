# Multi-TUI Installer Proposal (Stage-Native, Non-Monolithic)

## Status

Draft proposal for implementation planning.

## Goal

Build a multi-TUI installation system using `tui-kit` where each TUI owns a single installation concern and stage boundary, instead of creating one large "do everything" installer.

This proposal intentionally avoids a monolithic, highly-branching installer flow. The design is explicit, stage-native, and reproducible.

## Design Intent

1. Keep installation as multiple focused TUIs, each with narrow scope.
2. Avoid a single "smart" installer that guesses user intent.
3. Require explicit artifact/state handoff between TUIs.
4. Make each TUI runnable independently for debugging and testing.
5. Keep commands transparent: users can see exactly what each TUI will execute.

## Stage Mapping and TUI Set

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
