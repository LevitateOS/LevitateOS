# TEAM_402: Feature - Disk-Based Root Filesystem

**Date**: 2026-01-10
**Status**: Design Complete - Ready for Implementation
**Type**: Feature

---

## Summary

Implement disk-based root filesystem support for LevitateOS, enabling the OS to be installed to disk and run persistently.

After this feature, users can install LevitateOS to a disk and have changes persist across reboots.

---

## Goal

**Installable, persistent operating system.**

```bash
# Install to disk
/sbin/levitate-install /dev/vda

# Reboot - system runs from disk
reboot

# Changes persist
touch /my-file
reboot
ls /my-file  # Still exists!
```

---

## Scope

### In Scope (Milestone 1)

| Component | Description |
|-----------|-------------|
| pivot_root syscall | Linux-compatible root switching |
| MBR partition parsing | Detect disk partitions |
| Partition device nodes | /dev/vda1, etc. |
| Init root detection | Check for installed OS |
| Init root switching | Automatic pivot_root |
| Disk image resize | 1GB default (was 16MB) |
| Installer utility | Basic installation tool |

### In Scope (Milestone 2)

| Component | Description |
|-----------|-------------|
| Disk bootloader | Boot directly from disk |
| ext4 write support | Standard Linux filesystem |

### Out of Scope (Future)

| Component | Milestone |
|-----------|-----------|
| GPT partition table | Future |
| Multiple partitions | Future |
| Swap partition | Future |
| Encryption (LUKS) | Future |
| Network install | Future |

---

## Plan Documents

| Phase | Document | Status |
|-------|----------|--------|
| Discovery | `docs/planning/disk-root-filesystem/phase-1.md` | Complete |
| Design | `docs/planning/disk-root-filesystem/phase-2.md` | Complete |
| Implementation | `docs/planning/disk-root-filesystem/phase-3.md` | Pending |
| Integration | `docs/planning/disk-root-filesystem/phase-4.md` | Pending |
| Polish | `docs/planning/disk-root-filesystem/phase-5.md` | Pending |

---

## Resolved Design Questions

See `docs/questions/TEAM_402_disk_root_filesystem.md` for full rationales.

### Critical Questions (Resolved)

| ID | Question | Decision | Rationale |
|----|----------|----------|-----------|
| Q1 | pivot_root design? | **A) Linux-compatible** | ABI compatibility—scripts/containers expect pivot_root(2) |
| Q2 | Root filesystem type? | **A) ext4** | FAT32 has NO symlinks/permissions—breaks Unix compatibility |
| Q3 | Partition table? | **A) MBR only** | 1GB images don't need GPT |

### Important Questions (Resolved)

| ID | Question | Decision | Rationale |
|----|----------|----------|-----------|
| Q4 | Disk bootloader? | **A) ISO only** | Scope separation—bootloader is TEAM_403 |
| Q5 | Boot mode selection? | **A) Automatic** | Best UX—detect and switch silently |
| Q6 | Installer scope? | **B) Standard** | Error reporting and progress feedback required |

### Nice to Have (Resolved)

| ID | Question | Decision | Rationale |
|----|----------|----------|-----------|
| Q7 | Disk size default? | **B) 1GB** | Practical balance |
| Q8 | Persistence behavior? | **A) No reset** | YAGNI |
| Q9 | Multi-disk support? | **A) Single disk** | /dev/vda1 predictable for dev |

### Critical Path Impact

**ext4 write is a prerequisite** for TEAM_402 phase 3. No shortcuts—this is the cost of a competitive OS.

---

## Architecture

### Boot Flow

```
┌─────────────────────────────────────────────────────────────┐
│                      BOOT PROCESS                            │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ISO Boot (Limine)                                          │
│       ↓                                                      │
│  Load kernel + initramfs                                     │
│       ↓                                                      │
│  Kernel mounts initramfs as /                               │
│       ↓                                                      │
│  /init (from initramfs) runs                                │
│       ↓                                                      │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ Check: Does /dev/vda1 have installed OS?            │   │
│  │                                                      │   │
│  │ YES → mount disk, pivot_root, exec /sbin/init       │   │
│  │ NO  → stay in initramfs (live mode)                 │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Disk Layout

```
/dev/vda (1GB)
├── MBR (sector 0)
└── Partition 1 (sectors 2048+)
    └── FAT32/ext4 filesystem
        ├── bin/ → usr/bin
        ├── sbin/
        │   └── init
        ├── etc/
        │   ├── hostname
        │   ├── passwd
        │   └── fstab
        ├── usr/bin/
        │   └── coreutils, shell, ...
        └── ... (FHS structure)
```

---

## Implementation Priority

| Priority | Component | Effort | Blocker |
|----------|-----------|--------|---------|
| P0 | pivot_root syscall | Medium | Mount table |
| P0 | MBR parsing | Low | Block device |
| P1 | Partition device nodes | Low | TEAM_401 devtmpfs |
| P1 | Init root detection | Medium | TEAM_400 fork/exec |
| P1 | Init pivot logic | Medium | pivot_root |
| P2 | Disk image resize | Low | Nothing |
| P2 | Installer utility | Medium | All above |

---

## Success Criteria

- [ ] pivot_root syscall works on both architectures
- [ ] MBR partitions detected and device nodes created
- [ ] Init detects installed OS on disk
- [ ] Root successfully switches from initramfs to disk
- [ ] System runs entirely from disk after switch
- [ ] Installer creates working installation
- [ ] Changes persist across reboot
- [ ] Fallback to live mode works if no installation

---

## Dependencies

| Dependency | Team | Status | Required For |
|------------|------|--------|--------------|
| FHS structure | TEAM_401 | Planning | Disk contents |
| devtmpfs | TEAM_401 | Planning | /dev/vda nodes |
| fork/exec | TEAM_400 | Planning | Installer, init |
| Block device | - | ✅ Ready | Disk access |
| Mount syscall | - | ✅ Ready | Mounting disk |

---

## Related Teams

| Team | Relation |
|------|----------|
| TEAM_400 | General Purpose OS (fork/exec) |
| TEAM_401 | FHS (directory structure) |
| TEAM_403 | Disk bootloader (future) |

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| pivot_root complexity | Medium | High | Follow Linux semantics exactly |
| FAT32 corruption | Medium | Medium | Document limitation, plan ext4 |
| Init detection bugs | Medium | Medium | Robust fallback to live |
| Installer bugs | Medium | Low | Manual recovery possible |

---

## Implementation Order

1. MBR partition parsing
2. Partition device nodes (/dev/vda1)
3. pivot_root syscall
4. Mount table pivot support
5. Init root detection
6. Init pivot_root logic
7. Disk image resize (xtask)
8. Installer utility

---

## Notes

- **Prerequisite**: TEAM_400 (fork/exec) and TEAM_401 (FHS/devtmpfs)
- **ext4 required**: FAT32 has no symlinks/permissions—unacceptable for Unix
- **ISO boot only**: Disk bootloader is TEAM_403
- **Single partition**: Multiple partitions and GPT are future enhancements
- This enables **persistence** - the key differentiator from live boot
