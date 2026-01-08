#!/bin/bash
set -e

echo "BUILDING KERNEL..."
cargo build --package levitate-kernel --target x86_64-unknown-none --release

echo "BUILDING SHELL..."
cargo build --manifest-path userspace/Cargo.toml --target x86_64-unknown-none --release


echo "CHECKING FOR SYSCALL LOGS..."
# They should be stripped unless verbose-syscalls is enabled
if grep -r "SYSCALL" target/x86_64-unknown-none/release/levitate-kernel | grep -v "MSRs initialized"; then
    echo "WARNING: SYSCALL strings found in binary (might be normal if panic strings included)"
else
    echo "SYSCALL logs appear stripped."
fi

# We can't easily run QEMU and interact in this script without expect.
# But we can check if it builds.
echo "BUILD SUCCESS."
