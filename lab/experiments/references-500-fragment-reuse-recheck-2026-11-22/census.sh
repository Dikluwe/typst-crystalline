#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/$(basename "$d")"
export TEKT_LAB_TIMING_OUT="$out" PYTHONDONTWRITEBYTECODE=1
export TEKT_LAB_WORK="$(mktemp -d "$root/lab/target/fragment-census.XXXXXXXX")"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
export TEKT_EXE="$TEKT_LAB_WORK/candidate"
ln -s "$root/lab/experiments/page-three-suffix-memo-replay-2026-11-21/candidate.elf" "$TEKT_LAB_WORK/candidate"
ln -s "$root/lab/experiments/page-three-located-position-remap-2026-11-21/candidate.elf" "$TEKT_LAB_WORK/control"
cp "$root/lab/results/construction/diagnostics/page-three-suffix-memo-replay-2026-11-21/binaries.sha256" "$out/binaries.sha256"
for arm in control candidate; do
  nm -n -S -C --defined-only "$TEKT_LAB_WORK/$arm" > "$out/$arm.symbols"
done
TZ=America/Sao_Paulo date -Iseconds > "$out/census-start.txt"
python3 "$d/census.py" > "$out/census.log" 2>&1
TZ=America/Sao_Paulo date -Iseconds > "$out/census-end.txt"
tar -czf "$out/census-traces.tar.gz" -C "$TEKT_LAB_WORK" census-control census-candidate
