#!/bin/sh
# AcornOS's busybox init starts the probe: hook.sh ROOT PROBE_COMMAND.
# busybox init runs `::once` entries after every `::wait` one, so after the
# product inittab's OpenRC default runlevel.
set -eu
printf '::once:%s\n' "$2" >>"$1/etc/inittab"
