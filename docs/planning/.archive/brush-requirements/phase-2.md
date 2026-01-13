# Phase 2: Root Cause Analysis

**Bug:** Brush shell crash - rt_sigaction format mismatch  
**Team:** TEAM_438  
**Parent:** `docs/planning/brush-requirements/`

## Hypotheses

### H1: rt_sigaction Argument Format Mismatch ⭐ CONFIRMED

**Confidence:** HIGH (95%)

**Evidence:**
1. Linux `rt_sigaction` syscall signature:
   ```c
   int rt_sigaction(int signum, 
                    const struct sigaction *act,    // POINTER
                    struct sigaction *oldact,       // POINTER
                    size_t sigsetsize);
   ```

2. Our implementation signature:
   ```rust
   pub fn sys_sigaction(sig: i32, handler_addr: usize, restorer_addr: usize)
   ```

3. Brush/tokio calls `rt_sigaction` with struct pointers
4. We interpret pointer value as handler address → garbage stored
5. Signal setup fails → async runtime panics → ud2

### H2: Missing sigaction Struct Fields

**Confidence:** HIGH (90%)

**Evidence:**
1. Linux sigaction struct has:
   - `sa_handler` or `sa_sigaction` (function pointer)
   - `sa_mask` (sigset_t - 64-bit signal mask)
   - `sa_flags` (SA_SIGINFO, SA_RESTORER, SA_RESTART, etc.)
   - `sa_restorer` (signal trampoline)

2. Our implementation only stores handler address, ignores:
   - `sa_mask` (we use 32-bit, should be 64-bit)
   - `sa_flags` (not parsed from struct)
   - `sa_restorer` (not read from struct)

### H3: Signal Mask Size Mismatch

**Confidence:** MEDIUM (70%)

**Evidence:**
1. Linux uses 64-bit `sigset_t` (supports signals 1-64)
2. Our `blocked_signals` is 32-bit `AtomicU32`
3. `rt_sigaction` fourth argument is `sigsetsize` (typically 8 bytes)

## Key Code Areas

### Primary: Signal Syscall Handler

**File:** `crates/kernel/syscall/src/signal.rs`
**Functions:**
- `sys_sigaction()` - Lines 70-86 (WRONG FORMAT)
- `sys_sigprocmask()` - Lines 206-251 (may also have issues)

### Secondary: Syscall Dispatcher

**File:** `crates/kernel/syscall/src/lib.rs`
**Functions:**
- Dispatcher passes wrong arguments to `sys_sigaction`

### Tertiary: Task Signal State

**File:** `crates/kernel/sched/src/lib.rs`
**Fields:**
- `signal_handlers: Mutex<[usize; 32]>` - handler storage
- `blocked_signals: AtomicU32` - should be 64-bit
- `signal_trampoline: AtomicUsize` - restorer storage

## Investigation Strategy

### Step 1: Map the Execution Path ✅

Tokio signal flow:
1. `tokio::signal::unix::signal(SignalKind::child())` called
2. Uses `signal-hook-registry` crate internally
3. Calls `libc::sigaction()` → becomes `rt_sigaction` syscall
4. Passes `struct sigaction` pointer as arg1
5. Our kernel interprets pointer as handler address

### Step 2: Narrow Down Faulty Region ✅

**Confirmed location:** `crates/kernel/syscall/src/signal.rs:70-86`

The function signature is fundamentally wrong:
```rust
pub fn sys_sigaction(sig: i32, handler_addr: usize, restorer_addr: usize)
```

Should be:
```rust
pub fn sys_sigaction(sig: i32, act_ptr: usize, oldact_ptr: usize, sigsetsize: usize)
```

### Step 3: Validate Hypotheses ✅

**H1 Validated:** Analyzed brush source code, confirmed tokio uses standard sigaction.

**H2 Validated:** Linux sigaction struct requires reading from userspace pointer.

**H3 Partially Validated:** 64-bit sigset_t needed but not critical for initial fix.

## Root Cause Summary

**Root Cause:** `sys_sigaction` interprets struct pointers as direct values.

When tokio calls `rt_sigaction(SIGCHLD, &act, &oldact, 8)`:
- `act` is a pointer to sigaction struct (e.g., `0x7ffffffed000`)
- We store `0x7ffffffed000` as the signal handler address
- This is garbage - not a valid handler function
- When signal setup fails, tokio's async runtime panics
- Panic handler jumps to ud2 (abort)

---

## Phase 2 Status: COMPLETE

Root cause confirmed. Ready to proceed to Phase 3: Fix Design.
