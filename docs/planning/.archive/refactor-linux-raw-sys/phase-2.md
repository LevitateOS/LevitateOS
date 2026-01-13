# Phase 2 — Add Dependency and Delete Hardcoded Constants

**Refactor:** Migrate to linux-raw-sys  
**Team:** TEAM_419  
**Date:** 2026-01-10

---

## Purpose

Add `linux-raw-sys` dependency and DELETE all hardcoded constants. Let the compiler scream.

---

## Step 1 — Add linux-raw-sys Dependency

**File:** `crates/kernel/Cargo.toml`

```toml
[dependencies]
linux-raw-sys = { version = "0.9", default-features = false, features = ["general", "errno", "ioctl"] }
```

**Exit Criteria:** Dependency added, `cargo check` runs (will have errors)

---

## Step 2 — Delete syscall/constants.rs

**Action:** Delete entire file `crates/kernel/src/syscall/constants.rs`

**Remove from `syscall/mod.rs`:**
```rust
// DELETE THIS LINE:
pub mod constants;
```

**Exit Criteria:** File deleted, compiler errors about missing constants

---

## Step 3 — Delete Hardcoded Constants from Other Files

### 3.1 Delete from `syscall/mod.rs` (fcntl module)
Delete the entire `pub mod fcntl { ... }` block.

### 3.2 Delete from `syscall/errno` module  
Keep module but replace constants with linux-raw-sys imports.

### 3.3 Delete from `fs/mode.rs`
Delete all `S_IF*` and permission constants.

### 3.4 Delete from `fs/vfs/file.rs`
Delete `O_*` constants from `OpenFlags`.

### 3.5 Delete from `syscall/mm.rs`
Delete `PROT_*` and `MAP_*` constants.

### 3.6 Delete from `syscall/epoll.rs`
Delete `EPOLL_*` and `EPOLLIN`, etc. constants.

### 3.7 Delete from `syscall/signal.rs`
Delete `SIGINT`, `SIGKILL`, etc.

### 3.8 Delete from `arch/*/mod.rs`
Delete TTY/termios constants (`TCGETS`, `TIOCGWINSZ`, etc.)

---

## Expected Compiler Errors

After deletion, expect ~100+ errors like:
```
error[E0433]: failed to resolve: use of undeclared crate or module `constants`
error[E0425]: cannot find value `CLONE_VM` in this scope
error[E0425]: cannot find value `AT_FDCWD` in this scope
error[E0425]: cannot find value `S_IFREG` in this scope
```

**This is intentional.** The compiler is our friend.

---

## Exit Criteria for Phase 2

- [ ] `linux-raw-sys` dependency added
- [ ] All hardcoded constants deleted
- [ ] Compiler produces errors (expected)
- [ ] No shims or compatibility layers created
