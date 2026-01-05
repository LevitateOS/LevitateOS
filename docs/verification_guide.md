# TEAM_021: Manual Verification Guide

This guide defines the manual verification steps required to validate the LevitateOS kernel in QEMU, since many hardware interactions cannot be fully covered by unit tests.

## Prerequisites
- **Target**: `aarch64-unknown-none` toolchain
- **Emulator**: QEMU (`qemu-system-aarch64`)
- **Script**: `./run.sh` (builds and runs the kernel)

---

## 1. Boot Verification

**Goal**: Ensure the kernel initializes all subsystems without hanging.

**Procedure**:
1. Run `./run.sh`.
2. Observe the serial output (terminal).

**Expected Output**:
```text
*** ClaudeOS Rust Kernel ***
Heap initialized.
Exceptions initialized.
Core drivers initialized.
Initializing Timer...
Timer frequency (hex): ...
Timer initialized.
...
Interrupts enabled.
```

**Failure Modes**:
- Hang after `Heap initialized`: Likely MMU initialization failure or `core::fmt` hang.
- Instruction Abort (ESR 0x8600000X): `PXN` bit set on kernel code pages.
- Sync Exception (ESR 0x960000XX): Data Abort (invalid mapping or unaligned access).

---

## 2. Subsystem Benchmarks

### 2.1 MMU (Memory Management)
- **Check**: Look for `MMU initialized and enabled (identity mapped)`.
- **Validation**: If this prints, the kernel has successfully transitioned from physical to virtual addressing (identity map).
- **Edge Case**: If it hangs *immediately* after this, check `PXN` (Privileged Execute Never) flags on the kernel code region.

### 2.2 Timer & Interrupts
- **Check**: Look for `Timer initialized`.
- **Validation**: Wait 5-10 seconds. The specific `Timer frequency` (usually `0x3b9aca0` = ~62.5MHz) should be visible.
- **IRQ Check**: Press keys on the keyboard. If the system stays responsive (e.g., cursor moves), IRQs are handled correctly. If it hangs, the IRQ handler might have deadlocked.

### 2.3 Graphics (VirtIO GPU)
- **Check**: A QEMU window should appear.
- **Validation**:
  - Blue background (cleared framebuffer).
  - Red rectangle at (100, 100).
  - White mouse cursor.

### 2.4 Input (VirtIO Tablet)
- **Check**: Move your host mouse over the QEMU window.
- **Validation**: The white cursor inside the QEMU window should track your movement.
- **Note**: If the cursor is "jerky" or offset, ensure `-device virtio-tablet-device` is used instead of `virtio-mouse-device`.

---

---

## 4. Agentic Verification (Log Capture) (TEAM_120)

When verifying the system in an automated or agentic environment where a real-time serial terminal might not be available or persistent, use the "Background Capture" technique.

### Technique: Capturing Boot Logs
This technique runs the system in the background, waits for a sufficient time (e.g., 10 seconds), and captures all output to a file for later analysis.

**Command:**
```bash
cargo xtask run > boot_output.txt 2>&1 & sleep 10 && kill $! || true
```

**Validation:**
1. Check `boot_output.txt` for the expected milestones (e.g., `Starting init`, `LevitateOS Shell`).
2. This allows for precise string-based verification of the boot sequence without manual observation.

### Artifact Verification
Always check updated `tests/golden_boot.txt` after making changes to the boot sequence or log format. Run behavior tests to ensure regressions are caught:
```bash
cargo xtask test behavior
```
