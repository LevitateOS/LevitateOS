# TEAM_323: Debug Tools Questions

## Feature Reference
- Plan: `.plans/debug-tools/`
- Phase: 2 (Design)

---

## Q1: VM Lifecycle for Debug Commands

**Context:** `cargo xtask debug mem` and `debug regs` need a running VM to inspect.

**Question:** Should these commands:
- A) **Require existing VM** — Connect to `./qmp.sock`, fail if not running
- B) **Auto-start VM** — Start headless VM if not running, then connect

**Recommendation:** Option A (require existing) — starting a VM is slow and may not be what the user wants.

---

## Q2: VM Lifecycle for Shell Exec

**Context:** `cargo xtask shell exec "ls"` needs to send commands to the VM shell.

**Question:** Should this command:
- A) **Start fresh VM each time** — Predictable, isolated, but slow (~5-10s boot)
- B) **Attach to running VM** — Fast, but requires VM to already have shell prompt
- C) **Both modes** — Default to fresh, `--attach` flag for existing

**Recommendation:** Option C (both modes) — flexibility.

---

## Q3: Additional Debugging Needs

**Question:** Are there other debugging workflows you need beyond:
- Memory dump (`debug mem`)
- Register dump (`debug regs`)
- Shell command execution (`shell exec`)

**Examples of possible additions:**
- Stack trace from panic address
- Symbol lookup (address → function name)
- Breakpoint setting via QMP
- Trace log streaming
