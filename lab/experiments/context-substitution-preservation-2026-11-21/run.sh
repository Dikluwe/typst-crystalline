#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/context-substitution-preservation-2026-11-21"
ram=/dev/shm/tekt-context-preserve-20261121/bench
mkdir -p "$ram"
cp "$d/candidate.elf" "$ram/candidate"
cp "$d/../references-500-visible-textual-style-collapse-2026-11-21/candidate.elf" "$ram/control"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
test "$(sha256sum "$ram/control" | cut -d ' ' -f1)" = c2262d461c8ff488f1bc10feed4baffcfa0107b4cdeb54c39cb6d2568b7ff564
test "$(sha256sum "$ram/vanilla" | cut -d ' ' -f1)" = e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b
(cd "$ram" && sha256sum candidate control vanilla) > "$out/binaries.sha256"
for arm in candidate control; do nm -n -S -C --defined-only "$ram/$arm" > "$out/$arm.symbols"; done
cp "$d/../unicode-pdf-cost-2026-11-19/profiler.so" "$ram/profiler.so"
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/candidate"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
date -Iseconds > "$out/bench-start.txt"
python3 "$d/bench.py"
date -Iseconds > "$out/bench-end.txt"
cp "$ram/measurements.jsonl" "$out/"
(cd "$ram" && find cases -type f \( -name '*.log' -o -name maps -o -name samples.bin \) -print0 | tar --null -T - -czf "$out/profiles.tar.gz")
