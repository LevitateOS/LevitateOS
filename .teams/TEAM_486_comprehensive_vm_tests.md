# TEAM_486: Comprehensive VM Test Suite

## Goal
Create comprehensive in-VM tests that verify users can actually use the tools we provide for real tasks.

## Status: IN PROGRESS

---

## Test Groups

### Group 1: Coreutils
File operations actually work.
- cp copies files correctly
- mv moves/renames files
- rm deletes files
- mkdir/rmdir create/remove directories
- chmod changes permissions
- chown changes ownership
- ln creates links (hard and soft)
- touch creates/updates timestamps
- cat concatenates files
- dd copies with conversion

### Group 2: Text Processing
Text manipulation tools work correctly.
- head/tail extract lines
- grep finds patterns
- sort orders lines
- uniq removes duplicates
- wc counts correctly
- cut extracts fields
- tr translates characters
- sed does substitutions
- tee duplicates output

### Group 3: Find/Search
File discovery works.
- find locates files by name
- find locates files by type
- find locates files by size
- find locates files by time
- find -exec runs commands
- xargs builds command lines
- xargs handles special characters

### Group 4: Diff/Compare
File comparison works.
- diff detects differences
- diff outputs unified format
- cmp compares binary files
- cmp reports first difference

### Group 5: Process Tools
Process management works.
- ps lists processes
- ps shows process tree
- kill sends signals
- kill terminates processes
- pgrep finds by name
- pkill kills by name
- pidof gets PID

### Group 6: Networking
Network tools function.
- ip addr shows addresses
- ip link manages interfaces
- ip route shows routing
- ping reaches localhost
- ping reaches gateway (if configured)
- ss shows sockets

### Group 7: Editor
Helix editor works.
- hx opens files
- hx creates new files
- hx saves changes
- hx basic editing (insert, delete)

### Group 8: Shell
Shell features work.
- pipes connect commands
- redirects work (>, >>, <)
- variables expand
- command substitution works
- scripts execute
- exit codes propagate
- background jobs (&)

### Group 9: Auth/Users
Authentication works.
- whoami shows current user
- id shows uid/gid
- su switches users
- sudo runs as root
- login accepts credentials
- permissions enforced (can't read others' files)

---

## Implementation Plan

Each group becomes a separate test category in `levitate-test`.
Tests output: `group.testname: PASS/FAIL (reason)`

Start with Group 1 (Coreutils) as foundation - most other tests depend on basic file ops.

---

## Files

- `tools/levitate-test/src/main.rs` - test runner
- `xtask/src/test/mod.rs` - quick test command
