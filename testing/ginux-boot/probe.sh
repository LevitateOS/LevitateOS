#!/bin/sh
# ginux-boot's probe, started by the init (see the OS's hook.sh):
#   ginux-probe TIMEOUT OUT
# Writes recab's boot-health verdict to OUT and recab's stderr to OUT.err, then
# powers the instance off through the init, which ends `ginux native`.

# The one place the verdict is asked for. NOTE: recab gains `health` with fix
# G19 (its output is specified in G31's progress entry). Until then recab
# rejects the subcommand, and ginux-boot.sh reports that error as the FAIL.
health() {
    recab health --json --timeout "$1"
}

health "$1" >"$2" 2>"$2.err"
poweroff
