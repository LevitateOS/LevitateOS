# Next Night's Work - Iteration 41+

**Date**: 2026-02-04 evening
**Priority**: Phase 8 (CRITICAL) → Phase 10 (HIGH) → Phase 9 (LOW)

---

## 🔴 CRITICAL PRIORITY: Phase 8 - Redesign Install-Tests Harness (40 tasks)

**DO NOT "FIX" THE BROKEN HARNESS - THROW IT OUT AND BUILD NEW ONE**

### Why This Matters
- Install-tests has been broken for weeks - Console I/O buffering doesn't work
- Blocks verification of 8 installation tasks
- Can't test Phase 10 display support without working test harness
- User directive: "completely rewrite and completely redesign"

### Tasks (in order):

**8.1: Research alternatives (5 tasks)**
- Option A: QEMU QMP (JSON protocol for VM control)
- Option B: rexpect (Rust expect library)
- Option C: Network signaling (test agent in VM)
- Option D: Screenshot + OCR
- **Decision: Pick ONE and document why**

**8.2: Proof of Concept (7 tasks)**
- Create NEW crate: `testing/install-tests-v2/`
- Implement ONLY boot detection with chosen approach
- Test 10 times - must succeed 10/10
- If fails: STOP, pick different approach, retry
- Document PoC results in TEAM_154

**8.3: Implement new harness (7 tasks)**
- Design test API
- Implement VM lifecycle (start, boot, commands, shutdown)
- Implement test phases 1-6
- Add distro abstraction (AcornOS vs IuppiterOS)

**8.4: Migrate tests (7 tasks)**
- Port AcornOS Phases 1-6 tests to new harness
- Port IuppiterOS tests
- Throw away old implementation

**8.5: Replace old harness (7 tasks)**
- Rename old → `install-tests-OLD-BROKEN/`
- Promote new → `install-tests/`
- Delete old code entirely
- Update TEAM_154

**8.6: Verify (7 tasks)**
- Run all 8 previously BLOCKED tests
- Must pass 10/10 times each

**Success criteria**: Can run `cargo test -p install-tests` and ALL tests pass reliably

---

## 🟡 HIGH PRIORITY: Phase 10 - IuppiterOS Display Support (47 tasks)

**CORRECTION**: IuppiterOS was WRONG - it's GUI on touchscreen, NOT headless!

### Why This Matters
- IuppiterOS currently has ZERO display packages (wrong spec)
- Should boot to Cage compositor with iuppiter-dar fullscreen on touchscreen
- Missing: mesa, GTK, webkit2gtk, fonts, input handling (~300MB)

### Tasks (in order):

**10.1: Display Foundation (6 tasks)**
- Add mesa, mesa-dri-gallium, libdrm (GPU drivers)
- Add libinput, libxkbcommon (touchscreen input)
- Add fonts (dejavu, liberation)

**10.2: Wayland Compositor (7 tasks)**
- Add cage, wlroots, seatd packages
- Create OpenRC service /etc/init.d/iuppiter-kiosk
- Enable in default runlevel

**10.3: GTK + WebView (7 tasks)**
- Add gtk+3.0, webkit2gtk stack (~150MB)
- Add gstreamer for media playback

**10.4: iuppiter-dar Integration (7 tasks)**
- Build DAR: `cd /home/vince/Projects/iuppiter-dar && bun install && bun run build && cargo tauri build`
- Copy binary to /opt/iuppiter/iuppiter-dar
- Configure Cage to launch DAR

**10.5: Kiosk Lockdown (6 tasks)**
- Disable virtual terminals
- Disable compositor escapes
- Auto-restart DAR on crash

**10.6: Touchscreen Config (4 tasks)**
- Verify touchscreen udev rules
- Test touch input works

**10.7: Live vs Installed (4 tasks)**
- Data partition persistence

**10.8: E2E Verification (7 tasks)**
- Boot on real hardware with touchscreen
- Verify DAR works

**Success criteria**: Boot ISO, see DAR fullscreen on touchscreen, touch input works

---

## 🟢 LOW PRIORITY: Phase 9 - Custom Kernel (4 tasks)

**Only if time permits after Phase 8 + Phase 10**

- Build custom kernel from linux/ submodule
- Replace Alpine linux-lts in ISO
- Optimize config for refurbishment server

---

## Task Count

- **Total**: 150 tasks
- **Complete**: 97 (65%)
- **Blocked**: 8 (will unblock after Phase 8)
- **Remaining**: 45 (Phase 8: 40, Phase 10: 47, overlap: 42)

---

## Key Files

- **PRD**: `.ralph/prd.md` - Full task list with acceptance criteria
- **Progress**: `.ralph/progress.txt` - Iteration logs (40 iterations complete)
- **Requirements**: `.teams/TEAM_211_iuppiter-immutable-kiosk-dar-requirements.md`
- **Test blocker**: `.teams/TEAM_154_install-tests-broken-boot-detection.md`

---

## Expected Timeline

**Phase 8 (install-tests)**: 12-16 hours
- Research: 2-3h
- PoC: 3-4h
- Implementation: 4-6h
- Migration: 2-3h
- Verification: 1h

**Phase 10 (display)**: 12-19 hours
- Display stack: 4-6h
- Kiosk service: 2-3h
- DAR integration: 4-6h
- Lockdown: 2-4h

**Total**: 24-35 hours (can be parallelized if using multiple iterations)

---

## Success Criteria for Tomorrow Morning

**Minimum (acceptable):**
- Phase 8 complete: install-tests work reliably (10/10 runs)
- Phase 10 started: Display packages added, Cage compositor working

**Target (good):**
- Phase 8 complete: All 8 BLOCKED tasks now pass
- Phase 10 complete: IuppiterOS boots to DAR fullscreen on touchscreen

**Stretch (excellent):**
- Phase 8 complete
- Phase 10 complete
- Phase 9 started: Custom kernel building

---

## Notes for Ralph

1. **Phase 8 is CRITICAL** - Don't waste time "fixing" stdio buffering again. Pick a NEW approach.
2. **Phase 10 requires Phase 8** - Can't verify display support without working tests
3. **Read TEAM_211** for full display requirements and package lists
4. **iuppiter-dar source**: `/home/vince/Projects/iuppiter-dar` (Tauri app - build before installing)
5. **Don't commit build artifacts** - Both repos now have .gitignore

Good luck! 🚀
