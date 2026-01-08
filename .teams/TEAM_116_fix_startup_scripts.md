# Team 116 - Fix Startup Scripts for x86_64

## Task
Investigate and fix a bug in `run-term.sh` (and potentially other scripts) where x86_64 should be running the ISO with Limine but isn't.

## Status
- [x] Claimed team number 116
- [x] Analyze `run-term.sh`
- [x] Analyze `run.sh`, `run-gui.sh`, `run-vnc.sh`
- [x] Fix identified bugs in `run-term.sh` and `run-vnc.sh`
- [x] Verify changes (built ISO successfully)

## Findings
- `run-term.sh` was incorrectly trying to boot the x86_64 ELF directly with `-kernel`, which bypassed Limine and the required ISO structure.
- `run-vnc.sh` was similarly misconfigured for x86_64 and was performing unnecessary manual binary conversion.
- `run-gui.sh` already had the correct Limine ISO logic.
- `run.sh` is a wrapper and is correct.
- Successfully verified that `cargo xtask build iso --arch x86_64` works, which is now the standard for these scripts.

## Added CLI Tools (xtask)
- **`cargo xtask build initramfs`**: Build only the initramfs CPIO without rebuilding all of userspace (faster for content-only changes).
- **`cargo xtask image status`**: Show disk image size and list contents of the FAT32 partition using `mdir`.
- **`cargo xtask run gdb [--wait]`**: Run QEMU with GDB server enabled on port 1234. Use `--wait` to freeze at the first instruction.
- **`cargo xtask kill`**: Forcefully kill any lingering QEMU instances (integrated into `main.rs`).
