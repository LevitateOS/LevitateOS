# x86_64 Context Switch Corruption (TEAM_297)

## The Symptom
The x86_64 kernel was crashing with `EXCEPTION: INVALID OPCODE` at `RIP=0x100b9` (Userspace) immediately after the shell attempted to read input. This address `0x100b9` corresponded to the middle of a `call` instruction bytes, implying `RIP` was corrupted. It should have been `0x100cf`.

## The Investigation
We isolated the issue to `sys_read`'s blocking logic.
- **Normal Flow**: `sys_read` calls `yield_now()`, which calls `switch_to()`.
- **Reproducible**: `yield` consistently caused the crash.
- **Fix Experiment**: Temporarily replacing `yield_now()` with a busy loop (`spin_loop`) inside `sys_read` **eliminated the crash**, proving that the issue lies strictly within the suspend/resume path.

## The Root Cause
The `Context Switch` logic (or interrupt handling during the switch) is corrupting the `SyscallFrame` which resides at the top of the Kernel Stack. When the task resumes and `sys_read` returns, it pops a corrupted Return Address (`RCX`) from the stack, jumping to `0x100b9`.

## Verification Technique: GDB Watchpoints
We used an automated GDB Watchpoint script to trap the memory write that corrupted the stack:
```gdb
# debug_watch_final.gdb
define hook-stop
  if $watch_set == 1
     if $pc == 0xffffffff80010093 # Ignore legitimate push in syscall_entry
         continue
     end
     set $val = *(unsigned long long*)$addr
     # Check for known bad value
     if $val == 0x100b9
        printf "CRITICAL CORRUPTION FOUND! Val=0x%x PC=0x%x\n", $val, $pc
        quit
     end
     continue
  end
end
```
This technique is highly effective for catching memory corruption when the corrupted address is reused by valid code (allows filtering).

## Recommendations for Future Teams
- **Isolate Context Switching**: If a crash depends on blocking/yielding, try replacing the yield with a busy loop (if interrupts enabled) to confirm if context switching is the culprit.
- **Check Stack Alignment**: Ensure `kernel_stack_top` and `TSS.rsp0` maintain 16-byte alignment as required by x86_64 ABI.
