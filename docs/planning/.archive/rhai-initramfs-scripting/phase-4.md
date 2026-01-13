# Phase 4: Integration - Rhai Initramfs Scripting

## Integration Points

### Commands That Use Initramfs

| Command | Current Behavior | New Behavior |
|---------|------------------|--------------|
| `cargo xtask build all` | Calls `create_initramfs()` | Calls `initramfs::build_initramfs()` |
| `cargo xtask build userspace` | Calls `create_initramfs()` | Calls `initramfs::build_initramfs()` |
| `cargo xtask build initramfs` | Calls `create_initramfs()` | Calls `initramfs::build_initramfs()` |
| `cargo xtask build iso` | Calls `create_initramfs()` or `create_test_initramfs()` | Calls `initramfs::build_initramfs()` with appropriate `BUILD_TYPE` |
| `cargo xtask run` | Uses pre-built initramfs | No change |

### Build Flow Integration

```
┌─────────────────────────────────────────────────────────────────┐
│                     cargo xtask build all                        │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 1. Build sysroot (if missing)                                   │
│    - super::sysroot::build_sysroot()                            │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 2. Build userspace (bare-metal binaries)                        │
│    - build_userspace()                                          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 3. Build initramfs (NEW - Rhai-based)                           │
│    ┌──────────────────────────────────────────────────────────┐ │
│    │ a. Load initramfs.rhai                                   │ │
│    │ b. Execute script, collect entries                       │ │
│    │ c. For each App entry:                                   │ │
│    │    - Clone repo if needed                                │ │
│    │    - Build against sysroot                               │ │
│    │ d. Copy files, userspace binaries, app binaries          │ │
│    │ e. Create symlinks                                       │ │
│    │ f. Generate CPIO archive                                 │ │
│    └──────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 4. Install to disk                                              │
│    - disk::install_userspace_to_disk()                          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ 5. Build kernel                                                 │
│    - build_kernel_with_features()                               │
└─────────────────────────────────────────────────────────────────┘
```

### Module Dependencies

```
main.rs
    └── build/
        ├── mod.rs
        ├── commands.rs ──────► initramfs.rs (NEW)
        │                           │
        │                           ├── Uses sysroot.rs for RUSTFLAGS
        │                           └── Replaces apps.rs
        └── sysroot.rs
```

## Test Strategy

### Unit Tests

```rust
// xtask/src/build/initramfs.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_script() {
        let script = r#"
            userspace("init", #{ required: true });
        "#;
        let entries = parse_script_string(script, "x86_64", "release").unwrap();
        assert_eq!(entries.len(), 1);
        match &entries[0] {
            InitramfsEntry::Userspace { name, required } => {
                assert_eq!(name, "init");
                assert!(*required);
            }
            _ => panic!("Expected Userspace entry"),
        }
    }

    #[test]
    fn test_arch_conditional() {
        let script = r#"
            if ARCH == "x86_64" {
                file("x86.bin", "firmware.bin");
            }
            if ARCH == "aarch64" {
                file("arm.bin", "firmware.bin");
            }
        "#;

        let x86_entries = parse_script_string(script, "x86_64", "release").unwrap();
        assert_eq!(x86_entries.len(), 1);

        let arm_entries = parse_script_string(script, "aarch64", "release").unwrap();
        assert_eq!(arm_entries.len(), 1);
    }

    #[test]
    fn test_app_with_all_options() {
        let script = r#"
            app("test", #{
                repo: "https://example.com/repo",
                package: "test-pkg",
                binary: "test-bin",
                features: ["a", "b"],
                required: false,
                symlinks: ["x", "y"],
                no_default_features: true,
            });
        "#;
        let entries = parse_script_string(script, "x86_64", "release").unwrap();
        match &entries[0] {
            InitramfsEntry::App {
                name, repo, package, binary, features, required, symlinks, no_default_features
            } => {
                assert_eq!(name, "test");
                assert_eq!(repo, "https://example.com/repo");
                assert_eq!(package, "test-pkg");
                assert_eq!(binary, "test-bin");
                assert_eq!(features, &vec!["a".to_string(), "b".to_string()]);
                assert!(!*required);
                assert_eq!(symlinks, &vec!["x".to_string(), "y".to_string()]);
                assert!(*no_default_features);
            }
            _ => panic!("Expected App entry"),
        }
    }

    #[test]
    fn test_syntax_error() {
        let script = "this is not valid rhai {{{";
        let result = parse_script_string(script, "x86_64", "release");
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_required_field() {
        let script = r#"
            app("test", #{});  // Missing repo
        "#;
        let entries = parse_script_string(script, "x86_64", "release").unwrap();
        // App with empty repo should be caught during build, not parse
        match &entries[0] {
            InitramfsEntry::App { repo, .. } => {
                assert!(repo.is_empty());
            }
            _ => panic!("Expected App entry"),
        }
    }
}
```

### Integration Tests

```rust
// xtask/tests/initramfs_integration.rs

#[test]
fn test_default_script_loads() {
    // Ensure default initramfs.rhai can be parsed
    let entries = initramfs::load_script("x86_64", "release").unwrap();
    assert!(!entries.is_empty());

    // Should have init and shell
    let has_init = entries.iter().any(|e| matches!(e,
        InitramfsEntry::Userspace { name, .. } if name == "init"
    ));
    let has_shell = entries.iter().any(|e| matches!(e,
        InitramfsEntry::Userspace { name, .. } if name == "shell"
    ));
    assert!(has_init);
    assert!(has_shell);
}

#[test]
fn test_build_type_conditionals() {
    let release_entries = initramfs::load_script("x86_64", "release").unwrap();
    let test_entries = initramfs::load_script("x86_64", "test").unwrap();

    // Test entries might have additional test fixtures
    // (depends on script content)
}
```

### End-to-End Tests

Add to existing behavior tests:
1. `cargo xtask build all` succeeds
2. Resulting initramfs contains expected binaries
3. OS boots and can execute binaries from initramfs

## Impact Analysis

### Backward Compatibility

| Aspect | Impact | Mitigation |
|--------|--------|------------|
| CLI commands | None - same commands | N/A |
| Build output | Same CPIO archive | Verify with diff |
| CI pipelines | None | N/A |
| Developer workflow | Must create `initramfs.rhai` | Ship default script |

### Breaking Changes

1. **apps.rs removal**: Code that imports `build::apps::*` will break
   - Mitigation: Update all call sites before removal
   - Affected: `main.rs` build command handlers

2. **New dependency**: `rhai` crate added
   - Impact: Slightly longer xtask compile time
   - Mitigation: Use release builds for xtask

### Performance Impact

| Operation | Before | After | Notes |
|-----------|--------|-------|-------|
| Script loading | N/A | ~50ms | One-time cost |
| Entry parsing | Instant (static) | ~10ms | Rhai execution |
| App building | Same | Same | No change |
| CPIO creation | Same | Same | No change |

### Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Rhai breaking change | Low | Medium | Pin to specific version |
| Script syntax confusion | Medium | Low | Good documentation, examples |
| Performance regression | Low | Low | Profile before/after |
| CI failures | Medium | Medium | Test thoroughly before merge |
