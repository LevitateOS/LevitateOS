# TEAM_050: Test Command Auto-Shutdown + Journald/NTP Fixes

## Tasks Completed

### 1. Test command auto-shutdown
`cargo run -- test -c "command"` now properly executes the command and shuts down.

**Key fix:** The shell needs 2 seconds after "Startup finished" to be ready to accept input. The original 200ms-500ms was too short.

### 2. Auto-build for test and run
- `cargo run -- test` auto-builds initramfs before testing
- `cargo run -- run` auto-builds initramfs and ISO before running

### 3. Journal socket fix
Enabled journald sockets in sockets.target.wants:
- `systemd-journald.socket`
- `systemd-journald-dev-log.socket`

This fixes: `dbus-daemon.service: Failed to connect stdout to the journal socket`

### 4. NTP support via chronyd
Added for `timedatectl set-ntp true` support:
- `chronyd` binary (from Rocky's chrony package, not systemd-timesyncd)
- `chronyd.service` unit
- `chrony` user/group
- `/usr/sbin/chronyd` symlink (service expects this path)
- `/etc/sysconfig/chronyd` config
- `/etc/chrony.conf` config
- `/usr/lib/systemd/ntp-units.d/50-chronyd.list` (tells timedatectl to use chronyd)
- Enabled in multi-user.target.wants

**Result:** `timedatectl set-ntp true` now returns EXIT_CODE=0 and shows `NTP service: active`

Note: `System clock synchronized: no` is expected since QEMU test has no network.

## Changes

### leviso/src/qemu.rs
- Wait 2 seconds (not 200ms) after boot for shell to be ready
- Use `mon:stdio` serial mode
- Detect completion via `___LEVISO_CMD_DONE___` marker
- 30 second timeout

### leviso/src/main.rs
- `Commands::Test` auto-builds initramfs
- `Commands::Run` auto-builds initramfs + ISO

### leviso/src/initramfs.rs
- Enable journald sockets
- Add chronyd binary, service, user, config, directories
- Add `/usr/sbin/chronyd` symlink (binary is copied to `/bin/chronyd`)
- Add `/etc/sysconfig/chronyd` for OPTIONS
- Add `/usr/lib/systemd/ntp-units.d/50-chronyd.list` for timedatectl integration
