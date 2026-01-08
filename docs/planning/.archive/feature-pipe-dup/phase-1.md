# Phase 1: Discovery

**Parent**: `docs/planning/feature-pipe-dup/`  
**Team**: TEAM_233  
**Status**: Not Started

## Purpose
Understand current VFS/fd infrastructure and identify implementation approach for pipes.

## Success Criteria
- Current fd table structure documented
- Pipe integration points identified
- Blocking I/O mechanism understood

---

## Step 1: Analyze Current File Descriptor Infrastructure

### UoW 1.1: Document FD Table Structure

**Objective**: Understand how fds are allocated and managed.

**Tasks**:
1. Read `kernel/src/task/fd_table.rs`
2. Document:
   - How fds are allocated
   - How VfsFile handles are stored
   - Current fd operations (read, write, close)
3. Identify gaps for pipe support

**Exit Criteria**: FD table structure documented with pipe integration points.

---

### UoW 1.2: Analyze VfsFile Trait

**Objective**: Understand the VFS file abstraction.

**Tasks**:
1. Read `kernel/src/fs/vfs/file.rs`
2. Document:
   - VfsFile trait methods
   - How read/write are dispatched
   - How close is handled
3. Determine if Pipe can implement VfsFile

**Exit Criteria**: VfsFile interface documented, Pipe compatibility confirmed.

---

## Step 2: Identify Blocking Mechanism

### UoW 2.1: Document Futex/Blocking for I/O

**Objective**: Understand how to implement blocking read/write for pipes.

**Tasks**:
1. Review futex implementation (`kernel/src/syscall/sync.rs`)
2. Document how blocking can be used for:
   - Blocking read (when pipe empty)
   - Blocking write (when pipe full)
3. Consider O_NONBLOCK flag behavior

**Exit Criteria**: Blocking strategy documented.

---

## Deliverables
- FD table analysis document
- VfsFile trait compatibility notes
- Blocking I/O strategy
