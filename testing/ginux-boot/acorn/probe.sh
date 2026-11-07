#!/bin/sh
# Runs once OpenRC's default runlevel is done (inittab ::once). The host reads
# /ginux-probe.out from the instance root.
{
    echo "softlevel $(cat /run/openrc/softlevel 2>/dev/null)"
    # " name   [  status  ]" for every service in every runlevel.
    rc-status -a 2>/dev/null | awk '/^(Dynamic )?Runlevel:/ { next }
        NF { gsub(/[][]/, ""); print "service", $1, $NF }'
} >/ginux-probe.tmp 2>&1
mv /ginux-probe.tmp /ginux-probe.out
