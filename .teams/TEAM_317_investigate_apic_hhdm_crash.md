# TEAM_317: Investigate APIC HHDM Crash

## Bug Report

**Symptom:** Page fault when accessing APIC via Limine HHDM

**Crash Details:**
```
EXCEPTION: PAGE FAULT
Accessed Address: ffff8000fee000f0
Error Code: 0
```

**Analysis:**
- Address breakdown: `0xFFFF8000FEE000F0`
  - `0xFFFF800000000000` = PHYS_OFFSET (Limine HHDM base)
  - `0xFEE00000` = APIC base address
  - `0xF0` = APIC_REGISTER_SPURIOUS offset
- The crash occurs in `apic::APIC.init()` when trying to read the spurious register

## Root Cause

**Limine's HHDM only maps RAM, not MMIO regions.**

The HHDM (Higher Half Direct Map) provided by Limine maps physical RAM into the higher half, but MMIO regions like APIC (0xFEE00000) are NOT included because they're not RAM.

## Solution

Skip APIC/IOAPIC init during early boot - they require explicit MMIO mapping which we don't have set up yet. Use legacy PIC mode instead (which was working before).

## Files Modified

- `crates/hal/src/x86_64/mod.rs` - Skip APIC/IOAPIC init, keep PIT

## Status: FIXING
