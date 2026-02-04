# TEAM_165: Phase 3 Task 3.4 — OpenRC Init System Configuration

**Date**: 2026-02-04
**Iteration**: 10 (Haiku)
**Status**: ✅ COMPLETED

## What Was Done

Verified that OpenRC is correctly installed and configured as the init system (not systemd) during AcornOS rootfs build. This fulfills task 3.4: "OpenRC installed and configured as init system (not systemd)".

## Key Findings

The infrastructure for OpenRC configuration was **already fully implemented**:

1. **OPENRC component** (src/component/definitions.rs:210-257):
   - Phase::Init phase (executed as phase 3)
   - Creates /etc/init.d and /etc/conf.d directories
   - Creates 5 runlevel directories: sysinit, boot, default, nonetwork, shutdown
   - **Critical**: Symlinks /sbin/init → /bin/busybox (not openrc-init!)
     - Reason: busybox init reads /etc/inittab for console gettys
     - openrc-init doesn't properly handle inittab respawn lines
   - Copies OpenRC support infrastructure from /usr/libexec/rc
   - Copies /etc/rc.conf configuration
   - Copies 27 init scripts from Alpine source (hostname, networking, sshd, chronyd, dhcpcd, iwd, local, etc.)
   - Enables services in proper runlevels via symlinks

2. **OpenRC binaries in rootfs**:
   - /usr/sbin/openrc (43k) ✓
   - /usr/sbin/openrc-run (39k) ✓
   - /usr/sbin/openrc-init (18k) ✓
   - /usr/sbin/openrc-shutdown (27k) ✓
   - /sbin/init → /bin/busybox symlink ✓

3. **OpenRC configuration**:
   - /etc/rc.conf (14k) ✓
   - /etc/init.d/ contains 27 executable scripts ✓
   - /etc/conf.d/ directory created ✓

4. **Runlevel configuration**:
   - /etc/runlevels/sysinit: devfs, dmesg, hwdrivers, mdev, modules, procfs, sysfs (7 services)
   - /etc/runlevels/boot: bootmisc, fsck, hostname, hwclock, localmount, networking, root, seedrng, swap, sysctl (10 services)
   - /etc/runlevels/default: dhcpcd (1 service)
   - /etc/runlevels/shutdown: killprocs, mount-ro, savecache (3 services)

5. **Init scripts format**:
   - All start with `#!/sbin/openrc-run` shebang
   - Proper OpenRC metadata (description, extra_commands, etc.)
   - Example: /etc/init.d/sshd provides proper start/stop/reload functions

## Verification

Built rootfs and verified:

- OpenRC binaries present and executable ✓
- /sbin/init is symlink to busybox ✓
- /etc/rc.conf configuration file exists ✓
- /etc/init.d/ contains 27 executable init scripts ✓
- All init scripts use openrc-run shebang ✓
- Runlevels properly configured with symlinks ✓
- **Confirmed: NO systemd present** ✓
  - No /usr/lib/systemd/ directory
  - No /etc/systemd/ directory
  - No systemd binaries

## Files Modified

None. The implementation was complete. Only verified existing functionality and updated documentation.

## Decisions & Rationale

- /sbin/init → busybox (not openrc-init) is **critical** for live system
  - Busybox init reads /etc/inittab for getty respawn lines
  - openrc-init would not properly spawn login prompts
  - Component comment documents this explicitly
- Phase ordering ensures OPENRC runs after UTILITIES (which copies OpenRC binaries)
- All init scripts from Alpine are copied, enabling standard services
- Runlevel symlinks enable/disable services without editing scripts

## Blockers

None. Task completed successfully.

## Next Task

Task 3.5: "eudev installed for device management (not systemd-udevd)"
