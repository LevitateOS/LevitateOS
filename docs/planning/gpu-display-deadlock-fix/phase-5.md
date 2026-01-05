# Phase 5 — Cleanup, Regression Protection, and Handoff

**Bug:** GPU Display Deadlock  
**Team:** TEAM_084  
**Status:** PENDING (execute after Phase 4)

---

## 5.1 Cleanup Tasks

### Task 1: Remove Breadcrumbs
- [ ] Remove BREADCRUMB comment from `kernel/src/gpu.rs` (lines 100-103)
- [ ] Remove any other investigation breadcrumbs added during debugging

### Task 2: Update Documentation
- [ ] Update `docs/GOTCHAS.md` — Mark GPU Display deadlock as RESOLVED
- [ ] Update `TODO.md` — Move GPU deadlock to Completed section
- [ ] Update `docs/planning/interactive-shell-phase8b/POSTMORTEM.md` — Note fix

### Task 3: Re-enable Dual Console
- [ ] Locate where TEAM_083 disabled dual console callback
- [ ] Re-enable the GPU output callback in `kernel/src/main.rs`
- [ ] Verify both UART and GPU receive output

### Task 4: Code Cleanup
- [ ] Remove any dead code left from workarounds
- [ ] Ensure no `#[allow(dead_code)]` on actively used functions
- [ ] Remove temporary debug logging if any was added

---

## 5.2 Regression Protection

### Behavioral Invariants to Protect

| Invariant | How to Verify |
|-----------|---------------|
| GPU drawing doesn't deadlock | Boot kernel, observe GPU output |
| Dual console works | Both UART and GPU show same messages |
| Cursor blinks | Watch QEMU window for blinking cursor |
| Scroll works | Fill terminal with text, verify scrolling |

### Documentation for Future Teams

Add to `docs/GOTCHAS.md`:
```markdown
### GPU Display API Pattern (RESOLVED)

When using embedded_graphics with the GPU, always manage the lock scope yourself:

```rust
let mut gpu_guard = GPU.lock();
if let Some(state) = gpu_guard.as_mut() {
    let mut display = Display::new(state);
    // Draw operations...
    Text::new("Hello", point, style).draw(&mut display).ok();
    // Flush within same scope
    state.flush();
}
```

Do NOT create Display outside of a GPU lock scope.
```

---

## 5.3 Handoff Checklist

Before closing this bugfix:

- [ ] All Phase 4 UoWs complete
- [ ] `cargo build` succeeds
- [ ] `cargo test` passes
- [ ] Kernel boots without deadlock
- [ ] Dual console output verified
- [ ] Cursor blinking verified
- [ ] Documentation updated
- [ ] Breadcrumbs removed
- [ ] Team file updated with final status
- [ ] TODO.md updated

---

## 5.4 Team File Update

Update `.teams/TEAM_084_gpu_deadlock_investigation.md` with:
- Final status: COMPLETE
- Summary of fix implemented
- Verification results
- Any remaining notes

---

**Phase 5 Complete when all checkboxes are checked.**
