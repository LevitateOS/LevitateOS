# TEAM_160: Component System Migration Strategy

**Date:** 2026-02-10
**Objective:** Migrate leviso components to use distro-builder's Op enum, reducing component definition LoC from ~2,500 to ~400-500

## Current State

**Leviso Component System (~2,500 LoC)**
```rust
// Each component has a function like:
pub fn create_filesystem(ctx: &BuildContext) -> Result<()> {
    // Manual implementation of directory creation, symlinks, file writes
    fs::create_dir_all(ctx.staging.join("..."))?;
    std::os::unix::fs::symlink(...)?;
    fs::write(...)?;
}
```

Called from `component/builder.rs` in a fixed order.

**Distro-builder Pattern (~100 LoC per distro)**
```rust
impl Installable for MyComponent {
    fn name(&self) -> &str { "MyComponent" }
    fn phase(&self) -> Phase { Phase::Filesystem }
    fn ops(&self) -> Vec<Op> {
        vec![
            Op::Dir("etc".into()),
            Op::Symlink("var/run".into(), "/run".into()),
            Op::WriteFile("etc/hosts".into(), "127.0.0.1 localhost\n".into()),
        ]
    }
}
```

## Migration Path

###Step 1: Create Installable Wrappers (Non-Breaking)
Create thin wrapper components that implement Installable but call existing functions:

```rust
struct FilesystemComponent;

impl Installable for FilesystemComponent {
    fn name(&self) -> &str { "Filesystem" }
    fn phase(&self) -> Phase { Phase::Filesystem }
    fn ops(&self) -> Vec<Op> {
        // For now: empty vec, actual ops built by calling old function
        // This allows executor to recognize it as Installable
        vec![]
    }
}
```

Benefits:
- No breaking changes to existing code
- Allows gradual migration
- Tests continue to pass

### Step 2: Migrate Components One by One

**Order (by size/complexity):**
1. Filesystem (symlinks, dirs) — ~100 LoC → 20 LoC
2. Packages (simple recipe calls) — ~80 LoC → 15 LoC
3. Firmware (dir copies) — ~120 LoC → 25 LoC
4. Modules (config file writes) — ~90 LoC → 20 LoC
5. PAM (file writes) — ~150 LoC → 40 LoC
6. Services (more complex) — ~200+ LoC → 100+ LoC
7. Binaries (requires custom Op or wrapper) — ~300+ LoC → Handle specially

**Not migrating (too distro-specific or complex):**
- Live overlay (LevitateOS-specific)
- Binaries with library dependencies (needs custom Rocky logic)

###Step 3: Update Component Builder

Replace procedural loop with Installable trait executor:

```rust
pub fn build_system(ctx: &BuildContext) -> Result<()> {
    let components: Vec<Box<dyn Installable>> = vec![
        Box::new(FilesystemComponent),
        Box::new(PackagesComponent),
        // ...
    ];

    for component in components {
        let ops = component.ops();
        for op in ops {
            distro_builder::executor::execute_generic_op_ctx(ctx, &op)?;
        }
    }
}
```

### Step 4: Eliminate Custom Functions

Once all components are migrated to Op enum:
- Delete `component/custom/*.rs` implementation functions
- Keep only the Installable impl structs
- Delete `component/builder.rs` procedural logic
- Replace with generic Op executor loop

## Risk Mitigation

| Risk | Mitigation |
|------|-----------|
| Breaking existing code | Use wrapper structs, keep old functions temporarily |
| Tests fail | Run tests after each component |
| Checkpoint 1 breaks | Verify after each migration |
| Lost functionality | Comprehensive audit before deletion |

## Effort Estimate

| Task | LoC Change | Hours |
|------|-----------|-------|
| Create Installable wrappers | +100 | 1 |
| Migrate 5 simple components | -200 | 2 |
| Migrate services (complex) | -100 | 2 |
| Handle binary operations | -100 | 1 |
| Verify & testing | — | 1 |
| **Total** | **-200** | **7** |

## End State

```
Before: 11,200 LoC total
  - component/: 2,500 LoC
  - artifacts: 850 LoC (already delegating)
  - build/executor: 400 LoC (wrapped)
  - other: 7,450 LoC

After: 10,500 LoC total (-700)
  - component/: 1,800 LoC (-700)
  - artifacts: 850 LoC (unchanged)
  - build/executor: 400 LoC (unchanged)
  - other: 7,450 LoC (unchanged)
```

Further reductions possible:
- Eliminate build orchestration (~400 LoC) → 10,100 LoC
- Consolidate library dependencies → 9,800 LoC

Could reach 8-9k LoC with full consolidation, but 2-4k "thin wrapper" would require massive architectural change (using Alpine+OpenRC systems instead of Rocky+systemd).

## Recommendation

This migration is **feasible and safe**. Start with Filesystem component as proof of concept, then continue with others based on complexity/benefit ratio.

Once Op-based system is in place, further refactoring (build orchestration, library handling) becomes easier.
