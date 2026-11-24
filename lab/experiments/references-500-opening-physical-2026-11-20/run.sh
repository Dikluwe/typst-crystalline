#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-opening-physical-2026-11-20"
ram="$(mktemp -d /dev/shm/tekt-opening-physical-run.XXXXXXXX)"
finish() {
    local status=$?
    for name in census directed; do
        if test -d "$ram/$name"; then
            mkdir -p "$out/$name"
            (cd "$ram/$name" && find . -type f \( -name '*.log' -o -name '*.typ' \) -print0 | tar --null -T - -cf -) |
                tar -C "$out/$name" -xf -
        fi
    done
    for f in "$ram"/*.json "$ram"/*.json.gz; do
        if test -f "$f"; then cp "$f" "$out/"; fi
    done
    if test -f "$ram/raster/omit/page-01.png"; then
        cp "$ram/raster/omit/page-01.png" "$out/reference-first-page.png"
    fi
    rm -rf -- "$ram"
    exit "$status"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/benchmark-mount.txt"
cp "$d/../references-500-pdf-export-reuse-2026-11-20/candidate.elf" "$ram/control"
cp "$d/candidate.elf" "$ram/variant"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
(cd "$ram" && sha256sum control variant vanilla) > "$out/binaries.sha256"
(cd "$ram" && awk '$2 != "variant"' "$out/binaries.sha256" |
    diff - <(printf '%s\n' '7c592c5993eebd42475a3a77366efe5f0bb102dc9606bc0527564a6152273c94  control'     'e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b  vanilla'))
test "$(awk '$2=="variant"{print $1}' "$out/binaries.sha256")" = "$(cat "$d/candidate.sha256")"
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1
date -Iseconds > "$out/benchmark-start.txt"
python3 "$d/bench.py"
date -Iseconds > "$out/benchmark-end.txt"
cd "$root"
git diff HEAD --stat > "$out/diffstat-after.txt"
git diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.toml Cargo.lock | sha256sum > "$out/product-after.sha256"
cmp "$out/diffstat-before.txt" "$out/diffstat-after.txt"
cmp "$out/product-before.sha256" "$out/product-after.sha256"
