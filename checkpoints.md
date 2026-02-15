# Checkpoints

Human-maintained progress table for the checkpoint-based dev loop in
`testing/install-tests` (CLI: `cargo run --bin checkpoints -- ...`).

## Legend

- `OK`: verified working for this output target
- `HALF`: verified working but incomplete/compromised (see CP0)
- `X`: not verified yet (either not implemented or not tested)
- `-`: not applicable for this output target

## Checkpoint Definitions

- `CP0` (Build): custom Linux kernel + bootable ISO.
  - If the ISO builds but the kernel provenance is wrong (kernel release suffix does not match the distro), this is `HALF`.
- `CP1`: Live Boot (ISO boots in QEMU and reaches a known-good marker)
- `CP2`: Live Tools
  - LevitateOS/AcornOS: full live environment tooling is present and works (intended for interactive debugging/repair during live boot).
  - RalphOS/IuppiterOS: only the minimal live tooling needed for provisioning and diagnostics is present (these OSes primarily ship as installed-disk images at CP8, not as general-purpose live environments).
- `CP3`: Installation (scripted install to disk succeeds)
- `CP4`: Installed Boot (boots from disk after install)
- `CP5`: Automated Login (harness can login + run commands)
- `CP6`: Daily Driver Tools (expected tools present on installed system)
- `CP7`: Slot B Trial Boot (A/B systems)
  - Populate the inactive system slot (`B`) and successfully boot it at least once.
  - Run a minimal health check on the `B` boot (enough to justify committing the slot in a real update flow).
- `CP8`: Release Images (produce distributable installed-disk image(s))
  - RalphOS: `qcow2` + raw `.img`
  - IuppiterOS: raw `.img` (primary release target; intended to be `dd`'d to SSDs for servers)

## Mutable Mode (Open Question)

For public-facing distros (LevitateOS/AcornOS), we need to decide whether to support a mutable (in-place) system mode at all.

- Pros: convenience for power users; fewer reboots for iterative local changes.
- Cons: much larger blast radius for LLM-driven recipes; harder to keep systems reproducible and supportable; drift bugs.
- Current bias: A/B immutable is the default; mutable (if it exists) is an explicit opt-in for daredevils, and never applies to RalphOS/IuppiterOS.

## Progress Table

Columns represent distinct output targets (different ISOs and/or installed-disk artifact sets).
Only mark a cell `OK`/`HALF` after that exact output target has been verified (by the checkpoint harness or an equivalent reproducible run); otherwise keep it `X`.

| Checkpoint | LevitateOS |  |  |  | RalphOS |  | AcornOS |  |  |  | IuppiterOS |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Target | x86_64 |  | aarch64 |  | x86_64 | aarch64 | x86_64 |  | aarch64 |  | x86_64 |
| Mutability | A/B | mutable | A/B | mutable | A/B | A/B | A/B | mutable | A/B | mutable | A/B |
| CP0 (Build) | X | OK | X | X | X | X | X | HALF | X | X | X |
| CP1 (Live Boot) | X | OK | X | X | X | X | X | OK | X | X | X |
| CP2 (Live Tools) | X | OK | X | X | X | X | X | OK | X | X | X |
| CP3 (Installation) | X | X | X | X | X | X | X | X | X | X | X |
| CP4 (Installed Boot) | X | X | X | X | X | X | X | X | X | X | X |
| CP5 (Automated Login) | X | X | X | X | X | X | X | X | X | X | X |
| CP6 (Daily Driver Tools) | X | X | X | X | X | X | X | X | X | X | X |
| CP7 (Slot B Trial Boot) | X | - | X | - | X | X | X | - | X | - | X |
| CP8 (Release Images) | X | X | X | X | X | X | X | X | X | X | X |

Notes:
- x86_64 AcornOS (mutable): CP0 is `HALF` if the kernel release suffix does not match `-acorn`.
