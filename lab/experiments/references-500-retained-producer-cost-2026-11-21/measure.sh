#!/usr/bin/env bash
set -euo pipefail
d=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$d" rev-parse --show-toplevel)
results="$root/lab/results/construction/diagnostics"
receipts="$results/$(basename "$d")"
out="$receipts${3:+/$3}"
mkdir -p "$out"
stage="$2"
ram=$(mktemp -d /dev/shm/tekt-retained-producer-measure.XXXXXXXX)

finish() {
    local status=$?
    cat /proc/loadavg > "$out/$stage-load-after.txt"
    date -Iseconds > "$out/$stage-end.txt"
    printf '%s\n' "$status" > "$out/$stage-exit.txt"
    git -C "$root" diff HEAD --stat > "$out/diffstat-after.txt"
    git -C "$root" diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.toml Cargo.lock .cargo | sha256sum > "$out/product-after.sha256"
    rm -rf -- "$ram"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/$stage-mount.txt"
cp "$1"/{reuse,proof} "$ram/"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
(
    cd "$ram"
    sha256sum -c "$receipts/retained-elf.sha256"
    sha256sum -c "$receipts/no-producers-elf.sha256"
    awk '$2=="vanilla"' "$results/references-500-context-index-retention-2026-11-21/binaries.sha256" | sha256sum -c -
    sha256sum reuse proof vanilla > "$out/binaries.sha256"
)
export TEKT_LAB_WORK="$ram" TEKT_CONTEXT_OUT="$out" TEKT_EXE="$ram/reuse" PYTHONDONTWRITEBYTECODE=1
export TEKT_VISIBLE_GOLD="$results/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
date -Iseconds > "$out/$stage-start.txt"
cat /proc/loadavg > "$out/$stage-load-before.txt"
python3 -B "$d/bench.py" "$stage"
cmp "$receipts/diffstat-before.txt" <(git -C "$root" diff HEAD --stat)
