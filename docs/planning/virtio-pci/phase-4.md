# Phase 4: Integration and Testing - VirtIO PCI Migration

**Feature:** VirtIO PCI Transport for GPU
**Team:** TEAM_113

## 1. Test Strategy
Since we cannot easily mock QEMU's PCI behavior in unit tests, verification will rely on Integration Tests and Manual/Visual verification.

## 2. Integration Tests
- **Boot Test:** standard `cargo xtask test behavior` should pass (it verifies kernel boots).
- **PCI Enumeration Log:** Kernel logs should show:
  > [PCI] Scanning Bus 0...
  > [PCI] Found device 1AF4:1050 at 00:02.0

## 3. Manual Verification
- **Run VNC:** `cargo xtask run-vnc`
- **Criteria:**
  - QEMU window opens (or VNC connects).
  - Screen is **NOT** black/error message.
  - Screen shows the purple test pattern or the terminal output.

## 4. Impact Analysis
- **Performance:** PCI BAR access is uncached (Device memory). Framebuffer flush might be slower than WB-Cacheable + Flush, but it is *correct*.
- **Boot Time:** Scanning 256 buses might be slow if not optimized. We can limit scan to Bus 0 for now as `virt` is simple.
