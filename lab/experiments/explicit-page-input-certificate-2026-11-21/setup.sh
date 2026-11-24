#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/explicit-page-input-certificate-2026-11-21"
ram=/dev/shm/tekt-context-preserve-20261121/bench-explicit-pages
mkdir -p "$ram"
cp "$d/candidate.elf" "$ram/candidate"
cp "$d/../physical-paragraph-coverage-2026-11-21/candidate.elf" "$ram/control"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
test "$(sha256sum "$ram/control" | cut -d ' ' -f1)" = e403f924330882bebbdcfd5d4ea36fc582a1bea7c272053417a493c7b35b0fea
test "$(sha256sum "$ram/vanilla" | cut -d ' ' -f1)" = e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b
(cd "$ram" && sha256sum candidate control vanilla) > "$out/binaries.sha256"
for arm in candidate control; do nm -n -S -C --defined-only "$ram/$arm" > "$out/$arm.symbols"; done
cp "$d/../unicode-pdf-cost-2026-11-19/profiler.so" "$ram/profiler.so"
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/candidate"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
python3 "$d/bench.py" "$@"
