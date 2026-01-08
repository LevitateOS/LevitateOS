# Phase 4: Integration and Testing - VirtIO PCI Migration

**Feature:** VirtIO PCI Transport for GPU
**Team:** TEAM_113 (created), TEAM_114 (revised)

## 1. Test Strategy
Verification relies on Integration Tests and Manual/Visual verification.

## 2. Integration Tests
- **Boot Test:** `cargo xtask test behavior` should pass
- **PCI Enumeration Log:** Kernel logs should show:
  ```
  [PCI] Scanning Bus 0...
  [PCI] Found VirtIO GPU at 00:XX.0
  [GPU] Initialized via PCI transport
  ```

## 3. Manual Verification
- **Run VNC:** `cargo xtask run-vnc`
- **Criteria:**
  - QEMU window opens (or VNC connects)
  - Screen shows content (NOT "Display output is not active")
  - Purple test pattern or terminal output visible

## 4. Impact Analysis
- **Performance:** PCI BAR access is uncached - correct behavior
- **Boot Time:** Only scan Bus 0 (QEMU virt is simple)

## 5. Handoff Checklist (Rule 10)
- [ ] Project builds cleanly
- [ ] `cargo xtask test behavior` passes
- [ ] GPU displays content in QEMU
- [ ] Team file updated with results
- [ ] Dead MMIO GPU code removed (Rule 6)

## 6. Rollback Plan
If PCI GPU fails:
1. Revert QEMU flags to `virtio-gpu-device`
2. GPU will show "DISABLED" message (current stub)
3. Investigate PCI issues separately
