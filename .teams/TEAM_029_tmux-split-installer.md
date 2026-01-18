# TEAM_029: tmux Split Installer

## Objective
Replace the TypeScript shell wrapper with native tmux split:
- Left pane: Real bash shell (native)
- Right pane: Docs-only Ink TUI

## Why This Approach
- Native bash in tmux = full functionality (no PTY limitations)
- Docs viewer stays as TypeScript/Ink (good at rendering formatted content)
- Shift+Tab switches panes (intuitive)

## Changes Made

### 1. Simplified `installer/src/index.tsx`
- Removed Shell component import and usage
- Made DocsPanel full-width standalone app
- Removed focus management (not needed - tmux handles panes)

### 2. Deleted `installer/src/components/Shell.tsx`
- No longer needed - bash runs natively in tmux

### 3. Created `installer/bin/levitate-installer`
- Shell script that launches tmux with split panes
- Configures Shift+Tab (BTab) to switch panes
- Left: bash shell, Right: docs viewer

### 4. Updated `installer/package.json`
- Added `bin` field for the launcher script

### 5. Updated `kickstarts/levitate-live.ks`
- Added tmux and nodejs to packages
- Updated symlink path to new launcher location

## Keybindings
| Key | Action |
|-----|--------|
| Shift+Tab | Switch between shell and docs |
| Ctrl+B z | Zoom current pane |
| exit / Ctrl+D | Exit shell |
| Ctrl+C | Exit docs viewer |

## Testing
```bash
cd installer
npm run build
./bin/levitate-installer
```

## Status: COMPLETE

All changes implemented:
- [x] Simplified index.tsx to docs-only viewer
- [x] Removed Shell.tsx
- [x] Created tmux launcher script
- [x] Updated package.json (bun instead of node)
- [x] Updated kickstart (added tmux, unzip, bun install, new symlink, updated banner)
- [x] Build verified

## Dev VM (added later)

Created `installer/xtask/` for VM-based development testing:

```bash
cd installer
cargo xtask vm setup    # Download Fedora cloud image (~500MB, once)
cargo xtask vm start    # Start VM with virtfs sharing
cargo xtask vm ssh      # SSH into VM
cargo xtask vm stop     # Stop VM
```

Features:
- Fedora cloud image with bun + tmux pre-installed (via cloud-init)
- virtfs shares entire project at `/mnt/share`
- SSH on port 2223
- `--detach` flag for background mode
- Convenience aliases: `build`, `installer`
