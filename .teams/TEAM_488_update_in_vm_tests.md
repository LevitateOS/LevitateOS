# TEAM 488: Update In-VM Test Suite for Fedora Binaries

## Status: Complete

## Goal
Update the levitate-test binary to test actual user-facing functionality with Fedora binaries instead of old vendor components.

## Changes Needed
1. Remove references to old binaries (hx, brush, vendor sudo)
2. Update smoke tests for Fedora binaries
3. Add user-focused test groups:
   - Shell: bash works, can run scripts
   - Auth: can read passwd/shadow, getent works
   - Process: ps, top, kill work
   - Network: ip, ping, ss work
   - Text: grep, sed, awk work
   - Compression: tar, gzip, xz work
   - Editors: nano, vi work

## Test Philosophy
- Test that things WORK, not just exist
- Focus on user workflows
- Each test should verify actual functionality
