# TEAM_139: LevitateOS Build System Fixes

## Status: Complete

## Problem

The leviso build had multiple pre-existing issues that prevented successful ISO generation. These were discovered while trying to test TEAM_138 (license tracking).

## Issues Fixed

### 1. Recipe JSON Output Corruption
- **File:** `tools/recipe/src/helpers/util/log.rs`
- **Issue:** `log()` function used `println!` (stdout) instead of `eprintln!` (stderr)
- **Impact:** Log messages corrupted JSON output, causing intermittent parse failures
- **Fix:** Changed `println!` to `eprintln!`

### 2. Missing Packages in packages.rhai
- **File:** `leviso/deps/packages.rhai`
- **Added packages:**
  - git-core
  - python3, python3-libs
  - zip, unzip
  - tree
  - pipewire, pipewire-libs, pipewire-pulseaudio, pipewire-alsa, pipewire-utils
  - wireplumber, wireplumber-libs
  - pulseaudio-utils, pulseaudio-libs
  - polkit, polkit-libs
  - upower, upower-libs
  - hwloc-libs (htop dependency)
  - ocl-icd (htop dependency)

### 3. Wrong EPEL URLs in epel.rhai
- **File:** `leviso/deps/epel.rhai`
- **Issues:**
  - htop version wrong: `3.4.0-1` → `3.3.0-5`
  - p7zip not in EPEL 10, need Fedora source
- **Fix:** Updated URLs

### 4. Non-ELF Files Breaking readelf (Shell Scripts, Unreadable Binaries)
- **File:** `leviso-elf/src/analyze.rs`
- **Issues:**
  - 7za is a shell script wrapper, not an ELF binary
  - sudo binaries have setuid permissions without read permission
- **Fix:** Added handling for "Failed to read file header" and "is not readable" errors
- **Result:** Non-ELF files return empty dependency list instead of failing

### 5. Missing Private Library Path for PulseAudio
- **File:** `leviso/src/build/libdeps.rs`
- **Issue:** `libpulsecommon-17.0.so` is in `/usr/lib64/pulseaudio/`, not standard paths
- **Fix:** Added `usr/lib64/pulseaudio` to `EXTRA_LIB_PATHS`

### 6. Binaries Not in Expected Locations (libexec vs sbin)
- **Files:** `distro-spec/src/shared/components.rs`, `leviso/src/component/definitions.rs`
- **Issues:** Several daemons are in libexec, not sbin:
  - `bluetoothd` → `/usr/libexec/bluetooth/`
  - `polkitd` → `/usr/lib/polkit-1/`
  - `udisksd` → `/usr/libexec/udisks2/`
  - `upowerd` → `/usr/libexec/`
- **Fix:**
  - Emptied `BLUETOOTH_SBIN`, `POLKIT_SBIN`, `UDISKS_SBIN`, `UPOWER_SBIN`
  - Added appropriate `config_trees` to copy libexec directories

### 7. Non-Existent Binaries in Component Lists
- **File:** `distro-spec/src/shared/components.rs`
- **Issues:**
  - `python` doesn't exist (only `python3`)
  - `7z`, `7zr` don't exist (only `7za`)
  - `pacmd` doesn't exist (pipewire provides `pactl` but not `pacmd`)
- **Fix:** Removed non-existent binaries from lists

### 8. Missing D-Bus Service File
- **File:** `leviso/src/component/definitions.rs`
- **Issue:** `org.freedesktop.ReserveDevice1.service` not present in Rocky
- **Fix:** Removed from pipewire config_files

### 9. Built-in systemd Units Listed as Files
- **File:** `distro-spec/src/shared/components.rs`
- **Issue:** `-.slice`, `system.slice`, `machine.slice` are built-in to systemd, not files
- **Fix:** Removed from ESSENTIAL_UNITS (kept only `user.slice`)

### 10. systemd-fstab-generator Not Being Copied
- **Files:** `distro-spec/src/shared/components.rs`, `leviso/src/component/executor.rs`
- **Issue:** Generator was listed in ESSENTIAL_UNITS but is a binary, not a unit
- **Fix:**
  - Removed from ESSENTIAL_UNITS
  - Added code in SystemdBinaries handler to copy system-generators directory

## All Changes Made

| File | Change | Status |
|------|--------|--------|
| `tools/recipe/src/helpers/util/log.rs` | `println!` → `eprintln!` | Done |
| `leviso/deps/packages.rhai` | Added missing packages | Done |
| `leviso/deps/epel.rhai` | Fixed URLs | Done |
| `leviso-elf/src/analyze.rs` | Handle non-ELF and unreadable files | Done |
| `leviso/src/build/libdeps.rs` | Added pulseaudio lib path | Done |
| `distro-spec/src/shared/components.rs` | Fixed binary lists, removed built-in slices | Done |
| `leviso/src/component/definitions.rs` | Fixed service config_trees/config_files | Done |
| `leviso/src/component/executor.rs` | Added system-generators copying | Done |

## Build Result

**ISO successfully built:** `output/levitateos.iso`

Note: Hardware compatibility warnings for Homelab/Server and Steam Deck profiles are expected - those kernel configs are optional for those specific hardware profiles.

## Next Steps

- Run `cargo run -- run` to boot the ISO
- Verify license tracking (TEAM_138) is working
- Run fsdbg to verify rootfs contents
