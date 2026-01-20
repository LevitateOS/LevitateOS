# TEAM_071: Fix Website Design Regressions

## Status: Complete

## Problem
Three design regressions in the Astro website migration:
1. Theme toggle doesn't work - icons don't update, theme doesn't persist properly
2. Light mode colors bleeding in dark mode for code blocks and inline code
3. Custom scrollbars missing (native browser scrollbars showing)

## Root Cause Analysis
1. **Theme toggle**: Used CSS `dark:block` / `dark:hidden` classes that didn't work reliably with Tailwind 4's `@custom-variant`
2. **Code block colors**: The `bg-muted` Tailwind class uses `var(--muted)` which should cascade from `.dark`, but wasn't being applied correctly. Also, Astro's `<Code>` component uses class `astro-code` not `shiki`, so Shiki styling rules weren't matching.
3. **Scrollbars**: Backup used React ScrollArea component, current version uses plain overflow

## Solution
1. **ThemeToggle.astro**: Use JavaScript to toggle icon visibility instead of relying on CSS dark: classes. Added `astro:page-load` event for View Transitions support.
2. **global.css**:
   - Updated Shiki selectors to include `.astro-code` class
   - Added explicit dark mode override: `.dark .bg-muted { background-color: oklch(0.3 0.02 300) !important; }`
   - Added custom scrollbar CSS using theme variables
3. **Components**: Updated CommandBlock, CodeBlock, InlineContent, InteractiveBlock to use `bg-[var(--muted)]` syntax

## Files Modified
- `website/src/components/ThemeToggle.astro` - JS-based icon toggling
- `website/src/styles/global.css` - scrollbars, Shiki selectors, explicit dark mode override
- `website/src/components/docs/CommandBlock.astro` - bg-[var(--muted)]
- `website/src/components/docs/InlineContent.astro` - bg-[var(--muted)]
- `website/src/components/docs/InteractiveBlock.astro` - bg-[var(--muted)]
- `website/src/components/CodeBlock.astro` - bg-[var(--muted)]

## Verification
```bash
cd website && bun run dev
```
- Test theme toggle: click button, icons should switch, theme should persist on refresh
- Test dark mode: code blocks should have dark gray backgrounds, not light gray
- Test scrollbars: should be styled (thin, themed) not native browser scrollbars
