# Syscall Requirements for General-Purpose OS

**Created**: 2026-01-10
**Status**: Reference Document

This document lists all syscalls required for a general-purpose Unix-compatible OS, organized by epic.

---

## Legend

- ✅ Implemented
- 🔨 In Progress
- ⏳ Planned
- ❌ Not Started

---

## Epic 1: Process Model (TEAM_400)

| Syscall | x86_64 | aarch64 | Status | Notes |
|---------|--------|---------|--------|-------|
| fork | 57 | 1071 (clone) | ⏳ | Clone process |
| vfork | 58 | 1071 (clone) | ⏳ | Lightweight fork |
| clone | 56 | 220 | ⏳ | General process creation |
| clone3 | 435 | 435 | ⏳ | Modern clone |
| execve | 59 | 221 | ⏳ | Execute program |
| execveat | 322 | 281 | ⏳ | Execute relative to fd |
| wait4 | 61 | 260 | ⏳ | Wait for child |
| waitid | 247 | 95 | ⏳ | Wait with options |
| exit | 60 | 93 | ✅ | Exit thread |
| exit_group | 231 | 94 | ⏳ | Exit process |
| getpid | 39 | 172 | ✅ | Get process ID |
| getppid | 110 | 173 | ⏳ | Get parent PID |
| gettid | 186 | 178 | ⏳ | Get thread ID |
| set_tid_address | 218 | 96 | ⏳ | Set clear_child_tid |
| prctl | 157 | 167 | ⏳ | Process control |

---

## Epic 2: Filesystem Hierarchy (TEAM_401)

### Device Operations

| Syscall | x86_64 | aarch64 | Status | Notes |
|---------|--------|---------|--------|-------|
| mknod | 133 | - | ⏳ | Create device node |
| mknodat | 259 | 33 | ⏳ | Create device at path |

### Mount Operations

| Syscall | x86_64 | aarch64 | Status | Notes |
|---------|--------|---------|--------|-------|
| mount | 165 | 40 | ✅ | Mount filesystem |
| umount2 | 166 | 39 | ⏳ | Unmount filesystem |
| pivot_root | 155 | 41 | ⏳ | Change root (TEAM_402) |

### Procfs Support

| Syscall | x86_64 | aarch64 | Status | Notes |
|---------|--------|---------|--------|-------|
| readlink | 89 | 78 | ✅ | Read symlink |
| readlinkat | 267 | 78 | ✅ | Read symlink at path |

---

## Epic 3: Disk Root (TEAM_402)

| Syscall | x86_64 | aarch64 | Status | Notes |
|---------|--------|---------|--------|-------|
| pivot_root | 155 | 41 | ⏳ | Switch root filesystem |
| chroot | 161 | 51 | ⏳ | Change root directory |
| sync | 162 | 81 | ⏳ | Sync filesystems |
| syncfs | 306 | 267 | ⏳ | Sync one filesystem |
| fsync | 74 | 82 | ✅ | Sync file |
| fdatasync | 75 | 83 | ⏳ | Sync file data |

---

## Epic 4: Users & Permissions (TEAM_405)

### Identity Query

| Syscall | x86_64 | aarch64 | Status | Notes |
|---------|--------|---------|--------|-------|
| getuid | 102 | 174 | ⏳ | Get real UID |
| geteuid | 107 | 175 | ⏳ | Get effective UID |
| getgid | 104 | 176 | ⏳ | Get real GID |
| getegid | 108 | 177 | ⏳ | Get effective GID |
| getresuid | 118 | 148 | ⏳ | Get real/eff/saved UID |
| getresgid | 120 | 150 | ⏳ | Get real/eff/saved GID |
| getgroups | 115 | 80 | ⏳ | Get supplementary groups |

### Identity Change

| Syscall | x86_64 | aarch64 | Status | Notes |
|---------|--------|---------|--------|-------|
| setuid | 105 | 146 | ⏳ | Set UID |
| setgid | 106 | 144 | ⏳ | Set GID |
| setreuid | 113 | 145 | ⏳ | Set real/effective UID |
| setregid | 114 | 143 | ⏳ | Set real/effective GID |
| setresuid | 117 | 147 | ⏳ | Set real/eff/saved UID |
| setresgid | 119 | 149 | ⏳ | Set real/eff/saved GID |
| setgroups | 116 | 81 | ⏳ | Set supplementary groups |
| setfsuid | 122 | 151 | ⏳ | Set filesystem UID |
| setfsgid | 123 | 152 | ⏳ | Set filesystem GID |

### File Permissions

| Syscall | x86_64 | aarch64 | Status | Notes |
|---------|--------|---------|--------|-------|
| chmod | 90 | - | ⏳ | Change file mode |
| fchmod | 91 | 52 | ⏳ | Change mode by fd |
| fchmodat | 268 | 53 | ⏳ | Change mode at path |
| chown | 92 | - | ⏳ | Change owner |
| fchown | 93 | 55 | ⏳ | Change owner by fd |
| fchownat | 260 | 54 | ⏳ | Change owner at path |
| lchown | 94 | - | ⏳ | Change symlink owner |
| access | 21 | - | ⏳ | Check access |
| faccessat | 269 | 48 | ⏳ | Check access at path |
| faccessat2 | 439 | 439 | ⏳ | Check access with flags |
| umask | 95 | 166 | ⏳ | Set file creation mask |

---

## Epic 5: Signals (Future)

### Signal Handling

