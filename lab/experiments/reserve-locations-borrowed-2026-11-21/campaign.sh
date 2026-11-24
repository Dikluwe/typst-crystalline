#!/usr/bin/env bash
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$here" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/$(basename "$here")"
ram=/dev/shm/tekt-context-preserve-20261121
bash "$here/release.sh" > "$out/build.log" 2>&1
bash "$here/run.sh" > "$out/benchmark.log" 2>&1
export TEKT_LAB_WORK="$ram/bench-reserve" PYTHONDONTWRITEBYTECODE=1
export TMPDIR="$TEKT_LAB_WORK" TEKT_EXE="$TEKT_LAB_WORK/candidate"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
python3 "$here/analyze.py" > "$out/analysis.log" 2>&1
python3 "$here/directed.py" > "$out/directed.log" 2>&1
tar -czf "$out/directed.tar.gz" -C "$TEKT_LAB_WORK" directed-all-states
