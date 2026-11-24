#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-outline-checkpoint-2026-11-21"
ram="$(mktemp -d /dev/shm/tekt-outline-checkpoint-root.XXXXXXXX)"
finish() {
    local status=$?
    for arm in both checkpoint replay; do
        if test -f "$ram/opening/$arm/gdb.log"; then cp "$ram/opening/$arm/gdb.log" "$out/root-$arm.log"; fi
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
cp "$d/../references-500-native-opening-estimate-2026-11-21/candidate.elf" "$ram/control"
cp "$d/candidate.elf" "$ram/variant"
cp "$d/../references-500-retained-break-projection-2026-11-21/candidate.elf" "$ram/replay"
cp "$out/grid.json" "$ram/grid-input.json"
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1
python3 "$d/root-census.py"
