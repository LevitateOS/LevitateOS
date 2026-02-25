# TUI Workspace (Stub)

Centralized TUI workspace for LevitateOS.

## Layout

- `apps/*`: executable TUI applications by stage/intent.
- `kit/core`: core runtime/primitives/components/chrome/theme.
- `kit/presets-install`: shared install UX compositions for S02/S03 apps.
- `kit/testing`: shared test harness/snapshot helpers for TUI apps.

## Ownership

- `apps/s02-live-tools/install-docs` is the canonical S02 session UX/docs app.
- Stage-native apps are canonical (for example `apps/s03-install/disk-plan`).
- `apps/recpart` is a temporary compatibility shim only, not a canonical owner.

This tree is scaffold-only and does not replace existing paths yet.
