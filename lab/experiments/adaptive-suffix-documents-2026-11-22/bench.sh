#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/$(basename "$d")"
mkdir -p "$out"
export TEKT_LAB_WORK="$(mktemp -d "$root/lab/target/adaptive-bench.XXXXXXXX")"
export TEKT_LAB_TIMING_OUT="$out" TEKT_EXE="$d/candidate.elf" PYTHONDONTWRITEBYTECODE=1
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
printf '%s\n' "$TEKT_LAB_WORK" > "$out/work.txt"
sha256sum "$d/candidate.elf" "$root/lab/typst-original/target/release/typst" > "$out/binaries.sha256"
TZ=America/Sao_Paulo date -Iseconds > "$out/start.txt"
python3 "$d/bench.py" > "$out/bench.log" 2>&1
TZ=America/Sao_Paulo date -Iseconds > "$out/end.txt"
