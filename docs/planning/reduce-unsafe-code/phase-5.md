# Phase 5 — Cleanup, Regression Protection, and Handoff

**TEAM_131 → TEAM_132** | Reduce Unsafe Code via Safe Abstractions

## Cleanup Tasks

### 1. Remove Obsolete SAFETY Comments

After migration, some SAFETY comments will be redundant (the unsafe is now internal to the wrapper). Review and clean up.

### 2. Update Documentation

- Add module-level docs to new abstraction modules
- Update `ARCHITECTURE.md` with new abstraction layers
- Document the "unsafe budget" pattern in kernel-development.md

### 3. Lint Configuration

Add clippy configuration to enforce new patterns:

```toml
# In .cargo/config.toml or Cargo.toml
[lints.clippy]
# Warn on raw volatile usage outside of wrapper modules
ptr_read_write_volatile = "warn"
```

---

## Regression Protection

### CI Check: Unsafe Count

Add a CI step that fails if unsafe count increases beyond threshold:

```bash
#!/bin/bash
# scripts/check_unsafe_count.sh
MAX_UNSAFE=60  # Target after refactor
ACTUAL=$(grep -rn "unsafe {" kernel/ levitate-hal/ levitate-virtio/ --include="*.rs" | wc -l)

if [ "$ACTUAL" -gt "$MAX_UNSAFE" ]; then
    echo "ERROR: Unsafe count $ACTUAL exceeds limit $MAX_UNSAFE"
    exit 1
fi
echo "Unsafe count: $ACTUAL (limit: $MAX_UNSAFE)"
```

### Documentation: Unsafe Budget

Each crate should document its "unsafe budget":

```rust
//! # Unsafe Budget
//!
//! This crate contains N unsafe blocks:
//! - 2 in `volatile.rs` (core volatile operations)
//! - 1 in `barrier.rs` (memory barriers)
//!
//! All other unsafe has been encapsulated in safe wrappers.
```

---

## Handoff Checklist

- [ ] All 4 abstraction modules implemented
- [ ] All call sites migrated
- [ ] Unsafe count reduced to <60 (from 148)
- [ ] All tests passing
- [ ] Documentation updated
- [ ] CI check added
- [ ] Team file updated with completion status

---

## Success Metrics

| Metric | Before | Target | Actual |
|--------|--------|--------|--------|
| Total unsafe blocks | 148 | <60 | TBD |
| Unsafe in queue.rs | 11 | 0 | TBD |
| Unsafe in gic.rs | 36 | <5 | TBD |
| Unsafe in mmu.rs | 21 | <5 | TBD |

---

## Future Work

After this refactor, consider:

1. **Type-safe MMIO regions** — Use phantom types to distinguish different device registers
2. **Formal verification** — Apply tools like `kani` to verify wrapper soundness
3. **Architecture abstraction** — Add x86_64 stubs for sysreg module

---

## Summary

This refactor reduces unsafe code by ~60% through 3 external crates:

1. **`aarch64-cpu`** — System registers + memory barriers + ARM intrinsics
2. **`safe-mmio`** — Volatile MMIO with aarch64 virtualization fix
3. **`intrusive-collections`** — Safe intrusive linked lists

Using battle-tested external crates instead of rolling our own ensures:
- Less code to maintain
- Proven correctness
- Community support and updates

**Expected reduction:** 147 unsafe blocks → ~60 (inherently unsafe MMU/context-switch code remains).
