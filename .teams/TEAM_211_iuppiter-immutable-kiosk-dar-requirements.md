# TEAM_211: IuppiterOS Additional Requirements - Immutable Base, Kiosk Mode, DAR Preinstalled

**Date:** 2026-02-04
**Status:** REQUIREMENTS GATHERING - Feasibility research only, no implementation yet
**Impact:** HIGH - Defines IuppiterOS production deployment model

---

## TL;DR

Three new requirements for IuppiterOS production deployment:
1. **Immutable base** - Read-only root filesystem (PARTIALLY IMPLEMENTED)
2. **Kiosk mode** - Locked-down single-application environment (NEEDS DESIGN)
3. **iuppiter-dar preinstalled** - Bundle the Tauri diagnostics application (FEASIBLE)

This document explores feasibility and implementation approaches for each requirement.

---

## Requirement 1: Immutable Base

### What It Means

**Immutable base** = Root filesystem cannot be modified after deployment. Configuration and state stored on writable partitions.

### Current Status: PARTIALLY IMPLEMENTED ✅

IuppiterOS **already has an immutable base** via EROFS:

**From current architecture:**
```rust
// IuppiterOS/src/artifact/rootfs.rs
// Creates read-only EROFS compressed filesystem
let erofs_output = ctx.output.join("filesystem.erofs");
build_erofs(&staging, &erofs_output, &ErOpts::default())?;
```

**Boot process:**
```
/init mounts EROFS rootfs (read-only)
  → Overlay with tmpfs for /etc, /var, /home (writable)
  → Switch root to overlay
```

**What's already immutable:**
- `/usr` - All binaries, libraries (EROFS)
- `/bin`, `/sbin`, `/lib` - Merged-usr symlinks to `/usr` (EROFS)
- `/opt/iuppiter` - Appliance binaries (EROFS)

**What's currently writable (tmpfs overlay):**
- `/etc` - Configuration files
- `/var` - Logs, data, state
- `/home` - User home directories
- `/tmp` - Temporary files

### Feasibility: ALREADY DONE ✅

**No additional work required** for immutable base. The current EROFS + overlay architecture already provides this.

### Potential Enhancement: Data Partition Separation

**Optional improvement:**
```
/dev/sda1 - EFI System Partition (FAT32, 512MB)
/dev/sda2 - Root (EROFS, 500MB, read-only) ← Currently tmpfs overlay
/dev/sda3 - Data (ext4, remaining space, writable)
```

Mount `/dev/sda3` to `/var/data` for:
- Refurbishment reports
- SMART logs
- Configuration backups
- Drive inventory database

**Benefits:**
- Persistent data across reboots (currently lost with tmpfs)
- Clear separation: system (immutable) vs. data (mutable)
- Easier backup/restore workflows

**Trade-offs:**
- Requires disk installation (not applicable to live ISO)
- Need migration plan for /etc config persistence

---

## Requirement 2: Kiosk Mode Preinstalled

### What It Means

**Kiosk mode** = Locked-down environment that launches a single application on boot, preventing access to underlying OS.

### Typical Kiosk Mode Features

1. **Auto-start application** - App launches on boot without manual intervention
2. **No desktop environment** - Direct framebuffer or minimal compositor
3. **Restricted input** - Disable Alt+Tab, Ctrl+Alt+F1, etc.
4. **No shell access** - Block virtual terminals and SSH (or lock down to admin user only)
5. **Application containment** - If app crashes, respawn automatically
6. **Peripheral lockdown** - Disable USB storage mounting, network configuration changes

### Current IuppiterOS Boot Flow

```
Kernel boot
  → OpenRC init
    → Services: networking, sshd, chronyd, eudev
    → Getty on ttyS0 (serial console)
      → Root autologin (LIVE ONLY)
        → Bash shell
```

**Current state:** Full shell access, no kiosk mode.

### Kiosk Mode Implementation Approaches

#### Option A: Cage (Wayland Kiosk Compositor)

**What it is:** Minimal Wayland compositor that runs a single fullscreen client.

**Architecture:**
```
OpenRC init
  → Start Cage compositor on /dev/fb0
    → Cage launches iuppiter-dar (fullscreen, no window borders)
      → User interacts with DAR only
```

**Required packages:**
```rust
// distro-spec/src/iuppiter/packages.rs
pub const KIOSK_PACKAGES: &[&str] = &[
    "cage",               // Wayland kiosk compositor
    "mesa",               // GPU drivers (DRI)
    "mesa-dri-gallium",   // Software rendering fallback
    "eudev",              // Device management (already present)
    "libinput",           // Input handling
    "xkeyboard-config",   // Keyboard layouts
];
```

