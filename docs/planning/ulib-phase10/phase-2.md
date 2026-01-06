# Phase 10: Userspace Standard Library (`ulib`) — Phase 2: Design

**Team:** TEAM_164  
**Created:** 2026-01-06  
**Status:** APPROVED (TEAM_165 review)  
**Parent:** `phase-1.md`

---

## 1. Proposed Solution

### 1.1 High-Level Description

Create `ulib`, a new crate in `userspace/ulib/` that provides:
1. **Global Allocator** - Heap allocation backed by `sbrk` syscall
2. **File Abstractions** - `File`, `OpenOptions` wrapping kernel file syscalls
3. **Environment** - `args()` and `vars()` for argument/environment access
4. **Time** - `Instant`, `Duration`, `sleep()` for timing
5. **Error Handling** - `io::Error` and `Result` types

The library will be a dependency of all userspace applications, replacing direct use of `libsyscall` for common operations.

### 1.2 User-Facing Behavior

Applications will be able to:
```rust
use ulib::prelude::*;

fn main() {
    // Heap allocation works
    let mut v = Vec::new();
    v.push(42);
    
    // File operations
    let file = File::open("/etc/passwd")?;
    let contents = file.read_to_string()?;
    
    // Arguments
    for arg in args() {
        println!("arg: {}", arg);
    }
    
    // Time
    let start = Instant::now();
    sleep(Duration::from_millis(100));
    println!("elapsed: {:?}", start.elapsed());
}
```

### 1.3 System Behavior

**Allocator flow:**
1. `ulib` registers a global allocator at startup
2. Allocator maintains a free list of blocks
3. When heap exhausted, calls `sbrk(increment)` to get more pages
4. Kernel maps new pages into process address space

**File flow:**
1. `File::open()` calls `sys_openat(AT_FDCWD, path, flags, mode)`
2. Kernel looks up path in VFS (initramfs for now)
3. Returns file descriptor or error
4. `file.read()` calls `sys_read(fd, buf, len)`
5. `drop(file)` calls `sys_close(fd)`

---

## 2. API Design

### 2.1 Module Structure

```
ulib/
├── src/
│   ├── lib.rs           # Main entry, prelude
│   ├── alloc.rs         # Global allocator
│   ├── fs.rs            # File, OpenOptions, Metadata
│   ├── io.rs            # Read, Write traits, Error, Result
│   ├── env.rs           # args(), vars()
│   ├── time.rs          # Instant, Duration, sleep()
│   └── sys.rs           # Raw syscall wrappers (re-export libsyscall)
```

### 2.2 Core Types

```rust
// alloc.rs
pub struct LosAllocator { /* heap state */ }
unsafe impl GlobalAlloc for LosAllocator { ... }

// fs.rs
pub struct File { fd: i32 }
pub struct OpenOptions { flags: u32, mode: u32 }
pub struct Metadata { size: u64, is_dir: bool, ... }

// io.rs
pub struct Error { code: i32, message: &'static str }
pub type Result<T> = core::result::Result<T, Error>;
pub trait Read { fn read(&mut self, buf: &mut [u8]) -> Result<usize>; }
pub trait Write { fn write(&mut self, buf: &[u8]) -> Result<usize>; }

// env.rs
pub fn args() -> Args;
pub fn vars() -> Vars;

// time.rs
pub struct Instant { ticks: u64 }
pub struct Duration { nanos: u64 }
pub fn sleep(duration: Duration);
```

### 2.3 New Syscalls Required

| Syscall     | NR  | Signature | Priority |
|-------------|-----|-----------|----------|
| `sbrk`      | 4   | `(increment: isize) -> usize` | P0 (must fix stub) |
| `openat`    | 9   | `(dirfd, path, path_len, flags, mode) -> i32` | P1 |
| `close`     | 10  | `(fd) -> i32` | P1 |
| `fstat`     | 11  | `(fd, statbuf) -> i32` | P2 |
| `getdents`  | 12  | `(fd, buf, len) -> isize` | P2 |
| `nanosleep` | 13  | `(req, rem) -> i32` | P2 |
| `time`      | 14  | `() -> u64` | P3 |

---

## 3. Data Model Changes

### 3.1 New Kernel Data Structures

**Per-process heap tracking:**
```rust
struct ProcessHeap {
    base: usize,      // Start of heap region
    current: usize,   // Current program break
    max: usize,       // Maximum allowed heap size
}
```

**File descriptor table:**
```rust
struct FdTable {
    files: [Option<FileHandle>; MAX_FDS],
    next_fd: i32,
}

struct FileHandle {
    kind: FileKind,
    offset: usize,
    flags: u32,
}

enum FileKind {
    Console(ConsoleKind),   // stdin/stdout/stderr
    Initramfs { data: &'static [u8] },
    // Future: VFS node
}
```

### 3.2 Migration Strategy
- No migration needed for existing data
- New structures added to task/process state
- Existing fd 0/1/2 become explicit entries in FdTable

