#!/usr/bin/env bash
set -euo pipefail
e="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$e" rev-parse --show-toplevel)"
o="$root/lab/results/construction/diagnostics/$(basename "$e")"
export TEKT_LAB_WORK=/dev/shm/tekt-context-preserve-20261121/bench-auto-checkpoint
mkdir -p "$TEKT_LAB_WORK"
cp "$e/../explicit-page-input-certificate-2026-11-21/candidate.elf" "$TEKT_LAB_WORK/control"
export TMPDIR="$TEKT_LAB_WORK" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$TEKT_LAB_WORK/control"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
cp "$root/lab/typst-original/target/release/typst" "$TEKT_LAB_WORK/vanilla"
test "$(sha256sum "$TEKT_LAB_WORK/vanilla" | cut -d' ' -f1)" = e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b
(cd "$TEKT_LAB_WORK" && sha256sum control vanilla) > "$o/binaries.sha256"
date -Iseconds > "$o/checkpoint-start.txt"
for case in original height reference; do
    python3 "$e/frontier.py" "$case"
done
date -Iseconds > "$o/checkpoint-end.txt"
python3 "$e/outputs.py"
date -Iseconds > "$o/bench-start.txt"
python3 "$e/bench.py"
date -Iseconds > "$o/bench-end.txt"
tar -czf "$o/controls.tar.gz" -C "$TEKT_LAB_WORK" checked-original checked-height checked-reference cold-controls
