#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-after-suffix-2026-11-21"
ram=/dev/shm/tekt-after-suffix-20261121
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/control"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
if [[ "$1" == control ]]; then
  (cd "$ram" && awk '$2=="control" || $2=="vanilla"' "$out/profile-binaries.sha256" | sha256sum --quiet -c)
  (cd "$ram" && sha256sum control vanilla) > "$out/binaries.sha256"
fi
date -Iseconds > "$out/topology-$1-start.txt"
python3 "$d/topology.py" "$1"
date -Iseconds > "$out/topology-$1-end.txt"
