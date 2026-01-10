# TEAM_355: Investigate x86_64 VM Exec Not Reaching Shell

**Status:** ✅ RESOLVED  
**Bug:** x86_64 VM exec testing doesn't reach shell prompt  
**Date:** 2025-01-09

## Root Cause

**PCI BAR allocation conflict** - The `PciMemoryAllocator` was recreated fresh for each `find_virtio_device()` call, causing GPU and Input devices to receive **overlapping BAR addresses**.

When `virtio-keyboard-pci` was present:
1. GPU allocated BARs starting at `PCI_MEM32_PA`
2. Input device ALSO allocated BARs starting at `PCI_MEM32_PA` (same address!)
3. Input BAR writes corrupted GPU memory regions
4. System hung after INPUT init due to GPU corruption

## Fix

Made the PCI memory allocator global/static (`crates/pci/src/lib.rs`):
- Replaced `PciMemoryAllocator` struct with atomic `PCI_MEM_NEXT` counter
- `pci_allocate()` now uses compare-exchange for thread-safe allocation
- BAR addresses are now properly sequential across devices

## Prior Work (Different Issue)

Previous TEAM investigations (296, 297, 301) were for a **different bug** - an INVALID OPCODE crash during shell execution. That issue was about syscall return path corruption.

This bug was about **boot hang before shell** - specifically the system stopping at `[INPUT] VirtIO Input initialized via PCI.` and never reaching `Detecting Initramfs...`.

## Verification

| Test | Result |
|------|--------|
| `qemu ... -device virtio-gpu-pci -device virtio-keyboard-pci` | ✅ Shell boots |
| `cargo xtask vm exec --arch x86_64 "echo hello"` | ✅ Works |
| `cargo xtask test behavior --arch x86_64` | ✅ Passes |

## Files Changed

- `crates/pci/src/lib.rs` - Made PCI BAR allocator global/atomic

## Handoff Checklist

- [x] Project builds cleanly
- [x] Behavior tests pass
- [x] vm exec works for x86_64
- [x] Team file updated
- [x] Golden logs updated (SILVER MODE)

