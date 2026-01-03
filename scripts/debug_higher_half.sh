#!/bin/bash
# TEAM_025: Debug script for higher-half kernel investigation
# This script runs QEMU with extended debugging for MMU/translation issues

set -e

PROJECT_ROOT="$(dirname "$0")/.."
cd "$PROJECT_ROOT"

KERNEL_BIN="kernel64_rust.bin"
DEBUG_LOG="qemu_higher_half_debug.log"

echo "=== Higher-Half Kernel Debug Script ==="
echo "TEAM_025: Use this to investigate the execute permission issue"
echo ""

# Build kernel
echo "Building kernel..."
cargo build --release --quiet
aarch64-linux-gnu-objcopy -O binary \
    target/aarch64-unknown-none/release/levitate-kernel \
    "$KERNEL_BIN"

echo "Kernel built: $KERNEL_BIN"
echo ""

# Show key symbols
echo "=== Key Symbols ==="
nm target/aarch64-unknown-none/release/levitate-kernel | grep -E "(kmain|__boot|__kernel|__page)" || true
echo ""

# Show program headers
echo "=== Program Headers (VMA/LMA) ==="
aarch64-linux-gnu-readelf -l target/aarch64-unknown-none/release/levitate-kernel | head -25
echo ""

# Run QEMU with debug logging
echo "=== Running QEMU with Debug Logging ==="
echo "Log file: $DEBUG_LOG"
echo "Debug flags: int,mmu,guest_errors"
echo ""

timeout 5s qemu-system-aarch64 \
    -M virt \
    -cpu cortex-a53 \
    -m 512M \
    -kernel "$KERNEL_BIN" \
    -display none \
    -serial stdio \
    -d int,mmu,guest_errors \
    -D "$DEBUG_LOG" \
    -no-reboot 2>&1 || true

echo ""
echo "=== QEMU Debug Log (last 50 lines) ==="
tail -50 "$DEBUG_LOG" 2>/dev/null || echo "No debug log generated"

echo ""
echo "=== Analysis Tips ==="
echo "1. Look for 'Translation fault' or 'Permission fault' in the log"
echo "2. Check the 'Faulting address' (FAR) value"
echo "3. Verify page table walks are finding valid entries"
echo "4. Compare ESR values against ARMv8 ARM"
echo ""
echo "For GDB debugging:"
echo "  qemu-system-aarch64 -s -S ... (add -s -S flags)"
echo "  gdb-multiarch target/aarch64-unknown-none/release/levitate-kernel"
echo "  (gdb) target remote :1234"
echo ""
