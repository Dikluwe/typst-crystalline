#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-visible-page-operators-2026-11-21"
ram="$(mktemp -d /dev/shm/tekt-visible-page-operators-measure.XXXXXXXX)"
finish() {
    local code=$?
    for name in directed.json timing.json nominal.json.gz verified.json operators.json; do
        if test -s "$ram/$name"; then cp "$ram/$name" "$out/"; fi
    done
    if test -d "$ram/directed"; then
        (cd "$ram" && find directed -type f -name '*.log' -print0 |
            tar --null -T - -czf "$out/directed-logs.tar.gz")
    fi
    if test "$code" != 0 && test -d "$ram/directed"; then
        (cd "$ram" && find directed -type f -name '*.pdf' -print0 |
            tar --null -T - -czf "$out/failed-pdfs.tar.gz")
    fi
    rm -rf -- "$ram"
    printf '%s\n' "$code" > "$out/run-exit.txt"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/benchmark-mount.txt"
cp "$d/candidate.elf" "$ram/candidate"
cp "$d/../references-500-visible-edit-one-root-2026-11-21/candidate.elf" "$ram/hybrid"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
(cd "$ram" && sha256sum candidate hybrid vanilla) > "$out/binaries.sha256"
test "$(awk '$2=="candidate"{print $1}' "$out/binaries.sha256")" = "$(cat "$d/candidate.sha256")"
test "$(awk '$2=="hybrid"{print $1}' "$out/binaries.sha256")" = b28bf1f4d548d23287ecb17ded6b1abbf1d538f2ee95e76b0ac8c1dee9ad7e2f
test "$(awk '$2=="vanilla"{print $1}' "$out/binaries.sha256")" = e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b
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
