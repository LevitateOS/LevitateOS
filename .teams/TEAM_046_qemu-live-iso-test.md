# TEAM_046: QEMU Live ISO Test

## Goal
Run the QEMU test in leviso to boot a live ISO showing a bash shell.

## Status
- [x] Add `test` subcommand to leviso
- [x] Fix missing mount/hostname in initramfs
- [x] Fix PATH in init script
- [x] Get bash shell working
- [x] Add `--gui` flag for QEMU GUI mode
- [x] Split main.rs into modules
- [x] Fix console output for GUI mode

## Changes Made
1. Added `Test` subcommand with `--gui` flag to leviso
2. Fixed init script to set PATH and use absolute paths
3. Added mount, umount, hostname to initramfs binaries
4. Split main.rs into: download.rs, extract.rs, initramfs.rs, iso.rs, qemu.rs, clean.rs
5. Fixed kernel console params: `console=ttyS0 console=tty0` (tty0 last for GUI)

## Commands
- `cargo run -- test` - serial console mode
- `cargo run -- test --gui` - QEMU GUI window

## Notes
Bash warnings about "cannot set terminal process group" and "no job control" are harmless when running as PID 1.
