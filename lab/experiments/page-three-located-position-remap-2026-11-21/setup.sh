#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/page-three-located-position-remap-2026-11-21"
ram=/dev/shm/tekt-context-preserve-20261121/bench-located-position-remap
mode="${1:-all}"
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/candidate"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
if [[ "$mode" != bench ]]; then
    mkdir -p "$ram"
    cp "$d/candidate.elf" "$ram/candidate"
    cp "$d/../page-three-sequence-continuation-2026-11-21/candidate.elf" "$ram/control"
    cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
    test "$(sha256sum "$ram/control" | cut -d ' ' -f1)" = 054e9e8b3912edb805fe5eb893fe9358028bd9a2c97532266e8016e18cc62c3e
    test "$(sha256sum "$ram/vanilla" | cut -d ' ' -f1)" = e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b
    (cd "$ram" && sha256sum candidate control vanilla) > "$out/binaries.sha256"
    for arm in candidate control; do nm -n -S -C --defined-only "$ram/$arm" > "$out/$arm.symbols"; done
    python3 "$d/controls.py" > "$out/controls.log" 2>&1
    tar -czf "$out/controls.tar.gz" -C "$ram" census-"candidate" census-"control" directed-census
fi
if [[ "$mode" != controls ]]; then
    date -Iseconds > "$out/bench-start.txt"
    python3 "$d/bench.py" > "$out/benchmark.log" 2>&1
    date -Iseconds > "$out/bench-end.txt"
fi
