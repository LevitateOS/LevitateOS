# Questions: BusyBox Integration

**Team:** TEAM_449  
**Feature:** BusyBox Integration  
**Status:** Answered

---

## Q1: Shell Exit Behavior

**Context:** When the user types `exit` in the shell, what should happen?

**Options:**
- **A) Respawn shell** - Shell restarts (BusyBox default with `::respawn` in inittab)
- **B) Halt/shutdown** - System stops
- **C) Configurable** - Use inittab to control behavior

**Recommendation:** A (respawn) - matches current behavior, safe default

**Your Answer:** **A (Respawn shell)**

---

## Q2: Procfs/Sysfs Mounting

**Context:** Should BusyBox init automatically mount `/proc` and `/sys`?

**Impact:**
- Needed for: `ps` (process list), `top`, `mount` display
- Requires: Kernel procfs/sysfs support

**Options:**
- **A) Yes** - Mount in init script (standard Linux behavior)
- **B) No** - Leave to user (simpler, but `ps` won't work fully)

**Note:** If kernel doesn't support procfs/sysfs yet, we can add mount commands but they'll fail silently until kernel support is added.

**Recommendation:** A (yes) - standard behavior, commands will just fail gracefully if kernel doesn't support yet

**Your Answer:** **A (Yes - mount in init)**

---

## Q3: Test Mode Handling

**Context:** Current init runs `eyra-test-runner` if present for automated testing.

**Options:**
- **A) Separate test initramfs** - Different initramfs for testing
- **B) Init script check** - BusyBox init checks for test binary
- **C) Remove automated testing** - Manual testing only

**Recommendation:** A (separate initramfs) - cleaner separation, no impact on production boot

**Your Answer:** **A (Separate test initramfs)**

---

## Q4: BusyBox Applet Selection

**Context:** BusyBox can include ~300 applets. More applets = larger binary.

**Options:**
- **A) Full** - All applets (~1.5MB binary)
- **B) Standard** - Common utilities (~1MB binary) - what I've planned
- **C) Minimal** - Just shell + basic coreutils (~500KB)

**Recommendation:** B (standard) - good balance of functionality and size

**Your Answer:** **B (Standard ~1MB)**

---

## Q5: Include vi Editor?

**Context:** vi adds ~50KB but provides file editing capability.

**Options:**
- **A) Yes** - Include vi
- **B) No** - Save space, use `cat >` for simple edits

**Recommendation:** A (yes) - very useful for debugging/development

**Your Answer:** **A (Yes - include vi)**

---

## Summary

| Question | Recommendation | Your Choice |
|----------|---------------|-------------|
| Q1: Shell exit | A (respawn) | A |
| Q2: Mount proc/sys | A (yes) | A |
| Q3: Test mode | A (separate) | A |
| Q4: Applets | B (standard) | B |
| Q5: vi editor | A (yes) | A |

**All questions answered. Phase 3 can proceed.**
