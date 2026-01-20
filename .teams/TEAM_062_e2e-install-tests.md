# TEAM_062: E2E Installation Test-Driven Development

## Status: COMPLETE

## Objective
Create an iterative test-driven development system for the LevitateOS installation process.

## Completed Work

### Phase 0: Add Missing Tools to leviso ✓
Added installation tools to `leviso/src/initramfs/mod.rs`:

**COREUTILS added:**
- `tar` - Extract stage3 tarball
- `sed` - Config file editing
- `bootctl` - systemd-boot installation
- `localedef` - Locale generation

**SBIN_UTILS added:**
- `sfdisk` - Non-interactive partitioning
- `useradd`, `groupadd`, `chpasswd` - User management

### Phase 0.5: Add Filesystem Modules ✓
Added kernel modules to `leviso/src/initramfs/modules.rs`:
- `ext4.ko.xz` and dependencies (`mbcache`, `jbd2`)
- `vfat.ko.xz` and dependency (`fat`)

Updated `leviso/profile/init` to load these modules at boot.

### Phase 1: install-tests Infrastructure ✓
Created `install-tests` submodule with:

**QEMU Infrastructure:**
- `src/qemu/builder.rs` - QemuBuilder for test VM configuration
- `src/qemu/console.rs` - Console with command execution and exit code capture

**Step Framework:**
- `src/steps/mod.rs` - Step trait and registry
- 16 installation steps across 5 phases

**CLI:**
```bash
cargo run -- list              # Show all steps
cargo run -- run               # Run all steps
cargo run -- run --step 3      # Run specific step
cargo run -- run --phase 2     # Run phase
```

### Git Submodules ✓
- `install-tests/` -> git@github.com:LevitateOS/install-tests.git
- `stage-3/` -> git@github.com:LevitateOS/stage-3.git

## Test Results

**Phase 1 (Boot):**
- Step 1: UEFI check - PASS (direct kernel boot surprisingly passes)
- Step 2: Clock sync - FAIL (year parsing issue with ANSI codes)

**Phase 2 (Disk):** ALL PASS
- Step 3: Identify disk - PASS
- Step 4: Partition (GPT) - PASS
- Step 5: Format (FAT32/ext4) - PASS
- Step 6: Mount - PASS

## Known Issues

1. **Year parsing in step 2** - The `date +%Y` output contains ANSI escape codes that break integer parsing. Low priority fix.

2. **UEFI not required for direct kernel boot** - Direct kernel boot bypasses UEFI firmware, so step 1 shouldn't pass. May need to boot from ISO for full UEFI testing.

## Key Decisions

### Command Execution Pattern
```rust
// Send: command; echo '___INSTALL_TEST_DONE___' $?
// Parse: marker at START of trimmed line, followed by exit code
```

### Output Filtering
Lines containing `root@` or `# ` are filtered out to avoid capturing command echo.

### Module Load Order
Filesystem modules must load in dependency order: `mbcache` → `jbd2` → `ext4`

## Files Modified

**leviso/src/initramfs/mod.rs:**
- Added tar, sed, bootctl, localedef to COREUTILS
- Added sfdisk, useradd, groupadd, chpasswd to SBIN_UTILS

**leviso/src/initramfs/modules.rs:**
- Added ext4, jbd2, mbcache, fat, vfat modules

**leviso/profile/init:**
- Added load_module function
- Added filesystem module loading

**install-tests/** (new submodule):
- Complete E2E test infrastructure
- 16 installation step implementations

## Next Steps

1. Fix year parsing bug in step 2
2. Implement stage-3 tarball builder
3. Test phases 3-5 once stage-3 is ready
4. Add ISO boot mode for true UEFI testing
