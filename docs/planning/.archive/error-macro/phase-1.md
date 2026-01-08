# Phase 1: Discovery

**Feature:** `define_kernel_error!` Macro  
**Author:** TEAM_153  
**Status:** Complete

---

## Feature Summary

**Problem:** LevitateOS has 7+ error types that all implement the same pattern manually:
- `code() -> u16`
- `name() -> &'static str`
- `Display` impl with format `E{code:04X}: {name}`
- `Error` trait impl

This leads to:
1. ~40 lines of boilerplate per error type
2. No compile-time enforcement of the pattern
3. Risk of format divergence between modules
4. Manual subsystem code calculation (`0xSSCC`)

**Solution:** A declarative macro that generates all boilerplate from a concise definition.

**Who Benefits:**
- Kernel developers adding new error types
- Code reviewers (less to check)
- Future maintainers (single source of truth for pattern)

---

## Success Criteria

1. Macro generates identical output to current manual implementations
2. Existing error types can migrate without behavioral change
3. New error types use macro (enforced by convention/review)
4. Error format `E{code:04X}: {name}` guaranteed by macro
5. Subsystem code validation possible at compile time

---

## Current State Analysis

### Existing Error Types (TEAM_152 implementation)

| Type | Location | Subsystem | Codes |
|------|----------|-----------|-------|
| `MmuError` | `levitate-hal/src/mmu.rs` | 0x01 | 5 variants |
| `ElfError` | `kernel/src/loader/elf.rs` | 0x02 | 9 variants |
| `SpawnError` | `kernel/src/task/process.rs` | 0x03 | 3 variants (nested) |
| `FsError` | `kernel/src/fs/mod.rs` | 0x05 | 7 variants (1 nested) |
| `BlockError` | `kernel/src/block.rs` | 0x06 | 4 variants |
| `NetError` | `kernel/src/net.rs` | 0x07 | 3 variants |
| `FdtError` | `levitate-hal/src/fdt.rs` | 0x09 | 2 variants |

### Current Pattern (Manual)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XxxError {
    /// Description (0xSSCC)
    Variant1,
    Variant2,
}

impl XxxError {
    pub const fn code(&self) -> u16 {
        match self {
            Self::Variant1 => 0xSS01,
            Self::Variant2 => 0xSS02,
        }
    }

    pub const fn name(&self) -> &'static str {
        match self {
            Self::Variant1 => "Description 1",
            Self::Variant2 => "Description 2",
        }
    }
}

impl core::fmt::Display for XxxError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "E{:04X}: {}", self.code(), self.name())
    }
}

impl core::error::Error for XxxError {}
```

### Workarounds Today

None - pattern is manually repeated in each module.

---

## Codebase Reconnaissance

### Modules Affected

| Module | Impact | Notes |
|--------|--------|-------|
| `levitate-hal/src/lib.rs` | Add `error` module | New macro location |
| `levitate-hal/src/mmu.rs` | Migration candidate | First to migrate |
| `levitate-hal/src/fdt.rs` | Migration candidate | Simple, 2 variants |
| `kernel/src/block.rs` | Migration candidate | TEAM_150's original |
| `kernel/src/loader/elf.rs` | Migration candidate | 9 variants |
| `kernel/src/task/process.rs` | Complex | Has nested errors |
| `kernel/src/fs/mod.rs` | Complex | Has nested errors |
| `kernel/src/net.rs` | Migration candidate | Simple, 3 variants |

### Public APIs Involved

- All error types are `pub` and used across crate boundaries
- `code()`, `name()` methods must remain identical
- `Display` format must remain identical

### Tests Impacted

- No direct unit tests for error types
- Integration tests may log error messages (format-sensitive)

### Constraints

1. **no_std compatibility** - Macro must work without std
2. **Nested errors** - Some errors contain other errors (SpawnError, FsError)
3. **Backward compatibility** - API must remain identical
4. **Incremental migration** - Can't change all at once

---

## Exit Criteria for Phase 1

- [x] Problem clearly defined
- [x] Current state documented
- [x] All affected modules identified
- [x] Constraints documented
- [x] Success criteria defined
