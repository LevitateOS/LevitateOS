# Phase 1: Discovery & Requirements - Nano Editor Support

## Objective

Enable running the `nano` text editor in LevitateOS initramfs. This requires implementing kernel features for **dynamic linking** (musl ld) and **terminal interaction** (ncurses).

## Why Nano?

| Reason | Description |
|--------|-------------|
| **Validation target** | Nano exercises dynamic linking, terminal I/O, and signal handling |
| **Real-world utility** | Users expect a text editor for config file editing |
| **Complexity ladder** | More complex than busybox vi, less than vim/emacs |
| **Dependency chain** | Requires ncurses → validates library loading works |

## Blockers Discovered (TEAM_471)

During the initramfs refactor, adding nano from Alpine packages revealed:

| Blocker | Symptom | Root Cause |
|---------|---------|------------|
| **File-backed mmap** | `Error loading shared library: Invalid argument` | `mmap(fd)` returns EINVAL |
| **Dynamic linker** | Libraries can't be mapped into memory | No `MAP_PRIVATE` with fd support |

Current error message:
```
[MMAP] Only MAP_ANONYMOUS supported, got flags=0x2
Error loading shared library libncursesw.so.6: Invalid argument
```

## Kernel Prerequisites

### Critical Path (Must Have)

These syscalls/features block nano from running at all:

| # | Feature | Current State | Required For |
|---|---------|---------------|--------------|
| 1 | **File-backed mmap** | **NOT IMPLEMENTED** | musl ld.so, all shared libraries |
| 2 | **MAP_PRIVATE (copy-on-write)** | **NOT IMPLEMENTED** | Library loading, lazy binding |
| 3 | **ELF PT_INTERP handling** | **COMPLETE (TEAM_470)** | Automatic dynamic linker invocation |

### Already Implemented

These features are ready and working:

| Feature | Implementation | Location |
|---------|----------------|----------|
| mprotect | Full | `syscall/src/mm.rs:352-403` |
| readlinkat | Full | `syscall/src/fs/link.rs:168` |
| pread64 | Full | `syscall/src/fs/fd.rs:629` |
| open/read/write | Full | `syscall/src/fs/` |
| fstat | Full | `syscall/src/fs/fd.rs` |
| poll/ppoll | Full | `syscall/src/sync.rs:165,371` |
| sigaction | Full | `syscall/src/signal.rs:123` |
| TIOCGWINSZ | Stub (80x24) | `syscall/src/fs/fd.rs:315-331` |
| TIOCSWINSZ | Stub (no-op) | `syscall/src/fs/fd.rs:333-350` |

### Enhancement Candidates

These work but may need improvement for nano:

| Feature | Current State | Enhancement Needed |
|---------|---------------|-------------------|
| TIOCGWINSZ | Returns fixed 80x24 | Return actual terminal size from virtio-gpu |
| SIGWINCH | Not sent | Send on terminal resize |
| readlink (syscall 89) | May need wrapper | Verify dispatch to readlinkat |

## Technical Analysis

### 1. File-Backed mmap (Critical)

**What musl's dynamic linker needs:**
```c
// From musl/ldso/dynlink.c
void *base = mmap(0, map_len, PROT_READ|PROT_EXEC,
                  MAP_PRIVATE, fd, off);
```

**Current kernel limitation:**
```rust
// crates/kernel/syscall/src/mm.rs:193-204
if flags & MAP_ANONYMOUS == 0 {
    log::warn!("[MMAP] Only MAP_ANONYMOUS supported");
    return Err(EINVAL);
}
if fd != -1 || offset != 0 {
    log::warn!("[MMAP] File-backed mappings not supported");
    return Err(EINVAL);
}
```

**Implementation requirements:**
1. Read file contents from VFS using `fd`
2. Allocate physical pages
3. Copy file data to pages (MAP_PRIVATE = copy-on-write semantics)
4. Map pages into user address space with requested protection
5. Handle page-aligned `offset` parameter
6. Track mapping as file-backed in VMA for potential CoW handling

**Simplified approach (no true CoW):**
- Copy entire mapped region eagerly on mmap
- Treat MAP_PRIVATE as "private copy, not shared with other processes"
- True CoW optimization can come later

### 2. MAP_PRIVATE Copy-on-Write

For MVP, we can skip true CoW and just copy:
```rust
// Pseudo-code for simplified MAP_PRIVATE
fn mmap_file_private(fd, len, offset, prot) {
    let file = get_file(fd)?;
    let data = file.read_at(offset, len)?;
    let pages = allocate_pages(len)?;
    copy_data_to_pages(data, pages);
    map_pages_to_user(pages, prot);
}
```

This is memory-inefficient but correct. CoW can be added later for fork().

### 3. ELF PT_INTERP Handling

When exec() loads an ELF with a PT_INTERP segment:
1. Instead of jumping to program's entry point
2. Load the interpreter (e.g., `/lib/ld-musl-x86_64.so.1`)
3. Pass original program path via auxv
4. Jump to interpreter's entry point

**Current state:** Need to check if kernel loader handles PT_INTERP.

### 4. musl Dynamic Linker Syscalls

From analyzing musl's `dynlink.c`:

