# TEAM_063: docs-tui Component Parity with Website

## Status: COMPLETED

## Summary

Achieved rendering parity between website's React docs components and docs-tui's Ink components. Both can now render all docs-content block types identically.

## Completed Tasks

### Phase 1: Make docs-content Pure Data
- Created `docs-content/scripts/build.ts` - generates static exports from content files
- Generated `docs-content/src/generated/index.ts` with docsNav and contentBySlug
- Updated `docs-content/src/index.ts` to export from `./generated` instead of `./discovery`
- Added `build` script to package.json
- Added `src/generated/` to .gitignore

**Key insight:** The `import.meta.glob` in discovery.ts only works in Vite. The build script pre-generates the same data structure so both website (Vite) and TUI (Bun) can consume it.

### Phase 2: Fix ListItem Children
- Updated `docs-tui/src/components/DocsPanel.tsx` to render nested children
- Children render with indentation and dimmed bullet points

### Phase 3: Fix ConversationMessage.list
- Updated conversation block rendering to show message.list items
- Lists render after message text with indentation

### Phase 4: Improve Table Column Width
- Changed from fixed 20-char width to dynamic calculation
- Columns now sized based on terminal width minus sidebar

### Additional Improvements
- Table cells now support RichText rendering (was only strings before)

## Files Modified

| File | Change |
|------|--------|
| `docs-content/scripts/build.ts` | NEW: Build script to generate pure data exports |
| `docs-content/src/generated/index.ts` | GENERATED: docsNav and contentBySlug |
| `docs-content/src/index.ts` | Changed export source to ./generated |
| `docs-content/package.json` | Added build script |
| `docs-content/.gitignore` | Added src/generated/ |
| `docs-tui/src/components/DocsPanel.tsx` | Fixed all rendering gaps |

## Verification

```bash
# Build docs-content
cd docs-content && bun run build
# Output: Found 18 content files, Generated src/generated/index.ts

# Test TUI
cd docs-tui && bun run dev
# TUI launches successfully, renders all content

# Website still works (typecheck errors are pre-existing)
cd website && bun run dev
```

## Usage

When adding/modifying content in `docs-content/src/content/`:

1. Make changes to content files
2. Run `cd docs-content && bun run build`
3. Both website and TUI will use updated content
