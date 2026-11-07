#!/bin/sh
# Boot an AcornOS or LevitateOS rootfs with its real init under Ginux and check
# how far it gets. See README.md.
#
#   ginux-boot.sh acorn|levitate --rootfs DIR --ginux DIR [--timeout SECONDS] [--keep]
#
# --rootfs  the OS's root filesystem tree; it is copied, never changed.
# --ginux   a Ginux checkout (LevitateOS/ginux, branch levitate/boot); built here.
# --keep    keep the work directory (staged root, console, audit) for debugging.
set -eu

usage() {
    echo "usage: $0 acorn|levitate --rootfs DIR --ginux DIR [--timeout SECONDS] [--keep]" >&2
    exit 2
}

[ $# -ge 1 ] || usage
os=$1
shift
case $os in acorn | levitate) ;; *) usage ;; esac
rootfs= ginux_dir= timeout=300 keep=
while [ $# -gt 0 ]; do
    case $1 in
    --rootfs) rootfs=${2:?}; shift 2 ;;
    --ginux) ginux_dir=${2:?}; shift 2 ;;
    --timeout) timeout=${2:?}; shift 2 ;;
    --keep) keep=1; shift ;;
    *) usage ;;
    esac
done
[ -n "$rootfs" ] && [ -n "$ginux_dir" ] || usage
here=$(cd "$(dirname "$0")" && pwd)
rootfs=$(cd "$rootfs" && pwd)
ginux_dir=$(cd "$ginux_dir" && pwd)
[ -x "$rootfs/sbin/init" ] || [ -L "$rootfs/sbin/init" ] ||
    { echo "ginux-boot: $rootfs has no /sbin/init" >&2; exit 2; }
for tool in sudo script cargo; do
    command -v $tool >/dev/null || { echo "ginux-boot: needs $tool" >&2; exit 2; }
done

echo "ginux-boot: building Ginux in $ginux_dir ($(git -C "$ginux_dir" rev-parse --short HEAD))"
(cd "$ginux_dir" && cargo build -q -p ginux-init --target x86_64-unknown-linux-gnu --release)
ginux=$ginux_dir/target/x86_64-unknown-linux-gnu/release/ginux

work=$(mktemp -d "${TMPDIR:-/var/tmp}/ginux-boot-$os.XXXXXX")
root=$work/root
cleanup() {
    sudo pkill -f "^$ginux native $here/$os/instance.toml $root" 2>/dev/null || true
    if [ -z "$keep" ]; then sudo rm -rf "$work"; else echo "ginux-boot: kept $work"; fi
}
trap cleanup EXIT INT TERM

echo "ginux-boot: staging $rootfs"
sudo cp -a "$rootfs" "$root"
sudo sh "$here/$os/stage.sh" "$root" "$ginux_dir"

# The console needs a terminal (systemd checks it); script(1) provides one.
echo "ginux-boot: booting $os (timeout ${timeout}s)"
start=$(date +%s)
script -qefc "sudo timeout $timeout $ginux native $here/$os/instance.toml $root 2>$work/audit.log" \
    "$work/console.log" >/dev/null </dev/null &
booted=$!
while kill -0 $booted 2>/dev/null && ! sudo test -e "$root/ginux-probe.out"; do
    sleep 1
done
elapsed=$(($(date +%s) - start))
sudo pkill -f "^$ginux native $here/$os/instance.toml $root" 2>/dev/null || true
wait $booted 2>/dev/null || true
if ! sudo test -e "$root/ginux-probe.out"; then
    echo "FAIL: $os never reached the probe in ${elapsed}s; last console lines:"
    tr -d '\r' <"$work/console.log" | tail -15
    keep=1
    exit 1
fi
sudo cat "$root/ginux-probe.out" >"$work/probe.out"
echo "ginux-boot: probe after ${elapsed}s"

# Judge: init finished, and everything not working is a known gap.
gaps=$(awk '$1 == "gap" { print $2 }' "$here/$os/expect")
is_gap() { echo "$gaps" | grep -qxF "$1"; }
# not_working: must be a known gap. working: a known gap listed here is fixed.
if [ "$os" = acorn ]; then
    grep -qx 'softlevel default' "$work/probe.out" && init_ok=1 || init_ok=
    init_what="OpenRC default runlevel"
    not_working=$(awk '$1 == "service" && $3 != "started" { print $2 }' "$work/probe.out" | sort -u)
    working=$(awk '$1 == "service" && $3 == "started" { print $2 }' "$work/probe.out" | sort -u)
else
    grep -qx 'unit multi-user.target active active' "$work/probe.out" && init_ok=1 || init_ok=
    init_what="systemd multi-user.target"
    not_working=$(awk '$1 == "unit" && $3 == "failed" { print $2 }' "$work/probe.out" | sort -u)
    working=$(awk '$1 == "unit" && $3 == "active" { print $2 }' "$work/probe.out" | sort -u)
fi

status=0
if [ -n "$init_ok" ]; then echo "PASS: $init_what reached"; else echo "FAIL: $init_what not reached"; status=1; fi
if [ "$os" = acorn ]; then
    echo "started: $(echo $working)"
else
    echo "running: $(awk '$1 == "unit" && $4 == "running" && $2 ~ /\.service$/ { printf "%s ", $2 }' "$work/probe.out")"
fi
for unit in $not_working; do
    if is_gap "$unit"; then
        echo "known gap: $unit"
    else
        echo "FAIL: $unit is not working and is not a known gap"
        status=1
    fi
done
for gap in $gaps; do
    echo "$working" | grep -qxF "$gap" &&
        echo "NOTE: known gap $gap now works; remove it from $os/expect"
done
[ $status -eq 0 ] && echo "PASS: $os boots under Ginux (${elapsed}s, $(echo "$gaps" | wc -l) known gaps)"
[ $status -eq 0 ] || keep=1
exit $status
