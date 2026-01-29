# LevitateOS Bare Metal Installation Testing Checklist

> **Purpose**: Comprehensive checklist of all human-executable tests required to validate LevitateOS installation on physical hardware. This cannot be automated or tested remotely—you must perform these yourself.

---

## Phase 1: Pre-Installation (Before Booting Installer)

### USB/Boot Media
- [ ] ISO burns to USB without errors (`dd` or similar tool)
- [ ] USB device is readable (not corrupted)
- [ ] Motherboard BIOS/UEFI detects the USB drive
- [ ] USB appears in boot order menu
- [ ] Can select USB as primary boot device
- [ ] System actually boots from USB (not from existing disk)

### Firmware/BIOS Issues
- [ ] Secure Boot is disabled (or you can disable it)
- [ ] TPM/Trusted Module not blocking boot
- [ ] USB boot mode supported (some old systems only support Legacy BIOS)
- [ ] UEFI mode is available (required by LevitateOS per CLAUDE.md)
- [ ] Can access BIOS/UEFI settings on your specific motherboard

---

## Phase 2: Live Environment Boot (You're in the Installer)

### Hardware Detection
- [ ] Keyboard works in live environment (can type)
- [ ] Mouse/trackpad works (if needed)
- [ ] Network card detected (check `ip link` or `lsblk`)
- [ ] Network gets IP address via DHCP (test with `ping 8.8.8.8`)
- [ ] `lsblk` shows all your disks correctly
- [ ] Disk size reported correctly (not truncated)
- [ ] CPU detected correctly (check `cat /proc/cpuinfo`)
- [ ] RAM detected correctly (check `free -h`)
- [ ] GPU detected (if you have one - check `lspci`)
- [ ] Audio card detected (if you need audio - check `aplay -l`)
- [ ] Any special hardware (WiFi cards, etc.) shows up in `lspci`

### Live Environment Stability
- [ ] Shell stays responsive (no hangs)
- [ ] Docs TUI works and is readable on your display
- [ ] Display resolution is correct (1920x1080+)
- [ ] No kernel errors or warnings in dmesg (check `dmesg | grep -i error`)

---

## Phase 3: Disk Partitioning (The Critical Part)

