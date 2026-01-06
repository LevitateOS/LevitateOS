# Buffered I/O Feature — Phase 2: Design

**Team:** TEAM_179  
**Created:** 2026-01-06  
**Status:** QUESTIONS PENDING  
**Parent:** `phase-1.md`

---

## 1. Proposed Solution

### 1.1 High-Level Description

Add two wrapper types to `ulib::io`:

1. **`BufReader<R: Read>`** - Buffers reads, provides line-oriented API
2. **`BufWriter<W: Write>`** - Buffers writes, auto-flushes when full

Both transparently implement their respective traits, allowing drop-in replacement.

### 1.2 User-Facing Behavior

```rust
use ulib::fs::File;
use ulib::io::{BufReader, BufWriter, Read, Write};

// Buffered reading
let file = File::open("/config.txt")?;
let mut reader = BufReader::new(file);
let mut line = String::new();
while reader.read_line(&mut line)? > 0 {
    println!("{}", line);
    line.clear();
}

// Buffered writing (conceptual - no file write yet)
let mut writer = BufWriter::new(stdout);
writer.write_all(b"Hello, buffered world!\n")?;
// Auto-flushed when writer drops
```

---

## 2. API Design

### 2.1 BufReader

```rust
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

/// TEAM_179: Buffered reader wrapper.
pub struct BufReader<R> {
    inner: R,
    buf: Vec<u8>,
    pos: usize,   // Next byte to read from buffer
    cap: usize,   // Valid bytes in buffer (buf[0..cap] is valid data)
}

impl<R: Read> BufReader<R> {
    /// Create with default buffer size.
    pub fn new(inner: R) -> Self;
    
    /// Create with custom buffer capacity.
    pub fn with_capacity(capacity: usize, inner: R) -> Self;
    
    /// Get reference to underlying reader.
    pub fn get_ref(&self) -> &R;
    
    /// Get mutable reference to underlying reader.
    pub fn get_mut(&mut self) -> &mut R;
    
    /// Consume and return underlying reader.
    pub fn into_inner(self) -> R;
    
    /// Returns buffered data without consuming.
    pub fn buffer(&self) -> &[u8];
    
    /// Read a line into the provided String.
    /// Returns bytes read (including newline), or 0 at EOF.
    pub fn read_line(&mut self, buf: &mut String) -> Result<usize>;
}

impl<R: Read> Read for BufReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}
```

### 2.2 BufWriter

```rust
/// TEAM_179: Buffered writer wrapper.
pub struct BufWriter<W: Write> {
    inner: Option<W>,  // Option for take() in drop
    buf: Vec<u8>,
}

impl<W: Write> BufWriter<W> {
    /// Create with default buffer size.
    pub fn new(inner: W) -> Self;
    
    /// Create with custom buffer capacity.
    pub fn with_capacity(capacity: usize, inner: W) -> Self;
    
    /// Get reference to underlying writer.
    pub fn get_ref(&self) -> &W;
    
    /// Get mutable reference to underlying writer.
    pub fn get_mut(&mut self) -> &mut W;
    
    /// Consume, flush, and return underlying writer.
    pub fn into_inner(self) -> Result<W>;
    
    /// Returns buffered data waiting to be written.
    pub fn buffer(&self) -> &[u8];
}

impl<W: Write> Write for BufWriter<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
}

impl<W: Write> Drop for BufWriter<W> {
    fn drop(&mut self);  // Best-effort flush
}
```

---

## 3. Behavioral Decisions (QUESTIONS)

### Q1: Default Buffer Size

**Question:** What should the default buffer capacity be?

**Options:**
- A) 1 KB (1024 bytes) - Conservative
- B) 4 KB (4096 bytes) - One page
- C) 8 KB (8192 bytes) - std::io default ⭐ Recommended
- D) Configurable only (no default)

**Recommendation:** C (8 KB) - Matches Rust std, good balance of memory vs syscall reduction.

---

### Q2: BufReader Behavior on Partial Buffer

**Question:** When internal buffer has data but user requests more than available, what to do?

