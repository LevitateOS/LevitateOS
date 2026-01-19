# TEAM_052: Syntax Highlighting for Code Blocks

## Goal
Add colored syntax highlighting for bash, rhai, json, yaml, toml in CodeBlock and FileBlock components.

## Approach
Using Shiki (industry standard, VS Code grammar engine) with github-dark theme.

## Files Modified
1. `website/package.json` - Added shiki dependency
2. `website/src/lib/highlighter.ts` (NEW) - Shiki singleton + highlight function
3. `website/src/components/CodeBlock.tsx` - Added highlighting with useEffect
4. `website/src/components/docs/DocsPage.tsx` - Added highlighting to FileBlockRenderer
5. `website/src/styles.css` - Added Shiki CSS overrides for transparent background

## Implementation Details

### Language Mapping
- `sh`, `zsh` → `bash`
- `yml` → `yaml`
- `rhai` → `rust` (Rhai syntax is Rust-like)

### Lazy Loading
Shiki highlighter is created lazily as a singleton, loaded only when first code block renders.

### Copy Button
Copy button still copies raw code (not HTML) - uses original `children`/`file.content` prop.

## Status
- [x] Install shiki via `bun add shiki`
- [x] Create highlighter singleton at `src/lib/highlighter.ts`
- [x] Update CodeBlock with useState/useEffect for async highlighting
- [x] Update FileBlockRenderer in DocsPage with same pattern
- [x] Add CSS overrides for transparent Shiki background
- [x] TypeScript compiles (pre-existing errors in unrelated shadcn components)

## Testing
To verify: run `bun run dev` and navigate to `/docs/cli-reference` or `/docs/recipe-format`
