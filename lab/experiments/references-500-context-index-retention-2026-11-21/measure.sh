#!/usr/bin/env bash
set -euo pipefail
d=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$d" rev-parse --show-toplevel)
results="$root/lab/results/construction/diagnostics"
out="$results/$(basename "$d")"
stage="$2"
ram=$(mktemp -d /dev/shm/tekt-context-index-measure.XXXXXXXX)
finish() {
    local status=$?
    date -Iseconds > "$out/$stage-end.txt"
    printf '%s\n' "$status" > "$out/$stage-exit.txt"
    git -C "$root" diff HEAD --stat > "$out/diffstat-after.txt"
    git -C "$root" diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.toml Cargo.lock .cargo | sha256sum > "$out/product-after.sha256"
    rm -rf -- "$ram"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/$stage-mount.txt"
cmp "$out/binaries.sha256" "$out/validated-binaries.sha256"
cp "$1"/{candidate,reuse,vanilla} "$ram/"
(cd "$ram" && sha256sum --check "$out/binaries.sha256")
export TEKT_LAB_WORK="$ram" TEKT_CONTEXT_OUT="$out" TEKT_EXE="$ram/candidate" PYTHONDONTWRITEBYTECODE=1
export TEKT_VISIBLE_GOLD="$results/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
date -Iseconds > "$out/$stage-start.txt"
python3 -B "$d/bench.py" "--$stage"
cmp "$out/diffstat-before.txt" <(git -C "$root" diff HEAD --stat)
