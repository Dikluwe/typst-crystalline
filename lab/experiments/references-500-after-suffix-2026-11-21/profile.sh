#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-after-suffix-2026-11-21"
ram=/dev/shm/tekt-after-suffix-20261121
mkdir -p "$ram"
cp "$d/../page-three-suffix-memo-replay-2026-11-21/candidate.elf" "$ram/control"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
cp "$d/../unicode-pdf-cost-2026-11-19/profiler.so" "$ram/profiler.so"
(cd "$ram" && sha256sum control vanilla profiler.so) > "$out/profile-binaries.sha256"
test "$(sha256sum "$ram/control" | cut -d ' ' -f1)" = d8967c643f23e1771d3c02f32d6b017b32c4ec6796ec87f15ff6871ee372eac5
test "$(sha256sum "$ram/vanilla" | cut -d ' ' -f1)" = e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b
test "$(sha256sum "$ram/profiler.so" | cut -d ' ' -f1)" = 69a8f4f7a9103d0cc302ffee138990ea744531aa89fa0b89ae6faa85251d1d30
cp "$root/lab/results/construction/diagnostics/page-three-suffix-memo-replay-2026-11-21/candidate.symbols" "$out/control.symbols"
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/control"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
date -Iseconds > "$out/profile-start.txt"
python3 "$d/profile.py" > "$out/profile.log" 2>&1
date -Iseconds > "$out/profile-end.txt"
(cd "$ram" && find profile-cases -type f \( -name maps -o -name samples.bin -o -name watch.log \) -print0 | tar --null -T - -czf "$out/profiles.tar.gz")
