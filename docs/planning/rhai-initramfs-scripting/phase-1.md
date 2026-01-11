# Phase 1: Discovery - Rhai Initramfs Scripting

## Feature Summary

### Problem
The current initramfs build system has configuration scattered across multiple Rust files:
- External apps defined in `xtask/src/build/apps.rs` (static `APPS` array)
- Userspace binaries hardcoded in `create_initramfs()` in `commands.rs`
- Static files hardcoded in `create_initramfs()`
- No way to have conditional logic (e.g., different contents for test vs production)
- Adding a new app requires modifying Rust code and recompiling xtask

### Who Benefits
- **Developers**: Easier to add/remove initramfs components without touching Rust
- **Contributors**: Lower barrier - edit a script file instead of learning xtask internals
- **CI/Testing**: Can have different initramfs configurations for different test scenarios
- **Future**: Foundation for user-customizable OS images

### Proposed Solution
Replace the static Rust registry with a Rhai script (`initramfs.rhai`) that declaratively defines initramfs contents with full programming capabilities (conditionals, loops, functions).

## Success Criteria

1. **Single source of truth**: All initramfs contents defined in one script file
2. **Programmable**: Support conditionals based on `ARCH`, `BUILD_TYPE`, etc.
3. **Backward compatible**: `cargo xtask build initramfs` works unchanged
4. **Fail-fast**: Missing required components fail with clear errors
5. **Extensible**: Easy to add new component types (apps, files, symlinks)
6. **Testable**: Script syntax errors caught early with good error messages

### Acceptance Tests
- [ ] `cargo xtask build all` works with new Rhai-based system
- [ ] Adding a new app only requires editing `initramfs.rhai`
- [ ] Conditional logic works (e.g., `if ARCH == "x86_64"`)
- [ ] Script errors produce clear, actionable messages
- [ ] Existing behavior preserved (same binaries in initramfs)

## Current State Analysis

### Current Architecture

```
xtask/src/build/
├── apps.rs       # Static APPS registry (ExternalApp structs)
├── commands.rs   # create_initramfs() with hardcoded logic
├── sysroot.rs    # c-gull sysroot building
└── mod.rs
```

### Current Flow
1. `build_all()` calls `apps::ensure_all_built()` for external apps
2. `create_initramfs()` iterates over `apps::APPS` registry
3. Hardcoded logic copies userspace binaries (init, shell)
4. Creates CPIO archive

### Pain Points
- **Scattered definitions**: Apps in `apps.rs`, userspace in `commands.rs`
- **No conditionals**: Can't vary contents by arch/build type
- **Recompile required**: Adding an app means editing Rust and rebuilding xtask
- **No validation**: Script-level validation impossible with static arrays

## Codebase Reconnaissance

### Files to Modify
| File | Changes |
|------|---------|
| `xtask/Cargo.toml` | Add `rhai` dependency |
| `xtask/src/build/mod.rs` | Add `initramfs` module |
| `xtask/src/build/initramfs.rs` | New: Rhai engine + InitramfsBuilder |
| `xtask/src/build/commands.rs` | Replace hardcoded logic with script execution |
| `xtask/src/build/apps.rs` | Keep for backward compat, or merge into initramfs.rs |

### New Files
| File | Purpose |
|------|---------|
| `initramfs.rhai` | Main initramfs definition script |
| `initramfs-test.rhai` | Optional: test-specific initramfs |

### APIs Affected
- `create_initramfs(arch)` - Will call Rhai engine instead of iterating static array
- `create_test_initramfs(arch)` - May use different script or script variables
- `apps::APPS` - May be deprecated or generated from script

### Tests Affected
- `apps.rs` unit tests - May need updating
- Behavior tests - Should pass unchanged (same output)

## Constraints

1. **No runtime dependencies**: Rhai is embedded, no external interpreter needed
2. **Build-time only**: Scripts run during `cargo xtask`, not at OS runtime
3. **Sandboxed**: Rhai scripts cannot access filesystem directly (only via exposed functions)
4. **Performance**: Script evaluation should be fast (<1s for typical scripts)
5. **Error handling**: Script errors must be clear and point to line numbers

## Open Questions

1. **Script location**: Root (`initramfs.rhai`) or `xtask/scripts/initramfs.rhai`?
2. **Multiple scripts**: One script with conditionals, or multiple script files?
3. **Backward compatibility**: Keep `apps.rs` as fallback, or fully migrate?
