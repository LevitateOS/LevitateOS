# TEAM_419: Refactor to linux-raw-sys

## Status: Plan Complete
## Started: 2026-01-10

## Objective
Replace all hardcoded Linux ABI constants in the kernel with `linux-raw-sys` crate.

**Approach:** No shims, no backward compatibility. Delete hardcoded constants, let compiler fail, fix all call sites.

## Planning Documents
**Location:** `docs/planning/refactor-linux-raw-sys/`

| Phase | File | Description |
|-------|------|-------------|
| 1 | `phase-1.md` | Discovery - map all constants to replace |
| 2 | `phase-2.md` | Add dependency, DELETE all hardcoded constants |
| 3 | `phase-3.md` | Fix all compiler errors with linux-raw-sys imports |
| 4 | `phase-4.md` | Cleanup and verification |

## Constants to Replace (~100+)

| Category | Count | Source Files |
|----------|-------|--------------|
| CLONE_* | 9 | syscall/constants.rs |
| RLIMIT_* | 11 | syscall/constants.rs |
| AT_* | 6 | syscall/mod.rs (fcntl) |
| S_IF* | 8 | fs/mode.rs |
| O_* | 10 | fs/vfs/file.rs |
| PROT_*/MAP_* | 8 | syscall/mm.rs |
| EPOLL_* | 10 | syscall/epoll.rs |
| SIG* | 4+ | syscall/signal.rs |
| TCGETS/TIOC* | 10+ | arch/*/mod.rs |
| errno | 20+ | syscall/mod.rs |

## Files to Delete
- `crates/kernel/src/syscall/constants.rs` (created by TEAM_418, now obsolete)
- `syscall/mod.rs` fcntl module (inline constants)

## Dependency to Add
```toml
linux-raw-sys = { version = "0.9", default-features = false, features = ["general", "errno", "ioctl"] }
```
