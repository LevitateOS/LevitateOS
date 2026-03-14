#!/bin/sh
set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
REPO_ROOT="$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)"
OUTPUT_DIR="${REPO_ROOT}/.artifacts/out/iuppiter"
KERNEL_OUTPUT_DIR="${KERNEL_OUTPUT_DIR:-${OUTPUT_DIR}}"
COMPAT_BUILD_STAGE_DIRNAME="${COMPAT_BUILD_STAGE_DIRNAME:-s02-live-tools}"
BUILD_STAGE_DIRNAME="${BUILD_STAGE_DIRNAME:-${COMPAT_BUILD_STAGE_DIRNAME}}"
RUN_OUTPUT_DIR="${RUN_OUTPUT_DIR:-${STAGE_OUTPUT_DIR:-${KERNEL_OUTPUT_DIR}/${COMPAT_BUILD_STAGE_DIRNAME}}}"
STAGE_OUTPUT_DIR="${STAGE_OUTPUT_DIR:-${RUN_OUTPUT_DIR}}"

case "${PRODUCT_NAME:-}" in
  live-tools) default_iso_filename="iuppiter-x86_64-live-tools.iso" ;;
  *) default_iso_filename="iuppiter-x86_64-s02_live_tools.iso" ;;
esac
ISO_PATH="${ISO_PATH:-${RUN_OUTPUT_DIR}/${default_iso_filename}}"
export ISO_PATH

exec "${SCRIPT_DIR}/00Build-build.sh"
