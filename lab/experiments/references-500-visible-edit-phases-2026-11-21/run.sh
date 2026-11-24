#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-visible-edit-phases-2026-11-21"
ram="$(mktemp -d /dev/shm/tekt-visible-phases.XXXXXXXX)"
finish() {
    local code=$?
    for f in "$ram"/measurements.jsonl "$ram"/*.symbols; do
        if test -f "$f"; then cp "$f" "$out/"; fi
    done
    if test -d "$ram/cases"; then
        (cd "$ram" && find cases -type f \( -name samples.bin -o -name maps -o -name watch.log \) -print0 |
            tar --null -T - -czf "$out/profiles.tar.gz")
    fi
    rm -rf -- "$ram"
    printf '%s\n' "$code" > "$out/run-exit.txt"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/mount.txt"
cp "$d/../references-500-outline-hybrid-2026-11-21/candidate.elf" "$ram/hybrid"
cp "$d/../references-500-watch-physical-2026-11-20/control.elf" "$ram/tekt"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
cp "$d/../unicode-pdf-cost-2026-11-19/profiler.so" "$ram/profiler.so"
(cd "$ram" && sha256sum hybrid tekt vanilla profiler.so) > "$out/binaries.sha256"
(cd "$ram" && sha256sum -c "$d/../references-500-hybrid-watch-cycle-2026-11-21/binaries.expected.sha256")
test "$(awk '$2 == "profiler.so" {print $1}' "$out/binaries.sha256")" = 69a8f4f7a9103d0cc302ffee138990ea744531aa89fa0b89ae6faa85251d1d30
for arm in hybrid tekt vanilla; do nm -n -S -C --defined-only "$ram/$arm" > "$ram/$arm.symbols"; done
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/tekt"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
cd "$root"
git rev-parse HEAD > "$out/head.txt"
git diff HEAD --stat > "$out/diffstat-before.txt"
git diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.toml Cargo.lock .cargo | sha256sum > "$out/product-before.sha256"
date -Iseconds > "$out/start.txt"
python3 "$d/sample.py"
date -Iseconds > "$out/end.txt"
git diff HEAD --stat > "$out/diffstat-after.txt"
git diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.toml Cargo.lock .cargo | sha256sum > "$out/product-after.sha256"
cmp "$out/diffstat-before.txt" "$out/diffstat-after.txt"
cmp "$out/product-before.sha256" "$out/product-after.sha256"
