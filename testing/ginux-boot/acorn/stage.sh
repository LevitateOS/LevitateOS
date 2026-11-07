#!/bin/sh
# Stage an AcornOS rootfs for `ginux native`, as root: stage.sh ROOT GINUX_DIR.
# The generic parts mirror Ginux's prepare_root
# (scripts/ginux-host-instance-helper.sh), which is systemd-specific; the init
# setup is AcornOS's own (busybox init + the inittab that
# distro/builder/src/pipeline/overlay.rs writes), with the probe in place of
# the getty.
set -eu
R=$1
HERE=$(dirname "$0")

mkdir -p "$R/run" "$R/proc/1" "$R/proc/self/fd" "$R/proc/sys/fs" \
    "$R/proc/sys/kernel/random" "$R/proc/pressure" "$R/sys/kernel/security" \
    "$R/sys/fs/cgroup" "$R/dev/shm" "$R/dev/pts" "$R/tmp" "$R/usr/local/sbin"
chmod 1777 "$R/tmp"
touch "$R/etc/resolv.conf"
printf '%s\000' /sbin/init >"$R/proc/1/cmdline"
printf '0::/\n' >"$R/proc/1/cgroup"
printf '0::/\n' >"$R/proc/self/cgroup"
printf 'cpu  1 0 1 100 0 0 0 0 0 0\nbtime 1\nprocesses 1\nprocs_running 1\nprocs_blocked 0\n' >"$R/proc/stat"
printf '1.00 0.00\n' >"$R/proc/uptime"
printf 'MemTotal:        1048576 kB\nMemFree:          524288 kB\nMemAvailable:     524288 kB\n' >"$R/proc/meminfo"
printf 'none / proc rw,nosuid,nodev,noexec,relatime 0 0\n' >"$R/proc/mounts"
printf '0.00 0.00 0.00 1/1 1\n' >"$R/proc/loadavg"
: >"$R/proc/cpuinfo"
# OpenRC reads /proc/filesystems; Gate 9's staging never needed it.
printf 'nodev\tproc\nnodev\ttmpfs\nnodev\tdevpts\n\text4\n' >"$R/proc/filesystems"
printf 'Ginux userspace kernel 0.1.0 (Linux ABI personality 6.12.0-ginux-abi1; x86_64)\n' >"$R/proc/version"
printf '1048576\n' >"$R/proc/sys/fs/nr_open"
printf '40\n' >"$R/proc/sys/kernel/cap_last_cap"
printf 'acorn\n' >"$R/proc/sys/kernel/hostname"
printf 'Linux\n' >"$R/proc/sys/kernel/ostype"
printf '6.12.0-ginux-abi1\n' >"$R/proc/sys/kernel/osrelease"
printf '4e45c53e-0e6a-4fb8-9f25-e0f11b40db9a\n' >"$R/proc/sys/kernel/random/boot_id"
: >"$R/sys/kernel/security/lsm"
printf 'cpu memory pids\n' >"$R/sys/fs/cgroup/cgroup.controllers"
touch "$R/sys/fs/cgroup/cgroup.procs" "$R/sys/fs/cgroup/cgroup.subtree_control"
cp --remove-destination /usr/share/zoneinfo/UTC "$R/etc/localtime"
for spec in 'console 5 0 600' 'null 1 3 666' 'zero 1 5 666' 'random 1 8 666' 'urandom 1 9 666'; do
    set -- $spec
    [ -e "$R/dev/$1" ] || mknod -m "$4" "$R/dev/$1" c "$2" "$3"
done

cat >"$R/etc/inittab" <<'EOF'
::sysinit:/sbin/openrc sysinit
::sysinit:/sbin/openrc boot
::wait:/sbin/openrc default
::once:/usr/local/sbin/ginux-probe
::ctrlaltdel:/sbin/reboot
::shutdown:/sbin/openrc shutdown
EOF
ln -sf /etc/init.d/networking "$R/etc/runlevels/boot/networking"
ln -sf /etc/init.d/sshd "$R/etc/runlevels/default/sshd"
ln -sf /etc/init.d/dhcpcd "$R/etc/runlevels/default/dhcpcd"
printf 'acorn\n' >"$R/etc/hostname"
install -m 755 "$HERE/probe.sh" "$R/usr/local/sbin/ginux-probe"

# Ginux runs root-launched workloads as the host nobody identity; its own
# prepare_root chowns the root the same way.
chown -R 65534:65534 "$R"
find "$R" -type d -exec chmod u+w {} +
chmod 600 "$R/etc/shadow"
