#!/usr/bin/env bash
set -euo pipefail
d=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$d" rev-parse --show-toplevel)
results="$root/lab/results/construction/diagnostics"
out="$results/$(basename "$d")"
ram=$(mktemp -d /dev/shm/tekt-heading-cost-clock.XXXXXXXX)
finish() {
    local status=$?
    date -Iseconds > "$out/resume-end.txt"
    printf '%s\n' "$status" > "$out/resume-exit.txt"
    if test -f "$ram/measurements.jsonl"; then cp "$ram/measurements.jsonl" "$out/"; fi
    git -C "$root" diff HEAD --stat > "$out/diffstat-after.txt"
    git -C "$root" diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.toml Cargo.lock .cargo | sha256sum > "$out/product-after.sha256"
    rm -rf -- "$ram"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/resume-mount.txt"
cp "$1"/{candidate,bridge,proof} "$ram/"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
(cd "$ram" && sha256sum --check "$out/binaries.sha256")
for arm in candidate bridge proof; do nm -n -S -C --defined-only "$ram/$arm" > "$ram/$arm.symbols"; done
export TEKT_LAB_WORK="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_CONTEXT_OUT="$out" TEKT_EXE="$ram/candidate"
export TEKT_VISIBLE_GOLD="$results/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
date -Iseconds > "$out/resume-start.txt"
python3 -B "$d/bench.py" --clock-only
cmp "$out/diffstat-before.txt" <(git -C "$root" diff HEAD --stat)
