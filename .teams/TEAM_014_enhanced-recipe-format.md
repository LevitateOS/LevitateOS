# TEAM_014: Enhanced Recipe Format for Package Manager

## Status: COMPLETED

## Goal
Implement enhanced recipe format supporting:
- Version constraints on dependencies
- Features/variants (compile-time options)
- Patches support
- Provides/conflicts relationships
- Split packages (one recipe → multiple packages)

## Summary of Changes

### New Files Created
- `crates/recipe/src/version.rs` - Version parsing and VersionConstraint enum
- `crates/recipe/src/features.rs` - Feature/variant handling and DepSpec
- `crates/recipe/src/executor/patch.rs` - Patch application executor

### Files Modified
- `crates/recipe/src/recipe.rs` - Added PatchSpec, Subpackage, new InstallFile variants
- `crates/recipe/src/lib.rs` - Updated exports
- `crates/recipe/src/executor/mod.rs` - Added patch phase and feature support
- `crates/recipe/src/executor/error.rs` - Added new error variants
- `crates/recipe/src/executor/install.rs` - Added new install targets
- `crates/recipe/src/bin/levitate.rs` - Updated for DepSpec
- `crates/recipe/Cargo.toml` - Added sha2 dependency
- `crates/recipe/tests/audit_bugs.rs` - Updated tests for fixed bugs

## New Recipe Syntax Examples

### Version Constraints
```lisp
(deps
  "openssl >= 1.1.0"
  "zlib"
  "glibc < 3.0"
  "libfoo ~= 2.0")
```

### Features
```lisp
(features
  (default "x264" "opus")
  (x264 "Enable H.264 support")
  (vulkan "Enable Vulkan acceleration" (implies "gpu")))
(deps
  "zlib"
  (if vulkan "vulkan-loader >= 1.3"))
```

### Patches
```lisp
(patches
  "patches/fix-ssl-crash.patch"
  (url "https://example.com/fix.patch" (sha256 "abc123"))
  (strip 1))
```

### Provides/Conflicts
```lisp
(provides "vi" "vim" "editor")
(conflicts "vim" "vim-minimal")
```

### Split Packages
```lisp
(subpackages
  (openssl-dev
    (description "OpenSSL development files")
    (deps "openssl = $VERSION")
    (install
      (to-include "include/openssl")
      (to-pkgconfig "openssl.pc"))))
```

### New Install Targets
```lisp
(install
  (to-include "include/*.h" "mylib")
  (to-pkgconfig "mylib.pc")
  (to-cmake "cmake/Config.cmake")
  (to-systemd "mylib.service")
  (to-completions "mylib.bash" "bash"))
```

## Tests
All 127 tests pass:
- 33 unit tests in lib
- 9 acquire tests
- 33 audit bug tests
- 8 build tests
- 6 cleanup tests
- 8 configure tests
- 5 e2e tests
- 14 executor tests
- 9 install tests
- 2 doc tests
