# Checkpoints

Status matrix for `testing/install-tests` (`cargo run --bin checkpoints -- ...`).

## Legend

- `OK`: verified for that exact output target
- `X`: not verified yet
- `-`: not applicable

## Checkpoints

- `CP0`: Build (kernel + bootable install artifact)
- `CP1`: Live Boot
- `CP2`: Live Tools
- `CP3`: Installation
- `CP4`: Installed Boot
- `CP5`: Automated Login
- `CP6`: Daily Driver Tools
- `CP7`: Slot B Trial Boot (A/B)
- `CP8`: Release Images

## CP8 Artifact Sets

- `LevitateOS`: public `ISO` + `qcow2` + raw `.img`
- `AcornOS`: public `ISO` + `qcow2` + raw `.img`
- `RalphOS`: public `qcow2` only (no public ISO)
- `IuppiterOS`: private raw `.img` only (no public ISO)

## Distro Behavior (Authoritative)

| Area | LevitateOS | RalphOS | AcornOS | IuppiterOS |
|---|---|---|---|---|
| Visibility | Public | Public | Public | Private/internal |
| Purpose | Stable Daily | Snappy Daily | Agent Sandbox | Specialized |
| Base stack | Rocky Linux / RPM rootfs | Rocky Linux / RPM rootfs | Alpine Linux / APK rootfs | Alpine Linux / APK rootfs |
| Toolchain | Glibc/systemd/GNU | Glibc/systemd/GNU | musl/OpenRC/busybox | musl/OpenRC/busybox |
| Package Manager | Recipe | Recipe | Recipe | Recipe |
| ISO live env | Full public live env | Minimal internal provisioning/diagnostics live env | Full public live env | Minimal internal provisioning/diagnostics live env |
| CP8 release target | Public `ISO` + `qcow2` + `.img` | Public `qcow2` (no public ISO) | Public `ISO` + `qcow2` + `.img` | Private `.img` (no public ISO) |



## Progress Table

| Checkpoint | Lev x86_64 A/B | Lev x86_64 mutable | Lev aarch64 A/B | Lev aarch64 mutable | Ralph x86_64 A/B | Ralph aarch64 A/B | Acorn x86_64 A/B | Acorn x86_64 mutable | Acorn aarch64 A/B | Acorn aarch64 mutable | Iuppiter x86_64 A/B |
|---|---|---|---|---|---|---|---|---|---|---|---|
| CP0 | X | OK | X | X | OK | X | X | X | X | X | X |
| CP1 | X | OK | X | X | X | X | X | OK | X | X | X |
| CP2 | X | OK | X | X | X | X | X | OK | X | X | X |
| CP3 | X | X | X | X | X | X | X | X | X | X | X |
| CP4 | X | X | X | X | X | X | X | X | X | X | X |
| CP5 | X | X | X | X | X | X | X | X | X | X | X |
| CP6 | X | X | X | X | X | X | X | X | X | X | X |
| CP7 | X | - | X | - | X | X | X | - | X | - | X |
| CP8 | X | X | X | X | X | X | X | X | X | X | X |

## Notes

- Levitate/Acorn A/B columns are expected to remain `X` until A/B install flow is implemented.
- Ralph live install env is internal even though Ralph is public; CP8 release target is public `qcow2`.
- Iuppiter remains private/internal; CP8 release target is non-public `.img`.
