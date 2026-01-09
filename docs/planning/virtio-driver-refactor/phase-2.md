# Phase 2: Structural Extraction

**TEAM_332** | VirtIO Driver Reorganization

## Target Design

### New Crate Structure

```
crates/
├── drivers/
│   ├── virtio-blk/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs          # Public API: BlockDevice trait impl
│   │       └── device.rs       # VirtIO block device logic
│   │
│   ├── virtio-input/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs          # Public API: InputDevice, read_event()
│   │       ├── device.rs       # VirtIO input device logic
│   │       └── keymap.rs       # Linux keycode mapping
│   │
│   ├── virtio-net/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs          # Public API: NetDevice trait impl
│   │       └── device.rs       # VirtIO net device logic
│   │
│   └── virtio-gpu/             # Rename from crates/gpu/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs          # Public API: Gpu, Display
│           ├── device.rs       # VirtIO GPU logic
│           └── framebuffer.rs  # Limine framebuffer fallback
│
├── virtio-transport/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs              # Transport trait, re-exports
│       ├── mmio.rs             # MMIO transport wrapper
│       └── pci.rs              # PCI transport wrapper
```

### Transport Abstraction

```rust
// crates/virtio-transport/src/lib.rs

/// Unified transport for VirtIO devices
pub enum Transport {
    Mmio(MmioTransport),
    Pci(PciTransport),
}

/// Trait for transport-agnostic device creation
pub trait VirtioTransport: virtio_drivers::transport::Transport {
    fn device_type(&self) -> DeviceType;
}

impl VirtioTransport for Transport {
    fn device_type(&self) -> DeviceType {
        match self {
            Transport::Mmio(t) => t.device_type(),
            Transport::Pci(t) => t.device_type(),
        }
    }
}
```

### Driver Trait

```rust
// crates/virtio-transport/src/lib.rs

/// Common interface for VirtIO device drivers
pub trait VirtioDriver: Send + Sync {
    /// Device type this driver handles
    const DEVICE_TYPE: DeviceType;
    
    /// Create driver from transport
    fn new<T: VirtioTransport>(transport: T) -> Result<Self, DriverError>
    where Self: Sized;
    
    /// Handle device interrupt
    fn handle_interrupt(&mut self);
}
```

## Extraction Strategy

### Order of Extraction

1. **virtio-transport** - Foundation, must come first
2. **virtio-input** - Already has PCI support, good test case
3. **virtio-blk** - Simple, well-understood
4. **virtio-net** - Similar to block
5. **virtio-gpu** - Most complex, rename + extend existing crate

### Coexistence Strategy

During extraction:
- New crates created alongside old code
- Kernel imports from new crates
- Old kernel/*.rs files become thin wrappers
- Once all call sites migrated, delete old code

## Modular Refactoring Rules

Per Rule 7:

1. **Each module owns its state** - Driver state in driver crate, not kernel
2. **Private fields** - Only expose intentional APIs
3. **No deep imports** - `use virtio_blk::BlockDevice`, not `use virtio_blk::internal::queue::*`
4. **File sizes** - Target <500 lines per file

---

## Phase 2 Steps

### Step 1: Create virtio-transport Crate

**File:** `phase-2-step-1.md`

**Goal:** Create unified transport abstraction

Tasks:
1. Create `crates/virtio-transport/` directory structure
2. Define `Transport` enum wrapping MMIO and PCI
3. Implement `VirtioTransport` trait
4. Re-export useful types from `virtio-drivers`
5. Add to workspace Cargo.toml

**Exit Criteria:**
- Crate compiles
- Can create Transport from either MMIO or PCI
- Unit tests pass

### Step 2: Extract virtio-input Crate

**File:** `phase-2-step-2.md`

**Goal:** Move input driver to dedicated crate

Tasks:
1. Create `crates/drivers/virtio-input/` structure
2. Move `linux_code_to_ascii()` and keymap logic
3. Create `InputDevice` struct using `Transport`
4. Implement `poll()` and `read_char()` in crate
5. Export clean public API

**Exit Criteria:**
- Crate compiles
- Can poll input events from both MMIO and PCI transports
- Kernel can use new crate (temporarily alongside old code)

### Step 3: Extract virtio-blk Crate

**File:** `phase-2-step-3.md`

**Goal:** Move block driver to dedicated crate

Tasks:
1. Create `crates/drivers/virtio-blk/` structure
2. Move block device logic from `kernel/src/block.rs`
3. Create `BlockDevice` struct using `Transport`
4. Implement `read_block()`, `write_block()` in crate
5. Add PCI transport support (currently MMIO only)

**Exit Criteria:**
- Crate compiles
- Block operations work on both transports
- FS layer can use new crate

### Step 4: Extract virtio-net Crate

**File:** `phase-2-step-4.md`

**Goal:** Move network driver to dedicated crate

Tasks:
1. Create `crates/drivers/virtio-net/` structure
2. Move net device logic from `kernel/src/net.rs`
3. Create `NetDevice` struct using `Transport`
4. Add PCI transport support

**Exit Criteria:**
- Crate compiles
- Network operations work on both transports

### Step 5: Reorganize virtio-gpu Crate

**File:** `phase-2-step-5.md`

**Goal:** Rename and extend existing GPU crate

Tasks:
1. Rename `crates/gpu/` to `crates/drivers/virtio-gpu/`
2. Move Limine framebuffer fallback from kernel to crate
3. Create unified `GpuBackend` enum in crate
4. Update all imports

**Exit Criteria:**
- Crate compiles at new location
- GPU works with VirtIO and Limine framebuffer
- Screenshot tests pass
