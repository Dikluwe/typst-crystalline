#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
old="$root/lab/experiments/page-three-suffix-memo-replay-2026-11-21"
out="$root/lab/results/construction/diagnostics/$(basename "$d")"
export TEKT_LAB_WORK="$(mktemp -d "$root/lab/target/fragment-recheck.XXXXXXXX")"
export TEKT_LAB_TIMING_OUT="$out"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
export PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$TEKT_LAB_WORK/candidate"
printf '%s\n' "$TEKT_LAB_WORK" > "$out/work.txt"
ln -s "$old/candidate.elf" "$TEKT_LAB_WORK/candidate"
ln -s "$old/../page-three-located-position-remap-2026-11-21/candidate.elf" "$TEKT_LAB_WORK/control"
ln -s "$root/lab/typst-original/target/release/typst" "$TEKT_LAB_WORK/vanilla"
(cd "$TEKT_LAB_WORK" && sha256sum -c "$root/lab/results/construction/diagnostics/$(basename "$old")/binaries.sha256") > "$out/binary-verification.log"
TZ=America/Sao_Paulo date -Iseconds > "$out/start.txt"
python3 "$old/bench.py" > "$out/bench.log" 2>&1
TZ=America/Sao_Paulo date -Iseconds > "$out/end.txt"
python3 "$d/directed.py" > "$out/directed.log" 2>&1
python3 "$d/navigation.py" > "$out/navigation.log" 2>&1
