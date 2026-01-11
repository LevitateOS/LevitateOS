# TEAM_425: Investigate CI aarch64 Linker Failure

## Status: CONFIRMED

## Symptom

**Expected**: aarch64 kernel build passes in CI
**Actual**: Build fails with `error: linker 'aarch64-linux-gnu-gcc' not found`

**Trigger**: Running `cargo build --target aarch64-unknown-none` in the standalone kernel CI

## Hypotheses

### H1: Missing cross-compiler installation (HIGH confidence)
**Evidence needed**: Compare original CI (1588fc9) with standalone CI (60a7245)
**Status**: CONFIRMED

The original CI workflow included:
```yaml
- name: Install dependencies
  run: |
    sudo apt-get update
    sudo apt-get install -y mtools curl cpio fdisk gcc-aarch64-linux-gnu
```

The standalone version (60a7245) removed this step entirely.

### H2: Wrong linker configuration in Cargo/rustc (LOW confidence)
**Evidence needed**: Check if there's a `.cargo/config.toml` specifying the linker
**Status**: RULED OUT - The linker name `aarch64-linux-gnu-gcc` is the standard Ubuntu package name

### H3: Target specification issue (LOW confidence)
**Evidence needed**: Check if `aarch64-unknown-none` target is correctly installed
**Status**: RULED OUT - Clippy passes (which requires the target), only linking fails

## Root Cause

When commit 60a7245 converted the CI to standalone mode, the dependency installation step was removed. This step was necessary because:

1. Ubuntu runners don't include cross-compilers by default
2. The aarch64 kernel binary requires `aarch64-linux-gnu-gcc` to link
3. x86_64 doesn't need a cross-compiler since the runner is x86_64

## Fix Applied

Added the missing installation step to `.github/workflows/ci.yml`:
```yaml
- name: Install aarch64 cross-compiler
  run: |
    sudo apt-get update
    sudo apt-get install -y gcc-aarch64-linux-gnu
```

## Verification

All CI jobs now pass:
- x86_64 build: PASS
- aarch64 build: PASS
- unit-tests: PASS

## Breadcrumbs

- `.github/workflows/ci.yml:41-44` - TEAM_425 CONFIRMED: Cross-compiler installation restored