### Before Partitioning
- [ ] You've verified you're targeting the **correct disk** (triple-check!)
- [ ] Any existing data is backed up (if this isn't a fresh disk)

### Partitioning Process
- [ ] `fdisk /dev/vda` (or your actual disk) opens without "No such file" error
- [ ] Partition table is created (`g` for GPT)
- [ ] EFI partition created (512MB, type 1)
- [ ] Root partition created (remainder of disk)
- [ ] Write command (`w`) actually commits changes
- [ ] `lsblk` shows new partitions after fdisk exits (vda1, vda2)
- [ ] Partitions are not already mounted (would block formatting)

### Filesystem Creation
- [ ] `mkfs.vfat /dev/vda1` completes without errors
- [ ] `mkfs.ext4 /dev/vda2` completes without errors (or your chosen filesystem)
- [ ] No "Device is busy" or "Read-only filesystem" errors
- [ ] `lsblk -f` shows correct filesystem types (vfat on vda1, ext4 on vda2)

### Mount Operations
- [ ] `mount /dev/vda2 /mnt` succeeds
- [ ] `mkdir -p /mnt/boot/efi` succeeds
- [ ] `mount /dev/vda1 /mnt/boot/efi` succeeds
- [ ] `df -h` shows both filesystems mounted at correct mount points
- [ ] Mounted filesystems are writable (test: `touch /mnt/testfile`)

---

## Phase 4: Root Filesystem Extraction

### recstrap Execution
- [ ] `recstrap --force /mnt` starts without errors
- [ ] No network errors during download (watch output)
- [ ] Process completes (may take 3-5 minutes)
- [ ] No "permission denied" errors
- [ ] No "out of space" errors
- [ ] `ls /mnt` shows extracted files (bin, etc, usr, var, root, etc.)
- [ ] `/mnt/etc/os-release` exists and contains LevitateOS info

### File System Integrity
- [ ] No warnings about filesystem corruption
- [ ] Directory structure looks complete (not truncated)
- [ ] Symlinks are created correctly

---

## Phase 5: Configuration & Chroot

### fstab Generation
- [ ] `recfstab /mnt` completes
- [ ] `cat /mnt/etc/fstab` shows two entries (/ and /boot/efi)
- [ ] UUIDs are present (not zeros or missing)
- [ ] Filesystem types match what you created (vfat, ext4)

### Chroot Entry
- [ ] `recchroot /mnt` enters the chroot
- [ ] Prompt shows you're in chroot (different prompt? `#` sign?)
- [ ] `pwd` returns `/` (root of installed system)
- [ ] `ls /` shows the system's actual directories
- [ ] Network still works inside chroot (test: `ping 8.8.8.8`)

### Bootloader Installation
- [ ] `bootctl install` completes without errors
- [ ] No "EFI variables not accessible" errors (would be a firmware issue)
- [ ] `ls /boot/efi/EFI/Boot` shows bootloader files created
- [ ] `ls /boot/efi/EFI/levitate` shows boot entries created (or similar)
- [ ] No errors about "firmware not installed"

### Exit & Unmount
- [ ] `exit` returns you to live environment
- [ ] `umount /mnt/boot/efi` succeeds
- [ ] `umount /mnt` succeeds
- [ ] No "Device or resource busy" errors (would mean files still open)

---

## Phase 6: First Boot (The Moment of Truth)

### Reboot Process
- [ ] `reboot` command completes (no hang)
- [ ] System actually powers down and restarts (doesn't hang)
- [ ] BIOS/UEFI splash screen appears
- [ ] Boot entry for LevitateOS is listed in UEFI boot menu
- [ ] Can select LevitateOS boot entry
- [ ] System boots from LevitateOS (not back to USB or existing OS)

### Kernel & Initramfs Loading
- [ ] Kernel loading messages appear (scroll output)
- [ ] No kernel panics (would show "Kernel panic" in red)
- [ ] Initramfs loads successfully
- [ ] Root filesystem is found and mounted
- [ ] systemd or init takes over (boot messages appear)
- [ ] No "unable to find root filesystem" errors
- [ ] No "filesystem not found" errors

### Login & Basic Functionality
- [ ] Login prompt appears (getty or similar)
- [ ] Can log in as root (user: root, password: if set)
- [ ] Shell prompt appears (`#` or `root@hostname`)
- [ ] `pwd` shows `/root` (you're logged in)
- [ ] `ls /` shows the filesystem (install worked)
- [ ] `uname -a` shows kernel details
- [ ] `cat /etc/os-release` shows LevitateOS info

---

## Phase 7: Hardware Functionality After Boot

### Network
- [ ] `ip link` shows network devices
- [ ] `ip addr` shows IP addresses assigned
- [ ] `ping 8.8.8.8` works (internet connectivity)
- [ ] DNS works (`ping google.com` or `nslookup google.com`)
- [ ] Network persists after reboot (if applicable)

### Storage & Disks
- [ ] `lsblk` shows correct disk layout
- [ ] `df -h` shows correct mount points and usage
- [ ] Can create files (`touch /tmp/test`, `echo hello > /tmp/test`)
- [ ] Can read files (`cat /tmp/test`)
- [ ] No filesystem errors in `dmesg`

### CPU, Memory, Power
- [ ] `nproc` shows correct number of CPU cores
- [ ] `free -h` shows correct RAM amount
- [ ] System is stable (no immediate crashes or hangs)
- [ ] Fan speed seems reasonable (not constantly maxed out)
- [ ] No hardware temperature warnings (if you can check)

### Display & Graphics (if applicable)
- [ ] Monitor(s) detected correctly
- [ ] Resolution is appropriate (readable text)
- [ ] GPU is recognized (check `lspci | grep -i vga`)
- [ ] No graphical glitches or corruption
- [ ] Framebuffer is functional

### Wireless/Special Hardware (if applicable)
- [ ] WiFi card detected (`ip link` or `lspci`)
- [ ] Can scan networks (`nmcli dev wifi list` or similar)
- [ ] Can connect to WiFi network
- [ ] Bluetooth device detected (if present)
- [ ] USB ports working (plug in USB drive, see if detected)

---

## Phase 8: Package Management & Stability

### Recipe/Package Manager
- [ ] `recipe --help` or `pacman --help` works
- [ ] `recipe list` or similar shows available packages
- [ ] Can attempt to install a package (small one like `nano` or `curl`)
- [ ] Installation completes without errors
- [ ] Installed binary runs (`which nano`, `nano --version`)

### System Stability
- [ ] System stays running for at least 30 minutes without crashes
- [ ] No filesystem corruption detected
- [ ] No OOM (Out of Memory) errors
- [ ] No hardware errors in `dmesg`
- [ ] Can open multiple terminals/shells

### Persistence
- [ ] Files you create persist after reboot
- [ ] Modified configuration stays after reboot
- [ ] System time is preserved (or syncs via NTP)

---

## Phase 9: Reboot Verification

### Second Boot Test
- [ ] Can reboot successfully (`reboot` command)
- [ ] System boots to login prompt again
- [ ] No kernel panics on second boot
- [ ] Network comes up automatically (if configured)
- [ ] Filesystem checks pass (no fsck hangs)

---

## Phase 10: Recovery & Rollback Plan

### If Something Breaks (Have These Ready)
- [ ] USB live environment still boots and allows recovery
- [ ] Can `recchroot /mnt` back into installed system for fixes
- [ ] Have a secondary bootable device ready (another distro's USB)
- [ ] Know how to access BIOS/UEFI to change boot order back

---

## Documentation Audit

- [ ] Docs TUI matched your actual installation experience
- [ ] No steps were skipped or unclear
- [ ] Any warnings in docs were relevant to your hardware
- [ ] You discovered any undocumented hardware-specific steps
- [ ] You documented any issues you encountered for others

---

## Final Sign-Off

After completing all above:
- [ ] System is installed and bootable
- [ ] All critical hardware works (CPU, RAM, disk, network)
- [ ] Can log in and use basic shell commands
- [ ] Feel confident to continue customizing your system

---

## Troubleshooting Record

### If ANY Test Fails

Document the following for your reference and to help the LevitateOS community:

**Test that failed:**
```
[Describe which checkbox failed]
```

**Exact error message:**
```
[Copy-paste the full error, include context]
```

**Hardware specifications:**
- CPU: [e.g., Intel i7-12700K]
- Motherboard: [e.g., ASUS ROG Strix Z690-E]
- Disk type: [e.g., NVMe Samsung 990 Pro]
- RAM: [e.g., 64GB DDR5]
- GPU: [e.g., RTX 4070]
- Network: [e.g., Intel i225]

**Which phase failed:**
```
[Phase 1-10 identifier]
```

**Command(s) that caused the failure:**
```bash
[Exact commands you ran]
```

**Workaround found (if any):**
```
[If you fixed it, describe the fix]
```

**Next steps:**
- [ ] Report to LevitateOS GitHub issues with this information
- [ ] Post to community forums with your hardware specs
- [ ] Document for your own records

---

## Quick Reference: Essential Commands

```bash
# Check system state
lsblk                          # Show disk layout
ip link                        # Show network devices
ip addr                        # Show IP addresses
free -h                        # Show memory
nproc                          # Show CPU count
dmesg | grep -i error          # Check for errors
cat /proc/cpuinfo              # CPU details
cat /etc/os-release            # OS info
uname -a                       # Kernel info

# Disk operations
fdisk /dev/sdX                 # Partition disk
mkfs.ext4 /dev/sdX1            # Format ext4
mkfs.vfat /dev/sdX2            # Format FAT32
mount /dev/sdX1 /mnt           # Mount filesystem
umount /mnt                    # Unmount filesystem

# Installation process
recstrap --force /mnt          # Extract root filesystem
recfstab /mnt                  # Generate fstab
recchroot /mnt                 # Enter chroot
bootctl install                # Install bootloader
exit                           # Exit chroot
reboot                         # Reboot system
```

---

## Notes for Self

Use this section to record observations specific to your hardware:

```
[Add your own notes here as you test]
```

---

**Last Updated:** 2026-01-29
**Purpose:** Comprehensive bare metal testing checklist for LevitateOS
**Status:** Ready for use
