#!/bin/sh
# Stage a copied rootfs for `ginux native`, as root:
#   stage.sh ROOT OS TIMEOUT OUT
# Adds Ginux's runtime fakes and the probe, which OS/hook.sh has the OS's init
# start; the probe gets TIMEOUT and OUT (see probe.sh). Nothing else in the
# rootfs changes.
set -eu
R=$1 OS=$2 TIMEOUT=$3 OUT=$4
HERE=$(dirname "$0")

# Ginux shows the guest no kernel /proc, /sys or /run; these are the files the
# boots read, as Ginux's own Gate 9 staging (prepare_root) writes them, plus
# /proc/filesystems (OpenRC reads it) and a /proc/meminfo that follows
# instance.toml's memory_mib. The systemd paths stay empty for OpenRC.
mem_kib=$(($(awk -F'= *' '/^memory_mib *=/ { print $2 }' "$HERE/instance.toml") * 1024))
cg=$R/sys/fs/cgroup
mkdir -p "$R/run/host" "$R/run/log/journal" "$R/run/systemd/journal" \
    "$R/run/systemd/propagate/.os-release-stage" "$R/proc/1" "$R/proc/self/fd" \
    "$R/proc/sys/fs" "$R/proc/sys/kernel/random" "$R/proc/pressure" \
    "$R/sys/kernel/security" "$cg/systemd" "$cg/init.scope" \
    "$cg/system.slice/system-modprobe.slice/modprobe@configfs.service" \
    "$R/dev/shm" "$R/dev/pts" "$R/tmp" "$R/usr/local/sbin"
chmod 1777 "$R/tmp"
rm -f "$R/run/initctl"
touch "$R/etc/resolv.conf"
printf '%s\000' /sbin/init >"$R/proc/1/cmdline" # instance.toml's init, no args
ln -sfn / "$R/proc/1/root"
printf '0::/\n' >"$R/proc/1/cgroup"
printf '0::/\n' >"$R/proc/self/cgroup"
printf 'cpu  1 0 1 100 0 0 0 0 0 0\nbtime 1\nprocesses 1\nprocs_running 1\nprocs_blocked 0\n' >"$R/proc/stat"
printf '1.00 0.00\n' >"$R/proc/uptime"
printf 'MemTotal: %16d kB\nMemFree: %17d kB\nMemAvailable: %12d kB\n' \
    "$mem_kib" $((mem_kib / 2)) $((mem_kib / 2)) >"$R/proc/meminfo"
printf 'none / proc rw,nosuid,nodev,noexec,relatime 0 0\n' >"$R/proc/mounts"
printf '0.00 0.00 0.00 1/1 1\n' >"$R/proc/loadavg"
: >"$R/proc/cpuinfo"
printf 'nodev\tproc\nnodev\ttmpfs\nnodev\tdevpts\n\text4\n' >"$R/proc/filesystems"
printf 'Ginux userspace kernel 0.1.0 (Linux ABI personality 6.12.0-ginux-abi1; contract gate9-systemd-v1; x86_64)\n' >"$R/proc/version"
printf '9223372036854775807\n' >"$R/proc/sys/fs/file-max"
printf '1048576\n' >"$R/proc/sys/fs/nr_open"
printf '40\n' >"$R/proc/sys/kernel/cap_last_cap"
printf 'ginux\n' >"$R/proc/sys/kernel/hostname" # what Ginux's uname answers
printf 'Linux\n' >"$R/proc/sys/kernel/ostype"
printf '6.12.0-ginux-abi1\n' >"$R/proc/sys/kernel/osrelease"
printf 'Ginux userspace kernel 0.1.0\n' >"$R/proc/sys/kernel/version"
printf '4e45c53e-0e6a-4fb8-9f25-e0f11b40db9a\n' >"$R/proc/sys/kernel/random/boot_id"
printf 'some avg10=0.00 avg60=0.00 avg300=0.00 total=0\n' >"$R/proc/pressure/cpu"
: >"$R/sys/kernel/security/lsm"
printf 'cpu memory pids\n' >"$cg/cgroup.controllers"
touch "$cg/cgroup.procs" "$cg/cgroup.subtree_control" "$cg/init.scope/cgroup.procs"
# A cgroup per slice and per installed systemd service.
for unit in "$cg/system.slice" "$cg/system.slice/system-modprobe.slice" \
    "$cg/system.slice/system-modprobe.slice/modprobe@configfs.service" \
    "$R"/usr/lib/systemd/system/*.service; do
    [ -e "$unit" ] || continue
    case $unit in "$R"/usr/*) unit=$cg/system.slice/${unit##*/} ;; esac
    mkdir -p "$unit"
    touch "$unit/cgroup.procs" "$unit/cgroup.events" "$unit/cgroup.kill"
done
# The device nodes Ginux's [devices] assignments back, always made fresh.
for spec in 'console 5 0 600' 'null 1 3 666' 'zero 1 5 666' 'random 1 8 666' 'urandom 1 9 666'; do
    set -- $spec
    rm -f "$R/dev/$1"
    mknod -m "$4" "$R/dev/$1" c "$2" "$3"
done

install -m 755 "$HERE/probe.sh" "$R/usr/local/sbin/ginux-probe"
sh "$HERE/$OS/hook.sh" "$R" "/usr/local/sbin/ginux-probe $TIMEOUT $OUT"

# Ginux runs root-launched workloads as the host's nobody (65534) and keeps
# the guest's credentials itself, so the tree must be nobody's: owned by it,
# its directories writable, and /etc/shadow (often mode 000) readable by it.
chown -R 65534:65534 "$R"
find "$R" -type d -exec chmod u+w {} +
chmod 600 "$R/etc/shadow"
