#!/bin/sh
echo "=== PROBE begin"
echo "--- softlevel: $(cat /run/openrc/softlevel 2>&1)"
for d in started starting failed inactive stopping; do
    echo "--- /run/openrc/$d: $(ls /run/openrc/$d 2>&1 | tr '\n' ' ')"
done
echo "--- rc-status -a"; rc-status -a 2>&1
echo "--- rc-status --crashed"; rc-status --crashed 2>&1; echo "(exit $?)"
echo "--- ls /run"; ls /run 2>&1 | tr '\n' ' '; echo
echo "--- ps"; ps -o pid,ppid,comm 2>&1
echo "--- rc-service networking status"; rc-service networking status 2>&1
echo "--- rc-service sshd status"; rc-service sshd status 2>&1
echo "--- id: $(id)"
echo "--- /var/empty: $(stat -c '%u:%g %a' /var/empty) /etc/passwd: $(stat -c '%u:%g' /etc/passwd)"
echo "=== PROBE end"