| Syscall | x86_64 | aarch64 | Status | Notes |
|---------|--------|---------|--------|-------|
| rt_sigaction | 13 | 134 | ❌ | Set signal handler |
| rt_sigprocmask | 14 | 135 | ❌ | Block/unblock signals |
| rt_sigreturn | 15 | 139 | ❌ | Return from handler |
| rt_sigsuspend | 130 | 133 | ❌ | Wait for signal |
| rt_sigpending | 127 | 136 | ❌ | Get pending signals |
| rt_sigtimedwait | 128 | 137 | ❌ | Wait with timeout |
| rt_sigqueueinfo | 129 | 138 | ❌ | Queue signal |
| kill | 62 | 129 | ❌ | Send signal |
| tgkill | 234 | 131 | ❌ | Send to thread |
| tkill | 200 | 130 | ❌ | Send to thread (old) |

### Process Groups & Sessions

| Syscall | x86_64 | aarch64 | Status | Notes |
|---------|--------|---------|--------|-------|
| getpgid | 121 | 155 | ❌ | Get process group |
| setpgid | 109 | 154 | ❌ | Set process group |
| getpgrp | 111 | - | ❌ | Get own process group |
| getsid | 124 | 156 | ❌ | Get session ID |
| setsid | 112 | 157 | ❌ | Create session |

---

## Epic 6: Networking (Future)

| Syscall | x86_64 | aarch64 | Status | Notes |
|---------|--------|---------|--------|-------|
| socket | 41 | 198 | ❌ | Create socket |
| bind | 49 | 200 | ❌ | Bind address |
| listen | 50 | 201 | ❌ | Listen for connections |
| accept | 43 | 202 | ❌ | Accept connection |
| accept4 | 288 | 242 | ❌ | Accept with flags |
| connect | 42 | 203 | ❌ | Connect to server |
| sendto | 44 | 206 | ❌ | Send data |
| recvfrom | 45 | 207 | ❌ | Receive data |
| sendmsg | 46 | 211 | ❌ | Send message |
| recvmsg | 47 | 212 | ❌ | Receive message |
| shutdown | 48 | 210 | ❌ | Shutdown socket |
| setsockopt | 54 | 208 | ❌ | Set socket option |
| getsockopt | 55 | 209 | ❌ | Get socket option |
| getsockname | 51 | 204 | ❌ | Get socket address |
| getpeername | 52 | 205 | ❌ | Get peer address |
| socketpair | 53 | 199 | ❌ | Create socket pair |

---

## Already Implemented (Reference)

| Syscall | x86_64 | aarch64 | Notes |
|---------|--------|---------|-------|
| read | 0 | 63 | ✅ |
| write | 1 | 64 | ✅ |
| open | 2 | - | ✅ |
| openat | 257 | 56 | ✅ |
| close | 3 | 57 | ✅ |
| fstat | 5 | 80 | ✅ |
| lstat | 6 | - | ✅ |
| stat | 4 | - | ✅ |
| newfstatat | 262 | 79 | ✅ |
| lseek | 8 | 62 | ✅ |
| mmap | 9 | 222 | ✅ |
| munmap | 11 | 215 | ✅ |
| mprotect | 10 | 226 | ✅ |
| brk | 12 | 214 | ✅ |
| ioctl | 16 | 29 | ✅ |
| readv | 19 | 65 | ✅ |
| writev | 20 | 66 | ✅ |
| dup | 32 | 23 | ✅ |
| dup2 | 33 | - | ✅ |
| dup3 | 292 | 24 | ✅ |
| fcntl | 72 | 25 | ✅ |
| getcwd | 79 | 17 | ✅ |
| chdir | 80 | 49 | ✅ |
| fchdir | 81 | 50 | ✅ |
| mkdir | 83 | - | ✅ |
| mkdirat | 258 | 34 | ✅ |
| rmdir | 84 | - | ✅ |
| unlink | 87 | - | ✅ |
| unlinkat | 263 | 35 | ✅ |
| rename | 82 | - | ✅ |
| renameat | 264 | 38 | ✅ |
| link | 86 | - | ✅ |
| linkat | 265 | 37 | ✅ |
| symlink | 88 | - | ✅ |
| symlinkat | 266 | 36 | ✅ |
| getdents64 | 217 | 61 | ✅ |
| pipe | 22 | - | ✅ |
| pipe2 | 293 | 59 | ✅ |
| poll | 7 | - | ✅ |
| ppoll | 271 | 73 | ✅ |
| nanosleep | 35 | 101 | ✅ |
| clock_gettime | 228 | 113 | ✅ |
| arch_prctl | 158 | - | ✅ (x86_64) |
| set_tls | - | - | ✅ (aarch64) |
| getrandom | 318 | 278 | ✅ |

---

## Syscall Count Summary

| Category | Count | Status |
|----------|-------|--------|
| Already Implemented | ~50 | ✅ |
| Epic 1 (Process) | 15 | ⏳ |
| Epic 2 (FHS) | 5 | ⏳ |
| Epic 3 (Disk) | 6 | ⏳ |
| Epic 4 (Users) | 25 | ⏳ |
| Epic 5 (Signals) | 15 | ❌ |
| Epic 6 (Networking) | 20 | ❌ |
| **Total New** | **~85** | |

---

## Critical Path Syscalls

These syscalls are blocking for general-purpose OS:

1. **fork/clone** - Can't spawn processes without it
2. **execve** - Can't run programs without it
3. **wait4** - Can't manage children without it
4. **getuid/setuid** - Can't have users without it
5. **chmod/chown** - Can't have permissions without it
6. **pivot_root** - Can't have disk root without it

Everything else can be stubbed or implemented incrementally.
