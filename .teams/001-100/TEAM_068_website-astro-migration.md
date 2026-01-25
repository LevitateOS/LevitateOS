# TEAM_068: Website Migration - TanStack Start to Astro

## Status: COMPLETED

## Objective
Full rewrite of website from TanStack Start to pure Astro:
- NO React - everything is pure `.astro` components
- NO shadcn/ui - recreate styles with Tailwind directly
- ZERO style regression - must match current visual design exactly
- `docs-content` package is shared - cannot be changed

## Progress

### Phase 0: Backup ✅
- [x] Move `website/` to `website-backup/`

### Phase 1: Project Setup ✅
- [x] Create fresh Astro project with Bun
- [x] Install dependencies (NO React)
- [x] Configure workspace reference for docs-content

### Phase 2: Copy Assets ✅
- [x] Copy public/* from backup
- [x] Copy styles.css to src/styles/global.css

### Phase 3: Configuration ✅
- [x] Create astro.config.mjs
- [x] Configure Tailwind integration (@tailwindcss/vite)
- [x] Configure Shiki syntax highlighting (dual themes)

### Phase 4: Base Layout ✅
- [x] Create BaseLayout.astro with theme script

### Phase 5: Core Vanilla JS Components ✅
- [x] ThemeToggle.astro (with sun/moon icons)
- [x] Copy button integrated into CodeBlock

### Phase 6: Layout Components ✅
- [x] Header.astro
- [x] Footer.astro
- [x] DocsLayout.astro
- [x] DocsSidebar.astro

### Phase 7: Docs Block Renderers ✅
- [x] InlineContent.astro
- [x] TextBlock.astro
- [x] CodeBlock.astro (with copy functionality)
- [x] TableBlock.astro
- [x] ListBlock.astro
- [x] ConversationBlock.astro
- [x] InteractiveBlock.astro
- [x] CommandBlock.astro
- [x] QABlock.astro
- [x] DocsPage.astro

### Phase 8: Pages ✅
- [x] index.astro (homepage with Hero, FeatureCards)
- [x] download.astro
- [x] docs/[slug].astro (with getStaticPaths)

### Phase 9: Build Verification ✅
- [x] Build completes successfully
- [x] 20 pages generated
- [x] Rhai syntax highlighted as Rust

### Phase 10: Cleanup
- [ ] Visual comparison (user should verify)
- [ ] Delete website-backup after verification

## Key Decisions
- Using Bun as package manager
- Using @tailwindcss/vite (direct Vite plugin)
- Using Astro's built-in Shiki for syntax highlighting
- Using vanilla JS for interactivity (theme toggle, copy buttons)
- Mapping `rhai` language to `rust` for syntax highlighting

## Files Created
- `astro.config.mjs` - Astro configuration
- `src/layouts/BaseLayout.astro` - HTML shell
- `src/components/ThemeToggle.astro` - Theme toggle button
- `src/components/CodeBlock.astro` - Code with syntax highlighting
- `src/components/Hero.astro` - Homepage hero
- `src/components/FeatureCard.astro` - Feature cards
- `src/components/ChatMessage.astro` - Chat UI
- `src/components/layout/Header.astro` - Navigation header
- `src/components/layout/Footer.astro` - Page footer
- `src/components/layout/DocsLayout.astro` - Docs layout wrapper
- `src/components/layout/DocsSidebar.astro` - Navigation sidebar
- `src/components/docs/InlineContent.astro` - Rich text parser
- `src/components/docs/TextBlock.astro` - Text renderer
- `src/components/docs/CommandBlock.astro` - Command with copy
- `src/components/docs/TableBlock.astro` - Table renderer
- `src/components/docs/ListBlock.astro` - List renderer
- `src/components/docs/ConversationBlock.astro` - Chat UI
- `src/components/docs/InteractiveBlock.astro` - Interactive steps
- `src/components/docs/QABlock.astro` - Q&A accordion
- `src/components/docs/DocsPage.astro` - Main docs orchestrator
- `src/pages/index.astro` - Homepage
- `src/pages/download.astro` - Download page
- `src/pages/docs/[slug].astro` - Dynamic docs routes

## Dependencies (Final)
```json
{
  "dependencies": {
    "astro": "^5.16.11",
    "@astrojs/check": "^0.9.6",
    "@levitate/docs-content": "workspace:*"
  },
  "devDependencies": {
    "typescript": "^5.7.2",
    "tailwindcss": "^4.1.18",
    "@tailwindcss/vite": "^4.1.18"
  }
}
```

## Issues Encountered
- Shiki doesn't have `rhai` language - solved by mapping to `rust`
- `@astrojs/check@^0.10.0` doesn't exist - used `^0.9.4` instead
