#!/usr/bin/bash
set -euo pipefail
# Reject unrelated/fork jobs before their checkout or commands run.
[[ ${GITHUB_REPOSITORY:-} == LevitateOS/LevitateOS ]]
[[ ${GITHUB_REF:-} == refs/heads/master ]]
[[ ${GITHUB_EVENT_NAME:-} == workflow_dispatch || ${GITHUB_EVENT_NAME:-} == schedule ]]
case ${GITHUB_WORKFLOW_REF:-} in
    LevitateOS/LevitateOS/.github/workflows/recab-updates.yml@refs/heads/master | \
    LevitateOS/LevitateOS/.github/workflows/recab-update-monitor.yml@refs/heads/master) ;;
    *) echo 'This runner accepts only the default-branch P14 workflows.' >&2; exit 1 ;;
esac