**OpenRC service:**
```bash
#!/sbin/openrc-run
# /etc/init.d/iuppiter-kiosk

description="IuppiterOS Kiosk Mode (Cage + DAR)"

command="/usr/bin/cage"
command_args="-d -- /opt/iuppiter/iuppiter-dar"
command_user="operator"
command_background=yes
pidfile="/run/iuppiter-kiosk.pid"

depend() {
    need localmount
    after bootmisc eudev
}
```

**Pros:**
- Minimal attack surface (no desktop environment bloat)
- Wayland = modern, secure display protocol
- Cage is actively maintained, designed for kiosks
- Works with hardware acceleration

**Cons:**
- Requires GPU drivers (adds ~50-100MB to rootfs)
- IuppiterOS is currently **headless** (serial console only)
- Adds display/keyboard/mouse dependencies

**Compatibility issue:** IuppiterOS was designed as a **headless appliance** (serial console primary, no display). Adding Cage requires:
- Display output support (framebuffer or DRM)
- Input device support (keyboard/mouse for DAR UI)
- GPU drivers (even software rendering adds packages)

**Is this a departure from "headless appliance"?** YES. Need user confirmation.

#### Option B: Framebuffer Direct Rendering (No Compositor)

**What it is:** DAR renders directly to `/dev/fb0` framebuffer using SDL2 or similar.

**Architecture:**
```
OpenRC init
  → Start iuppiter-dar on /dev/fb0 (fullscreen, no compositor)
    → DAR owns the framebuffer
```

**Required changes:**
- DAR needs framebuffer rendering backend (currently Tauri = WebView = needs compositor)
- Tauri doesn't support direct framebuffer rendering
- Would require rewriting DAR frontend

**Feasibility:** NOT FEASIBLE without major DAR rewrite.

#### Option C: X11 + matchbox-window-manager (Legacy Kiosk)

**What it is:** Minimal X11 window manager designed for embedded kiosks.

**Required packages:**
```rust
"xorg-server",
"matchbox-window-manager",
"mesa", "mesa-dri-gallium",
```

**Pros:**
- Well-tested in embedded/kiosk deployments
- Compatible with Tauri (uses WebView which needs X11/Wayland)

**Cons:**
- X11 is legacy (security concerns, larger attack surface)
- Heavier than Wayland (more packages)
- Same GPU/display requirements as Cage

#### Option D: Auto-launch in SSH Session (Minimal Kiosk)

**What it is:** Auto-start DAR when operator user logs in via SSH (no display).

**Architecture:**
```
Operator SSH login
  → .profile launches iuppiter-dar (TUI mode)
    → DAR runs in terminal
```

**Required changes:**
- DAR needs TUI (text UI) mode in addition to GUI
- Or: DAR launches in headless mode, operator uses web UI via browser

**Pros:**
- No display/GPU dependencies (keeps IuppiterOS headless)
- Minimal package additions
- Aligns with "headless appliance" design

**Cons:**
- Not true "kiosk mode" (operator can Ctrl+C, access shell)
- Requires DAR to support headless operation

### Feasibility Assessment: NEEDS CLARIFICATION

**Key question:** Does "kiosk mode" mean:
1. **Display kiosk** (physical monitor + keyboard/mouse, DAR GUI fullscreen)?
2. **SSH kiosk** (remote access, DAR auto-launches in TUI/web mode)?
3. **Hybrid** (supports both modes)?

**If display kiosk:**
- **Feasible** with Cage (Wayland compositor)
- **Requires:** Adding GPU drivers, display support, ~100MB rootfs increase
- **Conflicts with:** "Headless appliance" design (no display in PRD)

**If SSH kiosk:**
- **Feasible** with .profile auto-launch + restricted shell
- **Requires:** DAR TUI mode or web UI
- **Aligns with:** Headless appliance design

**Recommendation:** Clarify use case before choosing approach.

---

## Requirement 3: iuppiter-dar Preinstalled

### What It Is

**iuppiter-dar** = Tauri desktop application for HDD diagnostics and refurbishment.

**Location:** `/home/vince/Projects/iuppiter-dar`

**Tech stack:**
- **Frontend:** React 19 + TypeScript + Vite
- **Backend:** Rust + Tauri 2.5
- **Build output:** Platform-specific binary (AppImage, .deb, or standalone executable)

