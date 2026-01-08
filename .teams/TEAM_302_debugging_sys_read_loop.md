# Team 302: Debugging Syscall Infinite Loop

## Objective
Investigate and fix the infinite loop of `sys_read` (syscall 0) returning 0 in the userspace shell.

## Status
- [ ] Investigate `sys_read` implementation
- [ ] Investigate shell `read` loop
- [ ] Fix the issue

## Notes
- Userspace `sys_read` returns 0 immediately.
- Shell loops infinitely on this return.
