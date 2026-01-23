# LevitateOS Tool Quality Standards

Every production-ready tool MUST meet these standards before release.

---

## 1. Modular Structure

- [ ] Logical separation of concerns (even if single-file for small tools)
- [ ] No circular dependencies
- [ ] Each module testable independently

## 2. Error Handling

- [ ] Numbered error codes (E001, E002, etc.) for scripting
- [ ] Descriptive error messages to stderr
- [ ] FAIL FAST for required components (no warnings for required things)
- [ ] Graceful degradation for optional features only
- [ ] No `unwrap()`/`panic!()` in production paths

## 3. Testing

- [ ] Unit tests for all public functions
- [ ] Edge case tests (empty input, malformed input, boundary conditions)
- [ ] Integration tests (CLI behavior, exit codes)
- [ ] Tests for all error paths

## 4. CI Pipeline (5 jobs minimum)

- [ ] **test**: Run all tests with `cargo test --verbose`
- [ ] **clippy**: Lint with `-D warnings` (deny all warnings)
- [ ] **fmt**: Check formatting with `cargo fmt -- --check`
- [ ] **msrv**: Verify minimum supported Rust version (1.74)
- [ ] **release**: Auto-release on push to master with version bump

## 5. Unix Philosophy Compliance

- [ ] Silent on success (output only the result, no status messages)
- [ ] Loud on failure (clear error messages with codes)
- [ ] Do one thing well
- [ ] Work with other tools (pipeable output)
- [ ] Handle text streams properly

## 6. Documentation

- [ ] README.md: Usage, examples, what it does/doesn't do
- [ ] CLAUDE.md: Developer context and rules
- [ ] Inline comments for non-obvious logic only

## 7. Cargo.toml

- [ ] `rust-version = "1.74"` (MSRV)
- [ ] No unused dependencies
- [ ] Release profile: `strip = true`, `lto = true`
- [ ] `Cargo.lock` committed (for applications)

## 8. Edge Case Handling

- [ ] Empty/whitespace input validation
- [ ] Path validation (exists, is directory, permissions)
- [ ] Special character handling (escaping)
- [ ] Platform-specific quirks documented

---

## Gold Standard: recfstab

The `recfstab` tool exemplifies these standards:

| Requirement | recfstab Status |
|-------------|-----------------|
| Modular structure | 8 modules (lib, main, error, device, mount, filter, fstab, swap) |
| Error codes | E001-E006 with RecfstabError type |
| Unit tests | 72 tests across all modules |
| Integration tests | 19 tests in tests/integration.rs |
| CI pipeline | 5 jobs (test, clippy, fmt, msrv, release) |
| Unix philosophy | Silent success, filtered output |
| Documentation | README.md + CLAUDE.md |
| Cargo.toml | MSRV declared, release profile configured |
| Edge cases | 32 pseudo-fs, swap, zram, symlinks, escaping |

---

## Verification Checklist

After implementation, verify each tool:

```bash
cd <tool>
cargo test --verbose        # All tests pass
cargo clippy -- -D warnings # No warnings
cargo fmt -- --check        # Formatting OK
cargo +1.74 check           # MSRV OK
```

---

## Error Code Pattern

Use this pattern for error codes:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    /// E001: Description
    ErrorVariant,
}

impl ErrorCode {
    pub fn code(&self) -> &'static str {
        match self {
            ErrorCode::ErrorVariant => "E001",
        }
    }
}
```

Error messages format: `E001: description of what went wrong`
