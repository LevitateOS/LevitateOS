# Team 302 - Investigate Red-Zone Warning

## Bug Report
- **Symptom**: `'-red-zone' is not a recognized feature for this target (ignoring feature)` message during build.
- **Goal**: Investigate why this warning appears and create a bugfix plan.

## Environment Trace
- Target: `x86_64-unknown-none` (suspected based on red-zone context).
- Tools: LLVM/Clang/Rustc.

## Phase 1 - Understand the Symptom
- [ ] Reproduce the warning.
- [ ] Identify where `-red-zone` is being passed.

## Hypotheses
1. `rustc` target feature syntax has changed or is being misused.
2. The target spec file explicitly passes `-red-zone` in a way that LLVM 19+ (or current version) rejects as a feature flag.
3. A build script or `Cargo.toml` is passing it incorrectly.

## Investigation Log
- **2026-01-08**: Team 302 registered. Starting investigation.
