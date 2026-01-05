# Phase 4: Cleanup

## Dead Code Removal
- Remove `virtio-drivers` from `levitate-gpu/Cargo.toml` if it's no longer used.
- Remove old `gpu.rs` logic that wraps the external crate.

## Encapsulation
- ensure `GpuDriver` fields are private.
- Move protocol constants to an internal `protocol` module.

## File Size Check
- Ensure `levitate-gpu` remains modular and readable.
