#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-visible-edit-phases-2026-11-21"
ram="$(mktemp -d /dev/shm/tekt-visible-census.XXXXXXXX)"
finish() {
    local code=$?
    for arm in hybrid tekt hybrid-tail-control; do
        for file in events.json gdb.log addresses.json; do
            if test -f "$ram/census/$arm/$file"; then cp "$ram/census/$arm/$file" "$out/$arm-$file"; fi
        done
    done
    rm -rf -- "$ram"
    exit "$code"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
cp "$d/../references-500-outline-hybrid-2026-11-21/candidate.elf" "$ram/hybrid"
cp "$d/../references-500-watch-physical-2026-11-20/control.elf" "$ram/tekt"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
(cd "$ram" && sha256sum -c "$d/../references-500-hybrid-watch-cycle-2026-11-21/binaries.expected.sha256")
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/tekt"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
date -Iseconds > "$out/census-start.txt"
python3 "$d/census.py"
date -Iseconds > "$out/census-end.txt"