| Syscall | Purpose | Status |
|---------|---------|--------|
| `open` | Open library files | IMPLEMENTED |
| `read` | Read ELF headers | IMPLEMENTED |
| `pread` | Read specific ELF sections | IMPLEMENTED |
| `mmap` | Map library segments | **NOT IMPLEMENTED** |
| `mprotect` | Set segment permissions | IMPLEMENTED |
| `close` | Close library fd | IMPLEMENTED |
| `fstat` | Get file size | IMPLEMENTED |
| `readlink` | Resolve symlinks (for `/proc/self/exe`) | IMPLEMENTED |
| `munmap` | Cleanup on error | IMPLEMENTED |
| `madvise` | Memory hints (optional) | IMPLEMENTED (stub) |

### 5. ncurses Requirements

| Feature | Purpose | Status |
|---------|---------|--------|
| `TIOCGWINSZ` | Get terminal dimensions | IMPLEMENTED (fixed) |
| `TIOCSWINSZ` | Set terminal dimensions | IMPLEMENTED (no-op) |
| `SIGWINCH` | Terminal resize notification | NOT SENT |
| `poll/select` | Wait for input | IMPLEMENTED |
| `tcgetattr/tcsetattr` | Terminal mode control | Need to verify |

## Implementation Priority

### Phase 1: File-Backed mmap (P0 - Blocker)

Without this, nothing works.

**Scope:**
- Add file-backed mmap support with eager copy
- Support MAP_PRIVATE flag
- Validate with simple test (mmap a text file, read contents)

**Estimated complexity:** ~200 lines of kernel code

### Phase 2: PT_INTERP Handling (P0 - COMPLETE)

**Status: COMPLETE by TEAM_470**

TEAM_470 implemented full PT_INTERP support:
- ELF loader detects PT_INTERP segment
- Interpreter loaded at high fixed base (0x7f0000000000)
- auxv correctly set up (AT_PHDR, AT_ENTRY, AT_BASE)
- Verified working - interpreter executes until mmap fails

See `.teams/TEAM_470_feature_pt_interp_dynamic_linking.md` for details.

### Phase 3: Verification & Polish (P1)

**Scope:**
- Dynamic TIOCGWINSZ from virtio-gpu
- SIGWINCH on resize (optional for MVP)
- Test nano end-to-end

## Test Plan

### Unit Tests

| Test | Description |
|------|-------------|
| `mmap_file_read` | mmap a file, verify contents readable |
| `mmap_file_private` | mmap MAP_PRIVATE, write to it, verify original unchanged |
| `mmap_file_exec` | mmap executable segment, verify PROT_EXEC works |

### Integration Tests

| Test | Description |
|------|-------------|
| `dynamic_hello` | Compile simple dynamically-linked "hello world" |
| `ldd_test` | Run `ldd` on a dynamic binary, verify library discovery |
| `nano_launch` | Launch nano, verify it draws to terminal |
| `nano_edit` | Create/edit/save a file with nano |

## Dependencies

### On Existing Work

| Dependency | Team | Status |
|------------|------|--------|
| VFS file operations | TEAM_205+ | Complete |
| ELF loader | TEAM_various | Complete (static) |
| mmap anonymous | TEAM_228 | Complete |
| VMA tracking | TEAM_238 | Complete |

### Related Teams

| Team | Relevance | Status |
|------|-----------|--------|
| TEAM_470 | PT_INTERP and dynamic linking | **COMPLETE** |
| TEAM_471 | Discovered blockers, initramfs refactor | In Progress |
| TEAM_228 | Original mmap implementation | Complete (anonymous only) |
| TEAM_238 | VMA tracking, mmap cleanup | Complete |

## Files to Modify

| File | Changes |
|------|---------|
| `syscall/src/mm.rs` | Add file-backed mmap support |
| `kernel/src/loader/elf.rs` | Add PT_INTERP handling |
| `syscall/src/fs/fd.rs` | May need adjustments for mmap offset |
| `initramfs/initramfs.toml` | Add nano + ncurses + musl libraries |

## Questions Resolved

1. **Rhai vs TOML for initramfs?** → TOML (simpler, TEAM_471 decision)
2. **CoW for MAP_PRIVATE?** → Defer, eager copy is sufficient for MVP
3. **Static vs dynamic nano?** → Dynamic (tests more kernel functionality)

## Open Questions (Resolved)

1. **Does kernel loader already parse PT_INTERP?** → **YES (TEAM_470)** - Fully implemented
2. **Is there an existing file-backed mmap PR/branch?** → **NO** - This is the remaining blocker
3. **Should we expose AT_RANDOM in auxv?** → Already present in auxv setup

## Success Criteria

| Metric | Target |
|--------|--------|
| nano launches | Yes |
| nano shows correct terminal size | Yes |
| nano can edit and save files | Yes |
| nano handles Ctrl+C gracefully | Yes |
| Dynamic linking works for other musl programs | Yes |

## Next Steps

1. ~~Check if TEAM_470 has started file-backed mmap work~~ → TEAM_470 completed PT_INTERP, mmap is separate work
2. ~~Review ELF loader for PT_INTERP support status~~ → COMPLETE by TEAM_470
3. **Create Phase 2 document with detailed file-backed mmap implementation design** → NEXT
4. Implement file-backed mmap in `syscall/src/mm.rs`
5. Test with nano from Alpine packages
