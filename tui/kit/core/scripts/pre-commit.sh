#!/bin/sh
set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
PKG_ROOT="$(CDPATH= cd -- "${SCRIPT_DIR}/.." && pwd)"

cd "${PKG_ROOT}"

echo "tui/kit/core pre-commit: format check"
bun run format:check

echo "tui/kit/core pre-commit: lint"
bun run lint

echo "tui/kit/core pre-commit: typecheck"
bun run typecheck

echo "tui/kit/core pre-commit: tests"
bun run test
