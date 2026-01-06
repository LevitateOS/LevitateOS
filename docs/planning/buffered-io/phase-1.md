# Buffered I/O Feature — Phase 1: Discovery

**Team:** TEAM_179  
**Created:** 2026-01-06  
**Status:** Draft  
**Parent Phase:** Phase 10 (ulib)

---

## 1. Feature Summary

### 1.1 Short Description

Implement `BufReader` and `BufWriter` wrapper types that provide buffered access to any `Read` or `Write` implementor, reducing syscall overhead for small reads/writes.

### 1.2 Problem Statement

Currently, every `read()` or `write()` call on a `File` results in a syscall. For applications that read/write small amounts of data frequently (e.g., parsing line-by-line, writing formatted output), this creates significant overhead.

**Example problem:**
```rust
// This makes N syscalls for an N-byte file!
let mut file = File::open("config.txt")?;
let mut byte = [0u8; 1];
while file.read(&mut byte)? > 0 {
    process(byte[0]);
}
```

### 1.3 Who Benefits

- **Text processing utilities** (`cat`, `grep`, line-by-line readers)
- **Configuration parsers** (reading config files byte-by-byte or line-by-line)
- **Log writers** (frequent small writes)
- **Any I/O-heavy application**

---

## 2. Success Criteria

### 2.1 Acceptance Criteria

1. **AC1**: `BufReader::new(reader)` wraps any `Read` with internal buffer
2. **AC2**: `BufWriter::new(writer)` wraps any `Write` with internal buffer
3. **AC3**: `BufReader` provides `read_line()` and `lines()` iterator
4. **AC4**: `BufWriter` auto-flushes when buffer is full
5. **AC5**: `BufWriter` flushes on drop (best-effort)
6. **AC6**: Both implement their respective traits (`Read`/`Write`)

### 2.2 Definition of Done

- [ ] `BufReader<R: Read>` implemented
- [ ] `BufWriter<W: Write>` implemented
- [ ] `read_line()` works for `BufReader`
- [ ] Drop impl flushes `BufWriter`
- [ ] Basic usage compiles and works

---

## 3. Current State Analysis

### 3.1 How the System Works Today

**Without buffering:**
- `File::read()` → `libsyscall::read()` → kernel syscall (every time!)
- `libsyscall::write()` → kernel syscall (every time!)
- No line-oriented reading API

**Workarounds:**
- Read entire file into memory with large buffer
- Manually manage buffers in application code

### 3.2 Existing Infrastructure

**Already implemented:**
- `Read` trait with `read()` and `read_exact()` methods
- `Write` trait with `write()`, `write_all()`, and `flush()` methods
- `File` implements `Read`
- `Error` and `Result` types
- `alloc` crate available (Vec, String)

**Missing:**
- `BufReader` wrapper
- `BufWriter` wrapper
- Line-oriented reading (`read_line`, `lines`)

---

## 4. Codebase Reconnaissance

### 4.1 Code Areas to Touch

| Component | File | Changes |
|-----------|------|---------|
| BufReader | `userspace/ulib/src/io.rs` | Add BufReader struct and impl |
| BufWriter | `userspace/ulib/src/io.rs` | Add BufWriter struct and impl |
| Re-exports | `userspace/ulib/src/lib.rs` | Export new types |

### 4.2 Key Type Definitions (from std for reference)

```rust
// std::io::BufReader
pub struct BufReader<R> {
    inner: R,
    buf: Box<[u8]>,
    pos: usize,  // Current read position in buffer
    cap: usize,  // Valid bytes in buffer
}

// std::io::BufWriter  
pub struct BufWriter<W: Write> {
    inner: Option<W>,
    buf: Vec<u8>,
    panicked: bool,
}
```

### 4.3 Tests That May Be Impacted

- None currently (no existing BufReader/BufWriter tests)
- May want to add basic tests

---

## 5. Constraints

### 5.1 No-std Environment

- Must use `alloc` for `Vec`/`Box`
- No `std::io::BufRead` trait to reference directly

### 5.2 Buffer Sizing

- Default buffer size should be reasonable (8KB typical for std)
- `with_capacity()` constructors for custom sizes

### 5.3 Error Handling

- Must handle partial reads/writes correctly
- Flush errors on drop should be silent (can't propagate in Drop)

---

## 6. Phase 1 Outputs

### 6.1 Problem Understanding

✅ Problem clear: Unbuffered I/O is inefficient for small operations.

### 6.2 Solution Path

Implement standard `BufReader<R>` and `BufWriter<W>` patterns:
- Internal buffer managed by wrapper
- Refill/flush buffer as needed
- Implement `Read`/`Write` traits for transparent use

### 6.3 Complexity Estimate

**Medium complexity** (~150-200 lines):
- `BufReader`: ~80 lines (struct, new, Read impl, read_line)
- `BufWriter`: ~70 lines (struct, new, Write impl, Drop)
- Helper methods: ~30 lines

---

## Next Steps

→ Proceed to **Phase 2: Design** for API details and behavioral questions.
