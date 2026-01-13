# Phase 0: TUI Dashboard Design - Initramfs Builder

## Overview

A **non-interactive** TUI dashboard integrated into xtask that displays real-time build progress, status, and statistics when building initramfs. The dashboard is purely informational - no user input required.

## Command

```bash
# Build initramfs with TUI dashboard (default when TTY available)
cargo xtask build initramfs

# Explicit flags
cargo xtask build initramfs --tui      # Force TUI
cargo xtask build initramfs --no-tui   # Force simple output (for CI/pipes)
```

## Dashboard Layout

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  LEVITATE INITRAMFS BUILDER                                       x86_64    │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Phase: Adding symlinks                                    [████████░░] 80%  │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  ACTIVITY                                                                    │
│  ──────────────────────────────────────────────────────────────────────────  │
│   ✓  + /bin/busybox                                              1.2 MB     │
│   ✓  + /init                                                     1.2 MB     │
│   ✓  → /bin/sh -> busybox                                                   │
│   ✓  → /bin/ash -> busybox                                                  │
│   ✓  → /bin/cat -> busybox                                                  │
│   ◉  → /bin/ls -> busybox                                                   │
│      → /bin/cp -> busybox                                                   │
│      → /bin/echo -> busybox                                                 │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  STATISTICS                                                                  │
│  ──────────────────────────────────────────────────────────────────────────  │
│  Directories    8          Symlinks    47/60         Total Size   2.4 MB    │
│  Binaries       2          Files        5/7          Elapsed      0.3s      │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

## Key Design Principles

### 1. Non-Interactive

- **NO user input** - the dashboard is display-only
- No keyboard handling (except Ctrl+C for abort)
- Auto-completes and exits when done
- No prompts, no confirmations

### 2. Automatic TTY Detection

```rust
fn should_use_tui() -> bool {
    // Use TUI only when:
    // 1. stdout is a TTY
    // 2. NO_TUI env var is not set
    // 3. Not running in CI (CI=true not set)
    atty::is(atty::Stream::Stdout)
        && std::env::var("NO_TUI").is_err()
        && std::env::var("CI").is_err()
}
```

### 3. Graceful Fallback

When TUI is disabled, fall back to simple line output:
```
📦 Creating initramfs for x86_64...
  + /bin/busybox (1.2 MB)
  + /init (1.2 MB)
  → 60 symlinks created
  + 7 files added
✅ Initramfs created: target/initramfs/x86_64.cpio (3.2 MB) in 0.4s
```

## Implementation Architecture

### New Files in xtask

```
xtask/src/build/initramfs/
├── mod.rs              # Public API, orchestration
├── cpio.rs             # Pure Rust CPIO writer
├── manifest.rs         # TOML parser + validation
├── builder.rs          # Archive construction with events
└── tui.rs              # Non-interactive dashboard
```

### Dependencies

```toml
# xtask/Cargo.toml
[dependencies]
ratatui = "0.29"
crossterm = "0.28"
atty = "0.2"
serde = { version = "1", features = ["derive"] }
toml = "0.8"
```

### Event-Driven Architecture

The builder emits events, the TUI consumes them:

```rust
// Events emitted by builder
pub enum BuildEvent {
    // Phase transitions
    PhaseStart { name: &'static str, total: usize },
    PhaseComplete { name: &'static str },

    // Individual items
    DirectoryCreated { path: String },
    BinaryAdded { path: String, size: u64 },
    SymlinkCreated { link: String, target: String },
    FileAdded { path: String, size: u64 },

    // Completion
    BuildComplete {
        output_path: PathBuf,
        total_size: u64,
        duration: Duration
    },
    BuildFailed { error: String },
}
```

### TUI State Machine

```rust
pub struct Dashboard {
    // Header
    arch: String,

    // Progress
    current_phase: String,
    phase_progress: (usize, usize),  // (done, total)

    // Activity log (scrolling window of last N items)
    activity: VecDeque<ActivityItem>,
    max_activity_lines: usize,

    // Statistics
    stats: BuildStats,
    start_time: Instant,
}

struct ActivityItem {
    icon: char,           // '+' for file, '→' for symlink, '📁' for dir
    text: String,
    size: Option<u64>,
    status: ItemStatus,   // Pending, InProgress, Done
}

struct BuildStats {
    directories: usize,
    binaries: (usize, u64),  // (count, total_bytes)
    symlinks: (usize, usize), // (done, total)
    files: (usize, usize),
}
```

### Rendering Loop

```rust
pub fn run_build_with_tui(arch: &str) -> Result<PathBuf> {
    if !should_use_tui() {
        return run_build_simple(arch);
    }

    // Setup terminal
    let mut terminal = ratatui::init();
    crossterm::terminal::enable_raw_mode()?;

    let mut dashboard = Dashboard::new(arch);

    // Channel for events from builder
    let (tx, rx) = std::sync::mpsc::channel();

    // Run builder in thread
    let arch_owned = arch.to_string();
    let build_thread = std::thread::spawn(move || {
        build_initramfs_with_events(&arch_owned, |event| {
            tx.send(event).ok();
        })
    });

    // Render loop (non-blocking event receive)
    loop {
        // Process all available events
        while let Ok(event) = rx.try_recv() {
            dashboard.handle_event(&event);

            if matches!(event, BuildEvent::BuildComplete { .. } | BuildEvent::BuildFailed { .. }) {
                break;
            }
        }

        // Render current state
        terminal.draw(|f| dashboard.render(f))?;

        // Small sleep to avoid busy-loop
        std::thread::sleep(Duration::from_millis(16)); // ~60fps

        // Check if build is done
        if dashboard.is_complete() {
            break;
        }
    }

    // Cleanup terminal
    ratatui::restore();
    crossterm::terminal::disable_raw_mode()?;

    // Get result from build thread
    build_thread.join().unwrap()
}
```

