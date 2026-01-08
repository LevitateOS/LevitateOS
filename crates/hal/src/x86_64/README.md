# x86_64 Hardware Abstraction Layer

This directory contains the x86_64-specific HAL implementation, organized into logical compartments.

## Directory Structure

```
x86_64/
├── cpu/           # CPU structures (GDT, TSS, IDT, exceptions)
├── mem/           # Memory management (paging, MMU, frame allocator)
├── interrupts/    # Interrupt handling (APIC, IOAPIC, PIT)
├── io/            # I/O devices (serial, VGA, console)
├── boot/          # Boot protocols (Multiboot2)
└── mod.rs         # Main module with init functions
```

## Boot Flow

The following diagram shows the x86_64 boot sequence from BIOS/bootloader to kernel:

```mermaid
flowchart TD
    subgraph BIOS/Bootloader
        A[BIOS POST] --> B[GRUB/Multiboot2]
        B --> C[Load kernel at 0x200000]
    end

    subgraph boot.S Assembly
        C --> D[_start: Setup stack]
        D --> E[Setup GDT 32-bit]
        E --> F[Enable Long Mode]
        F --> G[Setup early page tables]
        G --> H[Jump to 64-bit]
        H --> I[Call kernel_main]
    end

    subgraph HAL Init
        I --> J[init_with_options]
        J --> K{switch_cr3?}
        K -->|Yes Multiboot| L[init_kernel_mappings]
        K -->|No Limine| M[Skip - use Limine tables]
        L --> N[Switch CR3]
        N --> O[Serial init]
        M --> O
        O --> P[GDT/IDT init]
        P --> Q[Exceptions init]
        Q --> R[HAL Ready]
    end

    subgraph Kernel Main
        R --> S[Console init]
        S --> T[Memory init]
        T --> U[Task init]
        U --> V[init::run]
    end
```

## Memory Layout

```mermaid
flowchart LR
    subgraph Physical Memory
        PA[0x0 - 0x100000<br/>Legacy BIOS]
        PB[0x200000<br/>Kernel Load]
        PC[0x800000 - 0x1000000<br/>Early Frame Pool]
        PD[0xB8000<br/>VGA Buffer]
        PE[0xFEC00000<br/>IOAPIC]
        PF[0xFEE00000<br/>Local APIC]
    end

    subgraph Virtual Memory
        VA[0xFFFF800000000000<br/>PMO - Physical Memory Offset]
        VB[0xFFFFFFFF80000000<br/>Kernel Higher-Half]
        VC[Identity Map<br/>First 1MB]
    end

    PA -.-> VC
    PB -.-> VB
    PA -.-> VA
```

## Compartment Details

### cpu/ - CPU Structures

```mermaid
classDiagram
    class GDT {
        +null: u64
        +kernel_code: u64
        +kernel_data: u64
        +user_data: u64
        +user_code: u64
        +tss: GdtTssEntry
        +init()
    }

    class TSS {
        +rsp0: u64
        +ist[7]: u64
        +set_kernel_stack()
    }

    class IDT {
        +entries[256]: IdtEntry
        +set_handler()
        +load()
    }

    class Exceptions {
        +page_fault_handler()
        +gp_fault_handler()
        +double_fault_handler()
        +irq_dispatch()
    }

    GDT --> TSS : contains
    IDT --> Exceptions : routes to
```

### mem/ - Memory Management

```mermaid
flowchart TB
    subgraph 4-Level Paging
        PML4[PML4 - Level 4<br/>512 entries]
        PDPT[PDPT - Level 3<br/>512 entries]
        PD[PD - Level 2<br/>512 entries]
        PT[PT - Level 1<br/>512 entries]
        PAGE[4KB Page Frame]

        PML4 --> PDPT
        PDPT --> PD
        PD --> PT
        PT --> PAGE
    end

    subgraph MMU Functions
        MAP[map_page]
        UNMAP[unmap_page]
        VIRT[virt_to_phys]
        PHYS[phys_to_virt]
    end

    subgraph Frame Allocator
        EARLY[EarlyFrameAllocator<br/>Bump allocator<br/>0x800000 - 0x1000000]
    end

    MAP --> PT
    EARLY --> MAP
```

### interrupts/ - Interrupt Handling

```mermaid
flowchart LR
    subgraph Hardware
        TIMER[PIT Timer<br/>IRQ 0]
        SERIAL[COM1<br/>IRQ 4]
        DEVICES[Other Devices]
    end

    subgraph IOAPIC
        ROUTE[Route IRQs<br/>to vectors]
    end

    subgraph LAPIC
        VECTOR[Deliver to CPU]
        EOI[Signal EOI]
    end

    subgraph CPU
        IDT2[IDT Lookup]
        HANDLER[Exception/IRQ Handler]
    end

    TIMER --> ROUTE
    SERIAL --> ROUTE
    DEVICES --> ROUTE
    ROUTE --> VECTOR
    VECTOR --> IDT2
    IDT2 --> HANDLER
    HANDLER --> EOI
```

### io/ - I/O Devices

```mermaid
flowchart TB
    subgraph Console Output
        PRINT[println! macro]
        WRITER[WRITER static]
        SERIAL[SerialPort<br/>COM1 0x3F8]
        VGA[VGA Buffer<br/>0xB8000]
    end

    PRINT --> WRITER
    WRITER --> SERIAL
    WRITER -.-> VGA

    subgraph Serial Port
        TX[Transmit byte]
        INIT[Initialize 115200 baud]
    end

    SERIAL --> TX
    SERIAL --> INIT
```

## Key Constants

| Constant | Value | Description |
|----------|-------|-------------|
| `KERNEL_VIRT_BASE` | `0xFFFFFFFF80000000` | Kernel higher-half base |
| `PHYS_OFFSET` | `0xFFFF800000000000` | Physical memory offset (PMO) |
| `PAGE_SIZE` | `0x1000` (4KB) | Standard page size |
| `HUGE_PAGE_SIZE` | `0x200000` (2MB) | Huge page size |
| `APIC_BASE` | `0xFEE00000` | Local APIC address |
| `IOAPIC_BASE` | `0xFEC00000` | I/O APIC address |

## Known Issues (TEAM_316)

1. **PMO Limited to 1GB** - The early page tables only map the first 1GB via PMO.
   APIC/IOAPIC addresses (~4GB) are outside this range, causing `phys_to_virt()` to
   return unmapped addresses.

2. **APIC Init Skipped** - Currently using legacy PIC mode because APIC access
   via `phys_to_virt()` crashes.

3. **Crash at 0x800200188** - Unresolved crash during TaskControlBlock creation.
   Needs binary-level analysis.

## Files

| File | Description |
|------|-------------|
| `cpu/gdt.rs` | Global Descriptor Table and TSS |
| `cpu/idt.rs` | Interrupt Descriptor Table |
| `cpu/exceptions.rs` | Exception handlers and IRQ dispatch |
| `mem/paging.rs` | Page table structures and operations |
| `mem/mmu.rs` | Memory mapping, address translation |
| `mem/frame_alloc.rs` | Early bump allocator for page frames |
| `interrupts/apic.rs` | Local APIC controller |
| `interrupts/ioapic.rs` | I/O APIC for external interrupts |
| `interrupts/pit.rs` | Programmable Interval Timer |
| `interrupts/state.rs` | Interrupt enable/disable/restore |
| `io/serial.rs` | COM1 serial port driver |
| `io/vga.rs` | VGA text mode buffer |
| `io/console.rs` | Console writer abstraction |
| `boot/multiboot2.rs` | Multiboot2 boot info parsing |
