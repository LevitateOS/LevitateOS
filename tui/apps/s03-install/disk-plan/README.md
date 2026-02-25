# s03 disk-plan (recpart-backed)

Canonical Stage 03 disk planning UI owner.

## Current Flow

Implemented thin shell pages:

1. `welcome`
2. `target-disk`
3. `mode-selection`
4. `plan-preview` (runs `recpart plan --json`)
5. `command-preview` (runs `recpart apply --dry-run --json` and displays exact emitted step commands)
6. `preflight-checks`
7. `destructive-confirm`
8. `apply-progress`
9. `result-handoff`
10. `failure-diagnostics`

## Ownership Policy

- UI app (`tui/apps/s03-install/disk-plan`) owns flow, page transitions, and evidence presentation.
- Backend tool (`tools/recpart`) owns partition/apply semantics, safety checks, and JSON contracts.
- Contract violations fail fast; schema version must match expected value (`schema_version = 1`).

## Run

```bash
just tui-s03-disk-plan -- --disk /dev/sdX --mode ab
```

## Keybinds

- `q` / `Esc` / `Ctrl-C`: quit
- `Enter` / `n` / `→` / `l`: next page
- `p` / `←` / `h`: previous page
- `m`: toggle mode (`ab`/`mutable`) on mode page
