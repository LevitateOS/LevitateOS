#!/bin/sh
# Stage a LevitateOS rootfs for `ginux native`, as root: stage.sh ROOT GINUX_DIR.
# systemd staging is Ginux's own (prepare_root in
# scripts/ginux-host-instance-helper.sh, as for Gate 9); this adds the probe.
set -eu
R=$1
GINUX_DIR=$2
HERE=$(dirname "$0")

install -D -m 755 "$HERE/probe.sh" "$R/usr/local/sbin/ginux-probe"
install -D -m 644 "$HERE/ginux-probe.service" "$R/etc/systemd/system/ginux-probe.service"
mkdir -p "$R/etc/systemd/system/multi-user.target.wants"
ln -sf /etc/systemd/system/ginux-probe.service \
    "$R/etc/systemd/system/multi-user.target.wants/ginux-probe.service"
sh "$GINUX_DIR/scripts/ginux-host-instance-helper.sh" prepare-root "$R"
