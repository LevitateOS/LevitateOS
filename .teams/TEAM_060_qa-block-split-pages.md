# TEAM_060: Q&A Block Type and Split Post-Installation/Troubleshooting Pages

## Task
1. Add new `qa` block type for flat-list Q&A sections
2. Split post-installation and troubleshooting from `06-installation-boot.ts` into separate pages
3. Create new troubleshooting page using Q&A format

## Files Modified/Created
- `docs-content/src/types.ts` - Added QABlock and QAItem types
- `website/src/components/docs/QABlock.tsx` - New renderer component
- `website/src/components/docs/index.ts` - Exported QABlock and QAItem types
- `website/src/components/docs/DocsPage.tsx` - Added case for qa blocks
- `docs-content/src/content/01-getting-started/07-post-installation.ts` - New page
- `docs-content/src/content/01-getting-started/08-troubleshooting.ts` - New page with Q&A blocks
- `docs-content/src/content/01-getting-started/06-installation-boot.ts` - Removed moved content, added links

## Progress
- [x] Add QABlock types
- [x] Create QABlock renderer
- [x] Update DocsPage switch
- [x] Update index exports
- [x] Create post-installation page
- [x] Create troubleshooting page
- [x] Update boot page
- [x] Run typecheck (docs-content passes; website has pre-existing shadcn/ui type issues)

## Implementation Details

### QABlock Type Structure
```typescript
export interface QABlock {
  type: "qa"
  items: QAItem[]
}

export interface QAItem {
  question: string | RichText
  answer: ContentBlock[]  // Can contain text, code, command, list blocks
}
```

### Renderer Design
- Flat list format with questions as bold headers
- Left border styling for visual hierarchy
- Answer content supports nested block types (text, code, command, list)

### New Pages
- `/docs/post-installation` - Package management and system updates
- `/docs/troubleshooting` - Q&A format for boot, network, and package issues

## Status: COMPLETE
