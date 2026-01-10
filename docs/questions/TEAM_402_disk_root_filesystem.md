# Open Questions: Disk-Based Root Filesystem (TEAM_402)

**Created**: 2026-01-10
**Status**: All Questions Answered
**Plan**: `docs/planning/disk-root-filesystem/`

---

## Critical Questions (Block Phase 3)

### Q1: pivot_root Implementation

**Question**: Should we implement Linux-compatible pivot_root or a custom mechanism?

**Options**:
- **A) Linux-compatible pivot_root** (Recommended)
- **B) Custom switch_root utility**
- **C) Kernel-internal root switch**

**Answer**: **A) Linux-compatible pivot_root**

**Rationale**: ABI compatibility goal—Linux scripts, systemd, and container runtimes expect `pivot_root(2)`. Standard interface, well-documented semantics.

---

### Q2: Root Filesystem Type

**Question**: What filesystem should be used for the disk root?

**Options**:
- **A) ext4 (read-write)** (Recommended)
  - Standard Linux filesystem
  - Journaling for reliability
  - Requires ext4 write support (not yet implemented)
  - Proper Unix semantics (permissions, symlinks, hard links)

- **B) FAT32 (read-write)**
  - Already have write support
  - No journaling (corruption risk)
  - **NO Unix permissions**
  - **NO symlinks**
  - Quick to implement

- **C) Custom simple filesystem**

**Answer**: **A) ext4 (read-write)**

**Rationale**: FAT32 has no symlinks and no Unix permissions—this fundamentally breaks Unix compatibility. A "general purpose Unix-compatible OS" cannot have a root filesystem that doesn't support `chmod`, `chown`, or symlinks. ext4 write support must be prioritized. This is a prerequisite, not optional.

**Implementation Note**: ext4 write is significant work (journaling, extent allocation, inode management). This is the cost of being a real OS. Prioritize as blocking work.

---

### Q3: Partition Table Format

**Question**: What partition table format should be supported?

**Options**:
- **A) MBR only** (Recommended for MVP)
- **B) GPT only**
- **C) Both MBR and GPT**

**Answer**: **A) MBR only**

**Rationale**: 1GB dev images don't need GPT's 128 partitions or >2TB support. MBR is 16-byte entries, simple parsing. Add GPT when needed.

---

## Important Questions (Should Answer Before Phase 3)

### Q4: Disk Bootloader

**Question**: Should LevitateOS be bootable directly from disk?

**Options**:
- **A) ISO boot only** (Recommended for MVP)
- **B) Install Limine to disk**
- **C) Custom bootloader**

**Answer**: **A) ISO boot only**

**Rationale**: Scope separation—this TEAM focuses on root switching. Disk bootloader is TEAM_403.

---

### Q5: Boot Mode Selection

**Question**: How should the user choose between live and installed mode?

**Options**:
- **A) Automatic detection** (Recommended)
- **B) Boot menu**
- **C) Kernel parameter**

**Answer**: **A) Automatic detection**

**Rationale**: Best UX—detect `/sbin/init` on disk, switch if found. No menu needed. Graceful fallback to live mode.

---

### Q6: Installer Scope

**Question**: What should the installer do?

**Options**:
- **A) Minimal** - partition, format, copy
- **B) Standard** (Recommended) - Add progress, errors, validation
- **C) Full** - Add partition editor, multi-partition, bootloader

**Answer**: **B) Standard**

**Rationale**: Installation failures must be clearly reported. Progress feedback for multi-second operations. No partition editor (scope creep).

---

## Nice to Have Questions (Can Defer)

### Q7: Disk Size Default

**Answer**: **B) 1GB** with `--size` override

**Rationale**: Balance between tight (512MB) and wasteful (4GB).

---

### Q8: Persistence Behavior

**Answer**: **A) No reset mechanism**

**Rationale**: "Factory reset" = run installer again. YAGNI.

---

### Q9: Multi-Disk Support

**Answer**: **A) Single disk only**

**Rationale**: `/dev/vda1` is predictable for dev. UUID-based detection is future work.

---

## Summary of Decisions

| Q | Decision | Key Point |
|---|----------|-----------|
| Q1 | Linux pivot_root | ABI compatibility |
| Q2 | **ext4** | FAT32 breaks Unix semantics (no symlinks/perms) |
| Q3 | MBR | Sufficient for 1GB images |
| Q4 | ISO boot | Scope separation |
| Q5 | Auto-detect | Best UX |
| Q6 | Standard installer | Error reporting matters |
| Q7 | 1GB default | Practical balance |
| Q8 | No reset | YAGNI |
| Q9 | Single disk | MVP scope |

---

## Critical Path Update

**Q2 changes the critical path**: ext4 write support is now a prerequisite for disk-based root.

ext4 write implementation is blocking. This is the cost of building a competitive general-purpose OS. No shortcuts.
