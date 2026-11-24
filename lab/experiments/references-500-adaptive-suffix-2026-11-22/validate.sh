#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/$(basename "$d")"
export TEKT_LAB_WORK="$(cat "$out/work.txt")"
export TEKT_LAB_TIMING_OUT="$out" TEKT_EXE="$d/candidate.elf" PYTHONDONTWRITEBYTECODE=1
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
TZ=America/Sao_Paulo date -Iseconds > "$out/validation-start.txt"
python3 "$d/navigation.py"
python3 "$d/dependency.py" > "$out/dependency.log" 2>&1
TZ=America/Sao_Paulo date -Iseconds > "$out/validation-end.txt"
