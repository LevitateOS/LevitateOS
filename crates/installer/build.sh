#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

echo "=== Installing Python dependencies ==="
pip install -q -r python/requirements.txt

echo "=== Building Rust installer ==="
cargo build --release

echo "=== Done ==="
echo "Run with: cargo run --release -p levitate-installer"
