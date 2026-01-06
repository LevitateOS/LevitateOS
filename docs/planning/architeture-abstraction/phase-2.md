# Phase 2: Structural Extraction - Arch Abstraction

## Target Design
New directory structure in `kernel/src`:
- `arch/mod.rs`: Architecture-independent interface.
- `arch/aarch64/mod.rs`: AArch64 implementation entry point.
- `arch/aarch64/boot.rs`: Migrated startup code.
- `arch/aarch64/exceptions.rs`: Migrated vector tables.
- `arch/aarch64/task.rs`: Migrated context switching.

## Extraction Strategy
1. Create `src/arch` and `src/arch/aarch64`.
2. Move `Context` and `SyscallFrame` to `arch/aarch64`.
3. Create generic traits/interfaces in `arch/mod.rs`.
4. Migrate assembly blocks to architecture-specific files.

## Steps
1. **Step 1: Define New Module Boundaries**
2. **Step 2: Extract Types and Interfaces**
3. **Step 3: Introduce New APIs**
