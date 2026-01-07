# Kernel Development Best Practices

This document captures architectural patterns and technical gotchas discovered during the development of LevitateOS.

## 1. Console & Deadlock Prevention

### The Dual-Console Trap
LevitateOS supports dual-console output (UART + GPU). The `println!` macro targets both.
- **Problem**: If a low-level component (like the GPU driver or an interrupt handler) calls `println!`, it can cause a recursive deadlock if the console system is already locked or if the callback itself triggers another log.
- **Gotcha**: A GPU flush failure that logs an error via `println!` will deadlock because `println!` is already holding the `WRITER` lock and is waiting for the GPU callback to finish.

### Best Practice: `serial_println!`
Always use `levitate_hal::serial_println!` (UART-only) for:
1. **Interrupt Handlers**: Standard `println!` is unsafe in IRQs due to dual-console complexity.
2. **Low-level Drivers**: Components like `gpu.rs` and `terminal.rs` should never use `println!`.
3. **Early Boot**: Before Dual Console is registered.

---

## 2. Safe Interrupt Handling

### IrqSafeLock
Any data structure shared between a thread and an interrupt handler (like `RX_BUFFER` or `GPU_TERMINAL`) **must** use `IrqSafeLock`. Standard `Spinlock` will deadlock if an interrupt occurs while the thread holds the lock.

### Verification Techniques
If the system feels unresponsive, use these "Heartbeat" diagnostics:
1. **IRQ Heartbeat**: Add `serial_println!("T")` in `TimerHandler`. If you don't see `T`s, interrupts are disabled at the CPU level or the GIC is misconfigured.
2. **UART Status**: Read the Flag Register (FR) at `UART_VA + 0x18`.
   - Bit 4 (`RXFE`): If this stays `1` while you type, the hardware is not receiving data.
   - Bit 5 (`TXFF`): If this is `1`, the UART is backed up.

---

## 3. GPU Rendering Performance

### The Flush Penalty
GPU memory flushes are extremely expensive.
- **Rule**: Never flush per character.
- **Pattern**: Perform all drawing logic first, then call `flush()` once at the end of the operation (e.g., at the end of `write_str` in `console_gpu.rs`).

---

---

## 5. Task Lifecycle & Transitions (TEAM_120)

### The Trampoline Pattern
When creating a new task, it must start in a kernel-controlled "trampoline" to ensure proper CPU state initialization (like `post_switch_hook`).

**Pattern:**
1. **`task_entry_trampoline`** (ASM): Calls `post_switch_hook`, then jumps to `x19`.
2. **`user_task_entry_wrapper`** (Rust): The function in `x19`. Configures `TTBR0` (user page tables) and calls `enter_user_mode`.

### `From<UserTask>` for `TaskControlBlock`
Always provide a conversion from high-level `UserTask` (which has ELF info) to the scheduler's `TaskControlBlock`. This ensures the TCB's `context` is correctly initialized with the trampoline and entry wrapper.

---

## 6. Global Resource Lifetime (TEAM_120)

### `'static` & `Copy/Clone` for Archives
Structures like `CpioArchive` that wrap kernel slices (e.g., initramfs) and are stored in global `static` variables **must**:
1. Use the `'static` lifetime if possible.
2. Implement/Derive `Copy` and `Clone`.

**Why**: This allows the structure to be moved out of a `Spinlock` or `IrqSafeLock` via copying, preventing the lock from being held during long operations (like userspace execution) which would otherwise cause a deadlock.

---

## 7. Multi-Boot & Page Table Transitions (TEAM_285)

### Safe Page Table Switching
Never perform an immediate `mov cr3` in assembly during the higher-half transition if you can avoid it. 
- **Pattern**: Stay on the bootloader's page tables (which guaranteed the jump to the kernel worked) until you are inside Rust code.
- **Benefit**: You can use Rust's type system and verified mapping logic to initialize your own tables before switching. This prevents silent hangs caused by slightly inconsistent mappings between the bootloader and the kernel.

### Dynamic Physical Memory Offset (PMO)
Limine and other modern bootloaders do not guarantee a fixed higher-half direct map (HHDM) offset.
- **Rule**: `PHYS_OFFSET` must be a dynamic variable, not a constant. 
- **Pattern**: Initialize it as early as possible in `kernel_main_unified` from `BootInfo`.

## 8. Recommended Libraries (The "Easy Life" List)

To improve reliability and reduce manual assembly/bit-shifting, future teams should prioritize these industry-standard crates:

| Category | Recommended Crate | Purpose |
| :--- | :--- | :--- |
| **Serial** | `uart_16550` | Robust COM1/COM2 handling. |
| **Interrupts** | `pic8259` | Standard PC PIC handling (if not using APIC). |
| **Input** | `pc-keyboard` | PS/2 scancode to KeyCode translation. |
| **CPU** | `raw-cpuid` | Feature detection without manual `asm!`. |
| **Paging** | `x86_64` (Full) | Use `structures::paging` instead of manual bit-masking. |
| **Memory** | `buddy_system_allocator` | Often more performant than linked lists for kernel heaps. |

## 9. Team Attribution
When documenting fixes or gotchas in code, always include your Team ID:
```rust
// TEAM_XXX: Reason for change
```
