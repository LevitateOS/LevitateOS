# Team 300: Investigate `run-term.sh` Failure

## Symptom Description
- **Expected Behavior**: `bash ./run-term.sh` should run successfully.
- **Actual Behavior**: Multiple build failures occurred:
    1. Unknown codegen option: `no-red-zone`.
    2. `unsafe_op_in_unsafe_fn` errors due to Rust 2024 edition.
    3. `arch::x86_64::cpu::get_pcr` access failure (private module).
    4. `arch::x86_64::gdt` unresolved import.
    5. Architecture-specific field access in `kernel/src/task/thread.rs`.

## Hypotheses
1. `no-red-zone` is deprecated in favor of `target-feature=-red-zone`. (Confirmed)
2. Rust 2024 requires explicit `unsafe` blocks in `unsafe fn`. (Confirmed)
3. Module visibility issues in `crate::arch::x86_64`. (Confirmed)

## Investigation Logs
- [2026-01-08] Team 300 started investigation.
- [2026-01-08] Fixed `no-red-zone` in `.cargo/config.toml` (switched to `target-feature=-red-zone`).
- [2026-01-08] Allowed `unsafe_op_in_unsafe_fn` in `Cargo.toml` as a temporary fix for Rust 2024.
- [2026-01-08] Fixed `get_pcr` access by using public re-exports in `crate::arch::cpu`.
- [2026-01-08] Fixed `gdt` import in `kernel/src/arch/x86_64/cpu.rs`.
- [2026-01-08] Refactored `kernel/src/task/thread.rs` to use arch-agnostic `Context::new` and frame setters.
- [2026-01-08] Verified successful boot and shell interaction with `run-term.sh`.
- [2026-01-08] Reduced logging noise by commenting out syscall traces in `arch/x86_64/syscall.rs`.
- [2026-01-08] **Current Status**: Investigating `INVALID OPCODE` panic at `0x100b9` after typing `cat hello.txt`.

## Current Focus
- Identifying why `cat` triggers an invalid opcode panic.
- Address is `0x100b9`, which is in userspace.

## Reusable Knowledge for Future Teams (Rule 10)
- **Rustflag Gotcha**: `no-red-zone` is deprecated. Always use `"-C", "target-feature=-red-zone"` in `.cargo/config.toml`.
- **Rust 2024 Transition**: The project is moving to Rust 2024. New `unsafe` code MUST be wrapped in `unsafe { ... }` blocks even if the surrounding function is `unsafe`. `unsafe_op_in_unsafe_fn = "allow"` is a temporary shim.
- **Task Abstraction**: Use `Context::new(stack_top, entry)` and `frame.set_return(val)`/`frame.set_sp(val)` instead of direct register access to keep code architecture-agnostic.

## Remaining TODOs
- [ ] Fix `INVALID OPCODE` panic at `0x100b9`.
- [ ] Clean up remaining high-frequency debug logs:
    - `[TTY_READ]`
    - `[ELF] WRITING GOT ENTRY`
- [ ] Properly address `unsafe_op_in_unsafe_fn` by adding `unsafe` blocks.
- [ ] Restore syscall logging but gate it behind a feature flag instead of commenting it out.
