# Archived: levitate-drivers-gpu

**Archived by:** TEAM_114
**Date:** 2026-01-05
**Reason:** Switching to `virtio-drivers::device::gpu::VirtIOGpu` for PCI migration

---

## Why Archived

1. **TEAM_107 Issues:** Custom driver had timeout issues that were undiagnosed
2. **PCI Migration:** Easier to use tested `virtio-drivers` GPU with `PciTransport`
3. **Lower Risk:** `virtio-drivers` is battle-tested, reduces maintenance burden

---

## Previous Purpose

This crate provided a custom VirtIO GPU driver implementation:
- `device.rs` - Full GPU device with MmioTransport integration
- `driver.rs` - Protocol state machine (GET_DISPLAY_INFO, RESOURCE_CREATE_2D, etc.)
- `protocol/` - VirtIO GPU protocol structures

---

## Restoration

If needed in future, restore by:
1. Move back to workspace root: `mv .archive/levitate-drivers-gpu ./`
2. Add to `Cargo.toml` workspace members
3. Add dependency in `kernel/Cargo.toml`

---

## Related

- `.questions/TEAM_107_gpu_driver_issues.md` - Documents the timeout bug
- `.questions/TEAM_114_gpu_driver_choice.md` - Decision to use virtio-drivers
