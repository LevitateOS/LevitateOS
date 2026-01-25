# TEAM_030: LevitateOS ISO Builder (levitateiso)

## Objective
Build `levitateiso` - a custom ISO maker that extracts pre-built binaries from Rocky 10 minimal ISO, applies LevitateOS overlay (autologin, branding), and creates a bootable LevitateOS ISO.

## Status: Complete

## Decisions Made
- Following archiso-style approach with profile/airootfs overlay
- Using rpm --install with --nodeps for package extraction
- Using dracut with dmsquash-live for live boot support
- Using xorriso for ISO creation
- Root password set to empty for live environment
- Autologin enabled via systemd drop-in

## Files Created
```
levitateiso/
├── Cargo.toml                                              # Crate definition
├── src/
│   └── main.rs                                             # ISO builder implementation
├── profile/
│   ├── packages.txt                                        # Package list (70+ packages)
│   └── airootfs/
│       └── etc/
│           ├── hostname                                    # "levitateos"
│           ├── motd                                        # ASCII banner + instructions
│           ├── os-release                                  # LevitateOS branding
│           └── systemd/system/getty@tty1.service.d/
│               └── autologin.conf                          # Root autologin
│       └── usr/local/bin/
│           └── levitate-installer                          # Installer script
└── templates/
    ├── grub.cfg                                            # EFI boot menu
    └── isolinux.cfg                                        # Legacy BIOS boot
```

## Usage

```bash
# Build ISO (requires root for mount/rpm)
cd levitateiso
sudo cargo run -- build

# Or with custom options
sudo cargo run -- build \
  --source-iso vendor/images/Rocky-10-latest-x86_64-minimal.iso \
  --output out/levitateos.iso \
  --profile profile

# List packages in source ISO
sudo cargo run -- list-packages vendor/images/Rocky-10-latest-x86_64-minimal.iso

# Test the resulting ISO
qemu-system-x86_64 -enable-kvm -m 4G -cdrom out/levitateos.iso
```

## Host Dependencies
- rpm (package installation)
- mksquashfs (squashfs creation)
- xorriso (ISO creation)
- dracut (initramfs generation)
- rsync (file copying)
- mount/umount (ISO mounting)

## Build Process
1. Mount Rocky 10 minimal ISO
2. Extract and install packages via `rpm --root --nodeps`
3. Apply airootfs overlay (branding, autologin)
4. Generate initramfs with `dracut --add dmsquash-live`
5. Create squashfs from rootfs
6. Setup EFI boot structure
7. Create hybrid ISO with xorriso

## Notes
- Source ISO: `vendor/images/Rocky-10-latest-x86_64-minimal.iso` (1.5G)
- Output boots directly to root shell (autologin)
- `levitate-installer` available for disk installation
- Volume label "LEVITATEOS" used for boot configuration
