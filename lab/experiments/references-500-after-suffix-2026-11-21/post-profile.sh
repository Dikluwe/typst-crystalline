#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-after-suffix-2026-11-21"
ram=/dev/shm/tekt-after-suffix-20261121
(cd "$ram" && sha256sum --quiet -c "$out/binaries.sha256")
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/candidate"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
date -Iseconds > "$out/post-profile-start.txt"
python3 "$d/profile.py" --combined > "$out/post-profile.log" 2>&1
date -Iseconds > "$out/post-profile-end.txt"
(cd "$ram" && find post-profile-cases -type f \( -name maps -o -name samples.bin -o -name watch.log \) -print0 | tar --null -T - -czf "$out/post-profiles.tar.gz")
python3 "$d/analyze.py" --combined > "$out/post-profile-summary.log"
