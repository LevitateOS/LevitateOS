#!/bin/sh
# LevitateOS's systemd starts the probe: hook.sh ROOT PROBE_COMMAND.
# The unit is enabled by the link written here; nothing runs `systemctl
# enable`, so it has no [Install] section.
set -eu
units=$1/etc/systemd/system
cat >"$units/ginux-probe.service" <<EOF
[Unit]
Description=ginux-boot probe
After=multi-user.target

[Service]
# exec, not oneshot: the probe waits for startup to finish, and a oneshot
# start job would keep the system "starting" until the probe exits.
Type=exec
ExecStart=$2
EOF
mkdir -p "$units/multi-user.target.wants"
ln -sf ../ginux-probe.service "$units/multi-user.target.wants/ginux-probe.service"
