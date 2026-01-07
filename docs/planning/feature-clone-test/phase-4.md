# Phase 4: Testing

## Test Strategy
Since `clone_test` *is* a test, the strategy is to run it.

## Execution
1. Build userspace: `cargo xtask build userspace`
2. Run in QEMU: `cargo xtask run`
3. In shell: `clone_test`

## Success Indicators
Output should look like:
```
[clone_test] Spawning thread...
[clone_test] Parent waiting for TID X...
[child] Hello from thread!
[child] Exiting...
[clone_test] Parent woke up!
[clone_test] Shared state verification: PASS
```
