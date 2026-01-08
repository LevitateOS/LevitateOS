# Investigation: x86_64 Invalid Opcode Recurrence

**Team ID:** TEAM_301
**Bug Summary:** `INVALID OPCODE` panic at `0x100b4` in shell on x86_64 ISO boot.
**Date:** 2026-01-08

## 1. Symptom

- **Context:** Booting x86_64 via `run.sh --iso`.
- **Observed:**
  - Shell banner prints ("LevitateOS Shell... Type 'help'...").
  - Immediately followed by `KERNEL PANIC: ... EXCEPTION: INVALID OPCODE` at `100b4`.
  - The `#` prompt is NOT clearly visible in the recent log snippet, though previous logs might have shown it. The panic seems to happen right around the end of initialization loop entry.

## 2. Hypotheses

1.  **Instruction Pointer Shift:** The previous panic was `0x100b9`. The code changed (added `_start`), likely shifting addresses. `0x100b4` might be the same "bad" instruction (or call site) as before.
2.  **Stack Alignment Ineffective:** The fix (naked `_start` with `and rsp, -16`) might not be sufficient or correct if the issue is inside `libsyscall` or related to how the compiler generates the `call`.
3.  **CPU Feature Mismatch:** Rust `x86_64-unknown-none` might be emitting instructions (AVX? newer SSE?) that `qemu64` doesn't support.
4.  **Memory Corruption:** The instruction stream itself is being corrupted at runtime.

## 3. Evidence Gathering Plan

1.  **Disassemble `shell`:** Map `0x100b4` to the source code.
    - `objdump -d userspace/target/x86_64-unknown-none/release/shell`
2.  **Check CPU Features:** Verify what `qemu64` supports vs what Rust emits.
3.  **Verify Reproduction:** Run locally to confirm if it happens every time.

## 4. Execution Log

- [ ] Disassemble shell.
