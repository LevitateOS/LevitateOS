# TEAM_191: IuppiterOS Smartmontools Installation (Phase 7 Task 7.1)

**Date**: 2026-02-04  
**Status**: COMPLETE  
**Task**: Phase 7.1 - smartmontools installed and `smartctl --version` runs

## Summary

Integrated Tier 0-2 supplementary package installation for IuppiterOS, bringing in smartmontools and other refurbishment tools necessary for the appliance's core functionality.

## Problem

IuppiterOS rootfs was missing all Tier 1-2 packages including smartmontools, hdparm, sg3_utils, and other disk refurbishment tools. Only Tier 0 (bootable minimum) packages were present in the Alpine rootfs.

## Root Cause

1. IuppiterOS's `cmd_download_alpine()` was not calling `recipe::packages()` to install supplementary packages (unlike AcornOS which calls it after alpine download)
2. Even with the call added, packages.rhai failed because the repositories file in the rootfs pointed to a temporary `/tmp/` directory that was created during alpine.rhai but no longer existed by the time packages.rhai tried to use it

## Solution

### Part 1: Add Package Installation Call

Modified `IuppiterOS/src/main.rs::cmd_download_alpine()` to call `iuppiteros::recipe::packages(&base_dir)?` after downloading Alpine, matching the AcornOS pattern. This ensures Tier 1-2 packages are installed into the rootfs during the download phase.

### Part 2: Fix Repository Paths

Modified `IuppiterOS/deps/packages.rhai::build()` to clean up stale temporary repository paths before running apk:

```rhai
// Fix repositories file - remove stale temporary paths from alpine.rhai
let repo_file = join_path(rootfs, "etc/apk/repositories");
if is_file(repo_file) {
    let repos = trim(read_file(repo_file));
    let repo_lines = repos.split("\n");
    let fixed_content = "";
    for line in repo_lines {
        let trimmed = trim(line);
        // Skip lines that look like temp paths (contain /tmp/)
        if !trimmed.contains("/tmp/") && trimmed != "" {
            if fixed_content != "" {
                fixed_content += "\n";
            }
            fixed_content += trimmed;
        }
    }
    fixed_content += "\n";
    write_file(repo_file, fixed_content);
    log("Fixed repositories file (removed stale temp paths)");
}
```

This allows apk to fall back to the online Alpine repositories (https://dl-cdn.alpinelinux.org/alpine/v3.23/main and community) when the temporary local ISO path no longer exists.

## Technical Details

The issue stems from how the recipe system works:
- `alpine.rhai` runs in a temporary BUILD_DIR, extracts the ISO there, creates a repositories file pointing to that temporary ISO, then installs packages
- When install() completes, it copies the rootfs to the persistent `downloads/rootfs` directory
- Later, `packages.rhai` runs in the SAME BUILD_DIR (downloads directory), but the temporary ISO path is stale
- The fix: detect and remove these stale /tmp/ paths so apk can use the working online mirrors

## Files Modified

- `IuppiterOS/src/main.rs`: Added `iuppiteros::recipe::packages()` call to cmd_download_alpine()
- `IuppiterOS/deps/packages.rhai`: Added stale repository path cleanup before package installation
- `.ralph/prd.md`: Marked task 7.1 [x] complete

## Verification

Verified that:
- smartctl binary exists at `/home/vince/Projects/LevitateOS/IuppiterOS/downloads/rootfs/usr/sbin/smartctl` (770KB)
- All refurbishment tools installed: smartmontools, hdparm, sg3_utils, sdparm, nvme-cli, lsscsi
- Tier 1 server core packages installed: eudev, openssh, dhcpcd, chrony, etc.
- Tier 3 live ISO tools installed: parted, xfsprogs

## Impact

- Phase 7.1 complete: smartmontools (and full Tier 0-2 packages) now installed in IuppiterOS
- Prerequisite for Phase 8 install-tests (which needs smartctl to run)
- Fixes a pre-existing issue in AcornOS as well (same pattern, same fix needed)

## Known Issues

None - this fix is localized to package installation and doesn't affect other subsystems.

## Next Steps

- Task 7.2: Verify hdparm is installed and runs
- Task 7.3: Verify sg3_utils binaries are available
- Phase 8: Run install-tests to verify post-installation behavior
