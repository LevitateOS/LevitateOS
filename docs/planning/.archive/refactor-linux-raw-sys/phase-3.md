# Phase 3 — Fix All Compiler Errors

**Refactor:** Migrate to linux-raw-sys  
**Team:** TEAM_419  
**Date:** 2026-01-10

---

## Purpose

Fix every compiler error by adding proper `linux-raw-sys` imports at each call site.

---

## Strategy

1. Run `cargo build` 
2. Take first error
3. Add import from `linux_raw_sys::*`
4. Repeat until clean

**NO SHIMS. Direct imports only.**

---

## Import Mapping Reference

### Clone Flags
```rust
use linux_raw_sys::general::{
    CLONE_VM, CLONE_FS, CLONE_FILES, CLONE_SIGHAND, CLONE_THREAD,
    CLONE_SETTLS, CLONE_PARENT_SETTID, CLONE_CHILD_CLEARTID, CLONE_CHILD_SETTID,
};
```

### AT_* Constants
```rust
use linux_raw_sys::general::{
    AT_FDCWD, AT_SYMLINK_NOFOLLOW, AT_REMOVEDIR, AT_SYMLINK_FOLLOW,
    AT_NO_AUTOMOUNT, AT_EMPTY_PATH,
};
```

### Error Numbers
```rust
use linux_raw_sys::errno::{ENOENT, ESRCH, EIO, EBADF, ENOMEM, ...};
// Note: linux-raw-sys errno values are positive, may need to negate
```

### File Mode Constants
```rust
use linux_raw_sys::general::{
    S_IFMT, S_IFSOCK, S_IFLNK, S_IFREG, S_IFBLK, S_IFDIR, S_IFCHR, S_IFIFO,
};
```

### Open Flags
```rust
use linux_raw_sys::general::{
    O_RDONLY, O_WRONLY, O_RDWR, O_CREAT, O_EXCL, O_TRUNC, O_APPEND,
    O_NONBLOCK, O_DIRECTORY, O_CLOEXEC,
};
```

### Memory Protection
```rust
use linux_raw_sys::general::{
    PROT_NONE, PROT_READ, PROT_WRITE, PROT_EXEC,
    MAP_SHARED, MAP_PRIVATE, MAP_FIXED, MAP_ANONYMOUS,
};
```

### Resource Limits
```rust
use linux_raw_sys::general::{
    RLIMIT_CPU, RLIMIT_FSIZE, RLIMIT_DATA, RLIMIT_STACK, RLIMIT_CORE,
    RLIMIT_RSS, RLIMIT_NPROC, RLIMIT_NOFILE, RLIMIT_MEMLOCK, RLIMIT_AS,
    RLIM_INFINITY,
};
```

### Signals
```rust
use linux_raw_sys::general::{SIGINT, SIGKILL, SIGCHLD, SIGCONT, ...};
```

### Epoll
```rust
use linux_raw_sys::general::{
    EPOLLIN, EPOLLOUT, EPOLLERR, EPOLLHUP, EPOLLET, EPOLLONESHOT,
    EPOLL_CTL_ADD, EPOLL_CTL_DEL, EPOLL_CTL_MOD,
};
```

### TTY/Termios
```rust
use linux_raw_sys::general::{
    TCGETS, TCSETS, TCSETSW, TCSETSF, TIOCGWINSZ, TIOCSWINSZ,
    TIOCGPTN, TIOCSPTLCK,
    // termios flags
    ECHO, ECHOE, ECHOK, ECHONL, ICANON, ISIG, IEXTEN, NOFLSH, TOSTOP,
    OPOST, ONLCR,
};
```

---

## Type Compatibility Notes

linux-raw-sys uses specific integer types. May need casts:

```rust
// linux-raw-sys might define as u32, we use u64
let flags = CLONE_VM as u64;

// errno might be i32, we return i64
return -(ENOENT as i64);
```

---

## Files to Fix (in order)

1. `syscall/process/mod.rs` - CLONE_* imports
2. `syscall/process/thread.rs` - CLONE_* usage
3. `syscall/process/resources.rs` - RLIMIT_* imports
4. `syscall/helpers.rs` - PATH_MAX
5. `syscall/fs/*.rs` - AT_*, O_* flags
6. `syscall/mm.rs` - PROT_*, MAP_*
7. `syscall/epoll.rs` - EPOLL_*
8. `syscall/signal.rs` - SIG*
9. `syscall/errno` references - all errno codes
10. `fs/mode.rs` - S_IF* constants
11. `fs/vfs/file.rs` - OpenFlags
12. `arch/*/mod.rs` - TTY constants

---

## Exit Criteria for Phase 3

- [ ] `cargo build --package levitate-kernel` succeeds
- [ ] All imports use `linux_raw_sys::*` directly
- [ ] No wrapper modules or re-exports
- [ ] Type casts added where needed
