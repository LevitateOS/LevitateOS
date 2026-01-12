# TEAM_447: Implement Shell Prerequisites

## Status: IN PROGRESS

## Objective
Implement the missing gaps from `docs/planning/shell-prerequisites.md` to enable full shell functionality.

## Priority Order

### Critical (Blocks Job Control)
1. **Signal Delivery** - `check_and_deliver_signals()` is a no-op
   - Push signal frame to user stack
   - Save current registers for `sigreturn()`
   - Redirect PC to handler address
   - Handle SA_SIGINFO, SA_RESTART flags

### Important
2. **setpgid for other processes** - Currently only works for self
3. **fchdir** - Change directory by fd
4. **TIOCSWINSZ** - Set terminal window size

## Progress Log

### Session 1
- [ ] Verify test baseline
- [ ] Implement signal delivery
- [ ] Implement setpgid for other processes
- [ ] Implement fchdir
- [ ] Implement TIOCSWINSZ
- [ ] Test all changes

## Files to Modify
- `crates/kernel/levitate/src/main.rs` - Signal delivery
- `crates/kernel/syscall/src/signal.rs` - Signal delivery logic
- `crates/kernel/syscall/src/process/identity.rs` - setpgid
- `crates/kernel/syscall/src/fs/fd.rs` - fchdir, TIOCSWINSZ
