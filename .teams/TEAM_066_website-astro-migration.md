# TEAM_066: Website Migration from TanStack Start to Astro

## Status: Plan Complete - Awaiting Approval

## Objective
**FULL REWRITE** - Delete current website, create pure Astro site from scratch.

- NO React
- NO shadcn/ui
- ZERO style regression

## Why Astro?
- Static content ships zero JS
- Built-in Shiki at build time
- Simpler than TanStack Start for docs sites
- No framework runtime overhead

## Constraints
- `docs-content` package is **shared** - cannot be changed
- Must preserve exact visual design (Tailwind classes copied verbatim)

## What Gets Deleted
- Entire `website/` directory (backed up first)
- All React dependencies
- All 74 shadcn/ui components
- TanStack Router/Start

## What Gets Created
- Pure Astro site
- All `.astro` components (copy Tailwind classes exactly)
- Vanilla JS for theme toggle, copy button
- Build-time Shiki highlighting

## Migration Plan
See: `/home/vince/.claude/plans/distributed-splashing-quill.md`

10 phases: Backup → Setup → Copy Assets → Config → BaseLayout → Vanilla JS → Layouts → Block Renderers → Pages → Visual Verification → Cleanup

## Critical Success Criteria
- Side-by-side screenshot comparison passes
- All pages look identical
- Theme toggle works
- Copy buttons work
- Code highlighting matches

## Key Technical Decisions (from audit)

1. **Tailwind**: Use `@astrojs/tailwind` integration, NOT `@tailwindcss/vite`
2. **Workspace**: Add `vite.ssr.noExternal` for docs-content resolution
3. **View Transitions**: Enabled for smooth navigation (theme persists via `astro:after-swap`)
4. **Shiki**: Built-in with `defaultColor: false` for CSS variable support
5. **Scripts**: Use `is:inline` only for theme init (must run before render)

## Potential Issues Identified

| Issue | Mitigation |
|-------|------------|
| workspace:* resolution | Add vite.ssr.noExternal config |
| Theme flash on navigation | astro:after-swap event handler |
| Bun + image optimization | Test build; fallback to Node if needed |
| Shiki CSS variables | Verify existing CSS matches Astro's output |

## Problems Encountered
- (none yet)
