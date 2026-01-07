# Phase 4: Integration and Testing

## Integration Points
- No changes to kernel required (unless bugs found).
- Binaries will be built as part of `levbox` package.
- `xtask` should pick them up automatically if they are in `src/bin/test/` (based on previous conversations about binary migration).

## Test Strategy
- **Manual Verification**: Run each binary in QEMU.
- **Automated Verification**: If `test_runner` supports it, add them to `suite_test_core` or run them sequentially.
- **Criteria**: All return 0 (success) and print PASS.

## Impact Analysis
- Trivial impact on build time.
- High value for regression protection.

