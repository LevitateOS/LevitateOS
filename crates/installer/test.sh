#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR/python"

echo "=== Running installer LLM tests ==="
python3 -m pytest tests/ -v --tb=short "$@"
