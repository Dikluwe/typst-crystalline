#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-origin-reservations-2026-11-21"
ram=/dev/shm/tekt-origin-reservations-20261121
mode="${1:-all}"
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/candidate"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
if [[ "$mode" != bench ]]; then
    mkdir -p "$ram"
    cp "$d/candidate.elf" "$ram/candidate"
    cp "$d/../references-500-source-plan-block-2026-11-21/candidate.elf" "$ram/control"
    cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
    test "$(sha256sum "$ram/control" | cut -d ' ' -f1)" = e03c2a83ee52d76ba5defd380fcda64d9e3b63413e1ee1aea6edfa3c3ef1b95c
    test "$(sha256sum "$ram/vanilla" | cut -d ' ' -f1)" = e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b
    (cd "$ram" && sha256sum candidate control vanilla) > "$out/binaries.sha256"
    for arm in candidate control; do nm -n -S -C --defined-only "$ram/$arm" > "$out/$arm.symbols"; done
    date -Iseconds > "$out/topology-candidate-start.txt"
    python3 "$d/topology.py" candidate
    date -Iseconds > "$out/topology-candidate-end.txt"
    date -Iseconds > "$out/controls-start.txt"
    python3 "$d/controls.py" > "$out/controls.log" 2>&1
    date -Iseconds > "$out/controls-end.txt"
    tar -czf "$out/controls.tar.gz" -C "$ram" directed
fi
if [[ "$mode" != controls ]]; then
    cat /proc/pressure/cpu > "$out/cpu-pressure-start.txt"
    date -Iseconds > "$out/bench-start.txt"
    python3 "$d/bench.py" > "$out/benchmark.log" 2>&1
    date -Iseconds > "$out/bench-end.txt"
    cat /proc/pressure/cpu > "$out/cpu-pressure-end.txt"
fi
