#!/usr/bin/env bash
set -euo pipefail
d="$(cd "$(dirname "$0")" && pwd)"
root="$(git -C "$d" rev-parse --show-toplevel)"
out="$root/lab/results/construction/diagnostics/references-500-visible-page-fonts-2026-11-21"
ram="$(mktemp -d /dev/shm/tekt-visible-page-fonts-measure.XXXXXXXX)"
finish() {
    local code=$?
    for name in directed.json timing.json nominal.json.gz verified.json operators.json fonts.json render-validation.json measurements.jsonl candidate.symbols hybrid.symbols vanilla.symbols; do
        if test -s "$ram/$name"; then cp "$ram/$name" "$out/"; fi
    done
    local dirs=(directed)
    if test -d "$ram/fresh-verification"; then dirs+=(fresh-verification); fi
    if test -d "$ram/directed"; then
        (cd "$ram" && find "${dirs[@]}" -type f -name '*.log' -print0 |
            tar --null -T - -czf "$out/directed-logs.tar.gz")
    fi
    if test -d "$ram/cases"; then
        (cd "$ram" && find cases -type f \( -name samples.bin -o -name maps -o -name watch.log \) -print0 |
            tar --null -T - -czf "$out/profiles.tar.gz")
    fi
    if test "$code" != 0 && test -d "$ram/directed"; then
        (cd "$ram" && find "${dirs[@]}" -type f -name '*.pdf' -print0 |
            tar --null -T - -czf "$out/failed-pdfs.tar.gz")
    fi
    rm -rf -- "$ram"
    printf '%s\n' "$code" > "$out/run-exit.txt"
}
trap finish EXIT
test "$(findmnt -n -o FSTYPE -T "$ram")" = tmpfs
findmnt -T "$ram" > "$out/benchmark-mount.txt"
cp "$d/candidate.elf" "$ram/candidate"
cp "$d/../references-500-visible-page-operators-2026-11-21/candidate.elf" "$ram/hybrid"
cp "$root/lab/typst-original/target/release/typst" "$ram/vanilla"
(cd "$ram" && sha256sum candidate hybrid vanilla) > "$out/binaries.sha256"
test "$(awk '$2=="candidate"{print $1}' "$out/binaries.sha256")" = "$(cat "$d/candidate.sha256")"
test "$(awk '$2=="hybrid"{print $1}' "$out/binaries.sha256")" = 97e1bd15c908c5e7bac422b2b5c374dceb9d27420e72f4af034144664d236cd4
test "$(awk '$2=="vanilla"{print $1}' "$out/binaries.sha256")" = e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b
export TEKT_LAB_WORK="$ram" TMPDIR="$ram" PYTHONDONTWRITEBYTECODE=1 TEKT_EXE="$ram/hybrid"
export TEKT_VISIBLE_GOLD="$root/lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"
date -Iseconds > "$out/benchmark-start.txt"
python3 "$d/bench.py"
date -Iseconds > "$out/benchmark-end.txt"
cp "$d/../unicode-pdf-cost-2026-11-19/profiler.so" "$ram/profiler.so"
test "$(sha256sum "$ram/profiler.so" | cut -d ' ' -f1)" = 69a8f4f7a9103d0cc302ffee138990ea744531aa89fa0b89ae6faa85251d1d30
for arm in candidate hybrid vanilla; do
    nm -n -S -C --defined-only "$ram/$arm" > "$ram/$arm.symbols"
done
date -Iseconds > "$out/profile-start.txt"
python3 "$d/profile.py"
date -Iseconds > "$out/profile-end.txt"
cd "$root"
git diff HEAD --stat > "$out/diffstat-after.txt"
git diff HEAD -- 00_nucleo/prompts 01_core 02_shell 03_infra 04_wiring Cargo.toml Cargo.lock .cargo | sha256sum > "$out/product-after.sha256"
cmp "$out/diffstat-before.txt" "$out/diffstat-after.txt"
cmp "$out/product-before.sha256" "$out/product-after.sha256"
