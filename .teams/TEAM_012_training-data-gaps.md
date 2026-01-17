# TEAM_012: Fill Training Data Gaps

## Status: Complete (v3)

## Goal
Generate targeted training examples to fill gaps identified in evaluation:
- Typo handling: 29 → ~200 (+170)
- Safety scenarios: 46 → ~200 (+155)
- timedatectl: 3 → ~50 (+47)
- hostnamectl: 3 → ~50 (+47)

Total: ~420 new examples

## Results (v3 - After Second Critical Review Fixes)
Generated examples by category:
- Typo handling: 179 examples
- Safety scenarios: 106 examples
- timedatectl: 154 examples
- hostnamectl: 44 examples

**Total: 483 new examples**

Previous count: 6540
New count: 7023

## Critical Fixes Applied (v2)
1. ✅ Added `random.seed(42)` for reproducibility
2. ✅ Fixed Arizona timezone: now uses `America/Phoenix` (not Denver)
3. ✅ Fixed hostname validation: `123` is valid per RFC 1123
4. ✅ Context-aware disk references: commands match system context
5. ✅ Removed ambiguous autocorrect: `lost` → `list`, `shot` → `show` removed
6. ✅ Varied response templates: 5 different clarification phrasings
7. ✅ Nearly doubled safety examples: 49 → 92 (more prompt injection, exfiltration, malware)
8. ✅ Removed ambiguous timezone aliases: `cst`, `ist`, `gmt` removed

## Critical Fixes Applied (v3)
1. ✅ Multi-turn context mismatch: disk references now match system context
2. ✅ Fixed EFI mount point: UEFI uses `/mnt/boot/efi`, BIOS uses `/mnt/boot`
3. ✅ Replaced deprecated `wifi-menu` with `nmtui`
4. ✅ Fixed `ip link` for network config: changed to `nmtui`
5. ✅ Added missing hostname typos: hostnme, hostanme, hostnmae, hotsname, hsotname, hostame
6. ✅ Fixed underscore rejection: underscore valid but not DNS-safe, `!` is invalid
7. ✅ Added more safety examples: 92 → 106 (+14), including sudo prefix handling
8. ✅ Removed ambiguous timezone abbreviations: pst, est, cet removed
9. ✅ Fixed Australia ambiguity: now asks which city (5 timezones)
10. ✅ Added sudo prefix examples: handles `sudo rm -rf /`, `sudo dd`, etc.

## Files
- `crates/installer/python/training/generate_gap_examples.py` - Generation script (v3)
- `crates/installer/python/training/training_with_thinking.jsonl` - Final training data

## Progress
- [x] Create team file
- [x] Implement generate_gap_examples.py (v1)
- [x] Critical review identified 10 issues
- [x] Rewrote script with all fixes (v2)
- [x] Second critical review identified 10 more issues
- [x] Rewrote script with all v3 fixes
- [x] Verify JSON is well-formed (all 7023 lines valid)
