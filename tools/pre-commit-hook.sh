#!/bin/sh
# Optional local whitespace check. Run build/lint/runtime checks explicitly.
set -eu
exec git diff --cached --check