### Current DAR Architecture

From `/home/vince/Projects/iuppiter-dar/README.md`:
```
React 19 Frontend (TypeScript, Tailwind)
         ↓ (Tauri IPC)
   src-tauri/ Orchestrator
         ↓
   Direct ioctl to /dev/sd*, /dev/nvme*
         ↓
   Hardware: SATA, NVMe, SAS, USB drives
```

**Key dependencies:**
- **smartctl** (already in IuppiterOS packages)
- **hdparm** (already in IuppiterOS packages)
- **sg3_utils** (already in IuppiterOS packages)
- **ddrescue** (NOT yet in packages - need to add)
- **typst** (PDF report generation - need to add)

### Preinstallation Approaches

#### Option A: Build DAR and Copy Binary to /opt/iuppiter/

**Process:**
```bash
cd /home/vince/Projects/iuppiter-dar
bun install
bun run build         # Vite builds frontend
cargo tauri build     # Builds Rust backend + bundles app

# Output (example):
src-tauri/target/release/iuppiter-dar  # Standalone binary
```

**Installation in IuppiterOS:**
```rust
// IuppiterOS/src/component/definitions.rs
pub static IUPPITER_DAR: Component = Component {
    name: "iuppiter-dar",
    phase: Phase::Final,
    ops: &[
        // Copy DAR binary from build output
        copy_file_external(
            "/home/vince/Projects/iuppiter-dar/src-tauri/target/release/iuppiter-dar",
            "opt/iuppiter/iuppiter-dar"
        ),
        // Set executable permissions
        file_mode("opt/iuppiter/iuppiter-dar", 0o755),
        // Create systemd/OpenRC service to auto-start DAR
        // (Depends on kiosk mode design from Requirement 2)
    ],
};
```

**Pros:**
- Simple integration
- DAR binary is self-contained (Tauri bundles frontend assets)
- No runtime dependencies beyond existing packages

**Cons:**
- Requires building DAR on host before building IuppiterOS
- Binary size ~20-50MB (Tauri bundles Chromium WebView)
- Need to coordinate DAR version with ISO build

#### Option B: Add DAR Source as Git Submodule

**Structure:**
```
IuppiterOS/
  deps/
    iuppiter-dar/  ← git submodule
```

**Build process:**
```rust
// IuppiterOS builder automatically builds DAR during ISO build
fn build_dar() -> Result<PathBuf> {
    let dar_dir = Path::new("deps/iuppiter-dar");

    // Install JS deps
    Command::new("bun").args(&["install"]).current_dir(&dar_dir).run()?;

    // Build frontend
    Command::new("bun").args(&["run", "build"]).current_dir(&dar_dir).run()?;

    // Build Tauri app
    Command::new("cargo").args(&["tauri", "build"]).current_dir(&dar_dir).run()?;

    Ok(dar_dir.join("src-tauri/target/release/iuppiter-dar"))
}
```

**Pros:**
- Automated build (no manual coordination)
- Version pinning via submodule commit hash
- Reproducible builds

**Cons:**
- Longer IuppiterOS build time (DAR builds from source)
- Requires Node.js/Bun on host (not just Rust)
- More complex build dependencies

#### Option C: Package DAR as Alpine APK (Future-Proof)

**What it is:** Create an Alpine APK package for DAR, add to custom repository.

**Process:**
1. Create Alpine APKBUILD for iuppiter-dar
2. Build APK using `abuild`
3. Host custom APK repository
4. Add to IuppiterOS package list

**Pros:**
- Clean packaging model (APK handles deps, upgrades)
- Can update DAR independently of OS
- Standard Alpine tooling

**Cons:**
- Requires APK packaging expertise
- Need to maintain custom Alpine repo
- Overkill for single-binary app

### Feasibility: HIGHLY FEASIBLE ✅

**Recommended approach:** **Option A** (copy binary from pre-built DAR)

**Rationale:**
- Simplest integration
- IuppiterOS build doesn't depend on Node.js toolchain
- DAR versioning controlled externally (update binary, rebuild ISO)

**Missing packages to add:**
```rust
// distro-spec/src/iuppiter/packages.rs
pub const REFURBISHMENT_PACKAGES: &[&str] = &[
    // ... existing packages ...
    "ddrescue",  // ADD - Bad sector remediation (DAR dependency)
    "typst",     // ADD - PDF report generation (DAR dependency)
];
```

