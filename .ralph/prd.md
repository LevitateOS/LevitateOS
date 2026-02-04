# PRD: AcornOS + IuppiterOS — Consolidated Build

## Goal

Build two Alpine-based OS variants using shared infrastructure:
- **AcornOS**: Desktop-ready base system (Alpine + OpenRC + musl)
- **IuppiterOS**: Touchscreen kiosk refurbishment appliance (GUI + OpenRC + musl + SAS/SCSI)

## Definition of Done

Both ISOs boot, install, and pass install-tests. AcornOS provides a login shell.
IuppiterOS boots to fullscreen GUI kiosk mode on touchscreen, auto-launching iuppiter-dar
(HDD diagnostics application). System provides refurbishment tools (smartmontools, hdparm, sg3_utils).

---

## Tasks

Tasks are ordered by dependency, not by variant. Tags: [acorn] [iuppiter] [shared]

### Phase 1: Both Builders Compile

- [x] 1.1 [acorn] `cargo check` passes for AcornOS crate with zero errors
- [x] 1.2 [acorn] `cargo run -- status` shows correct AcornOS configuration
- [x] 1.3 [acorn] `cargo run -- preflight` validates host tools (xorriso, mkfs.erofs, 7z, tar, cpio, curl)
- [x] 1.4 [acorn] All AcornOS commands match leviso equivalents (build, run, clean, status, preflight)
- [x] 1.5 [iuppiter] Create IuppiterOS Cargo.toml with same dependencies as AcornOS (distro-spec, distro-builder, clap, anyhow, tokio)
- [x] 1.6 [iuppiter] Create src/main.rs with clap CLI: build, run, status, preflight, clean commands
- [x] 1.7 [iuppiter] Implement IuppiterConfig (DistroConfig trait) using distro-spec::iuppiter constants
- [x] 1.8 [iuppiter] `cargo check` passes for IuppiterOS with zero errors
- [x] 1.9 [iuppiter] `cargo run -- status` shows IuppiterOS identity (OS_NAME, OS_ID, ISO_LABEL from distro-spec)
- [x] 1.10 [iuppiter] `cargo run -- preflight` validates host tools

### Phase 2: Alpine Package Pipeline

- [x] 2.1 [acorn] `cargo run -- download` fetches Alpine APK packages using the `recipe` crate for dependency resolution
- [x] 2.2 [acorn] APK extraction produces correct directory structure (musl, busybox, apk-tools at minimum)
- [x] 2.3 [acorn] Package dependency resolution works — all deps pulled in correct order via recipe
- [x] 2.4 [acorn] Alpine signing key verification works (keys from distro-spec/acorn/keys/)
- [x] 2.5 [iuppiter] IuppiterOS builder reuses AcornOS's Alpine package pipeline (same recipe integration)
- [x] 2.6 [iuppiter] Downloads use iuppiter package tiers from distro-spec::iuppiter::packages (NOT acorn list)
- [x] 2.7 [iuppiter] Verify package list matches kiosk appliance (no WiFi packages like iwd/wireless-regdb, but INCLUDES display stack)

### Phase 3: Rootfs Build

