#!/bin/sh
# Stage the AcornOS test root for `ginux native`, as root. Mirrors the generic
# parts of Ginux's prepare_root (scripts/ginux-host-instance-helper.sh) and
# AcornOS's own init setup (busybox init + inittab, as the builder writes it).
set -eu
SRC=$1      # the apk.static-built AcornOS root
R=$2        # the instance root to create
INIT=${3:-busybox}

rm -rf "${R:?}"
mkdir -p "$R"
cp -a "$SRC/." "$R/"

mkdir -p "$R/run" "$R/proc/1" "$R/proc/self/fd" "$R/proc/sys/fs" \
    "$R/proc/sys/kernel/random" "$R/proc/pressure" "$R/sys/kernel/security" \
    "$R/sys/fs/cgroup" "$R/dev/shm" "$R/dev/pts" "$R/tmp"
chmod 1777 "$R/tmp"
touch "$R/etc/resolv.conf"
if [ "$INIT" = busybox ]; then
    printf '%s\000' /sbin/init >"$R/proc/1/cmdline"
else
    printf '%s\000' /sbin/openrc-init >"$R/proc/1/cmdline"
fi
printf '0::/\n' >"$R/proc/1/cgroup"
printf '0::/\n' >"$R/proc/self/cgroup"
printf 'cpu  1 0 1 100 0 0 0 0 0 0\nbtime 1\nprocesses 1\nprocs_running 1\nprocs_blocked 0\n' >"$R/proc/stat"
printf '1.00 0.00\n' >"$R/proc/uptime"
printf 'MemTotal:        1048576 kB\nMemFree:          524288 kB\nMemAvailable:     524288 kB\n' >"$R/proc/meminfo"
printf 'none / proc rw,nosuid,nodev,noexec,relatime 0 0\n' >"$R/proc/mounts"
printf '0.00 0.00 0.00 1/1 1\n' >"$R/proc/loadavg"
: >"$R/proc/cpuinfo"
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

# AcornOS init: what distro/builder/src/pipeline/overlay.rs writes, with one
# getty on Ginux's console instead of tty1-6 and ttyS0.
cat >"$R/etc/inittab" <<'EOF'
::sysinit:/sbin/openrc sysinit
::sysinit:/sbin/openrc boot
::wait:/sbin/openrc default
console::respawn:/sbin/getty 38400 console
::ctrlaltdel:/sbin/reboot
::shutdown:/sbin/openrc shutdown
EOF
ln -sf /etc/init.d/networking "$R/etc/runlevels/boot/networking"
ln -sf /etc/init.d/sshd "$R/etc/runlevels/default/sshd"
ln -sf /etc/init.d/dhcpcd "$R/etc/runlevels/default/dhcpcd"
# openrc-init has no inittab; its getty is an agetty service on the console.
if [ "$INIT" = openrc-init ]; then
    ln -sf agetty "$R/etc/init.d/agetty.console"
    ln -sf /etc/init.d/agetty.console "$R/etc/runlevels/default/agetty.console"
fi
printf 'acorn\n' >"$R/etc/hostname"

printf 'root:ginux\n' | chroot "$R" /usr/sbin/chpasswd
chown -R 1000:1000 "$R"
find "$R" -type d -exec chmod u+w {} +
chmod 600 "$R/etc/shadow"
echo "staged $R ($INIT)"
