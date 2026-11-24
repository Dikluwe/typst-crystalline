#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-visible-edit-one-root-2026-11-21"
ram="$(mktemp -d /dev/shm/tekt-visible-one-root-measure.XXXXXXXX)"
finish() {
    local code=$?
    for name in directed.json timing.json nominal.json.gz verified.json; do
        if test -s "$ram/$name"; then cp "$ram/$name" "$out/"; fi
    done
    if test -d "$ram/directed"; then
        (cd "$ram" && find directed -type f -name '*.log' -print0 |
            tar --null -T - -czf "$out/directed-logs.tar.gz")
    fi
    rm -rf -- "$ram"
    printf '%s\n' "$code" > "$out/run-exit.txt"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/benchmark-mount.txt"
cp "$d/candidate.elf" "$ram/candidate"
cp "$d/../references-500-outline-hybrid-2026-11-21/candidate.elf" "$ram/hybrid"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
(cd "$ram" && sha256sum candidate hybrid vanilla) > "$out/binaries.sha256"
test "$(awk '$2=="candidate"{print $1}' "$out/binaries.sha256")" = "$(cat "$d/candidate.sha256")"
awk '$2=="hybrid" || $2=="vanilla"' "$out/binaries.sha256" |
    diff - <(awk '$2=="hybrid" || $2=="vanilla"' "$d/../references-500-hybrid-watch-cycle-2026-11-21/binaries.expected.sha256")
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/hybrid"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
date -Iseconds > "$out/benchmark-start.txt"
python3 "$d/bench.py"
date -Iseconds > "$out/benchmark-end.txt"
cd "$root"
git diff HEAD --stat > "$out/diffstat-after.txt"
git diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.toml Cargo.lock .cargo | sha256sum > "$out/product-after.sha256"
cmp "$out/diffstat-before.txt" "$out/diffstat-after.txt"
cmp "$out/product-before.sha256" "$out/product-after.sha256"
