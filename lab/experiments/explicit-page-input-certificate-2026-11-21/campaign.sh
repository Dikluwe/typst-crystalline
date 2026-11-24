#!/usr/bin/env bash
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$here" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/$(basename "$here")"
ram=/dev/shm/tekt-context-preserve-20261121/bench-explicit-pages
bash "$here/setup.sh" smoke > "$out/smoke.log" 2>&1
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/candidate"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
python3 "$here/bench.py" directed > "$out/directed.log" 2>&1
date -Iseconds > "$out/bench-start.txt"
python3 "$here/bench.py" explicit > "$out/explicit.log" 2>&1
python3 "$here/bench.py" references > "$out/references.log" 2>&1
date -Iseconds > "$out/bench-end.txt"
python3 "$here/census.py" > "$out/census.log" 2>&1
tar -czf "$out/controls.tar.gz" -C "$ram" census explicit-smoke explicit-directed explicit-fresh explicit-directed-fresh
