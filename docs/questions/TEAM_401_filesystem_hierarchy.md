# Open Questions: Filesystem Hierarchy Standard (TEAM_401)

**Created**: 2026-01-10
**Status**: All Questions Answered
**Plan**: `docs/planning/filesystem-hierarchy/`

---

## Critical Questions (Block Phase 3)

### Q1: Traditional vs Merged /usr Layout

**Question**: Should LevitateOS use traditional FHS layout or modern merged /usr?

**Options**:
- **A) Traditional Layout**
  - `/bin/` contains essential binaries (or symlinks)
  - `/usr/bin/` contains additional binaries
  - Clear separation of essential vs optional
  - Simpler initramfs structure
  - Can migrate to merged later

- **B) Merged /usr Layout** (Recommended)
  - `/bin -> /usr/bin` (symlink)
  - `/sbin -> /usr/sbin` (symlink)
  - `/lib -> /usr/lib` (symlink)
  - Modern Linux default (Fedora, Debian, etc.)
  - Simplifies package management
  - Requires symlinks at root level

**Answer**: **B) Merged /usr Layout**

**Rationale**: We're building a new OS—no legacy to maintain. Every major distro has moved to merged /usr (Fedora 2012, Debian 2022, Arch 2017). Starting traditional and migrating later creates unnecessary work. Ship modern from day one.

---

### Q2: devtmpfs or Static /dev

**Question**: How should device files in /dev be implemented?

**Options**:
- **A) Static in initramfs**
  - Device nodes created at build time in initramfs
  - Simple: no new filesystem needed
  - Can't add devices at runtime
  - PTY allocation would need special handling

- **B) devtmpfs** (Recommended)
  - New in-memory filesystem mounted at /dev
  - Devices created dynamically at boot
  - Supports runtime device creation (PTY, etc.)
  - More code but more flexible
  - Standard Linux approach

**Answer**: **B) devtmpfs**

**Rationale**: Functional requirement—PTY allocation needs dynamic device creation. No choice here.

---

### Q3: Essential /dev Nodes

**Question**: Which device nodes should be created at boot?

**Options**:
- **A) Minimal**
  - `/dev/null`, `/dev/zero`, `/dev/urandom`, `/dev/ptmx`, `/dev/pts/`
  - (5 entries)

- **B) Standard**
  - All of A, plus: `/dev/full`, `/dev/random`, `/dev/tty`, `/dev/console`
  - (9 entries)

- **C) Extended** (Recommended)
  - All of B, plus:
  - `/dev/fd/` - file descriptor directory
  - `/dev/stdin` -> `/dev/fd/0`
  - `/dev/stdout` -> `/dev/fd/1`
  - `/dev/stderr` -> `/dev/fd/2`
  - (13+ entries)

**Answer**: **C) Extended**

**Rationale**: `/dev/stdin`, `/dev/stdout`, `/dev/stderr` are used everywhere—shell scripts, programs that need to reopen stdio. Many programs expect these. `/dev/fd/` can be implemented as a simple synthetic directory that maps fd numbers to the process's open files—doesn't require full procfs.

---

## Important Questions (Should Answer Before Phase 3)

### Q4: procfs Scope for Milestone 1

**Question**: Should procfs be included in the initial FHS implementation?

**Options**:
- **A) Defer entirely**
  - Don't implement procfs in Milestone 1
  - Focus on basic FHS structure and devtmpfs
  - Add procfs as separate feature later

- **B) Minimal /proc/self** (Recommended)
  - Only implement `/proc/self/` symlink
  - `/proc/self/exe`, `/proc/self/fd/`, `/proc/self/cwd`
  - Enables self-introspection
  - Moderate effort

- **C) Basic procfs**
  - `/proc/{pid}/` directories
  - `/proc/meminfo`, `/proc/cpuinfo`
  - Significant effort

**Answer**: **B) Minimal /proc/self**

**Rationale**: `/proc/self/exe` is used by many programs to find their own executable path. Rust's `std::env::current_exe()` uses it. Without it, programs can't reliably locate themselves. This is a small scope addition that unlocks significant compatibility.

---

### Q5: /etc Content Scope

**Question**: What configuration files should /etc contain initially?

**Options**:
- **A) Minimal** - hostname, passwd (2 files)
- **B) Standard** - Add group, shells, profile (5 files)
- **C) Extended** - Add fstab, hosts, resolv.conf, os-release (9+ files)

**Answer**: **B) Standard**

**Rationale**: These files serve functional purposes—getpwuid/getgrgid need passwd/group, shells source profile. Extended adds network config we don't use yet.

---

### Q6: Binary Installation Location

**Question**: Where should the main binaries be installed?

**Options**:
- **A) All in /usr/bin** (Recommended)
- **B) Split by category**
- **C) Flat /bin only**

**Answer**: **A) All in /usr/bin**

**Rationale**: With merged /usr (Q1=B), `/bin` is a symlink to `/usr/bin` anyway. Install to `/usr/bin/`, the symlink handles compatibility.

---

## Nice to Have Questions (Can Defer)

### Q7: /var Structure

**Question**: What /var subdirectories should exist?

**Options**:
- **A) Minimal** - log, tmp (2 dirs)
- **B) Standard** - Add run symlink, cache (3 dirs + symlink)
- **C) Full** - Add lib, spool, mail (6+ dirs)

**Answer**: **B) Standard**

**Rationale**: `/var/run -> /run` symlink needed for compatibility. Cache directory useful. No mail/spool services.

---

### Q8: Random Number Quality

**Question**: What quality of randomness should /dev/urandom provide?

**Options**:
- **A) Simple PRNG**
  - Timestamp-seeded LCG
  - NOT cryptographically secure

- **B) Hardware RNG (x86_64)** (Recommended)
  - Use RDRAND instruction when available
  - Fallback to PRNG on older CPUs or aarch64

- **C) Full Entropy Pool**
  - Collect entropy from interrupts, timing
  - Cryptographic quality

**Answer**: **B) Hardware RNG**

**Rationale**: RDRAND is one instruction. On x86_64 (Ivy Bridge+, 2012), it's available. Provides vastly better randomness for near-zero additional complexity. Use PRNG fallback for aarch64 or old x86. Full entropy pool is future work for TLS/crypto.

---

## Summary of Decisions

| Q | Decision | Key Point |
|---|----------|-----------|
| Q1 | Merged /usr | Start modern, not legacy |
| Q2 | devtmpfs | Required for PTY |
| Q3 | Extended /dev | /dev/stdin etc. widely used |
| Q4 | Minimal /proc/self | /proc/self/exe needed for compatibility |
| Q5 | Standard /etc | Functional requirements |
| Q6 | /usr/bin | Follows from merged /usr |
| Q7 | Standard /var | Compatibility symlink |
| Q8 | RDRAND | One instruction, much better quality |