**Options:**
- A) Return buffered data immediately, don't refill ⭐ Recommended
- B) Refill buffer first, then return up to requested amount
- C) Always try to fill user's entire request

**Recommendation:** A - Standard behavior. Return what's buffered, let caller call again if needed.

---

### Q3: BufWriter Flush Trigger

**Question:** When should BufWriter automatically flush?

**Options:**
- A) Only when buffer is completely full ⭐ Recommended
- B) When buffer is 75%+ full
- C) On every write (defeats purpose)

**Recommendation:** A - Flush when full, or when explicitly requested.

---

### Q4: BufWriter Drop Error Handling

**Question:** If flush fails during Drop, what to do?

**Options:**
- A) Silently ignore error (can't propagate from Drop) ⭐ Recommended
- B) Panic
- C) Store error for later retrieval

**Recommendation:** A - Standard behavior. Users should explicitly flush if they care about errors.

---

### Q5: read_line Newline Handling

**Question:** Should `read_line` include the trailing newline in the result?

**Options:**
- A) Yes, include newline (matches std::io::BufRead) ⭐ Recommended
- B) No, strip newline
- C) Configurable

**Recommendation:** A - Match std behavior. Users can `trim()` if needed.

---

### Q6: read_line on Binary Data

**Question:** What if the file contains no newlines (binary file)?

**Options:**
- A) Read entire file into string (could be huge!)
- B) Read up to buffer size, return what's available ⭐ Recommended
- C) Return error

**Recommendation:** B - Return up to buffer size. User shouldn't use `read_line` on binary.

---

### Q7: Empty String Before read_line

**Question:** Should `read_line` clear the string before appending?

**Options:**
- A) No, append to existing content (std behavior) ⭐ Recommended
- B) Yes, clear first

**Recommendation:** A - Match std. Users clear explicitly if needed (shown in example).

---

### Q8: BufWriter write() Return Value

**Question:** What should `write()` return?

**Options:**
- A) Bytes accepted into buffer (may be less than input if buffer fills) ⭐ Recommended
- B) Always return full input length (queue internally)
- C) Block until all bytes written

**Recommendation:** A - Standard behavior. Return bytes actually buffered.

---

## 4. Implementation Complexity

### 4.1 Estimated Lines of Code

| Component | Lines | Complexity |
|-----------|-------|------------|
| BufReader struct + constructors | ~30 | Low |
| BufReader Read impl | ~25 | Medium |
| BufReader read_line | ~35 | Medium |
| BufWriter struct + constructors | ~25 | Low |
| BufWriter Write impl | ~30 | Medium |
| BufWriter Drop | ~10 | Low |
| **Total** | ~155 | **Medium** |

### 4.2 Dependencies

- `alloc::vec::Vec` - For dynamic buffers
- `alloc::string::String` - For read_line output
- Existing `Read` and `Write` traits

---

## 5. Open Questions Summary

| ID | Question | Recommendation | Status |
|----|----------|----------------|--------|
| Q1 | Default buffer size | C (8 KB) | ⏳ Pending |
| Q2 | Partial buffer read | A (return available) | ⏳ Pending |
| Q3 | Flush trigger | A (when full) | ⏳ Pending |
| Q4 | Drop error handling | A (ignore silently) | ⏳ Pending |
| Q5 | Newline in read_line | A (include) | ⏳ Pending |
| Q6 | Binary file read_line | B (up to buffer) | ⏳ Pending |
| Q7 | Clear string before read_line | A (no, append) | ⏳ Pending |
| Q8 | BufWriter write return | A (bytes buffered) | ⏳ Pending |

---

## 6. Implementation Order

Once questions answered:

1. **Step 1**: Implement `BufReader` struct and `Read` trait
2. **Step 2**: Add `read_line()` to `BufReader`  
3. **Step 3**: Implement `BufWriter` struct, `Write` trait, and `Drop`
4. **Step 4**: Add re-exports to `lib.rs`

---

## Next Steps

1. **User answers questions Q1-Q8**
2. Update this document with decisions
3. Proceed to implementation
