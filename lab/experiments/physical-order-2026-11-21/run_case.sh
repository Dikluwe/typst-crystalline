#!/usr/bin/env bash
set -euo pipefail
lab="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$lab/../../.." && pwd)"
label="$1"
binary="$2"
mode="${3:-nominal}"
work="$root/lab/target/physical-order-2026-11-21/runs/$label"
result="$root/lab/results/construction/diagnostics/physical-order-2026-11-21/$label"
mkdir -p "$work" "$result"
cp --reflink=auto "$binary" "$work/candidate"
cp --reflink=auto "$root/lab/typst-original/target/release/typst" "$work/vanilla"
test "$(sha256sum "$work/vanilla" | cut -d ' ' -f1)" = e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b
sha256sum "$work/candidate" "$work/vanilla" > "$result/binaries.sha256"
export TEKT_LAB_WORK="$work" TEKT_RESULT_DIR="$result" PYTHONDONTWRITEBYTECODE=1
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
date -Iseconds > "$result/start.txt"
python3 "$lab/observe.py" validate > "$result/validation.log" 2>&1
if [[ "$mode" != validation ]]; then
    python3 "$lab/observe.py" nominal > "$result/nominal.log" 2>&1
fi
if [[ "$mode" == census ]]; then
    nm -n -S -C --defined-only "$work/candidate" > "$result/tekt.symbols"
    cp "$root/lab/results/construction/diagnostics/references-500-published-7c912bd-2026-11-21/vanilla.symbols" "$result/vanilla.symbols"
    python3 "$lab/census.py" > "$result/census.log" 2>&1
fi
date -Iseconds > "$result/end.txt"
