# LevitateOS Gotchas & Known Issues

This document captures non-obvious issues that future teams should know about.

---

## Critical Issues

### 1. Boot Hijack Prevents Interactive Mode (TEAM_081)

**Location:** `kernel/src/main.rs:612`

**Problem:** TEAM_073 added code to demo userspace execution that runs immediately and never returns. This prevents the kernel from reaching its main loop where keyboard input is processed.

**Symptom:** System boots, shows messages, then appears frozen. No typing works.

**Fix:**
```rust
// Comment out this line:
// task::process::run_from_initramfs("hello", &archive);
```

---

### 2. Userspace Linker Script Conflict (TEAM_082)

**Location:** `userspace/hello/linker.ld`

**Problem:** Root `.cargo/config.toml` adds `-Tlinker.ld` for all aarch64 builds. This conflicts with userspace's `link.ld`. Cargo merges configs, so both scripts are used, causing "Cannot allocate memory" errors.

**Fix:** Userspace directories need an empty `linker.ld` stub file to satisfy the root config without adding conflicting sections.

---

### 3. Dual Console Only Works After Stage 3 (TEAM_081)

**Location:** `levitate-hal/src/console.rs`

**Problem:** The GPU terminal callback is registered AFTER GPU initialization. Any `println!` calls BEFORE Stage 3 only go to UART, not GPU.

**Note:** This is by design, but can be confusing when debugging early boot.

---

### 4. GPU Display API Pattern (RESOLVED by TEAM_086)

**Location:** `kernel/src/gpu.rs` - `Display` struct

**Status:** ✅ FIXED

**Original Problem:** The `Display` struct locked `GPU` internally on every draw operation, causing deadlock when flush was called afterward.

**Solution:** TEAM_086 refactored `Display` to accept `&mut GpuState` instead of locking internally:

```rust
// CORRECT PATTERN:
let mut gpu_guard = gpu::GPU.lock();
if let Some(gpu_state) = gpu_guard.as_mut() {
    let mut display = Display::new(gpu_state);
    Text::new("Hello", Point::new(10, 30), style).draw(&mut display).ok();
    gpu_state.flush();  // Safe - still holding the same lock
}
```

**Key Points:**
- `Display::new()` now requires `&mut GpuState`
- All draw operations and flush happen within the same lock scope
- No more deadlock potential

---

### 5. IrqSafeLock is NOT Re-entrant (TEAM_083)

**Location:** `levitate-hal/src/lib.rs`

**Problem:** `IrqSafeLock` wraps a `Spinlock` and disables interrupts. It does NOT support re-entrant locking. If you hold the lock and try to lock again from the same context, you will deadlock.

**Affects:**
- GPU operations (Display + flush)
- Any nested lock scenarios
- Timer/interrupt handlers that try to print (if they use the same locks)

**Pattern to avoid:**
```rust
let guard1 = SOME_LOCK.lock();
let guard2 = SOME_LOCK.lock();  // DEADLOCK!
```

---

## Build Gotchas

### Userspace Binaries Need Separate Build

Userspace binaries are NOT part of the main workspace. Build them separately:

```bash
cd userspace/hello
cargo build --release
```

Then copy to initramfs:
```bash
cp target/aarch64-unknown-none/release/hello ../../initrd_root/
cd ../..
./scripts/make_initramfs.sh
```

---

## Testing Gotchas

### Golden Tests Are Stale

The behavior tests in `tests/golden_boot.txt` reflect the OLD boot sequence before TEAM_073's userspace changes. They will fail until updated.

---

## Runtime Gotchas

### No Visual Echo in Userspace

The shell code reads from stdin but doesn't echo characters back. Users type "blind" - they won't see what they're typing until the command executes.

### Keyboard Input Sources

There are TWO keyboard input sources:
1. **UART** - Serial console input via `console::read_byte()`
2. **VirtIO Keyboard** - GPU keyboard via `input::read_char()`

Both need to be checked for full input coverage.

---

## Adding New Gotchas

When you discover a non-obvious issue:
1. Add it to this file with your TEAM_XXX
2. Include: Location, Problem, Symptom, Fix
3. Leave breadcrumbs in the code too
