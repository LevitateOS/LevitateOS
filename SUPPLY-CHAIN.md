# LevitateOS Supply Chain Documentation

**Last Updated:** 2026-01-22

This document describes all upstream sources used in LevitateOS builds, enabling supply chain transparency and auditability.

## Upstream Sources

### 1. Linux Kernel

| Attribute | Value |
|-----------|-------|
| Source | https://github.com/LevitateOS/linux (fork of kernel.org) |
| Upstream | https://kernel.org |
| Verification | Git commit signatures |
| Security | https://kernel.org/category/releases.html |

**Why a fork?** We maintain minimal patches for LevitateOS-specific needs. All patches are publicly visible in git history.

### 2. Userspace Binaries (Rocky Linux)

| Attribute | Value |
|-----------|-------|
| Source | Rocky Linux 10.1 x86_64 DVD ISO |
| URL | https://download.rockylinux.org/pub/rocky/10/isos/x86_64/Rocky-10.1-x86_64-dvd1.iso |
| Checksum | SHA256: `bd29df7f8a99b6fc4686f52cbe9b46cf90e07f90be2c0c5f1f18c2ecdd432d34` |
| Upstream | Fedora / Red Hat Enterprise Linux |
| RPM Signatures | Verified with Rocky Linux GPG keys |

**What we extract:**
- Core utilities (coreutils, util-linux, bash, etc.)
- System services (systemd, NetworkManager, etc.)
- Libraries (glibc, openssl, etc.)
- Firmware (linux-firmware package)

**Why Rocky Linux?**
- Binary compatible with RHEL
- Long-term support (10 years)
- Enterprise-grade security updates
- Packages are pre-compiled and tested
- Allows us to build a system in minutes, not hours

### 3. Bootloader

| Attribute | Value |
|-----------|-------|
| Component | systemd-boot |
| Source | Part of systemd (from Rocky Linux) |
| Upstream | https://github.com/systemd/systemd |

### 4. Firmware

| Attribute | Value |
|-----------|-------|
| Source | linux-firmware package (Rocky Linux) |
| Upstream | https://git.kernel.org/pub/scm/linux/kernel/git/firmware/linux-firmware.git |
| Coverage | WiFi (Intel, Atheros, Realtek, Broadcom), GPU, Bluetooth |

## Package Signature Verification

### Rocky Linux RPMs

Rocky Linux RPMs are signed with the Rocky Linux GPG key:

```
Key ID: 350D275D
Fingerprint: 7051 C470 A929 F454 CEBE 37B7 15AF 5DAC 350D 275D
```

The build system verifies:
1. ISO checksum before extraction
2. RPM signatures during extraction (via rpm --checksig)

### Kernel Commits

Kernel commits can be verified against maintainer signatures:
```bash
git verify-commit HEAD
```

## Build Process

```
┌─────────────────────────────────────────────────────┐
│                    Build Host                        │
├─────────────────────────────────────────────────────┤
│  1. Download Rocky ISO (verify SHA256)              │
│  2. Extract RPMs (verify signatures)                │
│  3. Build kernel (from LevitateOS/linux fork)       │
│  4. Assemble rootfs (selected packages)             │
│  5. Create initramfs                                │
│  6. Build ISO                                       │
└─────────────────────────────────────────────────────┘
```

### Reproducibility Goals

Currently working toward:
- [ ] Bit-for-bit reproducible ISO builds
- [ ] Published build environment (container image)
- [ ] Timestamped build logs
- [x] Documented source versions (this document)

## Self-Hosting for Organizations

EU organizations requiring supply chain control can:

### 1. Mirror Rocky Linux

```bash
# Set up local Rocky mirror
rsync -avz rsync://rocky.example.com/rocky/ /srv/mirror/rocky/
```

### 2. Configure Local Source

Set environment variables before building:
```bash
export ROCKY_URL="https://your-mirror.example.com/rocky/10/isos/x86_64/Rocky-10.1-x86_64-dvd1.iso"
```

### 3. Host Package Repository

Recipe (the LevitateOS package manager) supports custom repositories:
```toml
# /etc/recipe/repos.d/local.toml
[repository.local]
url = "https://packages.your-org.example.com/levitateos/"
priority = 10
```

### 4. Fork and Build Kernel

```bash
git clone https://github.com/LevitateOS/linux
# Apply your organization's patches
# Build with your signing keys
```

## Change Log

| Date | Change |
|------|--------|
| 2026-01-22 | Initial documentation |

## Audit Contact

For supply chain security questions or to report concerns:
- Email: security@levitateos.org
- GitHub: Open a security advisory

---

**Transparency is security.** All sources documented. All code public. Audit welcome.
