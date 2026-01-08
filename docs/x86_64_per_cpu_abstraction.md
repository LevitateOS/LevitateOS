# Missing Architectural Abstraction: x86_64 Per-CPU State

## 0. Current Status
The current x86_64 implementation uses **global static variables** (`CURRENT_KERNEL_STACK`, `USER_RSP_SCRATCH`) to manage state during syscall entry/exit.

**THIS IS UNSAFE FOR MULTICORE.** It is a temporary hack that only works because we are running single-threaded for now.

## 1. The Missing Abstraction: `GS` Segment
On x86_64, the standard way to handle per-CPU data is using the `GS` segment register.
1. `IA32_GS_BASE` MSR holds the pointer to the Process Control Block (PCB) or Thread Local Storage (TLS) for userspace.
2. `IA32_KERNEL_GS_BASE` MSR holds the pointer to the Per-CPU Data structure (containing the current kernel stack, current task pointer, etc.) for kernelspace.

## 2. The `swapgs` Instruction
The `swapgs` instruction atomically swaps the values in `IA32_GS_BASE` and `IA32_KERNEL_GS_BASE`.

### syscall Entry Flow
1. User enters kernel (syscall/interrupt). `GS` points to user data.
2. `swapgs` -> `GS` now points to Per-CPU Kernel Data.
3. Access per-cpu data via `gs:[offset]`:
   ```asm
   mov rsp, gs:[OFFSET_KERNEL_STACK]
   mov gs:[OFFSET_USER_RSP_SCRATCH], rbx
   ```

### syscall Exit Flow
1. Restore user state.
2. `swapgs` -> `GS` points back to user data.
3. `sysretq`

## 3. Implementation Plan for Next Team

1. **Define Per-CPU Structure:**
   ```rust
   #[repr(C)]
   pub struct CpuData {
       pub user_rsp_scratch: u64,
       pub kernel_stack_top: u64,
       pub current_task: *const Context,
       pub self_ptr: *const CpuData, // For validation
   }
   ```

2. **Allocate Per-CPU Data:**
   Allocate this structure for the Boot CPU (and eventually APs).

3. **Initialize MSRs:**
   Use `wrmsr` to write the address of `CpuData` to `IA32_KERNEL_GS_BASE`.

4. **Update `syscall_entry`:**
   Replace global `mov [rip + VAR]` with `swapgs` and `mov gs:[OFFSET]`.

## 4. Verification
- Use `rdmsr` to verify MSR contents.
- Verify `swapgs` behavior in QEMU monitor (`info registers`).
