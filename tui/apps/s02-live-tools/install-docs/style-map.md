# Docs TUI Style Map

Single-hop guide for style changes in `docs/tui`.

## First Hop

- Page chrome/layout: `src/presentation/ink/screens/install-viewer.tsx`
- Left navigation panel: `src/presentation/ink/components/sidebar.tsx`
- Bottom status/help line: `src/presentation/ink/components/status-bar.tsx`
- Theme colors/intents/layout constants: `src/presentation/ink/theme/index.ts`
- Document node dispatch (`intro`, `section`, `block`): `src/presentation/ink/document/renderer-registry.tsx`

## Block Style Owners

- `text`: `src/presentation/ink/blocks/text-block.tsx`
- `code`: `src/presentation/ink/blocks/code-block.tsx`
- `command`: `src/presentation/ink/blocks/command-block.tsx`
- `table`: `src/presentation/ink/blocks/table-block.tsx`
- `list`: `src/presentation/ink/blocks/list-block.tsx`
- `conversation`: `src/presentation/ink/blocks/conversation-block.tsx`
- `interactive`: `src/presentation/ink/blocks/interactive-block.tsx`
- `qa`: `src/presentation/ink/blocks/qa-block.tsx`
- `note`/warning/info cards: `src/presentation/ink/blocks/note-block.tsx`

Block plugin registration is in `src/presentation/ink/blocks/plugins.tsx`.

## Shared Style Primitives

Use these when a style must be consistent across multiple blocks.

- Command rows (prompt + full-row background): `src/presentation/ink/primitives/command-line-row.tsx`
- Multi-line command groups: `src/presentation/ink/primitives/command-line-series.tsx`
- Prefixed rich text rows (bullets/labels + wrapping): `src/presentation/ink/primitives/prefixed-rich-list-item.tsx`
- Table row rich text + truncation: `src/presentation/ink/primitives/rich-table-row.tsx`
- Rich text run utilities (pad/truncate/background): `src/presentation/ink/primitives/rich-text-runs.ts`
- Admonition frame chrome: `src/presentation/ink/primitives/admonition-frame.tsx`

## Rich Text Rendering

- Rich inline rendering and wrapping behavior: `src/presentation/ink/blocks/shared/rich-text-renderer.tsx`
- Syntax token rendering for code/command lines: `src/presentation/ink/blocks/shared/syntax-line.tsx`

## Rule of Thumb

- Change one block only: edit that block file.
- Change repeated visual behavior: edit the primitive and keep block files thin.
- Change colors globally: edit theme intents in `src/presentation/ink/theme/index.ts`.