- [x] 3.1 [shared] distro-builder integration: components use Installable trait + Op enum, executor processes ops
- [x] 3.2 [acorn] FHS directory structure created (/bin, /etc, /lib, /usr, /var, /tmp, /proc, /sys, /dev, /run, /home, /root)
- [x] 3.3 [acorn] Busybox symlinks created for all applets (/bin/sh → busybox, /bin/ls → busybox, etc.)
- [x] 3.4 [acorn] OpenRC installed and configured as init system (not systemd)
- [x] 3.5 [acorn] eudev installed for device management (not systemd-udevd)
- [x] 3.6 [acorn] /etc/inittab configured with getty on tty1 and ttyS0 (OpenRC console management)
- [x] 3.7 [acorn] Networking configured: dhcpcd, /etc/network/interfaces or equivalent
- [x] 3.8 [acorn] /etc/apk/repositories configured (Alpine v3.23 main + community) so `apk add` works post-boot
- [x] 3.9 [acorn] /etc/hostname set to distro-spec DEFAULT_HOSTNAME, /etc/hosts has localhost + hostname
- [x] 3.10 [acorn] /etc/resolv.conf configured (or dhcpcd manages it)
- [x] 3.11 [acorn] User creation works: doas (not sudo), root password for live, user in wheel group
- [x] 3.12 [acorn] /etc/os-release contains AcornOS identity from distro-spec
- [x] 3.13 [acorn] SSH: sshd installed, host keys generated, sshd_config allows root login for live ISO
- [x] 3.14 [acorn] All Tier 0-2 packages from distro-spec::acorn::packages installed in rootfs
- [x] 3.15 [acorn] Test instrumentation: /etc/profile.d/00-test.sh emits ___SHELL_READY___ marker for install-tests
- [x] 3.16 [acorn] Live overlay configuration: rootfs is EROFS (read-only), init creates tmpfs overlay
- [x] 3.17 [acorn] EROFS rootfs builds without errors (mkfs.erofs with zstd compression)
- [x] 3.18 [acorn] EROFS rootfs size < 500MB compressed
- [x] 3.19 [iuppiter] IuppiterOS rootfs: same FHS structure as AcornOS, using iuppiter package tiers
- [x] 3.20 [iuppiter] /etc/inittab: getty on ttyS0 (serial console primary), NOT tty1
- [x] 3.21 [iuppiter] /etc/os-release contains IuppiterOS identity from distro-spec
- [x] 3.22 [iuppiter] /etc/hostname set to "iuppiter" (from distro-spec)
- [x] 3.23 [iuppiter] Same test instrumentation as AcornOS (___SHELL_READY___ on serial console)
- [x] 3.24 [iuppiter] EROFS rootfs builds, size < AcornOS (fewer packages = smaller)

### Phase 4: Initramfs + Boot

