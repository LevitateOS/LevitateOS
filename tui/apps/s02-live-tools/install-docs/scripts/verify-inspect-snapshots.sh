#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)"
SNAP_DIR="$ROOT/tui/apps/s02-live-tools/install-docs/snapshots/inspect/120x36"
OUT_BASE="$ROOT/.artifacts/out/tui/install-docs-inspect-regression"

COLUMNS=120
ROWS=36
SECONDS=2
SLUGS=(installation-disk cli-reference faq helpers-commands helpers-overview)
EXPECTED=(
  001-installation-disk.txt
  002-cli-reference.txt
  003-faq.txt
  004-helpers-commands.txt
  005-helpers-overview.txt
)

if [ ! -d "$SNAP_DIR" ]; then
  echo "docs inspect regression: missing snapshot dir '$SNAP_DIR'" >&2
  exit 1
fi

mkdir -p "$OUT_BASE"

cargo run -p levitate-xtask -- docs inspect \
  --columns "$COLUMNS" \
  --rows "$ROWS" \
  --seconds "$SECONDS" \
  --out-dir "$OUT_BASE" \
  --slug "${SLUGS[0]}" \
  --slug "${SLUGS[1]}" \
  --slug "${SLUGS[2]}" \
  --slug "${SLUGS[3]}" \
  --slug "${SLUGS[4]}" >/dev/null

LATEST_RUN="$(ls -1dt "$OUT_BASE"/run-* 2>/dev/null | head -n 1 || true)"
if [ -z "$LATEST_RUN" ] || [ ! -d "$LATEST_RUN" ]; then
  echo "docs inspect regression: failed to locate generated run directory under '$OUT_BASE'" >&2
  exit 1
fi

for file in "${EXPECTED[@]}"; do
  if [ ! -f "$LATEST_RUN/$file" ]; then
    echo "docs inspect regression: missing generated snapshot '$LATEST_RUN/$file'" >&2
    exit 1
  fi
  if [ ! -f "$SNAP_DIR/$file" ]; then
    echo "docs inspect regression: missing baseline snapshot '$SNAP_DIR/$file'" >&2
    exit 1
  fi

  if ! diff -u "$SNAP_DIR/$file" "$LATEST_RUN/$file" >/dev/null; then
    echo "docs inspect regression: snapshot mismatch for '$file'" >&2
    echo "Compare:" >&2
    echo "  baseline:  $SNAP_DIR/$file" >&2
    echo "  generated: $LATEST_RUN/$file" >&2
    exit 1
  fi
done

echo "docs inspect regression: snapshots match (${#EXPECTED[@]} pages)"
