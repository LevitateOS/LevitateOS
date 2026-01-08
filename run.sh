#!/bin/bash
# run.sh - Canonical LevitateOS Launcher
# Wraps 'cargo xtask run' to provide a unified entry point.
#
# Usage:
#   ./run.sh              # Run in GUI mode (Default)
#   ./run.sh term         # Run in Terminal mode
#   ./run.sh --iso        # Force ISO boot (x86_64)
#   ./run.sh clean        # Clean artifacts
#
# This script delegates to the Rust build system (xtask) which handles
# compiling, image creation, and QEMU invocation correctly.

set -e

# Forward 'clean' to xtask clean
if [ "$1" = "clean" ]; then
    exec cargo xtask clean
    exit 0
fi

# Heuristic: If first arg matches a known subcommand, pass it through.
# Otherwise, default to 'default' (GUI) mode and pass args as flags.
case "$1" in
    default|term|test|gdb|pixel6|vnc)
        # Explicit subcommand provided
        exec cargo xtask run "$@"
        ;;
    *)
        # No subcommand, assume default (GUI)
        # Pass all arguments to 'default' command
        # Example: ./run.sh --iso -> cargo xtask run default --iso
        exec cargo xtask run default "$@"
        ;;
esac
