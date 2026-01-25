# TEAM_027: Bun + Turborepo Monorepo with Shared Docs Content

## Status: IN PROGRESS

## Goal
Add Bun + Turborepo to LevitateOS while **preserving existing folder structure**:
- `website/` stays at root (TanStack Start app)
- `installer/` stays at root (convert from Rust to TypeScript/Ink)
- New `docs-content/` at root for shared documentation
- Rust `recipe/` stays separate (not part of JS workspace)

## Key Constraints
- **Using Bun** as package manager (NOT pnpm)
- **NOT using standard turborepo folder structure** - NO apps/ or packages/ folders
- **NO moving directories** - website/ and installer/ stay at root
- **NO deleting installer/** - clean out Rust code, keep git repo
- **Preserve installer/.env** - contains API keys
- **Preserve installer/python/** - valuable training data

## Why This Change
- Native content sharing between website and TUI (no JSON conversion)
- Ink uses React - familiar component model
- Single language for both UIs
- Can share UI logic/components

## Implementation Phases

### Phase 1: Setup Turborepo (root level)
- [ ] Create root `package.json` with `workspaces` array
- [ ] Create `turbo.json` with build/dev/typecheck pipelines

### Phase 2: Create docs-content package
- [ ] Create `docs-content/` at root
- [ ] Create `package.json` with name `@levitate/docs-content`
- [ ] Copy types from website
- [ ] Extract content from route files
- [ ] Create manifest/index

### Phase 3: Update website
- [ ] Add `@levitate/docs-content` as workspace dependency
- [ ] Update route files to import from package
- [ ] Update DocsSidebar to use manifest

### Phase 4: Create Ink installer
- [ ] Remove Rust files from `installer/` (keep .git, .env, python/)
- [ ] Create TypeScript/Ink structure
- [ ] Create DocsViewer component
- [ ] Wire up navigation

## Decisions Made
- Using Bun workspaces (not pnpm)
- Package name: `@levitate/docs-content`
- Ink for TUI (React-based terminal UI)
- Flat structure at root (no apps/ or packages/ folders)

## Problems Encountered
(none yet)

## Notes
- Website stays at `website/` (not moved)
- Installer stays at `installer/` (Rust cleaned, TypeScript added)
- Rust `recipe/` crate stays separate (not part of JS monorepo)
