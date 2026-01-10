# TEAM_337: AArch64 GPU Display Not Active Bugfix

**Date:** 2026-01-09  
**Status:** ✅ FIX APPLIED - PENDING USER VERIFICATION  
**Type:** Bugfix

## Bug Report

- **Symptom:** QEMU shows "Display output is not active" on AArch64
- **Platform:** AArch64 (QEMU virt machine)
- **Reproduction:** Run `cargo xtask run --arch aarch64`

## User Context

> "for aarch I remember that it went from gpu to pci to machine I guess"

This suggests the user recalls a different initialization path for AArch64 GPU that involves PCI and machine-level configuration.

## Planning Location

`docs/planning/aarch64-gpu-display-fix/`

## Current Status

- TEAM_336 attempted to add MMIO GPU support by making los_gpu generic over transport
- Build succeeds for both architectures
- GPU still shows "Display output is not active" on AArch64
- Need deeper investigation into the actual GPU initialization flow
