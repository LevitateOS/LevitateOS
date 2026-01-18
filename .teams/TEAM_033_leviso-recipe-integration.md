# TEAM_033: Leviso Recipe Integration

## Status: IN PROGRESS

## Objective
Modify leviso to:
1. Use the `recipe` binary to download Rocky Linux 10 ISO if it doesn't exist
2. Build an archiso-style ISO using Rocky Linux 10 prebuilt binaries

## Current State
- leviso has archiso-style profile system
- recipe crate has Rhai-based package executor with download/extract/install helpers

## Plan
1. Explore current leviso architecture
2. Create recipe for downloading Rocky Linux 10 ISO
3. Integrate recipe execution into leviso build process
4. Extract packages from Rocky ISO and build custom ISO
