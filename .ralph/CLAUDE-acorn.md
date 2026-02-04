# CLAUDE.md — AcornOS Ralph Loop

You are working on **AcornOS**, a desktop-ready Linux distribution built on Alpine Linux.
You are being run in a Ralph loop. Read the PRD and progress file every iteration.

## What AcornOS Is

AcornOS is a sibling to LevitateOS. Same architecture, different base:

| | LevitateOS | AcornOS |
|---|---|---|
| Base packages | Rocky Linux RPMs | Alpine APKs |
| Init | systemd | OpenRC |
| libc | glibc | musl |
| Coreutils | GNU | busybox |
| Shell | bash | ash |
| Device manager | udev (systemd) | eudev |

Both produce: bootable ISO → EROFS rootfs + overlay → UKI boot via systemd-boot.

## Repository Structure (Submodules)

```
LevitateOS/                    # Parent repo (you are here)
├── AcornOS/                   # AcornOS builder — YOUR PRIMARY WORKSPACE
├── distro-spec/               # Shared specs (constants, paths, services)
├── distro-builder/            # Shared build abstractions (traits, ops)
├── leviso/                    # LevitateOS builder — REFERENCE ONLY
├── leviso-elf/                # ELF analysis (shared)
├── testing/
│   ├── install-tests/         # E2E tests — USE TO GRADE YOUR WORK
│   └── rootfs-tests/          # Behavioral tests
├── tools/
│   ├── recstrap/              # System extractor
│   ├── recfstab/              # Fstab generator
│   ├── recchroot/             # Chroot helper
│   ├── recqemu/               # QEMU launcher
│   ├── recuki/                # UKI builder
│   └── reciso/                # ISO utilities
└── linux/                     # Kernel source (submodule)
```

## Layer Boundaries (CRITICAL — DO NOT CROSS)

### You MAY modify:
- `AcornOS/` — builder implementation (your main workspace)
- `distro-spec/src/acorn/` — AcornOS-specific specs ONLY
- `distro-builder/` — shared abstractions IF both distros benefit

### You MAY read (reference only):
- `leviso/` — how LevitateOS does it (copy patterns, not code)
- `distro-spec/src/levitate/` — see what LevitateOS defines
- `testing/install-tests/` — understand test expectations
- `tools/` — understand tool APIs

### You MUST NOT modify:
- `leviso/` — do NOT change the LevitateOS builder
- `distro-spec/src/levitate/` — do NOT change LevitateOS specs
- `distro-spec/src/shared/` — only if BOTH distros need it, and carefully
- `testing/install-tests/src/steps/` — do NOT change test expectations to make them pass
- Anything that would break LevitateOS

### The rule: if removing AcornOS would leave LevitateOS broken, you crossed a boundary.

## How to Test

```bash
# Build AcornOS ISO
cd AcornOS && cargo run -- build

# Boot in QEMU (serial)
cd AcornOS && cargo run -- run

# Run install-tests against AcornOS
cd testing/install-tests && cargo run --bin install-tests -- run --distro acorn

# Run rootfs-tests
cd testing/rootfs-tests && cargo test -- --ignored --test-threads=1
```

If install-tests doesn't support `--distro acorn` yet, your first task is to make it work
using the `distro/acorn.rs` module that already exists in install-tests.

## Shared Infrastructure (USE IT — DO NOT REIMPLEMENT)

These crates already exist and work. AcornOS should call them, not duplicate them.

### distro-spec (`distro-spec/src/acorn/`)
Single source of truth for AcornOS constants. Already defines:
- `packages.rs` — tier 0-3 package lists (what to install)
- `services.rs` — OpenRC services and runlevels
- `boot.rs` — kernel modules for initramfs
- `uki.rs` — UKI boot entries and kernel cmdline
- `paths.rs` — OS identity, partition labels, mount points
- `keys/` — Alpine signing keys

**Always pull constants from here. Never hardcode paths, package names, or service lists.**

### distro-builder (`distro-builder/`)
Shared build abstractions:
- `Installable` trait + `Op` enum — component installation system
- `DistroConfig` trait — distro identification
- `artifact::erofs` — EROFS rootfs builder
- `artifact::cpio` — CPIO/initramfs builder
- `executor/` — directory, file, and user operations
- `build/context.rs` — build context trait

**Look at how leviso uses distro-builder. Mirror that pattern in AcornOS.**

### tools/ (Shared CLI tools — already work)

| Tool | What it does | How AcornOS uses it |
|------|-------------|---------------------|
| **recstrap** | Extracts rootfs to disk (like pacstrap) | `recstrap` installs AcornOS to target partition |
| **recinit** | Builds initramfs (busybox for live, systemd for installed) | Called during ISO build to create initramfs |
| **reciso** | Creates bootable UEFI ISO via xorriso | Final step: kernel + initramfs + rootfs → ISO |
| **recipe** | Rhai-based declarative package orchestrator | Resolves Alpine APK dependencies, extracts packages |
| **recqemu** | QEMU command builder | Powers `cargo run -- run` for testing |
| **recfstab** | Generates /etc/fstab from mounts | Post-install: writes fstab for installed system |
| **recchroot** | Enters chroot with bind mounts | Post-install: configures system inside chroot |
| **recuki** | Builds Unified Kernel Images | Creates UKI for systemd-boot |

**These are LIBRARIES you depend on via Cargo.toml, not scripts you call.** Check leviso's Cargo.toml to see how it imports them. Your AcornOS Cargo.toml should have similar dependencies.

### leviso-elf
ELF binary analysis — finds and copies shared library dependencies. Used by recinit to bundle libraries into initramfs. You don't call this directly; recinit uses it internally.

## Timeout Awareness

Be thoughtful about commands that might hang or take unexpectedly long:

- If a command produces no output for 2+ minutes, it's probably stuck. Kill it, note in progress, move on.
- Prefer `cargo check` over `cargo build` when you only need to verify compilation.
- When running QEMU for install-tests, use timeouts on expect-style waits — don't wait forever for a prompt that may never come.
- If a download stalls, kill and retry once. If it fails again, mark the task BLOCKED.
- Always commit your work BEFORE starting long-running operations (builds, QEMU boots). That way if you get killed, the work is saved.

**Not every task needs a full build.** Match the verification to the task:
- Changed a `.rs` file? `cargo check` is enough.
- Changed boot config? You need `cargo run -- build` + QEMU.
- Changed install logic? You need install-tests.

## Commit Rules

- Commit after EVERY meaningful change (one feature, one fix, one test)
- Commit message format: `feat(acorn): description` or `fix(acorn): description`
- Commit inside the relevant submodule first, then update parent pointer
- Run `cargo check` before committing — never commit broken code

## What "Desktop Ready" Means

AcornOS boots, installs, and provides a base system where a user can:
1. Boot from ISO in UEFI mode
2. Run installer (recstrap) to install to disk
3. Reboot into installed system
4. Login with user account
5. Have working networking (wired)
6. Install additional packages via `apk`
7. Install a Wayland compositor (sway, etc.) post-install

You do NOT need to include a desktop environment. You need a functional base.

## Progress Tracking

After each iteration:
1. Update `.ralph/acorn-progress.txt` with what you did
2. Update `.ralph/acorn-learnings.txt` with patterns/gotchas discovered
3. If ALL PRD items are complete and tests pass, output: <promise>COMPLETE</promise>
