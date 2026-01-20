# TEAM_070: Website Design Fixes

## Status: In Progress

## Task
Fix design regressions in the Astro website migration:
1. Light mode bleeding in dark mode
2. Custom scrollbars missing (regressed from TanStack version)

## Approach
1. Compare website-backup (old TanStack code) with current website (Astro)
2. Identify missing CSS styles
3. Port over the custom scrollbar styles and fix dark mode issues

## Files to Investigate
- `website-backup/` - Old TanStack source code
- `website/src/styles/global.css` - Current global styles
- Code blocks and sidebar components

## Decisions
- TBD

## Problems Encountered
- TBD
