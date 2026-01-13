# Phase 5: Polish - Rhai Initramfs Scripting

## Cleanup Tasks

### Code Cleanup

- [ ] Remove `xtask/src/build/apps.rs` after migration complete
- [ ] Remove unused imports from `commands.rs`
- [ ] Run `cargo clippy` on xtask, fix warnings
- [ ] Run `cargo fmt` on all modified files

### Error Message Polish

Ensure all error messages are actionable:

```rust
// Bad
bail!("Script error");

// Good
bail!(
    "Failed to parse initramfs.rhai at line {}:\n  {}\n\n\
     Hint: Check Rhai syntax at https://rhai.rs/book/",
    line_number, error_message
);
```

### Logging Improvements

```rust
// Verbose mode output
println!("📜 Loading initramfs.rhai...");
println!("   ARCH={}, BUILD_TYPE={}", arch, build_type);
println!("📋 Found {} entries:", entries.len());
for entry in &entries {
    match entry {
        InitramfsEntry::File { dest, .. } => println!("   📄 file: {}", dest),
        InitramfsEntry::Userspace { name, .. } => println!("   📦 userspace: {}", name),
        InitramfsEntry::App { name, .. } => println!("   🔧 app: {}", name),
        InitramfsEntry::Symlink { link_name, .. } => println!("   🔗 symlink: {}", link_name),
    }
}
```

## Documentation Updates

### Files to Update

| File | Changes |
|------|---------|
| `CLAUDE.md` | Update "Build & Development Commands" section |
| `README.md` | Add section about initramfs customization |
| `docs/ARCHITECTURE.md` | Document Rhai scripting system |

### CLAUDE.md Updates

Add to Build & Development Commands:

```markdown
### Initramfs Configuration

The initramfs contents are defined in `initramfs.rhai` at the repository root.

```bash
# Build initramfs from script
cargo xtask build initramfs

# The script supports:
# - Static files: file("src", "dest")
# - Userspace binaries: userspace("name", #{ required: true })
# - External apps: app("name", #{ repo: "...", ... })
# - Conditionals: if ARCH == "x86_64" { ... }
```

See `initramfs.rhai` for full documentation of available functions.
```

### Inline Script Documentation

The default `initramfs.rhai` should be self-documenting:

```rhai
// ============================================================================
// initramfs.rhai - LevitateOS Initramfs Definition
// ============================================================================
//
// This script defines what goes into the initial RAM filesystem (initramfs).
// The initramfs is loaded by the bootloader and mounted as the root filesystem
// during early boot.
//
// SYNTAX REFERENCE:
// -----------------
//
// Static files:
//   file("source/path", "dest_name")
//   file("source/path", "dest_name", #{ required: false })
//
// Userspace binaries (bare-metal, from crates/userspace):
//   userspace("binary_name", #{ required: true })
//
// External apps (cloned from git, built against c-gull sysroot):
//   app("name", #{
//       repo: "https://github.com/...",   // Required
//       package: "crate-name",            // Default: name
//       binary: "output-binary",          // Default: name
//       features: ["a", "b"],             // Default: []
//       required: true,                   // Default: true
//       symlinks: ["alias1", "alias2"],   // Default: []
//       no_default_features: false,       // Default: false
//   })
//
// Symlinks:
//   symlink("target", "link_name")
//
// BUILT-IN VARIABLES:
// -------------------
//   ARCH       - "x86_64" or "aarch64"
//   BUILD_TYPE - "release", "test", or "verbose"
//
// CONDITIONALS:
// -------------
//   if ARCH == "x86_64" { ... }
//   if BUILD_TYPE == "test" { ... }
//
// ============================================================================
```

## Handoff Notes

### For Future Maintainers

1. **Rhai version**: Pinned to 1.23.x. Test before upgrading.
2. **Script location**: `initramfs.rhai` at repo root. Do not move without updating all references.
3. **Error handling**: All Rhai errors include line numbers. Preserve this.
4. **Thread safety**: Builder uses `Arc<Mutex<>>` for Rhai callback compatibility.

### Known Limitations

1. **No script imports**: Cannot `import` other .rhai files (Phase 1 limitation)
2. **No custom build commands**: All apps use standard cargo build
3. **Sequential builds**: Apps built one at a time (parallelization deferred)

### Future Enhancements

1. **Script imports**: `import "apps/coreutils.rhai"`
2. **Parallel builds**: Build independent apps concurrently
3. **Pre-built binaries**: Download instead of build for faster CI
4. **Validation command**: `cargo xtask initramfs validate`
5. **Dry-run mode**: Show what would be built without building

## Checklist Before Merge

- [ ] All existing tests pass
- [ ] New unit tests for Rhai parsing
- [ ] Integration test for default script
- [ ] `cargo xtask build all` works on x86_64
- [ ] `cargo xtask build all` works on aarch64
- [ ] CLAUDE.md updated
- [ ] Team file updated with completion status
- [ ] No clippy warnings
- [ ] Code formatted with rustfmt

## Post-Merge Tasks

1. Monitor CI for any regressions
2. Update any external documentation
3. Announce in project changelog
4. Consider blog post about Rhai for OS configuration
