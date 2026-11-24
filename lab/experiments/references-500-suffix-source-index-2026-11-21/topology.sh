#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-suffix-source-index-2026-11-21"
ram=/dev/shm/tekt-suffix-source-index-20261121
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/control"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
if [[ "$1" == control ]]; then
  mkdir -p "$ram"
  cp "$d/../references-500-after-suffix-2026-11-21/candidate.elf" "$ram/control"
  cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
  test "$(sha256sum "$ram/control" | cut -d ' ' -f1)" = 53ca01500b641bcf12aa3db002d4b075e8031aa84b4232c4c34ffc2cbafa796f
  test "$(sha256sum "$ram/vanilla" | cut -d ' ' -f1)" = e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b
  (cd "$ram" && sha256sum control vanilla) > "$out/binaries.sha256"
  nm -n -S -C --defined-only "$ram/control" > "$out/control.symbols"

fi
date -Iseconds > "$out/topology-$1-start.txt"
python3 "$d/topology.py" "$1"
date -Iseconds > "$out/topology-$1-end.txt"
