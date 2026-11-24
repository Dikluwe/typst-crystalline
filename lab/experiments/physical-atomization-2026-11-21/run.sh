#!/usr/bin/env bash
set -euo pipefail
root=/repos/Antigravity/typst-crystalline
lab="$root/lab/experiments/physical-order-2026-11-21"
here="$root/lab/experiments/physical-atomization-2026-11-21"
work="$root/lab/target/physical-atomization-2026-11-21/run"
result="$root/lab/results/construction/diagnostics/physical-atomization-2026-11-21"
mkdir -p "$work" "$result"
cp --reflink=auto "$here/candidate-binary" "$work/candidate"
cp --reflink=auto "$root/lab/typst-original/target/release/typst" "$work/vanilla"
test "$(sha256sum "$work/vanilla" | cut -d ' ' -f1)" = e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b
sha256sum "$work/candidate" "$work/vanilla" "$lab/reuse-binary" > "$result/binaries.sha256"
export TEKT_LAB_WORK="$work" TEKT_RESULT_DIR="$result" PYTHONDONTWRITEBYTECODE=1
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
export TEKT_PUBLISHED_EXE="$lab/reuse-binary"
date -Iseconds > "$result/start.txt"
python3 "$lab/observe.py" validate > "$result/validation.log" 2>&1
python3 "$lab/directed.py" > "$result/directed.log" 2>&1
python3 "$lab/analyze_directed.py" > "$result/analyze-directed.log" 2>&1
TEKT_PAIRS=20 python3 "$lab/pair_sources.py" > "$result/paired.log" 2>&1
python3 "$lab/observe.py" nominal > "$result/nominal.log" 2>&1
date -Iseconds > "$result/end.txt"
