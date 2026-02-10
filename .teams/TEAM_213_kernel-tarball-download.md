# TEAM_213: Shared Kernel Tarball Download + Build

**Date:** 2026-02-10
**Status:** Complete (compiles, URLs verified, SHA256 verified, submodule removed)

## What was implemented

Replaced the shared `linux` git submodule approach with per-distro kernel tarball downloads from cdn.kernel.org, enabling different kernel versions per distro.

| Distro | Version | Channel |
|---|---|---|
| LevitateOS | 6.18.4 | LTS |
| AcornOS | 6.19 | Mainline |
| IuppiterOS | 6.18.4 | LTS |

## Key decisions

1. **KernelSource struct in distro-spec** — single source of truth for version, URL, SHA256, localversion per distro
2. **Tarball primary, submodule fallback** — all .rhai recipes and Rust code try tarball first, fall back to git submodule
3. **SHA256 placeholders** — first download will skip verification; user updates distro-spec with real hashes after
4. **Theft mode preserved with warnings** — DEV ONLY labels clearly state version/kconfig mismatches
5. **curl for download** — simpler than adding reqwest dependency, curl is already required on host

## Files modified

### New files
- `distro-spec/src/shared/kernel.rs` — KernelSource struct + ACORN_KERNEL, IUPPITER_KERNEL, LEVITATE_KERNEL constants

### Modified files
- `distro-spec/src/shared/mod.rs` — added kernel module + re-exports
- `distro-spec/src/acorn/mod.rs` — added KERNEL_SOURCE re-export
- `distro-spec/src/iuppiter/mod.rs` — added KERNEL_SOURCE re-export
- `distro-spec/src/levitate/mod.rs` — added KERNEL_SOURCE re-export
- `distro-builder/src/build/kernel.rs` — added download_kernel_tarball() + acquire_kernel_source()
- `AcornOS/deps/linux.rhai` — tarball download primary, submodule fallback, DEV ONLY warnings on theft
- `IuppiterOS/deps/linux.rhai` — tarball download primary, submodule fallback, DEV ONLY warnings on theft
- `leviso/deps/linux.rhai` — tarball download primary, submodule fallback (no theft - primary builder)
- `AcornOS/src/main.rs` — cmd_build_kernel/cmd_build_with_kernel/cmd_download_linux use acquire_kernel_source + theft warnings
- `IuppiterOS/src/main.rs` — same changes as AcornOS
- `AcornOS/src/rebuild.rs` — unchanged (already handles missing Makefile gracefully)

## Hardening done
- Real SHA256 hashes from kernel.org's PGP-signed sha256sums.asc
- Versions verified against kernel.org: 6.19 (mainline), 6.18.9 (stable)
- curl retry (--retry 3), resume (-C -), progress bar (--progress-bar)
- SHA256 always verified, no placeholder bypass
- `linux` git submodule fully removed (.gitmodules, git index, all Rust/Rhai references)
- URLs verified reachable via range request

## Known issues
- Full end-to-end test (download + build + boot) not run yet (would take ~1hr for kernel compile)
- No PGP signature verification (only SHA256 from the PGP-signed sums file)
