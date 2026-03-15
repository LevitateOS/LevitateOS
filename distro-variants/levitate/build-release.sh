#!/bin/sh
set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
REPO_ROOT="$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)"

export DISTRO_ID="levitate"
export PRODUCT_NAME="${PRODUCT_NAME:-base-rootfs}"
exec "${REPO_ROOT}/distro-variants/_shared/build-release.sh"
