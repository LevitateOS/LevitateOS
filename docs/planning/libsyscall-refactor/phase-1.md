# Phase 1: Discovery and Safeguards

## Refactor Summary
`userspace/libsyscall/src/lib.rs` has grown to 1484 lines and contains all syscall logic (numbers, wrappers, errno). We will split it into logical modules (`fs`, `mm`, `process`, `sysno`, `errno`) to improve maintainability and readability.

## Success Criteria
- [ ] `lib.rs` acts primarily as a facade, re-exporting symbols from submodules.
- [ ] Syscall numbers live in `sysno.rs`.
- [ ] Errno codes live in `errno.rs`.
- [ ] Functional wrappers live in domain-specific modules (`fs`, `process`, `mm`, etc.).
- [ ] Public API remains 100% compatible (no changes to `core_utils` or `levbox` required).

## Behavioral Contracts
- The `libsyscall` crate exports must remain unchanged.
- Function signatures (e.g. `read(fd: usize, buf: &mut [u8]) -> isize`) must be preserved.
- Constants (e.g. `O_RDONLY`, `SYS_READ`) must remain available at the top level.

## Existing Context
- `libsyscall` is a `no_std` crate.
- It is consumed by `core_utils` and `levbox`.
- Tests are primarily integration tests in `levbox/src/bin/test/`.

## Golden/Regression Tests
- `cargo build -p levbox` must succeed at all times.
- `suite_test_core` usage of `libsyscall` must not break.
- We will rely on the compiler to catch API breakages (renames, moving without re-exporting).

## Open Questions
- Should we group by "standard" headers (e.g. `fcntl`, `unistd`) or by logical domain (`fs`, `process`)?
  - Decision: Logical domain (`fs`, `process`, `mm`) is more Rust-idiomatic and cleaner than C header mirroring. `lib.rs` will flatten the structure for compatibility.

## Plan

### Step 1: Create Module Structure
- Create `sysno.rs`, `errno.rs`, `fs.rs`, `process.rs`, `mm.rs`, `io.rs`.

### Step 2: Move Constants
- Move syscall numbers to `sysno.rs`.
- Move errno to `errno.rs`.
- Move flags (O_*, PROT_*) to respective modules.

### Step 3: Move Functions
- Move wrappers to respective modules.

### Step 4: Re-export
- Use `pub use module::*;` in `lib.rs`.
