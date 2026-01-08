# Phase 3: Migration

**Parent:** [README.md](./README.md)  
**Depends:** Phase 2 complete  
**Status:** Planned

---

## Migration Strategy

### Approach: Atomic Cutover

Once new drivers are extracted and verified:
1. Switch kernel to use new driver crates
2. Remove old crate dependencies
3. Delete old crates

### Order of Migration

1. GPU (highest risk, most complex)
2. Block (well-tested, critical for FS)
3. Net (lower priority)
4. Input (lowest risk)

---

## Call Site Inventory

### GPU Call Sites

| Location | Usage | Migration |
|----------|-------|-----------|
| `kernel/src/gpu.rs` | GpuState, Display | Switch to levitate-drivers-gpu |
| `kernel/src/terminal.rs` | Display wrapper | Update import |
| `kernel/src/main.rs` | GPU init, flush | Update import |
| `kernel/src/virtio.rs` | init_gpu() | Update to use new driver |

### Block Call Sites

| Location | Usage | Migration |
|----------|-------|-----------|
| `kernel/src/block.rs` | VirtIOBlk wrapper | Replaced by crate |
| `kernel/src/fs/*.rs` | Block device access | Use levitate-drivers-blk |

### Net Call Sites

| Location | Usage | Migration |
|----------|-------|-----------|
| `kernel/src/net.rs` | VirtIONet wrapper | Replaced by crate |

### Input Call Sites

| Location | Usage | Migration |
|----------|-------|-----------|
| `kernel/src/input.rs` | VirtIOInput wrapper | Replaced by crate |
| `kernel/src/main.rs` | input::poll() | Update import |

---

## Steps

### Step 1: Migrate GPU
**File:** `phase-3-step-1.md`

Switch from levitate-gpu to levitate-drivers-gpu

Tasks:
1. Update kernel/Cargo.toml - remove levitate-gpu, add levitate-drivers-gpu
2. Update kernel/src/gpu.rs imports
3. Update kernel/src/terminal.rs imports
4. Update kernel/src/virtio.rs to use new driver
5. Run tests - verify golden boot passes
6. Verify graphical output in QEMU

---

### Step 2: Migrate Block
**File:** `phase-3-step-2.md`

Switch to levitate-drivers-blk

Tasks:
1. Update kernel/Cargo.toml
2. Update kernel/src/fs/ to use new crate
3. Delete kernel/src/block.rs (now in crate)
4. Run tests

---

### Step 3: Migrate Net
**File:** `phase-3-step-3.md`

Switch to levitate-drivers-net

Tasks:
1. Update kernel/Cargo.toml
2. Delete kernel/src/net.rs
3. Update any net-related code in main.rs
4. Run tests

---

### Step 4: Migrate Input
**File:** `phase-3-step-4.md`

Switch to levitate-drivers-input

Tasks:
1. Update kernel/Cargo.toml
2. Delete kernel/src/input.rs
3. Update main.rs to use new crate
4. Run tests

---

### Step 5: Migrate Filesystem
**File:** `phase-3-step-5.md`

Switch to levitate-fs

Tasks:
1. Update kernel/Cargo.toml - remove embedded-sdmmc, ext4-view
2. Update kernel/src/fs/ to use levitate-fs
3. Run tests

---

## Rollback Plan

If any migration step fails:
1. Revert Cargo.toml changes
2. Restore deleted files from git
3. Document what went wrong in team file
4. Fix issues before retrying

---

## Perfection Criteria

- [ ] All call sites migrated to new crates
- [ ] No old crate dependencies remain
- [ ] Golden boot test passes
- [ ] All regression tests pass
- [ ] QEMU graphical output works