---

## 4. Behavioral Decisions (QUESTIONS)

### Q1: Heap Initial Size and Growth
**Question:** What should be the initial heap size and growth increment?

**Options:**
- A) Start with 0, grow by 4KB on first allocation
- B) Start with 64KB, grow by 64KB
- C) Start with 4KB, double on exhaustion

**Recommendation:** Option A - minimal start, grow as needed.

**Decision:** ✅ **A** — Per Rule 20 (Simplicity) and Rule 16 (Energy Awareness)

---

### Q2: File Descriptor Allocation Strategy
**Question:** How should file descriptors be allocated?

**Options:**
- A) Always use lowest available (POSIX behavior)
- B) Incrementing counter (simpler)
- C) Random (security benefit)

**Recommendation:** Option A - POSIX-compliant.

**Decision:** ✅ **A** — Per Rule 18 (Least Surprise)

---

### Q3: What Happens When Heap Exhausted?
**Question:** Behavior when sbrk cannot grow heap further?

**Options:**
- A) Return null pointer (let allocator handle OOM)
- B) Panic the process
- C) Block until memory available

**Recommendation:** Option A - standard allocator behavior.

**Decision:** ✅ **A** — Per Rule 14 (Fail Fast) and Rule 6 (Robust Error Handling)

---

### Q4: Read-Only Initramfs vs Writable Files
**Question:** Should opened initramfs files be writable (in memory)?

**Options:**
- A) Read-only only (initramfs is immutable)
- B) Copy-on-write into heap (allows modification)
- C) Defer until real filesystem

**Recommendation:** Option A - simplest, matches initramfs semantics.

**Decision:** ✅ **A** — Per Rule 20 (Simplicity) and Rule 11 (Separation)

---

### Q5: Argument Passing Mechanism
**Question:** How should argc/argv be passed to userspace?

**Options:**
- A) Stack-based (Linux ABI compatible)
- B) Special syscall `getargs()`
- C) Memory region at fixed address

**Recommendation:** Option A - follows spec, enables future compatibility.

**Decision:** ✅ **A** — Per Rule 18 (Least Surprise) and Rule 2 (Type-Driven Composition)

---

### Q6: Sleep Implementation
**Question:** How should `sleep()` be implemented?

**Options:**
- A) Busy loop with yield (works now)
- B) Timer-based wakeup (requires scheduler integration)
- C) Hybrid (yield with timeout check)

**Recommendation:** Option B for proper implementation, Option A as MVP fallback.

**Decision:** ✅ **B** — Per Rule 16 (Energy Awareness: "Race to Sleep") and Rule 9 (Non-blocking Design)

---

### Q7: Error Code Compatibility
**Question:** Should error codes match Linux errno values?

**Options:**
- A) Use Linux errno values (easier future compat)
- B) Continue custom error codes
- C) Map at library boundary

**Recommendation:** Option A - align with spec.

**Decision:** ✅ **A** — Per Rule 18 (Least Surprise) and Rule 3 (Expressive Interfaces)

---

## 5. Design Alternatives Considered

### 5.1 Full `std` Port
**Rejected because:** Requires too much infrastructure (threads, networking, full VFS). Can be a future goal.

### 5.2 Use `relibc` Directly
**Rejected because:** Heavy dependency, designed for Redox. Better to build minimal custom library.

### 5.3 Delay Until Full VFS
**Rejected because:** Blocks all userspace progress. Initramfs-only files are sufficient for MVP.

---

## 6. Open Questions Summary

| ID | Question | Options | Recommendation | Status |
|----|----------|---------|----------------|--------|
| Q1 | Heap initial size | 0/64KB/4KB | Start with 0 | ✅ **A** |
| Q2 | FD allocation | Lowest/Incrementing/Random | Lowest | ✅ **A** |
| Q3 | OOM behavior | Null/Panic/Block | Null | ✅ **A** |
| Q4 | Initramfs writability | RO/COW/Defer | RO | ✅ **A** |
| Q5 | Argument passing | Stack/Syscall/Fixed | Stack | ✅ **A** |
| Q6 | Sleep implementation | Busy/Timer/Hybrid | Timer | ✅ **B** |
| Q7 | Error codes | Linux/Custom/Map | Linux | ✅ **A** |

---

## 7. Dependencies on Phase 2 Completion

**All questions resolved by TEAM_165 (2026-01-06):**
1. ✅ All questions answered per `kernel-development.md` guidelines
2. ✅ API design approved
3. ✅ Syscall numbering confirmed (NR 9-14 for new syscalls)

---

## Next Steps

1. ~~Create question file in `.questions/TEAM_164_ulib_design.md`~~ ✅ Done
2. ~~User reviews and answers questions~~ ✅ Done (TEAM_165)
3. ~~Update this document with decisions~~ ✅ Done
4. **Proceed to Phase 3 (Implementation)** ← READY