- [x] 4.1 [acorn] Busybox-based initramfs builds using recinit (not dracut — Alpine doesn't use it)
- [x] 4.2 [acorn] /init script: mount ISO by label, find EROFS rootfs, mount read-only
- [x] 4.3 [acorn] /init script: create overlay (EROFS lower + tmpfs upper), switch_root to overlay
- [x] 4.4 [acorn] OpenRC starts as PID 1 after switch_root (verify with test boot)
- [x] 4.5 [acorn] Kernel modules from distro-spec::acorn::boot (21 modules: virtio, SCSI, NVME, USB, EROFS, overlay)
- [x] 4.6 [acorn] Initramfs includes module dependency files (modules.dep from depmod)
- [x] 4.7 [iuppiter] IuppiterOS initramfs: same /init script, different module set
- [x] 4.8 [iuppiter] Boot modules from distro-spec::iuppiter::boot (27 modules: core + SAS + SES + SG, NO USB)
- [x] 4.9 [iuppiter] SAS drivers included: mpt3sas, megaraid_sas, scsi_transport_sas
- [x] 4.10 [iuppiter] SCSI enclosure included: enclosure, ses (for LED/slot control)
- [x] 4.11 [iuppiter] SCSI generic included: sg (for SG_IO passthrough — smartctl needs this)

### Phase 5: ISO Build

- [x] 5.1 [acorn] UKI builds with AcornOS entries from distro-spec::acorn::uki (3 live + 2 installed)
- [x] 5.2 [acorn] systemd-boot loader.conf configured, ISO builds via reciso + xorriso (UEFI bootable)
- [x] 5.3 [acorn] ISO label matches distro-spec: "ACORNOS"
- [x] 5.4 [acorn] `cargo run -- run` launches QEMU with the built ISO (GUI mode)
- [x] 5.5 [iuppiter] UKI builds with IuppiterOS entries from distro-spec::iuppiter::uki (all have serial console cmdline)
- [x] 5.6 [iuppiter] All UKI entries include console=ttyS0,115200n8 in kernel cmdline
- [x] 5.7 [iuppiter] ISO label matches distro-spec: "IUPPITER"
- [x] 5.8 [iuppiter] ISO builds via reciso + xorriso (UEFI bootable)
- [x] 5.9 [iuppiter] `cargo run -- run --serial` launches QEMU in serial-only mode (no display)

### Phase 6: Boot & Login

- [x] 6.1 [acorn] QEMU boots AcornOS ISO: kernel loads, initramfs mounts EROFS, overlay created
- [x] 6.2 [acorn] OpenRC starts, services come up: networking, eudev, chronyd, sshd
- [x] 6.3 [acorn] Login prompt on serial console, root login works
- [x] 6.4 [acorn] Networking works: DHCP assigns IP on virtio NIC, DNS resolves
- [x] 6.5 [acorn] ___SHELL_READY___ marker appears on serial (proves test instrumentation works)
- [x] 6.6 [iuppiter] QEMU boots IuppiterOS ISO via `cargo run -- run --serial`
- [x] 6.7 [iuppiter] Serial console shows kernel boot messages, initramfs runs
- [x] 6.8 [iuppiter] OpenRC starts: networking, eudev, chronyd, sshd, iuppiter-engine (placeholder OK)
- [x] 6.9 [iuppiter] Login prompt on ttyS0 (serial), root login works
- [x] 6.10 [iuppiter] ___SHELL_READY___ marker appears on serial console
- [x] 6.11 [iuppiter] Networking works: DHCP on virtio NIC

### Phase 7: IuppiterOS Appliance Configuration

- [x] 7.1 [iuppiter] smartmontools installed and `smartctl --version` runs
- [x] 7.2 [iuppiter] hdparm installed and `hdparm --version` runs
- [x] 7.3 [iuppiter] sg3_utils installed: sg_inq, sg_sat_identify, sg_readcap all in PATH
- [x] 7.4 [iuppiter] sdparm, lsscsi, nvme-cli installed and in PATH
- [x] 7.5 [iuppiter] /var/data mount point exists (data partition for refurbishment artifacts)
- [x] 7.6 [iuppiter] /etc/iuppiter/ config directory exists
- [x] 7.7 [iuppiter] /opt/iuppiter/ binary directory exists
- [x] 7.8 [iuppiter] iuppiter-engine OpenRC service script in /etc/init.d/ (placeholder binary OK — just needs to start/stop cleanly)
- [x] 7.9 [iuppiter] Operator user created with wheel + disk group membership (disk group for /dev/sd* access)
- [x] 7.10 [iuppiter] udev rule: set mq-deadline I/O scheduler for rotational drives
- [x] 7.11 [iuppiter] /dev/sg* devices accessible after boot (SCSI generic for smartctl SG_IO passthrough)

### Phase 8: REDESIGN & REWRITE Install-Tests Harness (CRITICAL PRIORITY)

**🔴 STOP "FIXING" THE BROKEN HARNESS - THROW IT OUT AND START FRESH**

**User directive:** "completely rewrite and completely redesign that so that it works for crying out loud"

**Why the current approach is fundamentally broken:**
- Serial console I/O buffering has been "fixed" for weeks - still doesn't work
- Phase 6 post-reboot has been broken "for ages" - never worked
- Architecture is wrong: fighting against stdio pipes, threads, buffering
- Waste of time trying to patch a fundamentally flawed design

**STOP doing this:**
- ❌ Adding more logging to Console I/O
- ❌ Tweaking buffer sizes and flush() calls
- ❌ Fighting with thread synchronization
- ❌ Debugging why serial output disappears

**START doing this:**
- ✅ Design a NEW testing architecture that actually works
- ✅ Pick a DIFFERENT approach (QMP, expect, network signaling, etc.)
- ✅ Implement from scratch in a NEW crate
- ✅ Prove it works, THEN migrate tests over

#### 8.1: Design New Testing Architecture

**Research alternative approaches (pick ONE that actually works):**

- [ ] 8.1.1 **Option A: QEMU QMP (JSON protocol)** - Control QEMU directly, no serial parsing
  - QMP can: send keys, read screen, query VM state, take screenshots
  - Pros: Official QEMU protocol, designed for automation
  - Cons: More complex than serial, need JSON parsing
  - Research: Read QEMU QMP docs, find Rust QMP client library

- [ ] 8.1.2 **Option B: Expect-like scripting** - Use rexpect or similar
  - Pattern: spawn process, wait for pattern, send input, repeat
  - Pros: Designed for interactive automation, widely used
  - Cons: Still relies on stdio (but libs handle buffering correctly)
  - Research: Check if rexpect crate works with QEMU serial

- [ ] 8.1.3 **Option C: Network-based signaling** - Test agent inside VM signals completion
  - Pattern: VM boots, runs test script, sends HTTP/TCP signal to host
  - Pros: No stdio issues, explicit completion signals
  - Cons: Need network stack in initramfs
  - Research: How does cloud-init testing work?

- [ ] 8.1.4 **Option D: Screenshot + OCR** - Visual verification
  - Pattern: QEMU vnc/spice, take screenshots, OCR text
  - Pros: See exactly what user sees
  - Cons: Slow, OCR flaky, hard to script commands
  - Research: tesseract-ocr for screenshot text extraction

- [ ] 8.1.5 **Decision: Pick ONE approach** - Document choice in TEAM_154 with rationale

#### 8.2: Proof of Concept - Boot Detection

**Build minimal PoC to prove the chosen approach works:**

- [ ] 8.2.1 Create NEW crate: `testing/install-tests-v2/` (DO NOT modify old harness)
- [ ] 8.2.2 Implement ONLY boot detection with chosen approach
- [ ] 8.2.3 Test: Boot AcornOS ISO, detect ___SHELL_READY___ marker
- [ ] 8.2.4 Test: Boot IuppiterOS ISO, detect ___SHELL_READY___ marker
- [ ] 8.2.5 Test: Run 10 times - should succeed 10/10 (prove reliability)
- [ ] 8.2.6 If PoC fails: STOP, pick different approach from 8.1, try again
- [ ] 8.2.7 Document PoC results in TEAM_154

#### 8.3: Implement New Test Harness

**Only proceed if PoC proved reliable (10/10 success rate):**

- [ ] 8.3.1 Design test harness API - how tests will be written
- [ ] 8.3.2 Implement VM lifecycle: start, wait for boot, send commands, capture output, shutdown
- [ ] 8.3.3 Implement test phases: Phase 1 (boot), Phase 2 (disk), Phase 3 (base), Phase 4 (config), Phase 5 (bootloader)
- [ ] 8.3.4 Implement Phase 6 (post-reboot) - the "broken for ages" phase
- [ ] 8.3.5 Add distro abstraction: AcornOS vs IuppiterOS differences
- [ ] 8.3.6 Add error reporting: clear messages when tests fail
- [ ] 8.3.7 Test: Run Phase 1-5 for AcornOS - should pass

#### 8.4: Migrate Tests from Old Harness

**Port test cases to new harness (throw away old implementation):**

- [ ] 8.4.1 Port AcornOS Phase 1 tests (boot, ISO detection, clock)
- [ ] 8.4.2 Port AcornOS Phase 2 tests (disk partitioning, mounting)
- [ ] 8.4.3 Port AcornOS Phase 3 tests (recstrap, recfstab, recchroot)
- [ ] 8.4.4 Port AcornOS Phase 4 tests (timezone, hostname, users)
- [ ] 8.4.5 Port AcornOS Phase 5 tests (bootloader, kernel, services)
- [ ] 8.4.6 Port AcornOS Phase 6 tests (post-reboot verification)
- [ ] 8.4.7 Port IuppiterOS tests (same phases, different distro context)

#### 8.5: Replace Old Harness

**Delete broken code, promote new harness:**

- [ ] 8.5.1 Rename `testing/install-tests/` → `testing/install-tests-OLD-BROKEN/`
- [ ] 8.5.2 Rename `testing/install-tests-v2/` → `testing/install-tests/`
- [ ] 8.5.3 Update workspace Cargo.toml to use new harness
- [ ] 8.5.4 Update CI/scripts to use new harness
- [ ] 8.5.5 Delete old harness directory entirely (no looking back!)
- [ ] 8.5.6 Update TEAM_154: Document what was redesigned and why it works now
- [ ] 8.5.7 Close TEAM_154: Problem solved with new architecture

#### 8.6: Verify All Install-Tests Pass

**Run the newly unblocked tests:**

- [ ] 8.6.1 [acorn] Phase 1 (Boot): ISO detected, UEFI boot, clock
- [ ] 8.6.2 [acorn] Phase 2 (Disk): GPT partitioning, FAT32 ESP + ext4 root
- [ ] 8.6.3 [acorn] Phase 3 (Base System): recstrap, recfstab, recchroot
- [ ] 8.6.4 [acorn] Phase 4 (Config): timezone, hostname, root password, users
- [ ] 8.6.5 [acorn] Phase 5 (Bootloader): kernel + initramfs, systemd-boot, services
- [ ] 8.6.6 [acorn] Phase 6 (Post-reboot): installed system boots and login works
- [ ] 8.6.7 [iuppiter] All phases (1-6) pass with IuppiterOS distro context

**Success criteria:** All 7 tests pass reliably (10/10 runs each)

**Priority:** This phase MUST be completed before Phase 10 because you can't verify display support
without a working test harness. Don't waste time on display if you can't test it.

### Phase 8-OLD: Manual Install-Tests Verification (TEMPORARY)

**These tasks were previously "Phase 8" but are now blocked by Phase 8-NEW (harness fix).**
**Once harness is fixed, these become automated tests instead of manual verification.**

**AcornOS install-tests:**
- [x] 8.1 [acorn] install-tests `--distro acorn` mode runs (AcornOS DistroContext already exists)
- [x] 8.2 [acorn] Phase 1 (Boot): ISO detected, UEFI boot, system clock reasonable
- [BLOCKED] 8.3 [acorn] Phase 2 (Disk): GPT partitioning, FAT32 ESP + ext4 root, mounted correctly
- [BLOCKED] 8.4 [acorn] Phase 3 (Base System): recstrap extracts rootfs, recfstab generates fstab, recchroot works
- [BLOCKED] 8.5 [acorn] Phase 4 (Config): timezone, hostname, root password, user account created
- [BLOCKED] 8.6 [acorn] Phase 5 (Bootloader): kernel + initramfs copied to ESP, systemd-boot installed, services enabled
- [BLOCKED] 8.7 [acorn] Phase 6 (Post-reboot): installed system boots and login works (KNOWN BROKEN — may be BLOCKED)

**IuppiterOS install-tests:**
- [x] 8.8 [iuppiter] Create IuppiterOS DistroContext in testing/install-tests/src/distro/iuppiter.rs
- [x] 8.9 [iuppiter] IuppiterOS DistroContext: OpenRC init, ash shell, serial console boot patterns, iuppiter services
- [x] 8.10 [iuppiter] Register iuppiter in distro/mod.rs so `--distro iuppiter` is recognized
- [x] 8.11 [iuppiter] install-tests `--distro iuppiter` mode runs
- [BLOCKED] 8.12 [iuppiter] Phases 1-5 pass for IuppiterOS (same steps as AcornOS but with iuppiter identity)
- [BLOCKED] 8.13 [iuppiter] Phase 6 (Post-reboot): may be BLOCKED (same as AcornOS)

**IuppiterOS-specific verification (manual or scripted in QEMU):**
- [x] 8.14 [iuppiter] smartctl runs against QEMU virtual drive (exit 0 or known SMART error code)
- [x] 8.15 [iuppiter] lsscsi shows at least one device in QEMU
- [x] 8.16 [iuppiter] hdparm -I /dev/sda works in QEMU
- [x] 8.17 [iuppiter] No GPU/DRM kernel modules loaded (lsmod | grep drm returns empty)
- [x] 8.18 [iuppiter] /dev/sg* devices exist (SCSI generic loaded)
- [x] 8.19 [iuppiter] All OpenRC services running: networking, eudev, chronyd, sshd, iuppiter-engine
- [x] 8.20 [iuppiter] /var/data exists and is writable
- [x] 8.21 [iuppiter] iuppiter-engine service in rc-status output

**AcornOS install-tests:**
- [x] 8.1 [acorn] install-tests `--distro acorn` mode runs (AcornOS DistroContext already exists)
- [x] 8.2 [acorn] Phase 1 (Boot): ISO detected, UEFI boot, system clock reasonable
- [BLOCKED] 8.3 [acorn] Phase 2 (Disk): GPT partitioning, FAT32 ESP + ext4 root, mounted correctly
- [BLOCKED] 8.4 [acorn] Phase 3 (Base System): recstrap extracts rootfs, recfstab generates fstab, recchroot works
- [BLOCKED] 8.5 [acorn] Phase 4 (Config): timezone, hostname, root password, user account created
- [BLOCKED] 8.6 [acorn] Phase 5 (Bootloader): kernel + initramfs copied to ESP, systemd-boot installed, services enabled
- [BLOCKED] 8.7 [acorn] Phase 6 (Post-reboot): installed system boots and login works (KNOWN BROKEN — may be BLOCKED)

**IuppiterOS install-tests:**
- [x] 8.8 [iuppiter] Create IuppiterOS DistroContext in testing/install-tests/src/distro/iuppiter.rs
- [x] 8.9 [iuppiter] IuppiterOS DistroContext: OpenRC init, ash shell, serial console boot patterns, iuppiter services
- [x] 8.10 [iuppiter] Register iuppiter in distro/mod.rs so `--distro iuppiter` is recognized
- [x] 8.11 [iuppiter] install-tests `--distro iuppiter` mode runs
- [BLOCKED] 8.12 [iuppiter] Phases 1-5 pass for IuppiterOS (same steps as AcornOS but with iuppiter identity)
- [BLOCKED] 8.13 [iuppiter] Phase 6 (Post-reboot): may be BLOCKED (same as AcornOS)

**IuppiterOS-specific verification (manual or scripted in QEMU):**
- [x] 8.14 [iuppiter] smartctl runs against QEMU virtual drive (exit 0 or known SMART error code)
- [x] 8.15 [iuppiter] lsscsi shows at least one device in QEMU
- [x] 8.16 [iuppiter] hdparm -I /dev/sda works in QEMU
- [x] 8.17 [iuppiter] No GPU/DRM kernel modules loaded (lsmod | grep drm returns empty)
- [x] 8.18 [iuppiter] /dev/sg* devices exist (SCSI generic loaded)
- [x] 8.19 [iuppiter] All OpenRC services running: networking, eudev, chronyd, sshd, iuppiter-engine
- [x] 8.20 [iuppiter] /var/data exists and is writable
- [x] 8.21 [iuppiter] iuppiter-engine service in rc-status output

### Phase 9: Custom Kernel (If Time Permits)

- [ ] 9.1 [iuppiter] Kernel config based on .docs/56_KCONFIG_REFURB_SERVER.md
- [ ] 9.2 [iuppiter] Kernel builds from linux/ submodule source
- [ ] 9.3 [iuppiter] Custom kernel replaces Alpine linux-lts in ISO
- [ ] 9.4 [iuppiter] All SAS/SCSI/AHCI modules present, DRM for display (GPU drivers required for kiosk)

### Phase 10: IuppiterOS Display Stack & Kiosk Mode

**CRITICAL:** IuppiterOS was incorrectly built without display support. The PRD previously said
"headless" but this was WRONG. IuppiterOS is a touchscreen kiosk appliance running iuppiter-dar
(Tauri GUI app) fullscreen on boot.

#### 10.1: Display Foundation Packages

- [ ] 10.1.1 [iuppiter] Add mesa, mesa-dri-gallium, mesa-egl, mesa-gbm to distro-spec packages (GPU drivers)
- [ ] 10.1.2 [iuppiter] Add libdrm to packages (Direct Rendering Manager for kernel mode setting)
- [ ] 10.1.3 [iuppiter] Add libinput, libinput-dev, libxkbcommon, xkeyboard-config (touchscreen + input handling)
- [ ] 10.1.4 [iuppiter] Add font-dejavu, font-liberation, fontconfig (text rendering for GUI)
- [ ] 10.1.5 [iuppiter] Verify GPU drivers install correctly in rootfs (/usr/lib/dri/, /usr/lib/xorg/)
- [ ] 10.1.6 [iuppiter] Test: Boot ISO and verify DRM devices exist (/dev/dri/card0, /dev/dri/renderD128)

#### 10.2: Wayland Compositor (Cage Kiosk Mode)

- [ ] 10.2.1 [iuppiter] Add cage, wlroots packages (minimal Wayland kiosk compositor)
- [ ] 10.2.2 [iuppiter] Add seatd (seat management for Wayland)
- [ ] 10.2.3 [iuppiter] Create OpenRC service /etc/init.d/iuppiter-kiosk that starts Cage
- [ ] 10.2.4 [iuppiter] Configure Cage to run as operator user (not root)
- [ ] 10.2.5 [iuppiter] Service depends on: localmount, eudev, bootmisc (wait for /dev/dri)
- [ ] 10.2.6 [iuppiter] Enable iuppiter-kiosk in default runlevel (rc-update add iuppiter-kiosk default)
- [ ] 10.2.7 [iuppiter] Test: Boot ISO, Cage compositor starts on /dev/fb0 or DRM

#### 10.3: GTK + WebView Stack (Tauri Dependencies)

- [ ] 10.3.1 [iuppiter] Add gtk+3.0, glib, cairo, pango, gdk-pixbuf, atk (GTK3 stack)
- [ ] 10.3.2 [iuppiter] Add webkit2gtk (Tauri WebView engine - THE BIG DEPENDENCY ~150MB)
- [ ] 10.3.3 [iuppiter] Add harfbuzz, fribidi (text shaping for WebKit)
- [ ] 10.3.4 [iuppiter] Add libsoup (HTTP library - WebKit dependency)
- [ ] 10.3.5 [iuppiter] Add gstreamer, gst-plugins-base, gst-plugins-good (media playback for WebKit)
- [ ] 10.3.6 [iuppiter] Verify all GTK/WebKit libraries install to /usr/lib/ in rootfs
- [ ] 10.3.7 [iuppiter] Rootfs size check: Should be ~300-350MB compressed (up from 39MB, acceptable)

#### 10.4: iuppiter-dar Application Integration

**Source:** `/home/vince/Projects/iuppiter-dar` (Tauri app - Rust backend + React frontend)

- [ ] 10.4.1 [iuppiter] Build iuppiter-dar: `cd /home/vince/Projects/iuppiter-dar && bun install && bun run build && cargo tauri build`
- [ ] 10.4.2 [iuppiter] Verify DAR binary exists: `src-tauri/target/release/iuppiter-dar`
- [ ] 10.4.3 [iuppiter] Copy DAR binary to IuppiterOS staging: `/opt/iuppiter/iuppiter-dar` (mode 0755)
- [ ] 10.4.4 [iuppiter] Update /etc/init.d/iuppiter-kiosk to launch DAR: `cage -- /opt/iuppiter/iuppiter-dar`
- [ ] 10.4.5 [iuppiter] Add ddrescue package (DAR dependency - bad sector remediation)
- [ ] 10.4.6 [iuppiter] Add typst package if available in Alpine (PDF report generation - may need custom build)
- [ ] 10.4.7 [iuppiter] Test: Boot ISO, Cage launches DAR fullscreen on display

#### 10.5: Kiosk Security Lockdown

- [ ] 10.5.1 [iuppiter] Disable virtual terminals in /etc/inittab (comment out tty1-6 gettys, keep ttyS0 for debug)
- [ ] 10.5.2 [iuppiter] Configure Cage to disable compositor escape sequences (no Alt+Tab, no Ctrl+Alt+Fn)
- [ ] 10.5.3 [iuppiter] Create respawn wrapper for DAR in OpenRC service (auto-restart on crash)
- [ ] 10.5.4 [iuppiter] Lock down SSH to operator user only (disable root SSH in /etc/ssh/sshd_config)
- [ ] 10.5.5 [iuppiter] Test: Attempt to escape kiosk (Ctrl+Alt+F1, Alt+F4, etc.) - should fail
- [ ] 10.5.6 [iuppiter] Test: Kill DAR process - should auto-restart within 5 seconds

#### 10.6: Touchscreen Input Configuration

- [ ] 10.6.1 [iuppiter] Verify touchscreen udev rules in /etc/udev/rules.d/ (libinput should auto-detect)
- [ ] 10.6.2 [iuppiter] Configure libinput for touchscreen calibration (if needed)
- [ ] 10.6.3 [iuppiter] Test: Touch input works in DAR UI (tap, drag, multi-touch gestures)
- [ ] 10.6.4 [iuppiter] Test: On-screen keyboard appears when text input focused (if DAR implements it)

#### 10.7: Live ISO vs Installed System

- [ ] 10.7.1 [iuppiter] Live ISO: Kiosk mode auto-starts on boot (for demo/testing)
- [ ] 10.7.2 [iuppiter] Installed system: Kiosk mode auto-starts on boot (production deployment)
- [ ] 10.7.3 [iuppiter] Data partition: /var/data persists across reboots on installed systems (ext4 on /dev/sda3)
- [ ] 10.7.4 [iuppiter] Configuration persistence: /etc/iuppiter/ configs survive reboots (bind mount from data partition)

#### 10.8: End-to-End Verification

- [ ] 10.8.1 [iuppiter] Boot live ISO on real hardware with touchscreen - DAR appears fullscreen
- [ ] 10.8.2 [iuppiter] Touch input works, DAR UI responds correctly
- [ ] 10.8.3 [iuppiter] DAR can detect drives (/dev/sda, /dev/nvme0n1) and run diagnostics
- [ ] 10.8.4 [iuppiter] smartctl, hdparm, sg3_utils work from DAR backend
- [ ] 10.8.5 [iuppiter] DAR can generate PDF reports (typst rendering works)
- [ ] 10.8.6 [iuppiter] Kill DAR - auto-restarts immediately
- [ ] 10.8.7 [iuppiter] No escape from kiosk mode (locked down correctly)

---

## Constraints

- AcornOS and IuppiterOS are SEPARATE git submodules
- Use `distro-spec/src/acorn/` and `distro-spec/src/iuppiter/` for ALL constants
- Use `distro-builder/` for ALL shared build abstractions
- Use AcornOS's declarative component system (Installable trait + Op enum + executor) — don't write imperative scripts
- Mirror leviso's architecture but do NOT copy-paste leviso code
- Use `recipe` crate for Alpine APK dependency resolution
- OpenRC, NOT systemd — init scripts in /etc/init.d/, not unit files
- musl, NOT glibc — watch for glibc-isms
- busybox, NOT GNU coreutils — ash not bash for system scripts
- IuppiterOS: GUI kiosk on touchscreen (Cage compositor + DAR fullscreen), serial console for debugging
- Kernel reuse: IuppiterOS can steal AcornOS kernel (same Alpine linux-lts) until Phase 9
