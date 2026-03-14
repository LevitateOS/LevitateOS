#!/bin/sh
set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
REPO_ROOT="$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)"
export DISTRO_ID="ralph"
export PRODUCT_NAME="${PRODUCT_NAME:-live-tools}"
export BUILD_TARGET_LABEL="${BUILD_TARGET_LABEL:-live-tools}"
export BUILD_STAGE_DIRNAME="${BUILD_STAGE_DIRNAME:-live-tools}"
exec "${REPO_ROOT}/distro-variants/_shared/build-release.sh"
