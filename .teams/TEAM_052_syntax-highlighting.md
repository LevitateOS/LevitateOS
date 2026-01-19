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

## Additional Work (continued from context recovery)

### New Content Block Types
Created `CommandBlock` and `InteractiveBlock` to eliminate `# comments` in bash code:

1. **CommandBlock** - Description header + command + optional output
   - `command` field supports `string | string[]` for multi-line commands
   - Description shown as header with border separator
   - Optional output shown dimmer with dashed border
   - Copy button only copies commands (not description)

2. **InteractiveBlock** - Step-by-step interactive sequences (e.g., fdisk)
   - Shows command + description pairs in a list format

### Component Split
Split monolithic DocsPage.tsx into separate component files:
- `CommandBlock.tsx`
- `InteractiveBlock.tsx`
- `FileBlock.tsx`
- `TableBlock.tsx`
- `ListBlock.tsx`
- `ConversationBlock.tsx`
- `TextBlock.tsx`
- `InlineContent.tsx`
- `DocsPage.tsx` (orchestrator)

### Dual Theme Support
Updated highlighter to use both github-light and github-dark themes with CSS variables for automatic light/dark mode switching.

## Status
- [x] Install shiki via `bun add shiki`
- [x] Create highlighter singleton at `src/lib/highlighter.ts`
- [x] Update CodeBlock with useState/useEffect for async highlighting
- [x] Update FileBlockRenderer with same pattern
- [x] Add CSS overrides for transparent Shiki background
- [x] Add dual theme support (github-light/github-dark)
- [x] Split DocsPage.tsx into separate component files
- [x] Create CommandBlock and InteractiveBlock types and renderers
- [x] Add array support for CommandBlock.command field
- [x] Update installation-disk.ts to use array format

## Testing
To verify: run `bun run dev` and navigate to `/docs/installation-disk`
