#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-visible-eval-2026-11-21"
ram="$(mktemp -d /dev/shm/tekt-eval-capacity.XXXXXXXX)"
finish() {
    for f in capacity-image.json capacity-measurements.jsonl; do
        if test -f "$ram/$f"; then cp "$ram/$f" "$out/"; fi
    done
    if test -d "$ram/cases"; then
        (cd "$ram" && find cases -type f \( -name samples.bin -o -name maps -o -name watch.log \) -print0 |
            tar --null -T - -czf "$out/capacity-profiles.tar.gz")
    fi
    rm -rf -- "$ram"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
cp "$d/../references-500-visible-stable-cids-2026-11-21/candidate.elf" "$ram/candidate"
cp "$d/../unicode-pdf-cost-2026-11-19/profiler.so" "$ram/profiler.so"
test "$(sha256sum "$ram/profiler.so" | cut -d ' ' -f 1)" = 69a8f4f7a9103d0cc302ffee138990ea744531aa89fa0b89ae6faa85251d1d30
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/candidate"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
date -Iseconds > "$out/capacity-start.txt"
python3 "$d/capacity_sample.py"
date -Iseconds > "$out/capacity-end.txt"
