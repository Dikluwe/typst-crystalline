#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-outline-checkpoint-2026-11-21"
ram="$(mktemp -d /dev/shm/tekt-outline-checkpoint-run.XXXXXXXX)"
finish() {
    local status=$?
    for name in census directed grid; do
        if test -d "$ram/$name"; then
            mkdir -p "$out/$name"
            (cd "$ram/$name" && find . -type f \( -name '*.log' -o -name '*.typ' \) -print0 | tar --null -T - -cf -) |
                tar -C "$out/$name" -xf -
        fi
    done
    if test "$status" != 0; then
        mkdir -p "$out/failed-pdfs"
        for name in census directed grid; do
            if test -d "$ram/$name"; then
                (cd "$ram" && find "$name" -type f -name '*.pdf' -print0 | tar --null -T - -cf -) |
                    tar -C "$out/failed-pdfs" -xf -
            fi
        done
    fi
    for f in "$ram"/*.json "$ram"/*.json.gz; do
        if test -f "$f"; then cp "$f" "$out/"; fi
    done
    if test -f "$ram/raster/checkpoint/page-01.png"; then
        cp "$ram/raster/checkpoint/page-01.png" "$out/reference-first-page.png"
    fi
    rm -rf -- "$ram"
    exit "$status"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/benchmark-mount.txt"
cp "$d/../references-500-native-opening-estimate-2026-11-21/candidate.elf" "$ram/control"
cp "$d/candidate.elf" "$ram/variant"
cp "$d/../references-500-retained-break-projection-2026-11-21/candidate.elf" "$ram/replay"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
(cd "$ram" && sha256sum control variant replay vanilla) > "$out/binaries.sha256"
(cd "$ram" && awk '$2 != "variant"' "$out/binaries.sha256" |
    diff - <(printf '%s\n' 'f7405d967a01f8ee7233a6411a58bf778a1693dcc214b719aa88993df4e61b5d  control' 'd047875773d4c5f436ff8bd220265b3219efd510de658d1ec125004889b6046a  replay' 'e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b  vanilla'))
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
