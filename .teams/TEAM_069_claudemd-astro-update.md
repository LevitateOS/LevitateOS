# TEAM_069: Update CLAUDE.md Website Section for Astro Migration

## Task
Update the outdated website documentation in CLAUDE.md to reflect the completed migration from TanStack Start to Astro 5.7.14.

## Changes
- Replace TanStack Start references with Astro
- Update directory structure to show Astro layout (pages/, layouts/)
- Remove shadcn/ui references (no longer used)
- Add info about docs-content workspace package
- Update commands (add preview, clarify typecheck = astro check)

## Status
- [x] Create team file
- [x] Edit CLAUDE.md website section
- [x] Verify changes

## Completed
CLAUDE.md website section updated from TanStack Start to Astro 5.7.14. Verified `bun run dev` works.

## Design Fixes (added later)
After Astro migration, design regressions were identified and fixed:
- Added `tw-animate-css` package for proper animations
- Added `@fontsource-variable/jetbrains-mono` for font loading
- Updated Hero.astro with proper `animate-in` classes (matching backup)
- Cleaned up FeatureCard and card styling in index.astro
- Removed unused `prose` classes from DocsLayout
