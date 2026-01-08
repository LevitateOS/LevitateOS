# External Debugging Tools for LevitateOS

This guide covers external tools that can be used to debug LevitateOS on both x86_64 and AArch64 architectures.

## 1. GDB (GNU Debugger)

GDB is the primary tool for kernel-level debugging.

### Setup
Run LevitateOS with the GDB server enabled:
```bash
cargo xtask run gdb --wait
```

### Connecting (x86_64)
```bash
gdb target/x86_64-unknown-none/release/levitate-kernel
(gdb) target remote :1234
(gdb) continue
```

### Connecting (AArch64)
You may need `gdb-multiarch` or `aarch64-linux-gnu-gdb`:
```bash
gdb-multiarch target/aarch64-unknown-none/release/levitate-kernel
(gdb) set arch aarch64
(gdb) target remote :1234
```

## 2. QEMU Monitor

The QEMU Monitor allows you to inspect the VM state (registers, memory, device status) while it's running.

### Access
In `run-term.sh` or `cargo xtask run term`, press `Ctrl+A C` to switch to the monitor. Press `Ctrl+A C` again to return to the serial console.

### Useful Commands
- `info registers`: Show CPU registers.
- `info mem`: Show active virtual memory mappings (x86 only).
- `info tlb`: Show TLB state.
- `xp /fmt addr`: Physical memory dump.
- `p /fmt addr`: Virtual memory dump.
- `system_reset`: Restart the VM.
- `quit` or `Ctrl+A X`: Exit QEMU.

## 3. NM & Objdump

These tools help analyze the compiled kernel binary.

### Listing Symbols
```bash
nm -C target/x86_64-unknown-none/release/levitate-kernel | c++filt | sort
```

### Disassembling
```bash
# x86_64
objdump -d -M intel target/x86_64-unknown-none/release/levitate-kernel | less

# AArch64
aarch64-linux-gnu-objdump -d target/aarch64-unknown-none/release/levitate-kernel | less
```

## 4. GDB Dashboards

For a better UI in the terminal, consider using:
- **[gdb-dashboard](https://github.com/cyrus-and/gdb-dashboard)**: Modular dashboard for GDB.
- **[GEF](https://github.com/hugsy/gef)**: GDB Enhanced Features for exploit dev & reverse engineering.

## 5. Serial Logging

LevitateOS uses the serial port for logging.
- `run-term.sh` shows this directly.
- `run-gui.sh` shows this in the terminal where it was launched.
- Use `--features verbose` when building to see more detailed logs.