## Visual Elements

### Icons

| Item Type | Icon | Example |
|-----------|------|---------|
| Directory | 📁 | `📁 /bin` |
| Binary | + | `+ /bin/busybox` |
| Symlink | → | `→ /bin/sh -> busybox` |
| File | 📄 | `📄 /etc/passwd` |

### Status Indicators

| Status | Symbol | Color |
|--------|--------|-------|
| Done | ✓ | Green |
| In Progress | ◉ | Yellow |
| Pending | (space) | Dim |
| Error | ✗ | Red |

### Progress Bar

```
[████████████░░░░░░░░] 60%
```

Uses Unicode block characters for smooth progress.

## Manifest Integration

The TUI reads from `initramfs/initramfs.toml` to know totals upfront:

```toml
[meta]
version = 1

[layout]
directories = ["bin", "sbin", "etc", "dev", "proc", "sys", "tmp", "root"]

[binaries.busybox]
source = "toolchain/busybox-out/${arch}/busybox"
install = "/bin/busybox"
copy_as_init = true

[symlinks]
"/bin/sh" = "busybox"
"/bin/ash" = "busybox"
# ... 58 more symlinks

[files]
"/etc/inittab" = { source = "initramfs/files/etc/inittab", mode = 0o644 }
"/etc/passwd" = { source = "initramfs/files/etc/passwd", mode = 0o644 }
# ... more files
```

This allows the TUI to show accurate progress (e.g., "47/60 symlinks").

## Error Handling

### Build Errors

Displayed prominently in the activity area:
```
┌──────────────────────────────────────────────────────────────────────────────┐
│  ACTIVITY                                                                    │
│  ──────────────────────────────────────────────────────────────────────────  │
│   ✓  + /bin/busybox                                              1.2 MB     │
│   ✗  ERROR: Binary not found: toolchain/busybox-out/x86_64/busybox          │
│      Run 'cargo xtask build busybox' first                                  │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Validation Warnings

Show before build starts:
```
┌──────────────────────────────────────────────────────────────────────────────┐
│  ⚠ WARNINGS                                                                  │
│  ──────────────────────────────────────────────────────────────────────────  │
│   • nano is dynamically linked (may not work without kernel mmap support)    │
│   • Missing library: libncursesw.so.6                                        │
├──────────────────────────────────────────────────────────────────────────────┤
│  Phase: Validating manifest                                [██████████] 100% │
└──────────────────────────────────────────────────────────────────────────────┘
```

## Completion Screen

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  LEVITATE INITRAMFS BUILDER                                       x86_64    │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ✅ BUILD COMPLETE                                                           │
│                                                                              │
│     Output: target/initramfs/x86_64.cpio                                    │
│     Size:   3,215 KB                                                        │
│     Time:   0.42s                                                           │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  SUMMARY                                                                     │
│  ──────────────────────────────────────────────────────────────────────────  │
│  Directories    8          Symlinks    60/60         Total Size   3.2 MB    │
│  Binaries       2          Files        7/7          Elapsed      0.42s     │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

Stays visible for 1 second then exits, or exits immediately with `--quick`.

## Integration with xtask

### Command Structure

```rust
// xtask/src/main.rs

#[derive(Subcommand)]
enum BuildCommands {
    /// Build everything
    All,
    /// Build kernel
    Kernel,
    /// Build initramfs (with TUI dashboard)
    Initramfs {
        /// Force TUI dashboard even in non-TTY
        #[arg(long)]
        tui: bool,
        /// Disable TUI dashboard
        #[arg(long)]
        no_tui: bool,
    },
    // ...
}
```

### Build Flow

```rust
BuildCommands::Initramfs { tui, no_tui } => {
    let use_tui = match (tui, no_tui) {
        (true, _) => true,
        (_, true) => false,
        _ => should_use_tui(),
    };

    if use_tui {
        initramfs::build_with_tui(arch)?;
    } else {
        initramfs::build_simple(arch)?;
    }
}
```

## Testing

### Manual Testing

```bash
# Test TUI mode
cargo xtask build initramfs

# Test simple mode
cargo xtask build initramfs --no-tui

# Test CI mode
CI=true cargo xtask build initramfs
```

### Unit Tests

```rust
#[test]
fn test_dashboard_state_machine() {
    let mut dash = Dashboard::new("x86_64");

    dash.handle_event(BuildEvent::PhaseStart { name: "Directories", total: 8 });
    assert_eq!(dash.current_phase, "Directories");
    assert_eq!(dash.phase_progress, (0, 8));

    dash.handle_event(BuildEvent::DirectoryCreated { path: "/bin".into() });
    assert_eq!(dash.phase_progress, (1, 8));
    assert_eq!(dash.stats.directories, 1);
}
```

## Performance

- Render at 60fps max (16ms per frame)
- Activity log limited to last 20 items (scrolling window)
- Event processing is non-blocking
- Build runs in separate thread to not block rendering

## Next Steps

1. Implement `cpio.rs` (pure Rust CPIO writer)
2. Implement `manifest.rs` (TOML parser)
3. Implement `builder.rs` (event-emitting builder)
4. Implement `tui.rs` (dashboard)
5. Integrate into xtask build command
6. Create `initramfs/initramfs.toml` manifest
7. Create `initramfs/files/` with static content
8. Test and verify
