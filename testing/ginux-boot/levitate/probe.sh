#!/bin/sh
# Started with multi-user.target; waits for the boot to settle. The host reads
# /ginux-probe.out from the instance root.
export SYSTEMD_PAGER=cat SYSTEMD_COLORS=0
state=$(timeout 120 systemctl is-system-running --wait 2>/dev/null)
{
    echo "state $state"
    # "unit NAME ACTIVE SUB" for every loaded unit.
    systemctl list-units --all --no-legend --plain --no-pager 2>/dev/null |
        awk '{ print "unit", $1, $3, $4 }'
} >/ginux-probe.tmp 2>&1
mv /ginux-probe.tmp /ginux-probe.out
