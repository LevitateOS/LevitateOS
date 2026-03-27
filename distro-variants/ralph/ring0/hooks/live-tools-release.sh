#!/bin/sh
set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
REPO_ROOT="$(CDPATH= cd -- "$SCRIPT_DIR/../../../.." && pwd)"
export DISTRO_ID="ralph"
export PRODUCT_NAME="${PRODUCT_NAME:-live-tools}"
exec "${REPO_ROOT}/distro-variants/_shared/ring0/hooks/build-release.sh"
