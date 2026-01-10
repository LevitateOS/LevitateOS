# Team 302: Debugging Syscall Infinite Loop & Invalid Opcode

## Objective
Fix the `INVALID OPCODE` crash in `shell` and ensure robust memory isolation.

## Status
- [x] Synchronize EARLY_ALLOCATOR ranges (8-16MB)
- [x] Refactor MMU to use main allocator after boot
- [x] Remove "Emergency Patch" from `syscall_entry`
- [x] Verify `shell` prompt arrival

## Findings
- **Memory Collision**: HAL and Kernel disagreed on reserved ranges.
- **Hardcoding**: MMU was stuck on early allocator.
- **Unsafe Patch**: A hardcoded jump in assembly was corrupting return addresses.

## TODO
- [ ] Confirm `init` (PID 1) still functions correctly without the patch.
- [ ] Cleanup `verbose-syscalls` logging.
- [ ] Verify interactive shell input (typing 'help').
