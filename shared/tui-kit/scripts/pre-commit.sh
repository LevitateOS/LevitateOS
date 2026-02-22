#!/bin/sh
set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
PKG_ROOT="$(CDPATH= cd -- "${SCRIPT_DIR}/.." && pwd)"

cd "${PKG_ROOT}"

echo "shared/tui-kit pre-commit: format check"
bun run format:check

echo "shared/tui-kit pre-commit: lint"
bun run lint

echo "shared/tui-kit pre-commit: typecheck"
bun run typecheck

echo "shared/tui-kit pre-commit: tests"
bun run test
