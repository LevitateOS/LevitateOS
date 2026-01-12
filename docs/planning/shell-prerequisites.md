# Shell Prerequisites Checklist

This document lists ALL OS-level features that must be implemented BEFORE a POSIX shell (like dash, bash, or brush) can function correctly. Learned the hard way during TEAM_444/TEAM_445.

## Quick Reference

| Category | Critical | Important | Nice-to-Have |
|----------|----------|-----------|--------------|
| Process Management | fork, execve, waitpid, exit | clone, vfork | posix_spawn |
| File Descriptors | open, close, read, write, dup2, pipe | dup, dup3, pipe2 | splice, tee |
| File System | stat, fstat, getcwd, chdir, openat | access, readlink, getdents | faccessat |
| Memory | brk, mmap, munmap | mprotect, mremap | madvise |
| Signals | sigaction, kill, sigprocmask | sigreturn, rt_sigaction | signalfd |
| Terminal | ioctl (termios), tcsetpgrp | isatty, ttyname | openpty |
| Environment | execve envp handling | getenv in libc | setenv |

---

## 1. Process Management

### 1.1 Process Creation & Execution (CRITICAL)

| Syscall | Purpose | Shell Usage | Notes |
|---------|---------|-------------|-------|
| `fork()` | Create child process | Running any external command | Must copy entire address space correctly |
| `clone()` | Create process/thread | Threading, vfork emulation | Flags: CLONE_VM, CLONE_FILES, CLONE_SIGHAND |
| `execve()` | Replace process image | Running commands | Must handle argv, envp, shebang (#!) |
| `exit()` / `exit_group()` | Terminate process | Command completion | Return exit status to parent |
| `waitpid()` / `wait4()` | Wait for child | Reaping children, getting exit status | WNOHANG for job control |

**Gotchas discovered:**
- `fork()` must properly copy memory mappings, not just page tables
- `execve()` must clear signal handlers (SIG_DFL) except ignored ones
- Child processes inherit file descriptors - must handle CLOEXEC

### 1.2 Process Identity (IMPORTANT)

| Syscall | Purpose | Shell Usage |
|---------|---------|-------------|
| `getpid()` | Get process ID | $$ variable |
| `getppid()` | Get parent PID | $PPID variable |
| `getpgid()` / `getpgrp()` | Get process group | Job control |
| `setpgid()` / `setpgrp()` | Set process group | Job control - new job = new pgrp |
| `getsid()` / `setsid()` | Session management | Creating new session (login shell) |

### 1.3 Process Groups & Sessions (IMPORTANT for job control)

```
Session (setsid)
└── Process Group (setpgid) - "job"
    ├── Process (fork)
    ├── Process
    └── Process
```

**Required for job control (fg, bg, jobs):**
- Each pipeline becomes a process group
- Shell must call `setpgid()` in BOTH parent and child (race condition!)
- Foreground job gets the terminal via `tcsetpgrp()`

---

## 2. File Descriptors

### 2.1 Basic I/O (CRITICAL)

| Syscall | Purpose | Shell Usage |
|---------|---------|-------------|
| `open()` / `openat()` | Open file | Redirection: `cmd > file` |
| `close()` | Close fd | Cleanup after redirection |
| `read()` | Read data | Reading input, scripts |
| `write()` | Write data | Output, echo builtin |
| `lseek()` | Seek in file | `<>` redirection, here-docs |

**TEAM_444 discovery:** dash uses legacy `open()` (syscall 2), not just `openat()`. Must support both!

### 2.2 File Descriptor Manipulation (CRITICAL)

| Syscall | Purpose | Shell Usage |
|---------|---------|-------------|
| `dup()` | Duplicate fd | Saving stdin before redirect |
| `dup2()` | Duplicate to specific fd | `2>&1`, redirections |
| `dup3()` | dup2 + flags | O_CLOEXEC support |
| `pipe()` / `pipe2()` | Create pipe | Pipelines: `cmd1 \| cmd2` |
| `fcntl()` | fd operations | F_DUPFD, F_GETFL, F_SETFL, F_SETFD |

**Redirection implementation:**
```
# cmd > file 2>&1
1. fd = open("file", O_WRONLY|O_CREAT|O_TRUNC)
2. dup2(fd, 1)      # stdout -> file
3. dup2(1, 2)       # stderr -> stdout (which is now file)
4. close(fd)
5. exec cmd
```

### 2.3 Pipe Mechanics (CRITICAL)

```
Pipeline: cmd1 | cmd2 | cmd3

        cmd1              cmd2              cmd3
    ┌─────────┐       ┌─────────┐       ┌─────────┐
    │  stdout ├──────►│ stdin   │       │         │
    │         │ pipe1 │  stdout ├──────►│ stdin   │
    └─────────┘       └─────────┘ pipe2 └─────────┘
```

**Implementation steps:**
1. Create N-1 pipes for N commands
2. Fork each command
3. In child: dup2 pipe ends to stdin/stdout, close unused ends
4. In parent: close all pipe ends
5. Wait for all children

---

## 3. File System

### 3.1 File Information (CRITICAL)

| Syscall | Purpose | Shell Usage |
|---------|---------|-------------|
| `stat()` / `fstat()` / `lstat()` | Get file info | `test -f`, `[`, `[[` |
| `access()` / `faccessat()` | Check permissions | `test -r`, `-w`, `-x` |
| `readlink()` / `readlinkat()` | Read symlink | Path resolution |

**Test builtin needs:**
- `-e` exists: stat() succeeds
- `-f` regular file: S_ISREG(st_mode)
- `-d` directory: S_ISDIR(st_mode)
- `-r` readable: access(R_OK)
- `-w` writable: access(W_OK)
- `-x` executable: access(X_OK)
- `-s` non-empty: st_size > 0

### 3.2 Directory Operations (CRITICAL)

| Syscall | Purpose | Shell Usage |
|---------|---------|-------------|
| `getcwd()` | Get current directory | $PWD, pwd builtin |
| `chdir()` | Change directory | cd builtin |
| `getdents()` / `getdents64()` | Read directory | Glob expansion `*.txt` |
| `mkdir()` / `mkdirat()` | Create directory | mkdir builtin (if any) |

### 3.3 Path Resolution (IMPORTANT)

The shell must resolve commands via PATH:
```
PATH=/bin:/usr/bin

lookup("ls"):
  1. Try /bin/ls - stat(), check +x
  2. Try /usr/bin/ls - stat(), check +x
  3. Return first match or "not found"
```

---

## 4. Memory Management

### 4.1 Heap (CRITICAL)

| Syscall | Purpose | Shell Usage |
|---------|---------|-------------|
| `brk()` / `sbrk()` | Extend heap | malloc() in libc |
| `mmap()` | Map memory | Large allocations, loading |
| `munmap()` | Unmap memory | Free large allocations |
| `mprotect()` | Change protection | Stack guard pages |

**Note:** Most shells use malloc() heavily. libc needs working brk/mmap.

### 4.2 Memory for execve (CRITICAL)

When loading a new executable:
1. Parse ELF headers
2. `mmap()` code segment (PROT_READ|PROT_EXEC)
3. `mmap()` data segment (PROT_READ|PROT_WRITE)
4. Set up stack with argv, envp, auxv
5. Clear BSS (zero-filled pages)
6. Jump to entry point

---

## 5. Signals

### 5.1 Signal Handling (CRITICAL)

| Syscall | Purpose | Shell Usage |
|---------|---------|-------------|
| `sigaction()` | Set signal handler | Ctrl+C handling |
| `rt_sigaction()` | Modern sigaction | Same, 64-bit signal mask |
| `sigprocmask()` | Block/unblock signals | Critical sections |
| `kill()` | Send signal | kill builtin, job control |
| `sigreturn()` | Return from handler | Kernel restores context |

### 5.2 Required Signals

| Signal | Number | Default | Shell Handling |
|--------|--------|---------|----------------|
| SIGINT | 2 | Term | Interrupt foreground job |
| SIGQUIT | 3 | Core | Quit foreground job |
| SIGCHLD | 17 | Ignore | Notify parent of child status |
| SIGPIPE | 13 | Term | Broken pipe in pipeline |
| SIGTERM | 15 | Term | Graceful termination |
| SIGHUP | 1 | Term | Terminal hangup |
| SIGTSTP | 20 | Stop | Ctrl+Z - suspend foreground |
| SIGCONT | 18 | Continue | fg/bg builtins |
| SIGTTIN | 21 | Stop | Background read from tty |
| SIGTTOU | 22 | Stop | Background write to tty |

### 5.3 Job Control Signal Flow

```
User presses Ctrl+Z:
1. Terminal driver sends SIGTSTP to foreground pgrp
2. Foreground processes stop
3. Shell (in background pgrp) receives SIGCHLD
4. Shell's SIGCHLD handler calls waitpid(WUNTRACED)
5. Shell prints "[1]+ Stopped  command"
6. Shell becomes foreground again (tcsetpgrp)

User types "fg":
1. Shell calls kill(-pgrp, SIGCONT)
2. Shell calls tcsetpgrp(tty, pgrp)
3. Stopped job continues in foreground
```

---

## 6. Terminal / TTY

### 6.1 Terminal Control (CRITICAL)

| Operation | Method | Purpose |
|-----------|--------|---------|
| Get/set termios | `ioctl(TCGETS/TCSETS)` | Raw mode, echo, etc. |
| Get foreground pgrp | `ioctl(TIOCGPGRP)` | Job control |
| Set foreground pgrp | `ioctl(TIOCSPGRP)` | Job control |
| Get window size | `ioctl(TIOCGWINSZ)` | $COLUMNS, $LINES |
| Is a tty? | `ioctl(TCGETS)` succeeds | isatty() |

### 6.2 Termios Structure (CRITICAL)

**TEAM_445 discovery:** Uninitialized termios breaks everything!

```c
struct termios {
    tcflag_t c_iflag;    // Input flags
    tcflag_t c_oflag;    // Output flags
    tcflag_t c_cflag;    // Control flags
    tcflag_t c_lflag;    // Local flags
    cc_t c_cc[NCCS];     // Control characters
    speed_t c_ispeed;    // Input baud
    speed_t c_ospeed;    // Output baud
};
```

**Required initial values:**

| Flag | Value | Meaning |
|------|-------|---------|
| c_iflag | ICRNL \| IXON | CR->NL, XON/XOFF |
| c_oflag | OPOST \| ONLCR | Post-process, NL->CRNL |
| c_cflag | CS8 \| CREAD | 8-bit, enable receiver |
| c_lflag | ISIG \| ICANON \| ECHO \| ECHOE | Signals, line edit, echo |

**Control characters (c_cc):**

| Index | Name | Default | Key |
|-------|------|---------|-----|
| VINTR | Interrupt | 0x03 | Ctrl+C |
| VQUIT | Quit | 0x1C | Ctrl+\\ |
| VERASE | Erase char | 0x7F | DEL/Backspace |
| VKILL | Kill line | 0x15 | Ctrl+U |
| VEOF | End of file | 0x04 | Ctrl+D |
| VMIN | Min chars | 0x01 | - |
| VSTART | Start output | 0x11 | Ctrl+Q |
| VSTOP | Stop output | 0x13 | Ctrl+S |
| VSUSP | Suspend | 0x1A | Ctrl+Z |

### 6.3 Canonical vs Raw Mode

**Canonical mode (ICANON set):**
- Line buffered input
- Backspace, Ctrl+U work
- Read returns on newline or EOF
- Shell uses this for simple input

**Raw mode (ICANON cleared):**
- Character-at-a-time input
- No line editing by kernel
- Shell handles all editing
- Used by bash/zsh for readline

---

## 7. Environment & Arguments

### 7.1 Process Stack Layout (CRITICAL for execve)

```
High addresses
┌─────────────────────┐
│ Platform string     │ "x86_64"
├─────────────────────┤
│ Random bytes (16)   │ AT_RANDOM points here
├─────────────────────┤
│ Environment strings │ "PATH=/bin\0HOME=/root\0"
├─────────────────────┤
│ Argument strings    │ "./dash\0-c\0echo hi\0"
├─────────────────────┤
│ NULL                │ End of auxv
│ Auxiliary vector    │ AT_PAGESZ, AT_PHDR, etc.
│ ...                 │
├─────────────────────┤
│ NULL                │ End of envp
│ envp[n]             │ Pointers to env strings
│ ...                 │
│ envp[0]             │
├─────────────────────┤
│ NULL                │ End of argv
│ argv[argc]          │
│ ...                 │
│ argv[0]             │
├─────────────────────┤
│ argc                │ Argument count
└─────────────────────┘
Low addresses (stack pointer)
```

### 7.2 Required Auxiliary Vector Entries

| Entry | Value | Purpose |
|-------|-------|---------|
| AT_NULL (0) | 0 | End marker |
| AT_PAGESZ (6) | 4096 | Page size |
| AT_RANDOM (25) | ptr | 16 random bytes |
| AT_PHDR (3) | ptr | Program headers |
| AT_PHENT (4) | size | Size of phdr entry |
| AT_PHNUM (5) | count | Number of phdrs |
| AT_ENTRY (9) | addr | Entry point |
| AT_UID/GID (11-14) | ids | User/group IDs |

**Rust std requires AT_RANDOM** - will crash without it!

---

## 8. User & Permissions

### 8.1 User Identity (IMPORTANT)

| Syscall | Purpose | Shell Usage |
|---------|---------|-------------|
| `getuid()` / `geteuid()` | Get user ID | $UID, $EUID |
| `getgid()` / `getegid()` | Get group ID | Permission checks |
| `setuid()` / `setgid()` | Set IDs | su, sudo |
| `getgroups()` | Get supplementary groups | Permission checks |

### 8.2 File Permissions

- Shell needs to check execute permission before exec
- `access(X_OK)` or `stat()` + check mode bits
- setuid/setgid bits on executables

---

## 9. Time

| Syscall | Purpose | Shell Usage |
|---------|---------|-------------|
| `clock_gettime()` | Get current time | $SECONDS, time builtin |
| `nanosleep()` | Sleep | sleep builtin |
| `gettimeofday()` | Wall clock time | Timestamps |

---

## 10. Miscellaneous

### 10.1 Resource Limits

| Syscall | Purpose | Shell Usage |
|---------|---------|-------------|
| `getrlimit()` / `setrlimit()` | Resource limits | ulimit builtin |
| `prlimit64()` | Modern limits | Same |

### 10.2 umask

| Syscall | Purpose | Shell Usage |
|---------|---------|-------------|
| `umask()` | File creation mask | umask builtin |

### 10.3 Other

| Syscall | Purpose | Shell Usage |
|---------|---------|-------------|
| `uname()` | System info | uname, $OSTYPE |
| `ioctl()` | Device control | Everything terminal |

---

## Testing Checklist

Before declaring shell support complete, verify:

### Process Tests
- [ ] `echo $$` prints PID
- [ ] Simple command runs: `ls`
- [ ] Command with args: `ls -la /`
- [ ] Exit status works: `false; echo $?` prints 1

### Redirection Tests
- [ ] Output redirect: `echo hi > /tmp/test`
- [ ] Input redirect: `cat < /etc/passwd`
- [ ] Append: `echo hi >> /tmp/test`
- [ ] stderr redirect: `ls /nonexistent 2>/dev/null`
- [ ] Combined: `cmd > out 2>&1`

### Pipeline Tests
- [ ] Simple pipe: `echo hi | cat`
- [ ] Multi-stage: `cat /etc/passwd | grep root | wc -l`
- [ ] Exit status: `false | true; echo $?` prints 0

### Job Control Tests
- [ ] Ctrl+C interrupts foreground
- [ ] Ctrl+Z suspends foreground
- [ ] `fg` resumes stopped job
- [ ] `bg` runs stopped job in background
- [ ] `jobs` lists jobs
- [ ] `&` runs in background

### Terminal Tests
- [ ] Backspace works
- [ ] Ctrl+U clears line
- [ ] Ctrl+D on empty line exits
- [ ] Arrow keys (if readline)

### Script Tests
- [ ] Shebang scripts work: `#!/bin/sh`
- [ ] Sourcing works: `. ./script.sh`
- [ ] Here-doc: `cat <<EOF`

---

## Implementation Order

Recommended order for implementing shell support:

### Phase 1: Basic Execution
- fork, execve, waitpid, exit
- open, close, read, write
- Basic ELF loading
- Simple PATH resolution

### Phase 2: Redirection
- dup2, pipe
- File descriptor table per-process
- CLOEXEC handling

### Phase 3: Terminal
- termios ioctls
- Proper initial termios values
- Line discipline basics

### Phase 4: Signals
- sigaction, kill
- SIGCHLD for child reaping
- SIGINT/SIGTSTP delivery

### Phase 5: Job Control
- Process groups (setpgid)
- tcsetpgrp/tcgetpgrp
- Session management

### Phase 6: Polish
- Resource limits
- umask
- Full signal set

---

## Common Gotchas (from TEAM_444/TEAM_445)

1. **Termios must be initialized** - All-zeros termios = no keyboard input works
2. **Support both open() and openat()** - C programs use legacy open() syscall
3. **CLOEXEC is critical** - Leaked fds break pipes
4. **Fork must copy VMAs** - Not just page tables, the VMA metadata too
5. **Signals reset on exec** - Except SIG_IGN handlers
6. **Process groups race** - Call setpgid() in BOTH parent and child
7. **AT_RANDOM required** - Rust std crashes without it
8. **Stat struct size matters** - Must be exactly 128 bytes on aarch64

---

## References

- POSIX.1-2017 Shell & Utilities: https://pubs.opengroup.org/onlinepubs/9699919799/
- Linux man-pages: https://man7.org/linux/man-pages/
- dash source: https://git.kernel.org/pub/scm/utils/dash/dash.git
- musl libc source: https://git.musl-libc.org/cgit/musl/
