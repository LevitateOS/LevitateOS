# File Read Implementation — Phase 2: Design

**Team:** TEAM_177  
**Created:** 2026-01-06  
**Status:** QUESTIONS PENDING  
**Parent:** `phase-1.md`

---

## 1. Proposed Solution

### 1.1 High-Level Description

Extend `sys_read` to handle `InitramfsFile` file descriptors by:
1. Looking up the fd in the task's fd_table
2. If `InitramfsFile`, read from the INITRAMFS CpioArchive
3. Copy data to userspace buffer
4. Update the offset in the fd entry
5. Return bytes read

### 1.2 User-Facing Behavior

```rust
use ulib::fs::File;
use ulib::io::Read;

let mut file = File::open("/hello.txt")?;
let mut contents = String::new();
file.read_to_string(&mut contents)?;
println!("File contents: {}", contents);
```

### 1.3 System Behavior

**Read flow:**
1. User calls `file.read(&mut buf)`
2. ulib calls `libsyscall::read(fd, buf.as_mut_ptr(), buf.len())`
3. Kernel `sys_read` looks up fd in task's fd_table
4. For `InitramfsFile { file_index, offset }`:
   - Get file data from INITRAMFS via `file_index`
   - Calculate bytes to read: `min(len, file_size - offset)`
   - Copy bytes to userspace starting at current offset
   - Update offset in fd_table: `offset += bytes_read`
5. Return bytes_read (0 if at EOF)

---

## 2. API Design

### 2.1 Kernel Changes

```rust
// kernel/src/syscall/fs.rs

pub fn sys_read(fd: usize, buf: usize, len: usize) -> i64 {
    let task = crate::task::current_task();
    
    // Look up fd type
    let mut fd_table = task.fd_table.lock();
    let entry = match fd_table.get(fd) {
        Some(e) => e.clone(),
        None => return errno::EBADF,
    };
    drop(fd_table);
    
    match entry.fd_type {
        FdType::Stdin => read_stdin(buf, len, task.ttbr0),
        FdType::InitramfsFile { file_index, offset } => {
            read_initramfs_file(fd, file_index, offset, buf, len, &task)
        }
        _ => errno::EBADF,
    }
}

fn read_initramfs_file(
    fd: usize,
    file_index: usize,
    offset: usize,
    buf: usize,
    len: usize,
    task: &Task,
) -> i64 {
    // Get file data from initramfs
    // Copy to userspace
    // Update offset in fd_table
    // Return bytes read
}
```

### 2.2 ulib Changes

```rust
// userspace/ulib/src/fs.rs

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let ret = libsyscall::read(self.fd, buf);
        if ret < 0 {
            Err(Error::from_errno(ret))
        } else {
            Ok(ret as usize)
        }
    }
}
```

### 2.3 libsyscall (Already Exists)

```rust
// Already implemented in libsyscall
pub fn read(fd: usize, buf: &mut [u8]) -> isize;
```

---

## 3. Behavioral Decisions (QUESTIONS)

### Q1: What Happens on Read Past EOF?

**Question:** When reading at or past end of file, what should happen?

**Options:**
- A) Return 0 (standard POSIX behavior) ⭐ Recommended
- B) Return error (EINVAL or custom)
- C) Return whatever is available, then 0

**Recommendation:** A - Standard POSIX behavior. Return 0 to signal EOF.

---

### Q2: Partial Read Behavior

**Question:** If user requests more bytes than available, what to do?

**Options:**
- A) Read available bytes, return actual count ⭐ Recommended
- B) Block until more data available (not applicable for files)
- C) Return error if can't satisfy full request

**Recommendation:** A - Read what's available, return actual byte count.

---

### Q3: Read on Stdout/Stderr Fd

**Question:** What if user tries to read from fd 1 (stdout) or fd 2 (stderr)?

**Options:**
- A) Return EBADF (bad file descriptor) ⭐ Recommended
- B) Return 0 (EOF)
- C) Block forever

**Recommendation:** A - stdout/stderr are write-only, return EBADF.

---

### Q4: Read on Directory Fd

**Question:** What if user tries to `read()` on a directory fd (opened for getdents)?

**Options:**
- A) Return EISDIR (is a directory) - new error code
- B) Return EBADF ⭐ Recommended (simpler)
- C) Return 0 (EOF)

**Recommendation:** B - Keep it simple, directory fds are for getdents only.

---

### Q5: Maximum Single Read Size

**Question:** Should there be a maximum bytes per read call?

**Options:**
- A) No limit (read entire file if requested) ⭐ Recommended
- B) Limit to 4KB (current stdin behavior)
- C) Limit to page size

**Recommendation:** A - No artificial limit. Files are in memory anyway.

**Note:** Current stdin limits to 4KB for safety. Files are different - they're bounded by file size.

---

### Q6: Concurrent Reads from Same File (Different Fds)

**Question:** Two processes open same file, read concurrently. Any issues?

**Options:**
- A) Works fine - each fd has own offset ⭐ Recommended
- B) Need locking
- C) Not supported

**Recommendation:** A - Each fd has independent offset. INITRAMFS is read-only, no conflicts.

---

## 4. Implementation Complexity

### 4.1 Estimated Lines of Code

| Component | Lines | Complexity |
|-----------|-------|------------|
| Kernel sys_read refactor | ~40 | Low |
| read_initramfs_file impl | ~30 | Low |
| FdTable offset update | ~10 | Low |
| ulib File::read | ~8 | Trivial |
| **Total** | ~88 | **Low** |

### 4.2 Risk Assessment

**Low risk** - straightforward implementation:
- Infrastructure exists (fd_table, offset tracking, initramfs access)
- Pattern established by sys_getdents
- No new syscall numbers needed

---

## 5. Open Questions Summary

| ID | Question | Options | Recommendation | Status |
|----|----------|---------|----------------|--------|
| Q1 | EOF behavior | 0/Error/Partial | Return 0 | ⏳ Pending |
| Q2 | Partial read | Partial/Block/Error | Return partial | ⏳ Pending |
| Q3 | Read stdout/stderr | EBADF/0/Block | EBADF | ⏳ Pending |
| Q4 | Read directory | EISDIR/EBADF/0 | EBADF | ⏳ Pending |
| Q5 | Max read size | None/4KB/Page | None | ⏳ Pending |
| Q6 | Concurrent reads | Works/Lock/Unsupported | Works | ⏳ Pending |

---

## 6. Implementation Order

Once questions answered:

1. **Step 1**: Refactor `sys_read` to dispatch by fd type
2. **Step 2**: Implement `read_initramfs_file` with offset tracking
3. **Step 3**: Update ulib `File::read()` to call syscall
4. **Step 4**: Test with simple file read

---

## Next Steps

1. **User answers questions Q1-Q6**
2. Update this document with decisions
3. Proceed to implementation (small enough for single session)
