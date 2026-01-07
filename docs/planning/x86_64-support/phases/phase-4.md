# Phase 4: Integration and Testing — x86_64 Support

## Integration Strategy

Once the kernel reaches `kernel_main` and has basic HAL support, we must ensure it integrates with the rest of the OS.

### 1. VFS & Initramfs
- [ ] Ensure `cpio` parser works on x86_64 (endianness/alignment).
- [ ] Verify `tar` or `initramfs` can be located via Multiboot2 tags (instead of FDT).

### 2. Userspace Compatibility
- [ ] Build `libsyscall` for x86_64.
- [ ] Verify `syscall` instruction vs AArch64 `svc`.
- [ ] Port `init` and `levbox` to x86_64.

### 3. Testing
- [ ] Add x86_64 behavior tests to `tests/`.
- [ ] Implement `xtask test --arch x86_64`.
- [ ] Verify regression protection (Rule 4).

## Regression Protection
- [ ] Existing AArch64 tests must continue to pass.
- [ ] CI should run both AArch64 and x86_64 builds.
