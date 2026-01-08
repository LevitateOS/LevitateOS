# I/O Compartment

This compartment handles input/output devices for x86_64.

## Files

| File | Description |
|------|-------------|
| `serial.rs` | COM1 serial port driver |
| `vga.rs` | VGA text mode buffer |
| `console.rs` | Unified console writer abstraction |

## Serial Port (`serial.rs`)

The serial port provides early debug output before other subsystems are ready.

### COM Port Addresses

| Port | I/O Base | IRQ |
|------|----------|-----|
| COM1 | `0x3F8` | 4 |
| COM2 | `0x2F8` | 3 |
| COM3 | `0x3E8` | 4 |
| COM4 | `0x2E8` | 3 |

**LevitateOS uses COM1 (0x3F8)** for debug output.

### Register Layout

```
Base + 0: Data Register (R/W)
Base + 1: Interrupt Enable Register
Base + 2: FIFO Control Register
Base + 3: Line Control Register
Base + 4: Modem Control Register
Base + 5: Line Status Register
Base + 6: Modem Status Register
Base + 7: Scratch Register
```

### Line Status Register (Base + 5)

```mermaid
flowchart LR
    subgraph LSR Bits
        B0[Bit 0<br/>Data Ready]
        B1[Bit 1<br/>Overrun Error]
        B5[Bit 5<br/>TX Empty]
        B6[Bit 6<br/>TX Idle]
    end
```

### Initialization Sequence

```mermaid
sequenceDiagram
    participant Driver
    participant UART
    
    Driver->>UART: Disable interrupts (IER = 0)
    Driver->>UART: Enable DLAB (LCR bit 7)
    Driver->>UART: Set baud divisor (38400)
    Driver->>UART: 8N1 format (LCR = 0x03)
    Driver->>UART: Enable FIFO (FCR = 0xC7)
    Driver->>UART: Enable RTS/DSR (MCR = 0x0B)
    Note over Driver,UART: Serial port ready
```

### Baud Rate

```
Divisor = 115200 / desired_baud
For 38400 baud: divisor = 3
```

### API

```rust
impl SerialPort {
    /// Create a new serial port
    pub const fn new(base_port: u16) -> Self
    
    /// Initialize the serial port
    pub unsafe fn init(&self)
    
    /// Send a byte (blocking)
    pub fn send(&self, data: u8)
    
    /// Receive a byte (blocking)
    pub fn receive(&self) -> u8
    
    /// Try to read a byte (non-blocking)
    pub fn read_byte(&self) -> Option<u8>
}

// Implements fmt::Write for print! macros
impl fmt::Write for SerialPort
```

## VGA Text Mode (`vga.rs`)

VGA text mode provides 80x25 character display.

### Memory Layout

```
Physical: 0xB8000 - 0xB8FA0 (4000 bytes)
Virtual:  phys_to_virt(0xB8000)
```

### Character Format

Each character cell is 2 bytes:

```
┌─────────────────┬─────────────────┐
│ Byte 0 (even)   │ Byte 1 (odd)    │
├─────────────────┼─────────────────┤
│ ASCII Character │ Attribute       │
└─────────────────┴─────────────────┘

Attribute byte:
┌───┬───┬───┬───┬───┬───┬───┬───┐
│ 7 │ 6 │ 5 │ 4 │ 3 │ 2 │ 1 │ 0 │
├───┴───┴───┴───┼───┴───┴───┴───┤
│  Background   │  Foreground   │
└───────────────┴───────────────┘
```

### Colors

```mermaid
flowchart LR
    subgraph Standard Colors 0-7
        C0[0 Black]
        C1[1 Blue]
        C2[2 Green]
        C3[3 Cyan]
        C4[4 Red]
        C5[5 Magenta]
        C6[6 Brown]
        C7[7 Light Gray]
    end
    
    subgraph Bright Colors 8-15
        C8[8 Dark Gray]
        C9[9 Light Blue]
        CA[10 Light Green]
        CB[11 Light Cyan]
        CC[12 Light Red]
        CD[13 Pink]
        CE[14 Yellow]
        CF[15 White]
    end
```

### Screen Dimensions

| Property | Value |
|----------|-------|
| Width | 80 columns |
| Height | 25 rows |
| Total cells | 2000 |
| Buffer size | 4000 bytes |

### VgaWriter

```rust
pub struct VgaWriter {
    column_position: usize,
    color_code: ColorCode,
}

impl VgaWriter {
    /// Write a single byte
    pub fn write_byte(&mut self, byte: u8)
    
    /// Write a string
    pub fn write_string(&mut self, s: &str)
}
```

### Scrolling

When the cursor reaches the end of the last line:

```mermaid
flowchart TD
    A[Cursor at column 80] --> B[New line]
    B --> C[Copy rows 1-24 to 0-23]
    C --> D[Clear row 24]
    D --> E[Reset column to 0]
```

## Console (`console.rs`)

The console provides a unified, IRQ-safe interface for output.

### Static Writer

```rust
pub static WRITER: IrqSafeLock<SerialPort> = 
    IrqSafeLock::new(SerialPort::new(COM1));
```

### Why IrqSafeLock?

```mermaid
sequenceDiagram
    participant Kernel
    participant Lock as Mutex
    participant IRQ
    
    Kernel->>Lock: Acquire lock
    Note over Kernel,Lock: Printing...
    IRQ->>Kernel: Interrupt!
    IRQ->>Lock: Try to print
    Note over Lock: DEADLOCK!
    
    Note over Kernel,IRQ: With IrqSafeLock:
    Kernel->>Lock: Disable IRQ + Acquire
    Note over Kernel,Lock: Printing (IRQs off)
    Kernel->>Lock: Release + Restore IRQ
```

### Usage

```rust
// Via los_hal macros
los_hal::println!("Hello, world!");

// Direct access
{
    let mut writer = WRITER.lock();
    write!(writer, "Value: {}", 42).unwrap();
}
```

## Output Flow

```mermaid
flowchart TD
    subgraph Application
        PRINT[println! macro]
    end
    
    subgraph Console
        WRITER[WRITER static<br/>IrqSafeLock]
    end
    
    subgraph Serial
        SERIAL[SerialPort<br/>COM1 0x3F8]
        TX[TX Buffer]
        UART[UART Hardware]
    end
    
    subgraph Optional
        VGA[VGA Buffer<br/>0xB8000]
        SCREEN[Monitor]
    end
    
    PRINT --> WRITER
    WRITER --> SERIAL
    SERIAL --> TX
    TX --> UART
    UART --> |RS-232| HOST[Host Terminal<br/>QEMU -serial stdio]
    
    WRITER -.-> VGA
    VGA -.-> SCREEN
```

## Diagnostic Output

Early boot uses direct port I/O for diagnostics:

```rust
// Direct serial output (before console init)
unsafe {
    core::arch::asm!(
        "mov dx, 0x3f8",
        "mov al, 'X'",
        "out dx, al",
        out("ax") _,
        out("dx") _
    );
}
```

This is used in `boot.S` and early HAL init to trace boot progress.
