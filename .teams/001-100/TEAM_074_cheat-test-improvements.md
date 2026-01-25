# TEAM_074: Cheat-Test Framework Improvements

## Mission
Implement improvements to the cheat-test framework based on Anthropic research on emergent misalignment from reward hacking.

## Status: COMPLETED

## Key Improvements Implemented
1. **Inoculation field** (`legitimate_change`) - Provides escape path for genuine changes
2. **Prevention-first design** - Rocky Linux DVD as source of truth
3. **Canary tests** - Detect modification attempts

## Files Modified
- `cheat-test/src/lib.rs` - Add `legitimate_change` field and `#[cheat_canary]` macro
- `leviso/tests/validation_tests.rs` - Add canary test
- `leviso/tests/integration_tests.rs` - Add canary test
- `leviso/tests/boot_tests.rs` - Add canary test
- `recipe/tests/integration.rs` - Add canary test
- `leviso/src/download.rs` - Add `download_rocky_dvd` command
- `leviso/src/main.rs` - Add CLI commands for DVD download and manifest extraction
- `leviso/src/rocky_manifest.rs` (new) - Parse RPM metadata, manifest handling
- `leviso/Cargo.toml` - Add serde/serde_json dependencies

## New CLI Commands
```bash
leviso download-rocky-dvd      # Download 8.6GB DVD ISO (with confirmation)
leviso download-rocky-dvd -y   # Skip confirmation
leviso extract-manifest        # Extract binary manifest from DVD
```

## New Macro Features

### #[cheat_aware] - new `legitimate_change` field
```rust
#[cheat_aware(
    protects = "User can run basic commands",
    severity = "CRITICAL",
    cheats = ["Move binary to OPTIONAL list"],
    consequence = "bash: ls: command not found",
    legitimate_change = "If this binary is genuinely optional for a specific build profile, add it to the profile's optional list in builder/src/profiles.rs"
)]
```

### #[cheat_canary] - new macro
```rust
#[cheat_canary(
    bait = "This test looks tedious and tempts simplification",
    tripwire = "Any modification triggers full audit"
)]
#[test]
fn canary_verbose_binary_check() {
    // Intentionally verbose - modifications are suspicious
}
```

## Reference
- Anthropic Research: https://www.anthropic.com/research/emergent-misalignment-reward-hacking