**Build steps:**
1. Build DAR: `cd iuppiter-dar && bun run build && cargo tauri build`
2. Copy binary to IuppiterOS: `cp src-tauri/target/release/iuppiter-dar ../IuppiterOS/assets/`
3. Update IuppiterOS component to install from assets/
4. Rebuild IuppiterOS ISO

### DAR Runtime Dependencies Check

**Already in IuppiterOS packages:**
- ✅ smartctl (smartmontools)
- ✅ hdparm
- ✅ sg3_utils (sg_inq, sg_sat_identify, etc.)
- ✅ lsscsi
- ✅ nvme-cli

**Need to add:**
- ❌ ddrescue (bad sector remediation)
- ❌ typst (PDF generation - may need to build from source if not in Alpine)

**Tauri WebView dependencies:**
- ❌ webkit2gtk (WebView engine - LARGE dependency ~50MB)
- ❌ gtk3 (UI toolkit)
- ❌ glib, cairo, pango (GTK deps)

**Problem:** Tauri on Linux uses WebKitGTK, which requires:
```
webkit2gtk → gtk3 → cairo → pango → fontconfig → freetype → ...
```

This adds **~100-150MB** to the rootfs and pulls in desktop environment dependencies.

**Potential solution:** Build Tauri with custom renderer (WGPU) or investigate headless mode.

---

## Implementation Dependencies

### Requirement 1: Immutable Base
- **Status:** ✅ Already implemented (EROFS rootfs)
- **Effort:** 0 hours (done)
- **Packages:** None

### Requirement 2: Kiosk Mode
- **Status:** ⚠️ Needs design decision (display vs. headless)
- **Effort:** 8-16 hours (depends on approach)
- **Packages:**
  - Display kiosk: cage, mesa, libinput, xkeyboard-config (~100MB)
  - SSH kiosk: None (use .profile auto-launch)

### Requirement 3: iuppiter-dar Preinstalled
- **Status:** ✅ Feasible with binary copy
- **Effort:** 4-6 hours (component integration + testing)
- **Packages:**
  - ddrescue (~200KB)
  - typst (~10MB if available, may need custom build)
  - webkit2gtk + deps (~150MB) ← **BLOCKER**

**Major concern:** Tauri's WebKitGTK dependency adds significant rootfs bloat and desktop dependencies to a "headless appliance".

---

## USER ANSWERS (2026-02-04)

### 1. Kiosk Mode Scope
✅ **GUI on touchscreen** - Full graphical kiosk mode with physical touchscreen

### 2. Display Hardware
✅ **Physical display present** - Touchscreen monitor attached

### 3. WebKitGTK Bloat
✅ **Acceptable** - 150MB dependency increase is fine for Tauri

### 4. Headless Confusion
❌ **PRD WAS WRONG** - IuppiterOS was ALWAYS meant to have a display!
- The PRD incorrectly said "headless appliance"
- Current implementation is MISSING all display support packages
- Need to ADD: mesa, GPU drivers, Wayland, GTK, fonts, etc.

## CRITICAL ISSUE: Display Support Missing

**Current state:** IuppiterOS has ZERO display packages:
- ❌ No GPU drivers (mesa, mesa-dri-gallium)
- ❌ No display server (Wayland/X11)
- ❌ No compositor (Cage, Weston)
- ❌ No UI toolkit (GTK3, webkit2gtk)
- ❌ No fonts
- ❌ No input handling (libinput, touchscreen drivers)

**Correct requirement:** Full GUI stack for touchscreen kiosk

---

## Required Packages for Display Support

### GPU & Display Stack (~100MB)

```rust
// distro-spec/src/iuppiter/packages.rs
// NEW: Display support packages (currently MISSING)

pub const DISPLAY_PACKAGES: &[&str] = &[
    // GPU drivers
    "mesa",                 // OpenGL implementation
    "mesa-dri-gallium",     // Software + hardware rendering
    "mesa-egl",             // EGL for Wayland
    "mesa-gbm",             // Generic Buffer Management

    // DRM/KMS (kernel mode setting)
    "libdrm",               // Direct Rendering Manager

    // Wayland compositor (kiosk mode)
    "cage",                 // Minimal kiosk compositor
    "wlroots",              // Wayland compositor library (cage dependency)

    // Input handling
    "libinput",             // Input device handling
    "libinput-dev",         // Touchscreen support
    "xkeyboard-config",     // Keyboard layouts
    "libxkbcommon",         // Keyboard handling

    // Fonts (required for any text rendering)
    "font-dejavu",          // Default sans-serif font
    "font-liberation",      // Metric-compatible with Arial/Times
    "fontconfig",           // Font configuration
];
```

