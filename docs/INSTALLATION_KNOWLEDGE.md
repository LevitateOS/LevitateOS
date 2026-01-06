# LevitateOS Installation & Provisioning Knowledge

**TEAM_200 Discovery:** Transitioning from a "Live CD" (initramfs) model to a "Persistent OS" requires a formal **Provisioning Stage** and an **Installation Workflow**.

## 1. The Provisioning Gap
In early development, LevitateOS ran purely from `initramfs` or a small 16MB FAT32 image. To support large-scale storage (128GB+) and multi-user environments (Part III), we discovered a fundamental gap in the boot sequence.

### The Problem
*   `initramfs` (Stage 5) is ephemeral and read-only.
*   Persistent disks may be unformatted or lack the required hierarchy (`/home`, `/etc`).
*   Directly booting into a shell on an unprovisioned disk leads to "broken OS" feel.

### The Solution: Stage 4 (Provisioning)
We introduced **Stage 4: Provisioning** between `BootConsole` and `Discovery`. This stage is responsible for:
1.  **Disk Heartbeat**: Detecting if the persistent storage is primed.
2.  **OS Installation**: If the disk is "clean", the kernel uses `initramfs` as installation media to replicate the base system to the persistent disk.
3.  **Config Generation**: Automatically creating `/etc/fstab` and other "Real OS" anchors.

## 2. Sparse Disk Pattern
Expanding to **128GB** without consuming host SSD space requires the use of **Sparse Files**.
*   **Technique**: Use `truncate -s 128G tinyos_disk.img` in `xtask`.
*   **Gotcha**: The kernel block driver and FAT32/ext4 libraries must dynamically query the block count rather than relying on hardcoded constants.

## 3. Standard Hierarchy
A "Real OS" must follow a standard hierarchy even on first boot. The Provisioning stage should ensure:
*   `/bin`: All binaries from `initramfs` are mirrored here.
*   `/etc`: Contains `fstab` and eventually `passwd`/`shadow` (Part III).
*   `/home`: User directories (e.g., `/home/vince`).
*   `/tmp`: Mounted via Tmpfs.

## 4. Verification Techniques
Future teams should verify the provisioning flow by:
1.  Deleting `tinyos_disk.img` and running `cargo xtask run`.
2.  Checking logs for `[BOOT] Stage 4: Provisioning`.
3.  Verifying file persistence in `/home` after multiple reboots.

---
*Created by TEAM_200 - 2026-01-06*
