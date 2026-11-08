# ginux-boot: AcornOS and LevitateOS booting under Ginux

Boots an OS's root filesystem with its real init as PID 1 under
[Ginux](https://github.com/gmkurtzer/ginux), the way Ginux's Gate 9 boots
Rocky's systemd, and judges its boot health. No VM and no kernel of the OS's
own: Ginux supervises every system call.

A run passes when the instance powers itself off, recab's boot-health verdict
says startup finished, and the services that failed are exactly the known gaps
in `acorn/expect` or `levitate/expect`. Both OSes are judged by that one rule.

## Running it

```bash
testing/ginux-boot/ginux-boot.sh acorn --rootfs ROOTFS --ginux GINUX_CHECKOUT
testing/ginux-boot/ginux-boot.sh levitate --rootfs ROOTFS --ginux GINUX_CHECKOUT
```

- `--rootfs`: the product's `rootfs-source`, from a live-tools release
  (`.artifacts/out/<distro>/releases/live-tools/<run>/rootfs-source`), which
  carries recab. It is copied and never changed.
- `--ginux`: a Ginux checkout that contains the revision `ginux-boot.sh` names
  (`ginux_rev`). The script builds it.
- `--timeout`: seconds the boot may take (recab's `--timeout`); the instance
  gets a fixed allowance more to power off.
- `--keep` keeps the work directory after a pass (it is always kept after a
  failure): the staged root, `console.log`, `audit.log` (Ginux's stderr, every
  system call decision) and `health.json`.
- Needs `sudo` (Ginux and staging run as root, as Ginux's own tests do) and
  the tools the script checks for at start.

No run of this version is recorded yet.

## How it works

1. Copy the rootfs and stage it (`stage.sh`, the same for both OSes): Ginux's
   `/proc`, `/sys` and `/dev` fakes, the probe, and `<os>/hook.sh`, which has
   the OS's own init start the probe (a `::once` inittab line, or a systemd
   unit after `multi-user.target`).
2. Boot with `ginux native instance.toml ROOT` under `script(1)`, which gives
   the console a terminal, within a timeout.
3. The probe (`probe.sh`) writes `recab health --json` to the instance root
   and powers off through the init, which ends `ginux native`.
4. The driver judges that verdict against `<os>/expect`.