### GTK & WebView Stack for Tauri (~150MB)

```rust
pub const WEBVIEW_PACKAGES: &[&str] = &[
    // GTK3 (required by webkit2gtk)
    "gtk+3.0",
    "glib",
    "cairo",
    "pango",
    "gdk-pixbuf",
    "atk",

    // WebKitGTK (Tauri WebView engine)
    "webkit2gtk",

    // Additional GTK/WebKit deps
    "harfbuzz",             // Text shaping
    "fribidi",              // BiDi text support
    "libsoup",              // HTTP library (WebKit dep)
    "gstreamer",            // Media playback (WebKit dep)
    "gst-plugins-base",
    "gst-plugins-good",
];
```

### Total Addition: ~250-300MB to rootfs

**Current IuppiterOS rootfs:** 39MB (EROFS compressed)
**After display support:** ~330MB (EROFS compressed)

**Still smaller than AcornOS:** 190MB (AcornOS has desktop packages too)

---

## Recommendations

### Immediate: Fix Display Support

1. ✅ **Add DISPLAY_PACKAGES** to distro-spec/src/iuppiter/packages.rs
2. ✅ **Add WEBVIEW_PACKAGES** for Tauri support
3. ✅ **Test boot with display** (not just serial console)
4. ✅ **Update PRD** to remove "headless" references

### Implementation Order

**Phase 1: Display Stack** (Foundation)
1. Add GPU drivers (mesa, libdrm)
2. Add Cage compositor
3. Add fonts and input handling
4. Test: Boot to Cage compositor showing blank screen

**Phase 2: Kiosk Service** (Infrastructure)
1. Create OpenRC service for Cage
2. Configure auto-start on boot
3. Test: Boot to Cage with test application

**Phase 3: DAR Integration** (Application)
1. Add WebKitGTK and GTK3 packages
2. Build iuppiter-dar binary
3. Copy to /opt/iuppiter/iuppiter-dar
4. Configure Cage to launch DAR
5. Test: Boot to DAR fullscreen on touchscreen

**Phase 4: Kiosk Lockdown** (Security)
1. Disable virtual terminals (Ctrl+Alt+F1)
2. Disable compositor escape sequences
3. Configure input restrictions
4. Auto-restart DAR if it crashes

### Estimated Effort

- Display stack: 4-6 hours (package integration + testing)
- Kiosk service: 2-3 hours (OpenRC service + config)
- DAR integration: 4-6 hours (build + install + test)
- Lockdown: 2-4 hours (security hardening)

**Total: 12-19 hours** for full implementation

---

## Next Steps

1. ✅ **CORRECTED:** IuppiterOS has GUI on touchscreen (PRD was wrong)
2. 📝 **Update PRD:** Remove all "headless" references, add display requirements
3. 📦 **Add packages:** Display stack + WebView stack (~300MB added to rootfs)
4. 🔧 **Implement kiosk mode:** Cage compositor + DAR auto-launch
5. 🧪 **Test:** Boot to DAR fullscreen on touchscreen
6. 🔒 **Lockdown:** Disable escape routes, auto-restart on crash

**Ready to implement** - all questions answered, design complete

---

## Files Referenced

- `/home/vince/Projects/iuppiter-dar/README.md` - DAR project overview
- `/home/vince/Projects/iuppiter-dar/package.json` - Frontend build scripts
- `/home/vince/Projects/iuppiter-dar/src-tauri/Cargo.toml` - Rust backend
- `IuppiterOS/src/artifact/rootfs.rs` - EROFS rootfs builder (immutable base)
- `distro-spec/src/iuppiter/packages.rs` - Package definitions

---

## Feasibility Summary

| Requirement | Status | Effort | Notes |
|-------------|--------|--------|-------|
| 1. Immutable base | ✅ Done | 0h | Already implemented via EROFS |
| 2. Kiosk mode (GUI touchscreen) | ✅ Ready | 6-9h | Need to add display packages first |
| 3. DAR preinstalled | ✅ Ready | 4-6h | Tauri + WebKitGTK (~300MB total) |
| **CRITICAL FIX** | ⚠️ **Missing** | 4-6h | **Add all display support packages** |

**Overall:** All requirements are **ready to implement**. Main task is adding display stack that was incorrectly omitted.

**Rootfs size impact:**
- Current: 39MB (headless, WRONG)
- After display support: ~330MB (GUI kiosk, CORRECT)
- Still smaller than AcornOS: 190MB
