# TEAM_329: Screenshot Black Screen Detector Feature

**Date:** 2026-01-09  
**Status:** ✅ Completed  
**Feature:** Auto-detect black/empty screenshots in xtask

---

## Feature Summary

Add automatic detection of black/empty screenshots in the xtask screenshot tests so teams don't have to manually inspect images to determine if a display is working.

## Implementation

### Files Modified
- `xtask/Cargo.toml` - Added `image = "0.24"` dependency
- `xtask/src/tests/screenshot.rs` - Added `analyze_screenshot()` and `ScreenshotContent` enum

### Algorithm
Counts bright pixels (luminance > 128) instead of average brightness.
- **Black screen**: < 0.1% of pixels are bright
- **Has content**: >= 0.1% of pixels are bright

This correctly handles white text on black background terminals.

### Output Examples
```
[aarch64] ✅ Screenshot captured (brightness: 2.1 - display working)
[x86_64] ⚠️  BLACK SCREEN DETECTED (brightness: 0.0)
```

## Planning Location

`docs/planning/screenshot-black-detector/`

## Handoff Checklist
- [x] Project builds
- [x] Feature implemented and tested
- [x] Detection correctly identifies working vs black displays
- [x] Code commented with TEAM_329
