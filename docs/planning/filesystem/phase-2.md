# Phase 2: Design — Filesystem Feature

**Feature:** FAT32 Filesystem Support  
**Parent:** `/docs/planning/filesystem/`

---

## Proposed Solution

Create a `BlockDeviceIO` adapter that implements `fatfs` traits over our block driver, then expose a simple kernel API.

---

## API Design

```rust
// kernel/src/fs.rs

/// Initialize filesystem from block device
pub fn init() -> Result<(), &'static str>;

/// Read entire file contents
pub fn read_file(path: &str) -> Option<Vec<u8>>;

/// List directory entries  
pub fn list_dir(path: &str) -> Vec<String>;
```

---

## Data Model

```rust
struct BlockDeviceIO {
    position: u64,      // Current byte offset
    block_buf: [u8; 512], // Cached block for partial reads
    cached_block: Option<usize>, // Which block is cached
}
```

---

## Behavioral Decisions

| Question | Decision | Rationale |
|----------|----------|-----------|
| Read-only or read-write? | **Read-only** | Simpler, safer for initial impl |
| Partial block handling? | **Read-modify-write** | Required by fatfs Seek trait |
| Error handling? | **Return Option/Result** | No panic in FS code |
| Multiple disks? | **Single disk only** | Matches current block driver |

---

## Open Questions

> [!IMPORTANT]
> **Q1:** Should we buffer writes or immediately flush?
> - *Recommendation:* Immediate flush (no write cache) for simplicity

> [!IMPORTANT]
> **Q2:** What happens if disk is not FAT32?
> - *Recommendation:* Log error, continue boot without FS

> [!NOTE]
> **Q3:** Should `init()` be called automatically or manually?
> - *Recommendation:* Manual call in `kmain()` after VirtIO init

---

## Steps

### Step 1: Define BlockDeviceIO Struct
- **File:** `phase-2-step-1.md`
- **Goal:** Define struct and implement `new()`
- **UoW:** 1

### Step 2: Implement Read Trait
- **File:** `phase-2-step-2.md`
- **Goal:** Implement `fatfs::Read` for BlockDeviceIO
- **UoW:** 1

### Step 3: Implement Write Trait
- **File:** `phase-2-step-3.md`
- **Goal:** Implement `fatfs::Write` for BlockDeviceIO
- **UoW:** 1

### Step 4: Implement Seek Trait
- **File:** `phase-2-step-4.md`
- **Goal:** Implement `fatfs::Seek` for BlockDeviceIO
- **UoW:** 1

### Step 5: Define Public API
- **File:** `phase-2-step-5.md`
- **Goal:** Define `init()`, `read_file()`, `list_dir()`
- **UoW:** 1

→ **Next:** Phase 3 (Implementation)
