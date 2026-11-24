#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-one-opening-traversal-2026-11-20"
ram="$(mktemp -d /dev/shm/tekt-one-opening-root.XXXXXXXX)"
finish() {
    local status=$?
    for arm in both omit same-off witness wrong; do
        if test -f "$ram/$arm/gdb.log"; then cp "$ram/$arm/gdb.log" "$out/root-$arm.log"; fi
    done
    if test -f "$ram/root-census.json"; then cp "$ram/root-census.json" "$out/"; fi
    rm -rf -- "$ram"
    exit "$status"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
cp "$d/../references-500-two-opening-traversals-2026-11-20/candidate.elf" "$ram/control"
cp "$d/candidate.elf" "$ram/variant"
cp "$out/witness-input.json" "$ram/"
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1
python3 "$d/root-census.py"
