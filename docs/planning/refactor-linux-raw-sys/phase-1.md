# Phase 1 — Discovery and Safeguards

**Refactor:** Migrate to linux-raw-sys  
**Team:** TEAM_419  
**Date:** 2026-01-10

---

## Refactor Summary

Replace all hardcoded Linux ABI constants with the `linux-raw-sys` crate, which provides auto-generated constants from Linux kernel headers.

### Pain Points
- Hardcoded constants may drift from Linux ABI
- Manual maintenance burden
- No authoritative source

### Approach
**No shims. No backward compatibility. Break it and fix it.**
- Delete hardcoded constants
- Let compiler fail
- Fix all call sites with `linux-raw-sys` imports

---

## Success Criteria

### Before
```rust
// syscall/constants.rs - hardcoded
pub const CLONE_VM: u64 = 0x00000100;
pub const RLIMIT_NOFILE: u32 = 7;

// fs/mode.rs - hardcoded
pub const S_IFMT: u32 = 0o170000;
```

### After
```rust
// Direct imports from linux-raw-sys
use linux_raw_sys::general::{CLONE_VM, RLIMIT_NOFILE};
use linux_raw_sys::general::{S_IFMT, S_IFREG, S_IFDIR};
```

---

## Constants to Replace

### File: `syscall/constants.rs` (DELETE ENTIRELY)
| Constant | linux-raw-sys location |
|----------|------------------------|
| `PATH_MAX` | `linux_raw_sys::general::PATH_MAX` |
| `CLONE_*` | `linux_raw_sys::general::CLONE_*` |
| `RLIMIT_*` | `linux_raw_sys::general::RLIMIT_*` |
| `RLIM_INFINITY` | `linux_raw_sys::general::RLIM_INFINITY` |

### File: `syscall/mod.rs` (fcntl module)
| Constant | linux-raw-sys location |
|----------|------------------------|
| `AT_FDCWD` | `linux_raw_sys::general::AT_FDCWD` |
| `AT_SYMLINK_NOFOLLOW` | `linux_raw_sys::general::AT_SYMLINK_NOFOLLOW` |
| `AT_REMOVEDIR` | `linux_raw_sys::general::AT_REMOVEDIR` |
| `AT_SYMLINK_FOLLOW` | `linux_raw_sys::general::AT_SYMLINK_FOLLOW` |
| `AT_NO_AUTOMOUNT` | `linux_raw_sys::general::AT_NO_AUTOMOUNT` |
| `AT_EMPTY_PATH` | `linux_raw_sys::general::AT_EMPTY_PATH` |

### File: `syscall/errno.rs` (errno module)
| Constant | linux-raw-sys location |
|----------|------------------------|
| `ENOENT`, `ESRCH`, etc. | `linux_raw_sys::errno::*` |

### File: `fs/mode.rs` (DELETE OR REPLACE)
| Constant | linux-raw-sys location |
|----------|------------------------|
| `S_IFMT`, `S_IFREG`, `S_IFDIR`, etc. | `linux_raw_sys::general::S_IF*` |

### File: `fs/vfs/file.rs` (OpenFlags)
| Constant | linux-raw-sys location |
|----------|------------------------|
| `O_RDONLY`, `O_WRONLY`, `O_RDWR` | `linux_raw_sys::general::O_*` |
| `O_CREAT`, `O_TRUNC`, `O_APPEND` | `linux_raw_sys::general::O_*` |

### File: `syscall/mm.rs` (mmap)
| Constant | linux-raw-sys location |
|----------|------------------------|
| `PROT_*` | `linux_raw_sys::general::PROT_*` |
| `MAP_*` | `linux_raw_sys::general::MAP_*` |

### File: `syscall/epoll.rs`
| Constant | linux-raw-sys location |
|----------|------------------------|
| `EPOLL_*` | `linux_raw_sys::general::EPOLL_*` |

### File: `syscall/signal.rs`
| Constant | linux-raw-sys location |
|----------|------------------------|
| `SIGINT`, `SIGKILL`, etc. | `linux_raw_sys::general::SIG*` |

### File: `arch/*/mod.rs` (TTY constants)
| Constant | linux-raw-sys location |
|----------|------------------------|
| `TCGETS`, `TCSETS`, `TIOCGWINSZ` | `linux_raw_sys::general::TC*`, `TIOC*` |
| termios flags | `linux_raw_sys::general::*` |

---

## Behavioral Contracts

- All syscall return values unchanged
- All struct layouts unchanged (same `#[repr(C)]`)
- All constant values identical to Linux ABI

---

## Constraints

1. **`#![no_std]` required** - kernel cannot use std
2. **Type compatibility** - linux-raw-sys uses different integer types (may need casts)
3. **Feature flags** - need correct features enabled in linux-raw-sys

---

## Open Questions

1. Does linux-raw-sys support both x86_64 and aarch64? **YES** (arch-specific modules)
2. Are all our constants available? **Mostly YES** - may need to verify a few

---

## Phase 1 Steps

### Step 1 — Verify linux-raw-sys Compatibility
- [ ] Check `#![no_std]` support
- [ ] Check x86_64 and aarch64 support
- [ ] Verify constants exist for our use cases

### Step 2 — Run Baseline Tests
- [ ] `cargo xtask build kernel --arch x86_64`
- [ ] `cargo xtask build kernel --arch aarch64`
- [ ] Document current state

---

## Exit Criteria for Phase 1
- [ ] Baseline tests pass
- [ ] linux-raw-sys compatibility confirmed
- [ ] All constants mapped to linux-raw-sys equivalents
