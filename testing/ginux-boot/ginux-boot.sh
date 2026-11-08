#!/bin/sh
# Boot an AcornOS or LevitateOS rootfs with its real init under Ginux and judge
# its boot health. See README.md.
#
#   ginux-boot.sh acorn|levitate --rootfs DIR --ginux DIR [--timeout SECONDS] [--keep]
#
# --rootfs   the OS's rootfs-source tree; it is copied, never changed.
# --ginux    a Ginux checkout containing $ginux_rev; built here.
# --timeout  seconds the boot may take (default 300); the instance then gets
#            $poweroff_s more to power off.
# --keep     keep the work directory (staged root, console, audit) after a pass;
#            it is always kept after a failure.
set -euf

# The oldest Ginux with the boot fixes these runs need (LevitateOS/ginux,
# branch levitate/boot).
ginux_rev=5cd5522409375a96e6c940afe9b3c69fd1f734f3
poweroff_s=60

usage() {
    echo "usage: $0 acorn|levitate --rootfs DIR --ginux DIR [--timeout SECONDS] [--keep]" >&2
    exit 2
}
die() {
    echo "ginux-boot: $*" >&2
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
case $timeout in '' | *[!0-9]*) die "--timeout must be a number of seconds" ;; esac

for tool in sudo script cargo git timeout mknod awk jq; do
    command -v $tool >/dev/null || die "needs $tool"
done
here=$(cd "$(dirname "$0")" && pwd)
rootfs=$(cd "$rootfs" && pwd)
ginux_dir=$(cd "$ginux_dir" && pwd)

# /sbin/init must resolve inside the rootfs: absolute link targets are taken
# relative to it (directory links such as sbin -> usr/sbin are relative).
init=$rootfs/sbin/init
for _ in 1 2 3 4 5 6 7 8; do
    [ -L "$init" ] || break
    target=$(readlink "$init")
    case $target in /*) init=$rootfs$target ;; *) init=$(dirname "$init")/$target ;; esac
done
[ -f "$init" ] && [ -x "$init" ] || die "$rootfs/sbin/init does not resolve to an executable in the rootfs"

# read_gaps FILE: the names of FILE's `gap NAME REASON` lines, one per line;
# blank and `#` lines are skipped, and any other line is an error.
read_gaps() {
    awk '/^[[:space:]]*(#|$)/ { next }
        $1 == "gap" && NF >= 3 { print $2; next }
        { printf "%s:%d: not `gap NAME REASON`\n", FILENAME, FNR > "/dev/stderr"; bad = 1 }
        END { exit bad }' "$1"
}

gaps=$(read_gaps "$here/$os/expect") || exit 2

head=$(git -C "$ginux_dir" rev-parse HEAD)
git -C "$ginux_dir" merge-base --is-ancestor "$ginux_rev" "$head" ||
    die "$ginux_dir is at $head, which does not contain $ginux_rev (LevitateOS/ginux levitate/boot)"
echo "ginux-boot: building Ginux $head (needs $ginux_rev)"
(cd "$ginux_dir" && cargo build -q -p ginux-init --target x86_64-unknown-linux-gnu --release)
ginux=$ginux_dir/target/x86_64-unknown-linux-gnu/release/ginux

work=$(mktemp -d "${TMPDIR:-/var/tmp}/ginux-boot-$os.XXXXXX")
root=$work/root
out=ginux-probe.json # in the instance root; the probe writes it, and $out.err
# The work directory goes after a pass without --keep and stays otherwise.
cleanup() {
    status=$?
    if [ $status -eq 0 ] && [ -z "$keep" ]; then
        sudo rm -rf "$work" || echo "ginux-boot: could not remove $work" >&2
    else
        echo "ginux-boot: kept $work"
    fi
}
trap cleanup EXIT
trap 'exit 130' INT TERM

echo "ginux-boot: staging $rootfs"
sudo cp -a "$rootfs" "$root"
sudo sh "$here/stage.sh" "$root" "$os" "$timeout" "/$out"

# quote ARG...: the arguments as one sh command line, each single-quoted.
quote() {
    for arg; do
        printf "'%s' " "$(printf '%s' "$arg" | sed "s/'/'\\\\''/g")"
    done
}

# boot: run the instance until the probe powers it off, at most the boot
# timeout plus the poweroff allowance (then TERM, then KILL 10 s later).
# Sets $ended to how it ended. The console needs a terminal (systemd checks
# it), which script(1) provides; Ginux's stderr is its audit log.
boot() {
    limit=$((timeout + poweroff_s))
    echo "ginux-boot: booting $os (boot timeout ${timeout}s, instance limit ${limit}s)"
    start=$(date +%s)
    code=0
    script -qefc "$(quote sudo timeout -k 10 "$limit" "$ginux" native "$here/instance.toml" "$root") 2>$(quote "$work/audit.log")" \
        "$work/console.log" >/dev/null </dev/null || code=$?
    elapsed=$(($(date +%s) - start))
    case $code in
    0) ended="powered off after ${elapsed}s" ;;
    124) ended="still running at ${limit}s; stopped by timeout" ;;
    137) ended="still running at ${limit}s and ignored TERM; killed" ;;
    *) ended="exited with status $code after ${elapsed}s" ;;
    esac
    [ $code -eq 0 ]
}

# in_list WORD LIST...
in_list() {
    word=$1
    shift
    for item; do [ "$item" = "$word" ] && return 0; done
    return 1
}

status=0
boot || { echo "FAIL: the instance $ended"; status=1; }
[ $status -ne 0 ] || echo "ginux-boot: the instance $ended"

# recab's verdict (see probe.sh): finished, the init's state, failed names.
verdict=$work/health.json
if sudo test -e "$root/$out"; then sudo cat "$root/$out" >"$verdict"; else : >"$verdict"; fi
if ! jq -e '(.init | type == "string") and (.state | type == "string")
        and (.finished | type == "boolean") and (.failed | type == "array")' "$verdict" >/dev/null 2>&1; then
    echo "FAIL: no boot-health verdict from the probe"
    if sudo test -e "$root/$out.err"; then
        sudo cat "$root/$out.err" | sed 's/^/  recab: /'
    else
        echo "  (the probe never ran)"
    fi
    echo "last console lines:"
    tr -d '\r' <"$work/console.log" | tail -15
    exit 1
fi
init_system=$(jq -r .init "$verdict")
state=$(jq -r .state "$verdict")
failed=$(jq -r '.failed[]' "$verdict")

if [ "$(jq -r .finished "$verdict")" = true ]; then
    echo "PASS: $init_system startup finished ($state)"
else
    echo "FAIL: $init_system startup did not finish ($state)"
    status=1
fi
# The failed services and the known gaps must be the same set.
for name in $failed; do
    if in_list "$name" $gaps; then
        echo "known gap: $name"
    else
        echo "FAIL: $name failed and is not a known gap in $os/expect"
        status=1
    fi
done
for gap in $gaps; do
    in_list "$gap" $failed && continue
    echo "FAIL: known gap $gap did not fail; remove it from $os/expect"
    status=1
done
[ $status -eq 0 ] || exit 1
set -- $gaps
echo "PASS: $os boots under Ginux (${elapsed}s, $# known gaps)"
