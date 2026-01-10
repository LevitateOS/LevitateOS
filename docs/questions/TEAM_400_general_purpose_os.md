# Open Questions: General Purpose OS (TEAM_400)

**Created**: 2026-01-10
**Status**: All Questions Answered
**Plan**: `docs/planning/general-purpose-os/`

---

## Critical Questions (Block Phase 3)

### Q1: Fork Memory Strategy

**Question**: Should fork use eager copy or copy-on-write (CoW) for page duplication?

**Options**:
- **A) Eager Copy** (Recommended)
  - Copy all writable pages immediately on fork
  - Simpler implementation
  - No page fault handler changes needed
  - Wastes memory for fork+exec pattern

- **B) Copy-on-Write**
  - Mark pages read-only, copy on write fault
  - Standard Unix behavior
  - Memory efficient
  - Requires page fault handler modification

**Recommendation**: Option A for v1—CoW adds complexity without proven need. Optimize when profiling shows it matters.

**Answer**: **A) Eager Copy** - Start simple, optimize to CoW later if memory becomes an issue.

---

### Q2: execve Path Resolution

**Question**: How should execve resolve executable paths?

**Options**:
- **A) Initramfs only**
  - Only search initramfs for executables
  - Simplest implementation
  - Very limited

- **B) VFS path resolution** (Recommended)
  - Use existing VFS to resolve paths
  - Supports mounted filesystems
  - Standard behavior

- **C) PATH environment search**
  - Search PATH like shell does
  - Most compatible
  - Complex, usually shell's job

**Recommendation**: Option B - VFS resolution, shell handles PATH.

**Answer**: **B) VFS path resolution** - Standard behavior, shell handles PATH searching.

---

### Q3: Orphan Process Reparenting

**Question**: When a parent process exits, should its children be reparented to PID 1?

**Options**:
- **A) Yes, reparent to PID 1** (Recommended)
  - Standard Unix behavior
  - Allows orphans to be properly reaped
  - Requires init process awareness

- **B) No reparenting**
  - Simpler
  - Orphans become zombies forever
  - Not standard behavior

**Recommendation**: Option A for standard Unix semantics.

**Answer**: **A) Yes, reparent to PID 1** - Standard Unix semantics, proper zombie cleanup.

---

### Q4: poll() vs ppoll()

**Question**: Should we implement poll() separately or just use ppoll()?

**Options**:
- **A) ppoll only**
  - Already implemented (TEAM_360)
  - Some programs call poll() directly
  - Would need libc to translate

- **B) Both poll and ppoll** (Recommended)
  - Maximum compatibility
  - poll() can be thin wrapper around ppoll()
  - Low additional effort

**Recommendation**: Option B - implement poll() as wrapper.

**Answer**: **B) Both poll and ppoll** - Maximum compatibility, poll() is a thin wrapper.

---

## Important Questions (Should Answer Before Phase 3)

### Q5: FD_CLOEXEC Handling

**Question**: How should close-on-exec file descriptors be handled across fork/exec?

**Options**:
- **A) Track per-fd flag, close on exec** (Recommended)
  - Standard behavior
  - Requires FD table flag support
  - More complex

- **B) Ignore for v1**
  - Simpler
  - May cause fd leaks
  - Not standard

**Recommendation**: Option A - Track the flag. FD leaks can cause subtle bugs and security issues. The FD table already exists, adding a flag bit is low effort.

**Answer**: **A) Track per-fd flag, close on exec** - Prevents FD leaks, standard behavior.

---

### Q6: chmod Metadata Storage

**Question**: Should chmod actually store mode bits in VFS inodes?

**Options**:
- **A) Store in VFS**
  - More realistic
  - Requires VFS inode changes
  - Modes still not enforced (root)

- **B) No-op (just succeed)** (Recommended)
  - Simpler
  - Programs think chmod worked
  - No actual effect

**Recommendation**: Option B - No-op for v1. Since we're single-user (always root) and don't enforce permissions, storing modes adds complexity with no functional benefit. Can add later if needed.

**Answer**: **B) No-op (just succeed)** - Single-user OS, permissions not enforced anyway.

---

### Q7: Signal Reset on exec

**Question**: Which signals should be reset to default disposition on exec?

**Options**:
- **A) All except SIG_IGN** (Recommended)
  - Linux behavior
  - Ignored signals stay ignored
  - Others reset to default

- **B) Reset all**
  - Simpler
  - May break programs relying on inherited ignores

- **C) Keep all**
  - Simplest
  - Not standard

**Recommendation**: Option A - Standard Linux behavior. Programs commonly rely on SIG_IGN being inherited (e.g., nohup sets SIGHUP to SIG_IGN before exec). Low additional complexity.

**Answer**: **A) All except SIG_IGN** - Standard Linux behavior, nohup compatibility.

---

## Nice to Have Questions (Can Defer)

### Q8: pselect6 Implementation

**Question**: Should we implement pselect6 for signal-safe select operations?

**Options**:
- **A) Yes, implement pselect6**
  - Signal-safe version of select
  - Some programs require it
  - Additional implementation effort

- **B) No, defer to future** (Recommended)
  - select() covers most use cases
  - pselect6 is rarely needed
  - Can add when a real program needs it

**Recommendation**: Option B - Defer. Implement only if we encounter a program that specifically needs pselect6. YAGNI principle.

**Answer**: **B) No, defer to future** - YAGNI, add when needed.

---

### Q9: argv+envp Size Limit

**Question**: What's the maximum combined size of argv and envp for E2BIG?

**Options**:
- **A) 128KB (conservative)**
  - Matches older Linux kernels
  - Safe limit
  - May reject some valid invocations

- **B) 256KB** (Recommended)
  - Matches modern Linux (ARG_MAX)
  - Good compatibility
  - Reasonable memory usage

- **C) 2MB (generous)**
  - Very permissive
  - Matches some modern systems
  - Higher memory risk

- **D) No limit**
  - Maximum flexibility
  - Risk of OOM on malicious input
  - Not standard

**Recommendation**: Option B - 256KB matches Linux ARG_MAX. Large enough for practical use, bounded enough to prevent abuse.

**Answer**: **B) 256KB** - Matches Linux ARG_MAX, practical and bounded.

---

### Q10: getppid for Orphans

**Question**: Should getppid() return 1 for processes whose parent has exited (orphan processes)?

**Options**:
- **A) Yes, return 1 (init)** (Recommended)
  - Standard Unix/Linux behavior
  - Consistent with Q3 orphan reparenting
  - Programs may check getppid() == 1 to detect orphaning

- **B) Return original parent PID**
  - Simpler (no update needed)
  - Not standard
  - May confuse programs

- **C) Return 0**
  - Indicates "no parent"
  - Not standard
  - PID 0 has special meaning (kernel)

**Recommendation**: Option A - Return 1. Consistent with standard Unix behavior and Q3 reparenting decision. If we reparent orphans to init (PID 1), getppid() should reflect that.

**Answer**: **A) Yes, return 1 (init)** - Consistent with Q3, standard Unix behavior.

---

## Instructions

1. Review each question and options
2. Provide answer by filling in `**Answer**: _________________`
3. Once critical questions (Q1-Q4) are answered, Phase 3 can begin
4. Important questions (Q5-Q7) should be answered before implementation completes
5. Nice to have (Q8-Q10) can be deferred
