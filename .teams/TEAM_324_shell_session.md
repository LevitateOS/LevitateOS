# TEAM_324 — Persistent Shell Session

## Scope
Implement a persistent shell session mode that keeps QEMU alive and allows sending multiple commands via QMP sendkey.

## Problem Statement
The ephemeral `shell exec` command starts a new VM for each command (slow). 
For interactive debugging, we need a long-lived VM session where we can:
1. Send multiple commands without restarting
2. Take screenshots of the live display
3. Control the VM lifecycle explicitly

## Key Deliverables
- `cargo xtask shell start` — Start VM in background, keep alive
- `cargo xtask shell send "<text>"` — Send keystrokes via QMP sendkey
- `cargo xtask shell screenshot` — Take screenshot of running VM
- `cargo xtask shell stop` — Kill the VM

## Related Work
- TEAM_323: Ephemeral `shell exec` command
- Existing QMP client in `support/qmp.rs`

## Progress Log
- [x] Registered team
- [x] Phase 1: Discovery
- [x] Phase 2: Design
- [x] Phase 3: Implementation
- [ ] Phase 4: Testing
- [ ] Phase 5: Polish
