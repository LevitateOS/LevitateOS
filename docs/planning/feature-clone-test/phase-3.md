# Phase 3: Implementation

## Implementation Overview
Implement `clone_test.rs` in `levbox`.

## Steps

### Step 1: Create Binary Skeleton
- Create `userspace/levbox/src/bin/clone_test.rs`.
- Add basic `main` and imports.

### Step 2: Implement Stack and Shared State
- Define `Stack` struct with alignment.
- Define `static STACK` and `static SHARED_DATA`.
- Define `static CHILD_TID`.

### Step 3: Implement Child Logic
- `child_entry()` function.
- Writes to `SHARED_DATA`.
- Prints status.
- Calls `exit()`.

### Step 4: Implement Parent Logic (Main)
- Call `clone()`.
- Check result (TID).
- `futex_wait` on `CHILD_TID`.
- Verify `SHARED_DATA`.
- Print success/failure.

## Dependencies
- `levbox/Cargo.toml`: Ensure `clone_test` is picked up (auto-discovery should work).
