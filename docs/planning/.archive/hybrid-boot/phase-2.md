# Phase 2: Design — Hybrid Boot Specification

## Proposed Solution
Transition the current sequential `kmain` into a rigorous **Boot State Machine**. Each stage must explicitly define its entry/exit conditions and errors.

## API Design
### `BootStage` Enum
A central enum in `kernel/src/main.rs` to track system progression:
```rust
enum BootStage {
    EarlyHAL,   // SEC/SetupArch
    MemoryMMU,  // PEI/MMInit
    BootConsole,// DXE/ConsoleInit
    Discovery,  // DXE/BDS/VFS
    SteadyState // BDS/Init
}
```

## Behavioral Decisions
1. **[SPEC-1] Fallback Console**: If GPU Terminal fails to initialize or is disabled in DTB (Stage 3), the kernel must fallback to serial-only logging but continue to Stage 4.
2. **[SPEC-2] Non-Destructive Cursor**: Maintain the pixel save/restore invariant from `terminal.rs`.
3. **[SPEC-3] Interactive Backspace**: Explicitly handle ASCII `0x08` as a destructive erase with line-wrap.
4. **[SPEC-4] Initrd Failure Policy**: If Stage 4 (Discovery) fails to locate the initrd, the kernel must drop to a minimalist "Maintenance Shell" via UART/Console rather than a silent panic.

## Steps and Units of Work
### Step 1: State Machine Definition
- **UoW 1**: Define `BootStage` enum and transition logic in `phase-2-step-1-uow-1.md`.
- **UoW 2**: Implement stage error handling to support the SPEC-4 Maintenance Shell transition.

### Step 2: Interaction Specification Finalization
- **UoW 1**: Formalize ANSI escape sequence support level (Target: VT100 subset).
