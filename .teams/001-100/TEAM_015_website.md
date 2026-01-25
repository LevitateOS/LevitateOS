# TEAM_015: LevitateOS Website

## Goal
Create a documentation website for LevitateOS showcasing its unique features, installation process, and package manager.

## Stack
- **Framework**: Vite + React 18 + TanStack Router
- **UI**: shadcn/ui with Tailwind CSS
- **Hosting**: Cloudflare Pages (free tier)
- **ISOs**: Cloudflare R2 (when ready)

## Key USPs to Highlight
1. AI-powered installer (SmolLM3-3B, first of its kind)
2. S-expression package recipes (LLM-friendly)
3. Self-sufficient package manager (`levitate`)
4. Pure Wayland desktop (no X11)
5. musl + GNU stack (unusual combo)

## Progress
- [x] Create project with shadcn/ui preset
- [x] Set up TanStack Router
- [x] Build homepage with hero and feature cards
- [x] Create docs layout with sidebar
- [x] Write installation documentation
- [x] Write recipe format documentation
- [x] Add levitate CLI reference
- [x] Set up git submodule

## Decisions Made
- Using shadcn/ui preset with Lyra style, Stone base, Cyan accent
- JetBrains Mono font for code-focused aesthetic
- File-based routing with TanStack Router

## Notes
- Website lives in `website/` directory
- MDX for documentation content
- Shiki for syntax highlighting
