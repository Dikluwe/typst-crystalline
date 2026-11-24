#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-after-suffix-2026-11-21"
ram=/dev/shm/tekt-after-suffix-20261121
(cd "$ram" && sha256sum --quiet -c "$out/binaries.sha256")
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/candidate"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
export TEKT_LAB_TIMING_OUT="$out/repeat" TEKT_LAB_CASE_DIRECTORY=cases-repeat TEKT_LAB_ORDER_SEED=67261
date -Iseconds > "$out/repeat/bench-start.txt"
cat /proc/loadavg > "$out/repeat/load-start.txt"
python3 "$d/bench.py" > "$out/repeat/benchmark.log" 2>&1
date -Iseconds > "$out/repeat/bench-end.txt"
cat /proc/loadavg > "$out/repeat/load-end.txt"
