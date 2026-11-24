#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-visible-edit-one-root-2026-11-21"
ram="$(mktemp -d /dev/shm/tekt-visible-one-root-census.XXXXXXXX)"
finish() {
    local code=$?
    for arm in candidate hybrid; do
        if test -f "$ram/census/$arm/gdb.log"; then cp "$ram/census/$arm/gdb.log" "$out/$arm-gdb.log"; fi
    done
    if test -s "$ram/census.json"; then cp "$ram/census.json" "$out/"; fi
    rm -rf -- "$ram"
    exit "$code"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
cp "$d/candidate.elf" "$ram/candidate"
cp "$d/../references-500-outline-hybrid-2026-11-21/candidate.elf" "$ram/hybrid"
test "$(sha256sum "$ram/candidate" | cut -d ' ' -f 1)" = "$(cat "$d/candidate.sha256")"
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/hybrid"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
date -Iseconds > "$out/census-start.txt"
python3 "$d/census.py"
date -Iseconds > "$out/census-end.txt"
