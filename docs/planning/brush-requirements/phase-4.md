# Phase 4: Implementation and Tests

**Bug:** Brush shell crash - rt_sigaction format mismatch  
**Team:** TEAM_438  
**Parent:** `docs/planning/brush-requirements/`

## Implementation Overview

Rewrite `sys_sigaction` to properly parse Linux sigaction structs from userspace pointers.

---

## Step 1: Add sigaction Struct Definition

**File:** `crates/kernel/syscall/src/signal.rs`

**UoW 1.1:** Add struct definition and constants

```rust
/// Linux sigaction struct layout for x86_64
/// Total size: 32 bytes
#[repr(C)]
struct KernelSigaction {
    sa_handler: usize,      // offset 0: handler or SIG_IGN/SIG_DFL
    sa_flags: u64,          // offset 8: flags (SA_RESTORER, SA_SIGINFO, etc.)
    sa_restorer: usize,     // offset 16: signal trampoline (if SA_RESTORER)
    sa_mask: u64,           // offset 24: 64-bit signal mask
}

// Signal action constants
const SIG_DFL: usize = 0;
const SIG_IGN: usize = 1;

// sigaction flags
const SA_RESTORER: u64 = 0x04000000;
const SA_SIGINFO: u64 = 0x00000004;
const SA_RESTART: u64 = 0x10000000;
const SA_NODEFER: u64 = 0x40000000;
```

---

## Step 2: Update Task Signal Storage

**File:** `crates/kernel/sched/src/lib.rs`

**UoW 2.1:** Expand signal handler storage to include flags

Current:
```rust
pub signal_handlers: Mutex<[usize; 32]>,
pub signal_trampoline: AtomicUsize,
```

New:
```rust
pub signal_handlers: Mutex<[SignalAction; 64]>,
```

Where `SignalAction` is:
```rust
#[derive(Clone, Copy, Default)]
pub struct SignalAction {
    pub handler: usize,
    pub flags: u64,
    pub restorer: usize,
    pub mask: u64,
}
```

---

## Step 3: Rewrite sys_sigaction

**File:** `crates/kernel/syscall/src/signal.rs`

**UoW 3.1:** New function signature and implementation

```rust
/// TEAM_438: Proper rt_sigaction implementation with struct parsing
pub fn sys_sigaction(
    sig: i32,
    act_ptr: usize,
    oldact_ptr: usize,
    sigsetsize: usize,
) -> SyscallResult {
    // 1. Validate signal number
    if sig < 1 || sig >= 64 {
        return Err(EINVAL);
    }
    // SIGKILL and SIGSTOP cannot have handlers
    if sig == 9 || sig == 19 {
        return Err(EINVAL);
    }
    
    // 2. Validate sigsetsize (must be 8 for 64-bit sigset_t)
    if sigsetsize != 8 {
        return Err(EINVAL);
    }
    
    let task = current_task();
    let ttbr0 = task.ttbr0;
    
    // 3. If oldact_ptr is provided, write current action
    if oldact_ptr != 0 {
        let handlers = task.signal_handlers.lock();
        let old_action = &handlers[sig as usize];
        // Write 32-byte struct to userspace
        write_sigaction_to_user(ttbr0, oldact_ptr, old_action)?;
    }
    
    // 4. If act_ptr is provided, read and store new action
    if act_ptr != 0 {
        let new_action = read_sigaction_from_user(ttbr0, act_ptr)?;
        let mut handlers = task.signal_handlers.lock();
        handlers[sig as usize] = new_action;
        
        // If SA_RESTORER is set, also store the trampoline globally
        if new_action.flags & SA_RESTORER != 0 {
            task.signal_trampoline.store(new_action.restorer, Ordering::Release);
        }
    }
    
    Ok(0)
}
```

**UoW 3.2:** Helper functions for userspace struct I/O

```rust
fn read_sigaction_from_user(ttbr0: usize, ptr: usize) -> Result<SignalAction, u32> {
    // Read 32 bytes from userspace
    let mut bytes = [0u8; 32];
    for i in 0..32 {
        match crate::read_from_user(ttbr0, ptr + i) {
            Some(b) => bytes[i] = b,
            None => return Err(EFAULT),
        }
    }
    
    // Parse struct fields (little-endian)
    Ok(SignalAction {
        handler: u64::from_le_bytes(bytes[0..8].try_into().unwrap()) as usize,
        flags: u64::from_le_bytes(bytes[8..16].try_into().unwrap()),
        restorer: u64::from_le_bytes(bytes[16..24].try_into().unwrap()) as usize,
        mask: u64::from_le_bytes(bytes[24..32].try_into().unwrap()),
    })
}

fn write_sigaction_to_user(ttbr0: usize, ptr: usize, action: &SignalAction) -> Result<(), u32> {
    let mut bytes = [0u8; 32];
    bytes[0..8].copy_from_slice(&(action.handler as u64).to_le_bytes());
    bytes[8..16].copy_from_slice(&action.flags.to_le_bytes());
    bytes[16..24].copy_from_slice(&(action.restorer as u64).to_le_bytes());
    bytes[24..32].copy_from_slice(&action.mask.to_le_bytes());
    
    for i in 0..32 {
        if !crate::write_to_user_buf(ttbr0, ptr, i, bytes[i]) {
            return Err(EFAULT);
        }
    }
    Ok(())
}
```

---

## Step 4: Update Syscall Dispatcher

**File:** `crates/kernel/syscall/src/lib.rs`

**UoW 4.1:** Fix dispatcher to pass 4 arguments

Current:
```rust
Some(SyscallNumber::SigAction) => signal::sys_sigaction(
    frame.arg0() as i32,
    frame.arg1() as usize,
    frame.arg2() as usize,
),
```

New:
```rust
Some(SyscallNumber::SigAction) => signal::sys_sigaction(
    frame.arg0() as i32,
    frame.arg1() as usize,
    frame.arg2() as usize,
    frame.arg3() as usize,  // sigsetsize
),
```

---

## Step 5: Update sigprocmask for 64-bit Masks

**File:** `crates/kernel/syscall/src/signal.rs`

**UoW 5.1:** Update blocked_signals to 64-bit

This is a lower priority change. Initial fix can keep 32-bit mask.

---

## Step 6: Verification

**UoW 6.1:** Run tests

```bash
# Unit tests
cargo test --workspace

# Build and run
cargo xtask build kernel
cargo xtask build iso
timeout 15 cargo xtask run --term --headless 2>&1 | grep -E "sigaction|EXCEPTION"
```

**UoW 6.2:** Verify brush progress

Expected: Brush should get past signal setup and make more syscalls before any crash.

---

## Execution Order

| Step | UoW | Description | Size |
|------|-----|-------------|------|
| 1 | 1.1 | Add struct definition | Small |
| 2 | 2.1 | Update task signal storage | Medium |
| 3 | 3.1 | Rewrite sys_sigaction | Medium |
| 3 | 3.2 | Add helper functions | Small |
| 4 | 4.1 | Update dispatcher | Small |
| 6 | 6.1 | Run tests | Small |
| 6 | 6.2 | Verify brush | Small |

Total: ~5-6 small-medium UoWs, can be done in 1-2 sessions.

---

## Phase 4 Status: READY FOR IMPLEMENTATION

All UoWs defined. Ready to execute.
