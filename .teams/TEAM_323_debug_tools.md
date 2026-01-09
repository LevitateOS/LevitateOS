# TEAM_323 — VM Debug Tools Feature

## Scope
Implement debugging tools for interacting with and inspecting the LevitateOS VM.

## Problem Statement
Currently, there is no ergonomic way to:
1. Run arbitrary commands inside the VM shell from the host
2. Dump VM memory or registers without attaching GDB
3. Automate interactive debugging workflows

## Key Deliverables
- `cargo xtask shell exec "<command>"` — Run commands in VM shell
- `cargo xtask debug mem <addr>` — Dump memory via QMP
- `cargo xtask debug regs` — Dump CPU registers via QMP

## Related Work
- TEAM_116: GDB server support
- TEAM_320: GPU debug tracing
- TEAM_322: Xtask refactor (QemuBuilder)

## Progress Log
- [x] Registered team
- [x] Phase 1: Discovery (docs/planning/debug-tools/phase-1.md)
- [x] Phase 2: Design (docs/planning/debug-tools/phase-2.md)
- [x] Phase 3: Implementation (TEAM_325 completed)
- [x] Phase 4: Testing (TEAM_325 completed)
- [ ] Phase 5: Polish

## Implementation Notes
TEAM_325 implemented the debug module on 2026-01-09:
- Created `xtask/src/debug/mod.rs`
- Commands: `debug regs`, `debug mem`
- See `.teams/TEAM_325_review_debug_shell_impl.md` for details
