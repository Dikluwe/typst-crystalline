#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-native-opening-estimate-2026-11-21"
ram="$(mktemp -d /dev/shm/tekt-native-estimate-root.XXXXXXXX)"
finish() {
    local status=$?
    for arm in both omit same-off; do
        if test -f "$ram/$arm/gdb.log"; then cp "$ram/$arm/gdb.log" "$out/root-$arm.log"; fi
    done
    for name in root-census root-grid; do
        if test -f "$ram/$name.json"; then cp "$ram/$name.json" "$out/"; fi
    done
    if test -d "$ram/grid"; then
        (cd "$ram" && find grid -name gdb.log -print0 | tar --null -T - -czf "$out/root-grid-logs.tar.gz")
    fi
    rm -rf -- "$ram"
    exit "$status"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
cp "$d/../references-500-one-opening-traversal-2026-11-20/candidate.elf" "$ram/control"
cp "$d/candidate.elf" "$ram/variant"
cp "$out/grid.json" "$ram/grid-input.json"
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1
python3 "$d/root-census.py"
