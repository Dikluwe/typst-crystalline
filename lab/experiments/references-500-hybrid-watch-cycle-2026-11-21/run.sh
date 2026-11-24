#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21"
ram="$(mktemp -d /dev/shm/tekt-hybrid-watch-cycle.XXXXXXXX)"
finish() {
    local status=$?
    for name in inputs.json nominal.json.gz timing.json trace.json validation.json; do
        if test -s "$ram/$name"; then cp "$ram/$name" "$out/"; fi
    done
    if test -s "$ram/trace/hybrid/watch.log"; then cp "$ram/trace/hybrid/watch.log" "$out/hybrid-watch.log"; fi
    rm -rf -- "$ram"
    printf '%s\n' "$status" > "$out/run-exit.txt"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/benchmark-mount.txt"
cp "$d/../references-500-outline-hybrid-2026-11-21/candidate.elf" "$ram/hybrid"
cp "$d/../references-500-watch-physical-2026-11-20/control.elf" "$ram/tekt"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
(cd "$ram" && sha256sum hybrid tekt vanilla) > "$out/binaries.sha256"
(cd "$ram" && sha256sum -c "$d/binaries.expected.sha256")
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/tekt"
cd "$root"
git rev-parse HEAD > "$out/head.txt"
git diff HEAD --stat > "$out/diffstat-before.txt"
git diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.toml Cargo.lock .cargo | sha256sum > "$out/product-before.sha256"
date -Iseconds > "$out/benchmark-start.txt"
python3 "$d/bench.py"
date -Iseconds > "$out/benchmark-end.txt"
git diff HEAD --stat > "$out/diffstat-after.txt"
git diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.toml Cargo.lock .cargo | sha256sum > "$out/product-after.sha256"
cmp "$out/diffstat-before.txt" "$out/diffstat-after.txt"
cmp "$out/product-before.sha256" "$out/product-after.sha256"
