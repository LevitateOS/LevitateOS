#!/bin/bash
set -e

# Clean previous build
sudo rm -rf build/

# Build the ISO
sudo livemedia-creator \
    --ks kickstarts/levitate-live.ks \
    --no-virt \
    --resultdir build \
    --project "LevitateOS" \
    --releasever 43 \
    --make-iso

# Move ISO to root (needs sudo because livemedia-creator runs as root)
sudo mv build/images/boot.iso LevitateOS-1.0-x86_64.iso
sudo chown $USER:$USER LevitateOS-1.0-x86_64.iso

echo "Done: LevitateOS-1.0-x86_64.iso"
