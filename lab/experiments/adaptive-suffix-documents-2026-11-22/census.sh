#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/$(basename "$d")"
export TEKT_LAB_WORK="$(mktemp -d "$root/lab/target/adaptive-census.XXXXXXXX")"
export TEKT_LAB_TIMING_OUT="$out" TEKT_EXE="$d/candidate.elf" PYTHONDONTWRITEBYTECODE=1
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
printf '%s\n' "$TEKT_LAB_WORK" > "$out/census-work.txt"
TZ=America/Sao_Paulo date -Iseconds > "$out/census-start.txt"
python3 "$d/census.py" > "$out/census.log" 2>&1
python3 "$d/receipts.py"
TZ=America/Sao_Paulo date -Iseconds > "$out/census-end.txt"
