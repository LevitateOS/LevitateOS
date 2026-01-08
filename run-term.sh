#!/bin/bash
# run-term.sh - Run LevitateOS in terminal-only mode (WSL-like)
#
# Use this when you want to interact via the terminal.
# NO graphical window - pure terminal experience.
# Type directly in THIS terminal - input goes to VM.
#
# Flags:
#   --aarch64  - Run on AArch64 instead of x86_64 (default)
#
# Ctrl+C sends SIGINT to VM (if supported)
# Ctrl+A X to exit QEMU

set -e

# Default to x86_64, use --aarch64 for AArch64
ARCH="x86_64"
for arg in "$@"; do
    case $arg in
        --aarch64) ARCH="aarch64" ;;
    esac
done

cargo xtask build all --arch "$ARCH"

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  LevitateOS Terminal Mode (WSL-like) - $ARCH               ║"
echo "║                                                            ║"
echo "║  Type directly here - keyboard goes to VM                  ║"
echo "║  Ctrl+A X to exit QEMU                                     ║"
echo "║  Ctrl+A C to switch to QEMU monitor                        ║"
echo "╚════════════════════════════════════════════════════════════╝"

rm -f ./qmp.sock

# -nographic: No graphical display, disables SDL/GTK window
# -serial mon:stdio: Serial console + monitor multiplexed on stdio
# This gives WSL-like behavior where keyboard goes directly to serial
if [ "$ARCH" = "aarch64" ]; then
    BIN="kernel64_rust.bin"
    qemu-system-aarch64 \
        -M virt \
        -cpu cortex-a72 \
        -m 1G \
        -kernel "$BIN" \
        -nographic \
        -device virtio-gpu-pci,xres=1280,yres=800 \
        -device virtio-keyboard-device \
        -device virtio-tablet-device \
        -device virtio-net-device,netdev=net0 \
        -netdev user,id=net0 \
        -drive file=tinyos_disk.img,format=raw,if=none,id=hd0 \
        -device virtio-blk-device,drive=hd0 \
        -initrd initramfs.cpio \
        -serial mon:stdio \
        -qmp unix:./qmp.sock,server,nowait \
        -no-reboot
else
    # x86_64 uses Limine ISO boot
    ISO="levitate.iso"
    if [ ! -f "$ISO" ]; then
        echo "Building Limine ISO..."
        cargo xtask build iso --arch x86_64
    fi
    qemu-system-x86_64 \
        -M q35 \
        -cpu qemu64 \
        -m 1G \
        -boot d \
        -cdrom "$ISO" \
        -nographic \
        -device virtio-gpu-pci,xres=1280,yres=800 \
        -device virtio-keyboard-pci \
        -device virtio-tablet-pci \
        -device virtio-net-pci,netdev=net0 \
        -netdev user,id=net0 \
        -drive file=tinyos_disk.img,format=raw,if=none,id=hd0 \
        -device virtio-blk-pci,drive=hd0 \
        -serial mon:stdio \
        -qmp unix:./qmp.sock,server,nowait \
        -no-reboot
fi
